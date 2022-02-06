use std::{convert::Infallible, path::PathBuf, str::FromStr, sync::Arc};

use clap::{AppSettings::DeriveDisplayOrder, Parser};
use sbp::SbpIterExt;

use console_backend::{
    cli_options::ConnectionOpts,
    client_sender::TestSender,
    connection::Connection,
    settings_tab::SettingsTab,
    shared_state::SharedState,
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
        log::info!("Reseting settings to factory defaults");
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

/// Piksi settings operations.
#[derive(Parser)]
#[clap(
    name = "piksi-settings",
    version = include_str!("../version.txt"),
    setting = DeriveDisplayOrder,
)]
struct Opts {
    /// The serial port or TCP stream
    device: String,

    /// Read a setting value
    #[clap(long, conflicts_with_all = &["write", "import", "export"])]
    read: Option<ReadCmd>,

    /// Write a setting value
    #[clap(
        long,
        conflicts_with_all = &["read", "import", "export"]
    )]
    write: Option<Vec<WriteCmd>>,

    /// Export the devices settings
    #[clap(long, conflicts_with_all = &["import", "read", "write"])]
    export: Option<PathBuf>,

    /// Import an ini file
    #[clap(long, conflicts_with_all = &["read", "write", "export"])]
    import: Option<PathBuf>,

    /// Save settings to flash
    #[clap(long, conflicts_with_all = &["read", "export"])]
    save: bool,

    /// Reset settings to factory defaults
    #[clap(long, conflicts_with_all = &["read", "write", "import", "export", "save"])]
    reset: bool,

    #[clap(flatten)]
    conn: ConnectionOpts,
}

fn connect(opts: &Opts) -> Result<Arc<SettingsTab>> {
    let (reader, writer) =
        Connection::discover(opts.device.clone(), opts.conn)?.try_connect(None)?;
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

struct ReadCmd {
    group: String,
    name: Option<String>,
}

impl FromStr for ReadCmd {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(idx) = s.find('.') {
            let (group, name) = s.split_at(idx);
            Ok(ReadCmd {
                group: group.to_owned(),
                name: Some(name[1..].to_owned()),
            })
        } else {
            Ok(ReadCmd {
                group: s.to_owned(),
                name: None,
            })
        }
    }
}

#[derive(Debug)]
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
