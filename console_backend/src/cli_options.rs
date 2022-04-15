use std::{
    num::ParseIntError,
    ops::{Deref, Not},
    path::PathBuf,
    str::FromStr,
};

use clap::{AppSettings::DeriveDisplayOrder, Args, Parser};
use log::{debug, error};
use strum::VariantNames;

use crate::log_panel::LogLevel;
use crate::output::CsvLogging;
use crate::shared_state::SharedState;
use crate::types::{FlowControl, RealtimeDelay};
use crate::{
    client_sender::BoxedClientSender,
    constants::{AVAILABLE_BAUDRATES, AVAILABLE_REFRESH_RATES},
};
use crate::{
    common_constants::{SbpLogging, Tabs},
    connection::ConnectionManager,
};
use crate::{constants::LOG_FILENAME, errors::CONVERT_TO_STR_FAILURE};

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

#[cfg(windows)]
const BIN_NAME: &str = "swift-console.exe";
#[cfg(not(windows))]
const BIN_NAME: &str = "swift-console";

#[derive(Parser)]
#[clap(
    name = "Swift Console",
    about = "The Swift Console is a Graphic User Interface (GUI) program providing visual representation of what's happening inside the Swift Navigation GNSS receivers. Console displays information and allows to adjust the settings on the hardware.",
    bin_name = BIN_NAME,
    version = include_str!("version.txt"),
    setting = DeriveDisplayOrder,
)]
pub struct CliOptions {
    #[clap(flatten)]
    pub serial: SerialOpts,

    #[clap(flatten)]
    pub tcp: TcpOpts,

    #[clap(flatten)]
    pub file: FileOpts,

    /// Log SBP-JSON or SBP data to default / specified log file.
    #[clap(long)]
    pub sbp_log: Option<CliSbpLogging>,

    /// Set SBP log filename.
    #[clap(long)]
    pub sbp_log_filename: Option<PathBuf>,

    /// Record capnp messages.
    #[clap(long)]
    pub record_capnp_recording: bool,

    /// Read capnp messages from file.
    #[clap(long)]
    pub read_capnp_recording: Option<PathBuf>,

    /// Exit after number of seconds elapses.
    #[clap(long)]
    pub exit_after_secs: Option<f64>,

    /// Set log directory.
    #[clap(long)]
    pub log_dirname: Option<String>,

    /// Create a log file containing console debug information.
    #[clap(long)]
    pub log_console: bool,

    /// Log CSV data to default / specified log file.
    #[clap(long)]
    pub csv_log: bool,

    /// Show CSV logging button.
    #[clap(long)]
    pub show_csv_log: bool,

    /// Show Filio pane in Update tab.
    #[clap(long)]
    pub show_fileio: bool,

    /// Allow File Connections.
    #[clap(long)]
    pub show_file_connection: bool,

    /// Path to a yaml file containing known piski settings.
    #[clap(long)]
    pub settings_yaml: Option<PathBuf>,

    /// Disable antialiasing, images and plots will become optimized for efficiency not aesthetics and
    /// require less system resources.
    #[clap(long, parse(from_flag = Not::not))]
    pub no_antialiasing: bool,

    /// Use OpenGL, plots will become optimized for efficiency not aesthetics and require less system resources.
    #[clap(long, parse(from_flag = Not::not))]
    pub use_opengl: bool,

    /// Disable high dpi autoscaling, fonts and images will become optimized for efficiency not aesthetics and
    /// require less system resources.
    #[clap(long, parse(from_flag = Not::not))]
    pub no_high_dpi: bool,

    /// Change the refresh rate of the plots.
    #[clap(long, validator(is_refresh_rate))]
    pub refresh_rate: Option<u8>,

    /// Don't show prompts about firmware/console updates.
    #[clap(long)]
    pub no_prompts: bool,

    /// Exit when connection closes.
    #[clap(long)]
    pub exit_after: bool,

    /// Start console from specific tab.
    #[clap(long)]
    pub tab: Option<CliTabs>,

    /// Set the height of the main window.
    #[clap(long)]
    pub height: Option<u32>,

    /// Set the width of the main window.
    #[clap(long)]
    pub width: Option<u32>,
}

impl CliOptions {
    /// Get vector of filtered cli arguments.
    /// Primarily needed to prevent backend from thinking .py file is cli arg.
    ///
    /// # Returns
    /// - `filtered_args`: The filtered args parsed via CliOptions.
    pub fn from_filtered_cli() -> CliOptions {
        let args = std::env::args();
        eprintln!("args {:?}", args);
        let mut next_args = std::env::args().skip(1);
        let mut filtered_args: Vec<String> = vec![];
        for arg in args.filter(|a| !matches!(a.as_str(), "swiftnav_console.main" | "-m" | "--")) {
            if let Some(n_arg) = next_args.next() {
                if (arg.ends_with("python")
                    || arg.ends_with("python3")
                    || arg.ends_with("python.exe")
                    || arg.ends_with("pythonw.exe"))
                    && (n_arg.ends_with(".py")
                        || n_arg.ends_with("swift-console.exe")
                        || n_arg.ends_with("swift-console"))
                {
                    continue;
                }
            }
            filtered_args.push(arg);
        }
        debug!("filtered_args: {:?}", &filtered_args[1..]);
        CliOptions::parse_from(filtered_args)
    }
}

#[derive(Args)]
pub struct SerialOpts {
    /// The serialport to connect to.
    #[clap(
        long,
        conflicts_with_all = &["tcp"]
    )]
    pub serial: Option<PathBuf>,

    /// The baudrate for processing packets.
    #[clap(
        long,
        default_value = "115200",
        validator(is_baudrate),
        conflicts_with_all = &["tcp"]
    )]
    pub baudrate: u32,

    /// The flow control spec to use.
    #[clap(
        long,
        default_value = "None",
        conflicts_with_all = &["tcp"]
    )]
    pub flow_control: FlowControl,
}

#[derive(Args)]
pub struct TcpOpts {
    /// The TCP/IP host or TCP/IP host-port pair to connect with. For example: "192.168.0.222" or "192.168.0.222:55555"
    #[clap(
        long,
        conflicts_with_all = &["serial", "baudrate", "flow-control"]
    )]
    pub tcp: Option<HostPort>,
}

pub struct HostPort {
    pub host: String,
    pub port: u16,
}

impl FromStr for HostPort {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find(':') {
            let (host, port) = s.split_at(idx);
            Ok(HostPort {
                host: host.to_owned(),
                port: port[1..].parse()?,
            })
        } else {
            Ok(HostPort {
                host: s.to_owned(),
                port: 55555,
            })
        }
    }
}

#[derive(Args)]
pub struct FileOpts {
    /// Path to an SBP file.
    #[clap(
        long,
        conflicts_with_all = &["tcp", "serial", "baudrate", "flow-control"]
    )]
    pub file: Option<PathBuf>,
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
pub fn is_baudrate(br: &str) -> Result<(), String> {
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
/// - `conn_manager`: The Server state to start a specific connection.
/// - `shared_state`: The shared state for validating another connection is not already running.
/// - `client_sender`: Client Sender channel for communication from backend to frontend.
pub fn handle_cli(
    opt: CliOptions,
    conn_manager: &ConnectionManager,
    shared_state: SharedState,
    client_sender: &BoxedClientSender,
) {
    if let Some(serial) = opt.serial.serial {
        let serialport = serial.display().to_string();
        conn_manager.connect_to_serial(serialport, opt.serial.baudrate, opt.serial.flow_control);
    } else if let Some(tcp) = opt.tcp.tcp {
        if let Err(e) = conn_manager.connect_to_host(tcp.host, tcp.port) {
            error!("Failed to establish tcp connection: {}", e);
        };
    } else if let Some(file) = opt.file.file {
        let filename = file.display().to_string();
        conn_manager.connect_to_file(filename, RealtimeDelay::On, opt.exit_after);
    }
    if let Some(ref path) = opt.settings_yaml {
        sbp_settings::setting::load_from_path(path).expect("failed to load settings");
    }
    if let Some(folder) = opt.log_dirname {
        shared_state.set_logging_directory(PathBuf::from(folder));
    }
    shared_state.lock().logging_bar.csv_logging = CsvLogging::from(opt.csv_log);
    if opt.log_console {
        let filename = chrono::Local::now().format(LOG_FILENAME).to_string().into();
        shared_state.set_log_filename(Some(filename));
    }
    if let Some(path) = opt.sbp_log_filename {
        shared_state.set_sbp_logging_filename(Some(path));
    }
    if let Some(sbp_log) = opt.sbp_log {
        shared_state.set_sbp_logging(true, client_sender.clone());
        shared_state.set_sbp_logging_format(
            SbpLogging::from_str(&sbp_log.to_string()).expect(CONVERT_TO_STR_FAILURE),
        );
    }
    log::logger().flush();
}
