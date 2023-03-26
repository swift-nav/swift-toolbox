// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{num::ParseIntError, path::PathBuf, str::FromStr};

use clap::{ArgAction, Args, Parser};
use log::{debug, error};

use crate::common_constants::LogLevel;
use crate::constants::AVAILABLE_BAUDRATES;
use crate::output::CsvLogging;
use crate::shared_state::SharedState;
use crate::types::{FlowControl, RealtimeDelay};
use crate::{
    common_constants::{SbpLogging, Tabs},
    connection::ConnectionManager,
};
use crate::{constants::LOG_FILENAME, errors::CONVERT_TO_STR_FAILURE};
use strum::VariantNames;

#[cfg(windows)]
const BIN_NAME: &str = "swift-console.exe";
#[cfg(not(windows))]
const BIN_NAME: &str = "swift-console";

#[derive(Parser)]
#[clap(
    name = "Swift Console",
    about = "The Swift Console provides data visualization, settings management, and firmware update capabilities for Swift Navigation GNSS products.",
    bin_name = BIN_NAME,
    version = include_str!("version.txt"),
)]
pub struct CliOptions {
    #[clap(flatten)]
    pub serial: SerialOpts,

    #[clap(flatten)]
    pub tcp: TcpOpts,

    #[clap(flatten)]
    pub file: FileOpts,

    /// Log SBP_JSON or SBP data to default / specified log file.
    #[clap(long, value_parser = sbp_logger)]
    pub sbp_log: Option<SbpLogging>,

    /// Set SBP log filename.
    #[clap(long)]
    pub sbp_log_filename: Option<PathBuf>,

    /// Record capnp messages.
    #[clap(long, hide = true)]
    pub record_capnp_recording: bool,

    /// Read capnp messages from file.
    #[clap(long, hide = true)]
    pub read_capnp_recording: Option<PathBuf>,

    /// Run application without the backend. Useful for debugging.
    /// This mode must be run with a capnp recording file.
    #[clap(long, requires = "read_capnp_recording", hide = true)]
    pub debug_with_no_backend: bool,

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

    /// Disable map.
    #[clap(long)]
    pub disable_map: bool,

    /// Path to a yaml file containing known piksi settings.
    #[clap(long)]
    pub settings_yaml: Option<PathBuf>,

    /// Disable antialiasing, images and plots will become optimized for efficiency not aesthetics and
    /// require less system resources.
    #[clap(long, action = ArgAction::SetFalse)]
    pub no_antialiasing: bool,

    /// Use OpenGL, plots will become optimized for efficiency not aesthetics and require less system resources.
    #[clap(long, action = ArgAction::SetFalse)]
    pub use_opengl: bool,

    /// Disable high dpi autoscaling, fonts and images will become optimized for efficiency not aesthetics and
    /// require less system resources.
    #[clap(long, action = ArgAction::SetFalse)]
    pub no_high_dpi: bool,

    /// Don't show prompts about firmware/console updates.
    #[clap(long)]
    pub no_prompts: bool,

    /// Exit when file connection closes.
    #[clap(long)]
    pub exit_after_close: bool,

    /// Set the number of seconds after which the app automatically quits.
    #[clap(long)]
    pub exit_after_timeout: Option<f64>,

    /// Start console from specific tab.
    #[clap(long, value_parser = tabs)]
    pub tab: Option<Tabs>,

    /// Set the height of the main window.
    #[clap(long)]
    pub height: Option<u32>,

    /// Set the width of the main window.
    #[clap(long)]
    pub width: Option<u32>,

    /// Enable QML Debugging and profiling.
    #[clap(long, hide = true)]
    pub qmldebug: bool,

    /// SSH tunnel to a jumphost specified (`[username]:[password]@some.fqdn`)
    #[clap(long, hide = true)]
    pub ssh_tunnel: Option<PathBuf>,

    /// SSH tunnel forward port of remote IP and port to localhost (some.fqdn:port)
    #[clap(long, hide = true)]
    pub ssh_remote_bind_address: Option<PathBuf>,
}

impl CliOptions {
    /// Get vector of filtered cli arguments.
    /// Primarily needed to prevent backend from thinking .py file is cli arg.
    ///
    /// # Returns
    /// - `filtered_args`: The filtered args parsed via CliOptions.
    pub fn from_filtered_cli() -> CliOptions {
        let args = std::env::args();
        let mut next_args = std::env::args().skip(1);
        let mut filtered_args: Vec<String> = vec![];
        for arg in args.filter(|a| {
            !matches!(
                a.as_str(),
                "cProfile" | "swiftnav_console.main" | "-m" | "--"
            )
        }) {
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
    #[clap(long, conflicts_with_all = &["tcp"])]
    pub serial: Option<PathBuf>,

    /// The baudrate for processing packets when connecting via serial.
    #[clap(
        long,
        default_value = "115200",
        value_parser = is_baudrate,
        conflicts_with_all = &["tcp"]
    )]
    pub baudrate: u32,

    /// The flow control spec to use when connecting via serial.
    #[clap(long, default_value = "None", conflicts_with_all = &["tcp"])]
    pub flow_control: FlowControl,
}

#[derive(Args)]
pub struct TcpOpts {
    /// The TCP/IP host or TCP/IP host-port pair to connect with. For example: "192.168.0.222" or "192.168.0.222:55555"
    #[clap(long, conflicts_with_all = &["serial", "baudrate", "flow_control"])]
    pub tcp: Option<HostPort>,
}

#[derive(Clone)]
pub struct HostPort {
    pub host: String,
    pub port: u16,
}

impl FromStr for HostPort {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (host, port) = if let Some((host, port)) = s.split_once(':') {
            (host.to_owned(), port.parse::<u16>()?)
        } else {
            (s.to_owned(), 55555)
        };

        Ok(HostPort { host, port })
    }
}

#[derive(Args)]
pub struct FileOpts {
    /// Path to an SBP file.
    #[clap(long, conflicts_with_all = &["tcp", "serial", "baudrate", "flow_control"])]
    pub file: Option<PathBuf>,
}

/// Validation for the baudrate cli option.
///
/// # Parameters
/// - `br`: The user input baudrate.
///
/// # Returns
/// - `Ok`: The baudrate was found in AVAILABLE_BAUDRATES.
/// - `Err`: The tab was not found in AVAILABLE_BAUDRATES.
pub fn is_baudrate(br: &str) -> Result<u32, String> {
    if let Ok(br_) = br.parse::<u32>() {
        if AVAILABLE_BAUDRATES.contains(&br_) {
            return Ok(br_);
        }
    }
    Err(format!("possible values: {AVAILABLE_BAUDRATES:?}"))
}

pub fn sbp_logger(s: &str) -> Result<SbpLogging, String> {
    SbpLogging::from_str(s).map_err(|_| format!("possible values: {:?}", SbpLogging::VARIANTS))
}

pub fn tabs(s: &str) -> Result<Tabs, String> {
    Tabs::from_str(s).map_err(|_| format!("possible values: {:?}", Tabs::VARIANTS))
}

pub fn log_level(s: &str) -> Result<LogLevel, String> {
    LogLevel::from_str(s).map_err(|_| format!("possible values: {:?}", LogLevel::VARIANTS))
}

/// Start connections based on CLI options.
///
/// # Parameters
/// - `opt`: CLI Options to start specific connection type.
/// - `conn_manager`: The Server state to start a specific connection.
/// - `shared_state`: The shared state for validating another connection is not already running.
pub fn handle_cli(opt: CliOptions, conn_manager: &ConnectionManager, shared_state: SharedState) {
    if let Some(serial) = opt.serial.serial {
        let serialport = serial.display().to_string();
        conn_manager.connect_to_serial(serialport, opt.serial.baudrate, opt.serial.flow_control);
    } else if let Some(tcp) = opt.tcp.tcp {
        if let Err(e) = conn_manager.connect_to_host(tcp.host, tcp.port) {
            error!("Failed to establish tcp connection: {}", e);
        };
    } else if let Some(file) = opt.file.file {
        let filename = file.display().to_string();
        conn_manager.connect_to_file(filename, RealtimeDelay::On, opt.exit_after_close);
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
        shared_state.set_sbp_logging(true);
        shared_state.set_sbp_logging_format(
            SbpLogging::from_str(&sbp_log.to_string()).expect(CONVERT_TO_STR_FAILURE),
        );
    }
    log::logger().flush();
}
