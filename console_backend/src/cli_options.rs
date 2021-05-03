use std::{ops, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "swift_navigation_console", about = "Swift Navigation Console.")]
pub struct CliOptions {
    #[structopt(subcommand)]
    pub input: Option<Input>,

    // // Frontend Options

    /// Don't use opengl in plots.
    #[structopt(long = "no-opengl", parse(from_flag = ops::Not::not))]
    pub no_opengl: bool,

    /// Don't use opengl in plots.
    #[structopt(long = "refresh-rate")]
    pub refresh_rate: Option<u32>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Input type and corresponding options.")]
pub enum Input {
    Tcp {
        /// The TCP host to connect to.
        host: String,
    
        /// The port to use when connecting via TCP.
        #[structopt(long = "port", default_value="55555")]
        port: u32
    },
    Serial {
        /// The serialport to connect to.
        #[structopt(parse(from_os_str))]
        serialport: PathBuf,
    
        /// The baudrate for processing packets.
        #[structopt(long = "baudrate", default_value="115200")]
        baudrate: u32,
    
        /// The flow control spec to use.
        #[structopt(long = "flow-control", default_value="None")]
        flow_control: String
    },
    File {
        /// Open and run an SBP file.
        #[structopt(parse(from_os_str))]
        file_in: PathBuf
    }
}
