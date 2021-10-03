use clap::Clap;
use std::{
    ops::{Deref, Not},
    path::PathBuf,
    str::FromStr,
};
use strum::VariantNames;

use crate::constants::{AVAILABLE_BAUDRATES, AVAILABLE_REFRESH_RATES};
use crate::errors::{CONVERT_TO_STR_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE};
use crate::log_panel::LogLevel;
use crate::output::CsvLogging;
use crate::shared_state::SharedState;
use crate::types::{FlowControl, RealtimeDelay};
use crate::{
    common_constants::{SbpLogging, Tabs},
    connection::{Connection, ConnectionState},
};

#[derive(Debug)]
pub struct CliLogLevel(LogLevel);

impl Deref for CliLogLevel {
    type Target = LogLevel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CliLogLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CliLogLevel(LogLevel::from_str(s).map_err(|_| {
            format!("Must choose from available tabs {:?}", LogLevel::VARIANTS)
        })?))
    }
}

#[derive(Debug)]
pub struct CliTabs(Tabs);

impl Deref for CliTabs {
    type Target = Tabs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CliTabs {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CliTabs(Tabs::from_str(s).map_err(|_| {
            format!("Must choose from available tabs {:?}", Tabs::VARIANTS)
        })?))
    }
}

#[derive(Debug)]
pub struct CliSbpLogging(SbpLogging);

impl Deref for CliSbpLogging {
    type Target = SbpLogging;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CliSbpLogging {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CliSbpLogging(SbpLogging::from_str(s).map_err(|_| {
            format!("Must choose from available tabs {:?}", SbpLogging::VARIANTS)
        })?))
    }
}

#[derive(Clap, Debug)]
#[clap(name = "swift_navigation_console", about = "Swift Navigation Console.")]
pub struct CliOptions {
    #[clap(subcommand)]
    pub input: Option<Input>,

    /// Exit when connection closes.
    #[clap(long = "exit-after")]
    pub exit_after: bool,

    /// Enable CSV logging.
    #[clap(long = "csv-log")]
    pub csv_log: bool,

    /// Enable SBP-JSON or SBP logging.
    #[clap(long = "sbp-log")]
    pub sbp_log: Option<CliSbpLogging>,

    /// Set Console Log Level Filter. Default: INFO.
    #[clap(long = "log-level")]
    pub log_level: Option<CliLogLevel>,

    /// Set log directory.
    #[clap(long = "log-dirname")]
    pub dirname: Option<String>,

    // Frontend Options
    /// Don't use opengl in plots.
    #[clap(long = "no-opengl", parse(from_flag = Not::not))]
    pub no_opengl: bool,

    /// Change the refresh rate of the plots.
    #[clap(long = "refresh-rate", validator(is_refresh_rate))]
    pub refresh_rate: Option<u8>,

    /// Start console from specific tab.
    #[clap(long = "tab")]
    pub tab: Option<CliTabs>,

    /// Show CSV logging button.
    #[clap(long = "show-csv-log")]
    pub show_csv_log: bool,
}

impl CliOptions {
    /// Get vector of filtered cli arguments.
    /// Primarily needed to prevent backend from thinking .py file is cli arg.
    ///
    /// # Returns
    /// - `filtered_args`: The filtered args parsed via CliOptions.
    pub fn from_filtered_cli() -> CliOptions {
        let args = std::env::args();
        eprintln!("{:?}", args);
        let mut next_args = std::env::args().skip(1);
        let mut filtered_args: Vec<String> = vec![];
        for arg in args {
            if let Some(n_arg) = next_args.next() {
                if (arg.ends_with("python") || arg.ends_with("python.exe") || arg.ends_with("pythonw.exe"))
                    && (n_arg.ends_with(".py") || n_arg.ends_with("swiftnav-console.exe"))
                {
                    continue;
                }
            }
            filtered_args.push(arg);
        }
        eprintln!("{:?}", filtered_args);
        CliOptions::parse_from(filtered_args)
    }
}

#[derive(Clap, Debug)]
#[clap(about = "Input type and corresponding options.")]
pub enum Input {
    Tcp {
        /// The TCP host to connect to.
        host: String,

        /// The port to use when connecting via TCP.
        #[clap(long, default_value = "55555")]
        port: u16,
    },
    Serial {
        /// The serialport to connect to.
        #[clap(parse(from_os_str))]
        serialport: PathBuf,

        /// The baudrate for processing packets.
        #[clap(long, default_value = "115200", validator(is_baudrate))]
        baudrate: u32,

        /// The flow control spec to use.
        #[clap(long = "flow-control", default_value = "None")]
        flow_control: FlowControl,
    },
    File {
        /// Open and run an SBP file.
        #[clap(parse(from_os_str))]
        file_in: PathBuf,
    },
}

impl Input {
    pub fn into_conn(self) -> Connection {
        match self {
            Input::Tcp { host, port } => Connection::tcp(host, port),
            Input::Serial {
                serialport,
                baudrate,
                flow_control,
            } => Connection::serial(serialport.to_string_lossy().into(), baudrate, flow_control),
            Input::File { file_in } => Connection::file(
                file_in.to_string_lossy().into(),
                crate::types::RealtimeDelay::On,
                /*close_when_done=*/ false,
            ),
        }
    }
}

/// Validation for the refresh-rate cli option.
///
/// # Parameters
/// - `rr`: The user input refresh-rate.
///
/// # Returns
/// - `Ok`: The refresh-rate was found in AVAILABLE_REFRESH_RATES.
/// - `Err`: The tab was not found in AVAILABLE_REFRESH_RATES.
fn is_refresh_rate(rr: &str) -> Result<(), String> {
    if let Ok(rr_) = rr.parse::<u8>() {
        if AVAILABLE_REFRESH_RATES.contains(&rr_) {
            return Ok(());
        }
    }
    Err(format!(
        "Must choose from available refresh rates {:?}",
        AVAILABLE_REFRESH_RATES
    ))
}

/// Validation for the baudrate cli option.
///
/// # Parameters
/// - `br`: The user input baudrate.
///
/// # Returns
/// - `Ok`: The baudrate was found in AVAILABLE_BAUDRATES.
/// - `Err`: The tab was not found in AVAILABLE_BAUDRATES.
fn is_baudrate(br: &str) -> Result<(), String> {
    if let Ok(br_) = br.parse::<u32>() {
        if AVAILABLE_BAUDRATES.contains(&br_) {
            return Ok(());
        }
    }
    Err(format!(
        "Must choose from available baudrates {:?}",
        AVAILABLE_BAUDRATES
    ))
}

/// Start connections based on CLI options.
///
/// # Parameters
/// - `opt`: CLI Options to start specific connection type.
/// - `connection_state`: The Server state to start a specific connection.
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for validating another connection is not already running.
pub fn handle_cli(opt: CliOptions, connection_state: &ConnectionState, shared_state: SharedState) {
    if let Some(opt_input) = opt.input {
        match opt_input {
            Input::Tcp { host, port } => {
                connection_state.connect_to_host(host, port);
            }
            Input::File { file_in } => {
                let filename = file_in.display().to_string();
                connection_state.connect_to_file(filename, RealtimeDelay::On, opt.exit_after);
            }
            Input::Serial {
                serialport,
                baudrate,
                flow_control,
            } => {
                let serialport = serialport.display().to_string();
                connection_state.connect_to_serial(serialport, baudrate, flow_control);
            }
        }
    }
    if let Some(folder) = opt.dirname {
        shared_state.set_logging_directory(PathBuf::from(folder));
    }
    let log_level = if let Some(log_level_) = opt.log_level {
        (*log_level_).clone()
    } else {
        LogLevel::INFO
    };
    shared_state.set_log_level(log_level);
    let mut shared_data = shared_state.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
    (*shared_data).logging_bar.csv_logging = CsvLogging::from(opt.csv_log);
    if let Some(sbp_log) = opt.sbp_log {
        (*shared_data).logging_bar.sbp_logging =
            SbpLogging::from_str(&sbp_log.to_string()).expect(CONVERT_TO_STR_FAILURE);
    }
    log::logger().flush();
}
