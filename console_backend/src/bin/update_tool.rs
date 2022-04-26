use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use clap::{Args, Parser};
use indicatif::{ProgressBar, ProgressStyle};

use console_backend::cli_options::{SerialOpts, TcpOpts};
use console_backend::connection::Connection;
use console_backend::firmware_update;
use console_backend::swift_version::SwiftVersion;
use console_backend::types::MsgSender;
use console_backend::types::Result;
use sbp::link::{Link, LinkSource};
use sbp::SbpIterExt;
use sbp_settings::Client;

#[derive(Parser)]
#[clap(
    name = "swift-update-tool",
    version = include_str!("../version.txt"),
)]
struct Opts {
    update_file: PathBuf,

    #[clap(flatten)]
    serial: SerialOpts,

    #[clap(flatten)]
    tcp: TcpOpts,
}

const FIRMWARE_VERSION_GROUP: &str = "system_info";
const FIRMWARE_VERSION_SETTING: &str = "firmware_version";
const READ_TIMEOUT: Duration = Duration::from_secs(5);

fn get_firmware_version(link: Link<'static, ()>, msg_sender: MsgSender) -> Result<SwiftVersion> {
    let mut settings_client =
        Client::with_link(link, move |msg| msg_sender.send(msg).map_err(Into::into));

    let setting_value = settings_client
        .read_setting_with_timeout(
            FIRMWARE_VERSION_GROUP,
            FIRMWARE_VERSION_SETTING,
            READ_TIMEOUT,
        )?
        .ok_or(anyhow!("Couldn't read firmware version"))?
        .value
        .ok_or(anyhow!("Couldn't read firmware version"))?
        .to_string();

    SwiftVersion::from_str(&setting_value).map_err(Into::into)
}

fn get_connection(opts: &Opts) -> Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    let (reader, writer) = if let Some(ref serial) = opts.serial.serial {
        Connection::serial(
            serial.to_string_lossy().into(),
            opts.serial.baudrate,
            opts.serial.flow_control,
        )
        .try_connect(None)?
    } else if let Some(ref tcp) = opts.tcp.tcp {
        Connection::tcp(tcp.host.clone(), tcp.port)?.try_connect(None)?
    } else {
        return Err(anyhow!("No serialport or tcp string supplied"));
    };
    Ok((reader, writer))
}

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    let opts = Opts::parse();

    let (reader, writer) = get_connection(&opts)?;
    let source = LinkSource::new();
    let link = source.link();
    let msg_sender = MsgSender::new(writer);

    std::thread::spawn(move || {
        let messages = sbp::iter_messages(reader).log_errors(log::Level::Debug);
        for msg in messages {
            source.send(msg);
        }
    });

    let firmware_version = get_firmware_version(link.clone(), msg_sender.clone())?;

    let pb = ProgressBar::new(100);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar}] {pos}/{len}% ({eta} remaining)")
            .progress_chars("=> "),
    );
    firmware_update::firmware_update(
        link,
        msg_sender,
        &opts.update_file,
        &firmware_version,
        |msg| println!("> {}", msg),
        move |progress| pb.set_position(progress as u64),
    )?;

    Ok(())
}
