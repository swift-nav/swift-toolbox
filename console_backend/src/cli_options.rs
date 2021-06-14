use clap::Clap;
use std::{
    ops::{Deref, Not},
    path::PathBuf,
    str::FromStr,
};
use strum::VariantNames;

use crate::constants::{AVAILABLE_BAUDRATES, AVAILABLE_REFRESH_RATES};
use crate::types::FlowControl;
use crate::{
    common_constants::{SbpLogging, Tabs},
    types::Connection,
};

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
#[clap(about = "Cli subcommands.")]
pub enum CliCommand {
    /// Open the console.
    Open {
        #[clap(flatten)]
        opts: OpenOptions,
    },

    /// Perform file io operations.
    Fs {
        #[clap(subcommand)]
        opts: FileioCommands,
    },
}

impl CliCommand {
    /// Get vector of filtered cli arguments.
    /// Primarily needed to prevent backend from thinking .py file is cli arg.
    ///
    /// # Returns
    /// - `filtered_args`: The filtered args parsed via CliCommand.
    pub fn from_filtered_cli() -> CliCommand {
        let args = std::env::args();
        let mut next_args = std::env::args().skip(1);
        let mut filtered_args: Vec<String> = vec![];
        for arg in args {
            if let Some(n_arg) = next_args.next() {
                if (arg.ends_with("python") || arg.ends_with("python.exe"))
                    && n_arg.ends_with(".py")
                {
                    continue;
                }
            }
            filtered_args.push(arg);
        }
        CliCommand::parse_from(filtered_args)
    }
}

#[derive(Clap, Debug)]
#[clap(name = "swift_navigation_console", about = "Swift Navigation Console.")]
pub struct OpenOptions {
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

#[derive(Clap, Debug)]
#[clap(about = "Fileio operations.")]
pub enum FileioCommands {
    /// Write a file from local source to remote destination dest.
    Write {
        source: String,
        dest: String,
        #[clap(subcommand)]
        input: Input,
    },

    /// Read a file from remote source to local dest. If no dest is provided, file is read to stdout.
    Read {
        source: String,
        dest: Option<String>,
        #[clap(subcommand)]
        input: Input,
    },

    /// List a directory.
    List {
        path: String,
        #[clap(subcommand)]
        input: Input,
    },

    /// Delete a file.
    Delete {
        path: String,
        #[clap(subcommand)]
        input: Input,
    },
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
    pub fn into_conn(self) -> crate::types::Result<Connection> {
        match self {
            Input::Tcp { host, port } => Connection::tcp(host, port),
            Input::Serial {
                serialport,
                baudrate,
                flow_control,
            } => Connection::serial(serialport.to_string_lossy().into(), baudrate, flow_control),
            Input::File { file_in } => Connection::file(file_in.to_string_lossy().into()),
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
