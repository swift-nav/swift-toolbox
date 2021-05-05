use clap::Clap;
use std::{ops, path::PathBuf, str::FromStr};

use crate::common_constants::Tabs;
use crate::constants::{AVAILABLE_REFRESH_RATES, TAB_LIST};

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

    #[clap(long = "tab", validator(is_tab))]
    pub tab: Option<String>,
}

#[derive(Clap, Debug)]
#[clap(about = "Input type and corresponding options.")]
pub enum Input {
    Tcp {
        /// The TCP host to connect to.
        host: String,

        /// The port to use when connecting via TCP.
        #[clap(long, default_value = "55555")]
        port: u32,
    },
    Serial {
        /// The serialport to connect to.
        #[clap(parse(from_os_str))]
        serialport: PathBuf,

        /// The baudrate for processing packets.
        #[clap(long, default_value = "115200")]
        baudrate: u32,

        /// The flow control spec to use.
        #[clap(long = "flow-control", default_value = "None")]
        flow_control: String,
    },
    File {
        /// Open and run an SBP file.
        #[clap(parse(from_os_str))]
        file_in: PathBuf,
    },
}

/// Validation for the tab cli option.
///
/// # Parameters
/// - `tab`: The user input tab.
///
/// # Returns
/// - `Ok`: The tab was found in TAB_LIST.
/// - `Err`: The tab was not found in TAB_LIST.
fn is_tab(tab: &str) -> Result<(), String> {
    if Tabs::from_str(tab).is_ok() {
        return Ok(());
    }

    Err(format!("Must choose from available tabs {:?}", TAB_LIST))
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
    Err(format!("Must choose from available refresh rates {:?}", AVAILABLE_REFRESH_RATES))
}

