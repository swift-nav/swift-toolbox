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

use std::{convert::Infallible, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use sbp::SbpIterExt;

use console_backend::{
    cli_options::{SerialOpts, TcpOpts},
    client_sender::TestSender,
    connection::Connection,
    shared_state::SharedState,
    tabs::settings_tab::SettingsTab,
    types::{MsgSender, Result},
};

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let opts = Opts::parse();
    let settings = connect(&opts)?;

    log::info!("Loading settings...");
    settings.refresh();

    if let Some(path) = opts.export {
        settings.export(&path)?;
        log::info!("Exported settings to {}", path.display());
    } else if let Some(path) = opts.import {
        settings.import(&path)?;
        log::info!("Imported settings from {}", path.display());
    } else if let Some(read_cmd) = opts.read {
        read(read_cmd, &settings)?;
    } else if let Some(write_cmds) = opts.write {
        write(&write_cmds, &settings)?;
    } else if opts.reset {
        log::info!("Resetting settings to factory defaults");
        settings.reset(true)?;
    }

    if opts.save {
        log::info!("Saving settings to flash");
        settings.save()?;
    }

    Ok(())
}

fn read(read_cmd: ReadCmd, settings: &SettingsTab) -> Result<()> {
    let settings = if let Some(name) = read_cmd.name {
        vec![settings.get(&read_cmd.group, &name)?]
    } else {
        settings.group(&read_cmd.group)?
    };
    for entry in settings {
        println!(
            "{}.{}={}",
            entry.setting.group,
            entry.setting.name,
            entry.value.map(|s| s.to_string()).unwrap_or_default()
        )
    }
    Ok(())
}

fn write(write_cmds: &[WriteCmd], settings: &SettingsTab) -> Result<()> {
    for cmd in write_cmds {
        log::debug!("{cmd:?}");
        settings.write_setting(&cmd.group, &cmd.name, &cmd.value)?;
        log::info!("Wrote {}.{}={}", cmd.group, cmd.name, cmd.value);
    }
    Ok(())
}

/// A SwiftNav settings API client
#[derive(Parser)]
#[clap(
    name = "swift-settings",
    version = include_str!("../version.txt"),
    group = ArgGroup::new("conn").required(true).args(&["serial", "tcp"]),
    override_usage = "\
    swift-settings [OPTIONS]

    Examples:
        - Read a setting:
            swift-settings --serial /dev/ttyUSB0 --read imu.acc_range
        - Read a group of settings:
            swift-settings --serial /dev/ttyUSB0 --read imu
        - Write a setting value:
            swift-settings --serial /dev/ttyUSB0 --write imu.acc_range=2g
        - Write multiple settings and save to flash:
            swift-settings --serial /dev/ttyUSB0 -w imu.acc_range=2g -w imu.imu_rate=100 --save
        - Export a device's settings
            swift-settings --serial /dev/ttyUSB0 --export ./config.ini
        - Import a device's settings
            swift-settings --serial /dev/ttyUSB0 --import ./config.ini
    "
)]
struct Opts {
    #[clap(flatten)]
    serial: SerialOpts,

    #[clap(flatten)]
    tcp: TcpOpts,

    /// Read a setting or a group of settings
    #[clap(
        long,
        short,
        value_name = "GROUP[.SETTING]",
        conflicts_with_all = &["write", "import", "export", "reset"]
    )]
    read: Option<ReadCmd>,

    /// Write a setting value
    #[clap(
        long,
        short,
        value_name = "GROUP.SETTING=VALUE",
        conflicts_with_all = &["read", "import", "export", "reset"]
    )]
    write: Option<Vec<WriteCmd>>,

    /// Export the devices settings
    #[clap(
        long,
        value_name = "PATH",
        conflicts_with_all = &["import", "read", "write", "reset"]
    )]
    export: Option<PathBuf>,

    /// Import an ini file
    #[clap(
        long,
        value_name = "PATH",
        conflicts_with_all = &["read", "write", "export", "reset"]
    )]
    import: Option<PathBuf>,

    /// Save settings to flash. Can be combined with --write or --import to save after writing
    #[clap(
        long,
        conflicts_with_all = &["read", "export", "reset"]
    )]
    save: bool,

    /// Reset settings to factory defaults
    #[clap(
        long,
        conflicts_with_all = &["read", "write", "export", "import", "save"]
    )]
    reset: bool,
}

fn connect(opts: &Opts) -> Result<Arc<SettingsTab>> {
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
    let sender = MsgSender::new(writer);
    let shared_state = SharedState::new();
    let settings = Arc::new(SettingsTab::new(shared_state, TestSender::boxed(), sender));
    std::thread::spawn({
        let settings = Arc::clone(&settings);
        move || {
            let messages = sbp::iter_messages(reader).log_errors(log::Level::Debug);
            for msg in messages {
                settings.handle_msg(msg);
            }
        }
    });
    Ok(settings)
}

#[derive(Clone)]
struct ReadCmd {
    group: String,
    name: Option<String>,
}

impl FromStr for ReadCmd {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (group, name) = if let Some((group, name)) = s.split_once('.') {
            (group.to_owned(), Some(name.to_owned()))
        } else {
            (s.to_owned(), None)
        };

        Ok(ReadCmd { group, name })
    }
}

#[derive(Debug, Clone)]
struct WriteCmd {
    group: String,
    name: String,
    value: String,
}

impl FromStr for WriteCmd {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        const ERROR: &str = "write arguments must be of the form <GROUP>.<NAME>=<VALUE>";

        let eq_idx = s.find('=').ok_or(ERROR)?;
        let (setting, value) = s.split_at(eq_idx);
        let dot_idx = setting.find('.').ok_or(ERROR)?;
        let (group, name) = setting.split_at(dot_idx);
        Ok(WriteCmd {
            group: group.to_owned(),
            name: name[1..].to_owned(),
            value: value[1..].to_owned(),
        })
    }
}
