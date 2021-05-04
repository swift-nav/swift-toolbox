use clap::Clap;
use std::{ops, path::PathBuf};

#[derive(Debug, Clap)]
#[clap(name = "swift_navigation_console", about = "Swift Navigation Console.")]
pub struct CliOptions {
    #[clap(subcommand)]
    pub input: Option<Input>,

    // // Frontend Options
    /// Don't use opengl in plots.
    #[clap(long = "no-opengl", parse(from_flag = ops::Not::not))]
    pub no_opengl: bool,

    /// Don't use opengl in plots.
    #[clap(long = "refresh-rate")]
    pub refresh_rate: Option<u32>,
}

#[derive(Debug, Clap)]
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
