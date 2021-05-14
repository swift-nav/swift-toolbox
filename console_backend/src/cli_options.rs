use clap::Clap;
use std::{ops, path::PathBuf};

use crate::constants::{AVAILABLE_BAUDRATES, AVAILABLE_REFRESH_RATES};
use crate::types::{CliTabs, FlowControl};

#[derive(Clap, Debug)]
#[clap(name = "swift_navigation_console", about = "Swift Navigation Console.")]
pub struct CliOptions {
    #[clap(subcommand)]
    pub input: Option<Input>,

    /// Exit when connection closes.
    #[clap(long = "exit-after")]
    pub exit_after: bool,

    // // Frontend Options
    /// Don't use opengl in plots.
    #[clap(long = "no-opengl", parse(from_flag = ops::Not::not))]
    pub no_opengl: bool,

    /// Don't use opengl in plots.
    #[clap(long = "refresh-rate", validator(is_refresh_rate))]
    pub refresh_rate: Option<u8>,

    #[clap(long = "tab")]
    pub tab: Option<CliTabs>,
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
        for arg in args {
            if let Some(n_arg) = next_args.next() {
                if arg == "python" && n_arg.ends_with(".py") {
                    continue;
                }
            }
            filtered_args.push(arg);
        }
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
