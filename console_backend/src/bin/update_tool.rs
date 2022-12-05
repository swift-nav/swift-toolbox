use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use console_backend::constants::EXAMPLE_SERIAL_NAME;
use lazy_static::lazy_static;

use console_backend::cli_options::{SerialOpts, TcpOpts};
use console_backend::connection::Connection;
use console_backend::types::MsgSender;
use console_backend::types::Result;
use console_backend::updater::firmware_update::{self, LogOverwriteBehavior};
use console_backend::updater::swift_version::SwiftVersion;
use sbp::link::{Link, LinkSource};
use sbp::SbpIterExt;
use sbp_settings::Client;

lazy_static! {
    static ref USAGE: String = format!(
        "\
    swift-updater [OPTIONS]

    Examples:
        - Updating firmware using TCP/IP
            swift-updater --tcp 192.168.0.2222:55555 ./firmware.bin
        - Updating firmware using serial
            swift-updater --serial {serial} ./firmware.bin
    ",
        serial = EXAMPLE_SERIAL_NAME
    );
}

/// A SwiftNav firmware updater API client
#[derive(Parser)]
#[clap(
    name = "swift-updater",
    version = include_str!("../version.txt"),
    group = ArgGroup::new("conn").required(true).args(&["serial", "tcp"]),
    override_usage = &**USAGE
)]
struct Opts {
    /// The binary (.bin) file to write to flash
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
        .ok_or_else(|| anyhow!("Couldn't read firmware version"))?
        .value
        .ok_or_else(|| anyhow!("Couldn't read firmware version"))?
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

fn printer(
    line: &str,
    overwrite: LogOverwriteBehavior,
    last_state: &Arc<Mutex<LogOverwriteBehavior>>,
) {
    let mut last_overwrite = last_state.lock().expect("Mutex poisoned");

    // Logic here is needed here to avoid overwriting lines that shouldn't be overwritten
    match (*last_overwrite, overwrite) {
        (LogOverwriteBehavior::DontOverwrite, LogOverwriteBehavior::DontOverwrite) => {
            println!("> {line}")
        }
        (LogOverwriteBehavior::DontOverwrite, LogOverwriteBehavior::Overwrite) => {
            print!("> {line}")
        }
        (LogOverwriteBehavior::Overwrite, LogOverwriteBehavior::DontOverwrite) => {
            println!("\n> {line}")
        }
        (LogOverwriteBehavior::Overwrite, LogOverwriteBehavior::Overwrite) => print!("\r> {line}"),
    }

    *last_overwrite = overwrite;

    stdout().flush().expect("Couldn't flush stdout");
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

    let log_state = Arc::new(Mutex::new(LogOverwriteBehavior::Overwrite));
    let progress_state = log_state.clone();

    firmware_update::firmware_update(
        link,
        msg_sender,
        &opts.update_file,
        &firmware_version,
        move |msg, overwrite| printer(&msg, overwrite, &log_state),
        move |progress| {
            printer(
                &format!("Uploading image to device {:.2}%...", progress),
                LogOverwriteBehavior::Overwrite,
                &progress_state,
            )
        },
    )?;

    Ok(())
}
