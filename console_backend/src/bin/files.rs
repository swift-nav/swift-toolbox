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

use std::{
    borrow::Cow,
    fs::File,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use anyhow::{anyhow, Context};
use clap::{Args, Parser};
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use sbp::{link::LinkSource, SbpIterExt};

use console_backend::{
    cli_options::is_baudrate,
    connection::Connection,
    constants::EXAMPLE_SERIAL_NAME,
    fileio::Fileio,
    types::{FlowControl, MsgSender, Result},
};

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();
    let opts = Opts::parse();
    if let Some(target) = opts.list {
        list(target, opts.conn)
    } else if let Some(target) = opts.delete {
        delete(target, opts.conn)
    } else if let (Some(src), Some(dest)) = (opts.src, opts.dest) {
        transfer(src, dest, opts.conn)
    } else {
        Err(anyhow!(
            "file transfers require both <SRC> and <DEST> to be set"
        ))
    }
}

lazy_static! {
    static ref FILEIO_USAGE: String = format!(
        "\
    To copy a local file to a Swift device:
        swift-files <FILE_PATH> <HOST>:<FILE_PATH>
    To copy a file from a Swift device:
        swift-files <HOST>:<FILE_PATH> <FILE_PATH>
    To list files in a directory on a Swift device:
        swift-files --list <HOST>:<DIRECTORY_PATH>
    To delete a file on a Swift device:
        swift-files --delete <HOST>:<FILE_PATH>

    <HOST> can either be an IP address when the Swift device is connected
    via TCP (for eg: 192.168.0.222) or the name of the serial device when
    the Swift device is connected via serial (for eg: {EXAMPLE_SERIAL_NAME}).
    
    TCP Examples:
        - List files on Swift device:
            swift-files --list 192.168.0.222:/data/
        - Read file from Swift device:
            swift-files 192.168.0.222:/persistent/config.ini ./config.ini
        - Write file to Swift device:
            swift-files ./config.ini 192.168.0.222:/persistent/config.ini
        - Delete file from Swift device:
            swift-files --delete 192.168.0.222:/persistent/unwanted_file

    Serial Examples:
        - List files on Swift device:
            swift-files --list {EXAMPLE_SERIAL_NAME}:/data/
        - Read file from Swift device:
            swift-files {EXAMPLE_SERIAL_NAME}:/persistent/config.ini ./config.ini
        - Write file to Swift device:
            swift-files ./config.ini {EXAMPLE_SERIAL_NAME}:/persistent/config.ini
        - Delete file from Swift device:
            swift-files --delete {EXAMPLE_SERIAL_NAME}:/persistent/unwanted_file
    "
    );
}

/// A SwiftNav fileio API client
#[derive(Parser)]
#[clap(
    name = "swift-files",
    version = include_str!("../version.txt"),
    arg_required_else_help = true,
    override_usage = &**FILEIO_USAGE
)]
struct Opts {
    /// The source target
    src: Option<Target>,

    /// The destination when transfering files
    dest: Option<Target>,

    /// List a directory
    #[clap(long, short, value_name="TARGET", conflicts_with_all = &["dest", "delete"])]
    list: Option<Target>,

    /// Delete a file
    #[clap(long, value_name="TARGET", conflicts_with_all = &["dest", "list"])]
    delete: Option<Target>,

    #[clap(flatten)]
    conn: ConnectionOpts,
}

#[derive(Clone, Copy, Args)]
struct ConnectionOpts {
    /// The port to use when connecting via TCP
    #[clap(long, default_value = "55555", conflicts_with_all = &["baudrate", "flow_control"])]
    port: u16,

    /// The baudrate for processing packets when connecting via serial
    #[clap(
        long,
        default_value = "115200",
        value_parser = is_baudrate,
        conflicts_with = "port"
    )]
    baudrate: u32,

    /// The flow control spec to use when connecting via serial
    #[clap(long, default_value = "None", conflicts_with = "port")]
    flow_control: FlowControl,
}

fn list(target: Target, conn: ConnectionOpts) -> Result<()> {
    let remote = target.into_remote().context(
        "--list flag requires <TARGET> to be a remote target of the form <HOST>:<DIRECTORY_PATH>",
    )?;
    let mut fileio = remote.connect(conn)?;
    let files = fileio.readdir(remote.path)?;
    for file in files {
        println!("{file}");
    }
    Ok(())
}

fn delete(target: Target, conn: ConnectionOpts) -> Result<()> {
    let remote = target.into_remote().context(
        "--delete flag requires <TARGET> to be a remote target of the form <HOST>:<FILE_PATH>",
    )?;
    let fileio = remote.connect(conn)?;
    fileio.remove(remote.path)?;
    // without this sleep the program exits and the connection closes before the delete message
    // is sent. we could use https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_linger (once stable)
    // or https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_nodelay
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}

fn transfer(src: Target, dest: Target, conn: ConnectionOpts) -> Result<()> {
    match (src, dest) {
        (Target::Remote(src), Target::Local(dest)) => read(src, dest, conn),
        (Target::Local(src), Target::Remote(dest)) => write(src, dest, conn),
        (Target::Local(_), Target::Local(_)) => {
            Err(anyhow!("<SRC> and <DEST> cannot both be local paths"))
        }
        (Target::Remote(_), Target::Remote(_)) => {
            Err(anyhow!("<SRC> and <DEST> cannot both be remote paths"))
        }
    }
}

fn read(src: Remote, dest: PathBuf, conn: ConnectionOpts) -> Result<()> {
    let (dest, pb): (Box<dyn Write + Send>, _) = if dest.to_str() == Some("-") {
        (Box::new(io::stdout()), ReadProgress::stdout())
    } else {
        let dest = File::create(&dest)
            .with_context(|| format!("Could not open {:?} for writing", &dest))?;
        (Box::new(dest), ReadProgress::file())
    };
    let mut fileio = src.connect(conn)?;
    pb.set_message("Reading...");
    fileio.read_with_progress(src.path, dest, |n| {
        pb.inc(n);
    })?;
    pb.finish_with_message("Done");
    Ok(())
}

fn write(src: PathBuf, dest: Remote, conn: ConnectionOpts) -> Result<()> {
    let mut fileio = dest.connect(conn)?;
    let file =
        File::open(&src).with_context(|| format!("Could not open {:?} for reading", &src))?;
    let size = file.metadata()?.len();
    let pb = ProgressBar::new(size);
    pb.enable_steady_tick(Duration::from_millis(1000));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("=> "),
    );
    fileio.overwrite_with_progress(dest.path, file, |n| {
        pb.inc(n);
    })?;
    pb.finish();
    Ok(())
}

#[derive(Debug, Clone)]
enum Target {
    Local(PathBuf),
    Remote(Remote),
}

impl Target {
    fn into_remote(self) -> Option<Remote> {
        if let Self::Remote(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.find(':') {
            Some(idx) => {
                let (host, path) = s.split_at(idx);

                if path == ":" {
                    return Err(format!("No remote path given in '{s}'"));
                }

                Ok(Target::Remote(Remote {
                    host: host.to_owned(),
                    path: path[1..].to_owned(),
                }))
            }
            None => Ok(Target::Local(PathBuf::from(s))),
        }
    }
}

/// A host + path specified as <host>:<path>
#[derive(Debug, Clone)]
struct Remote {
    host: String,
    path: String,
}

impl Remote {
    fn connect(&self, conn: ConnectionOpts) -> Result<Fileio> {
        let (reader, writer) = discover(self.host.clone(), conn)?.try_connect(None)?;
        let source = LinkSource::new();
        let link = source.link();
        std::thread::spawn(move || {
            let messages = sbp::iter_messages(reader).log_errors(log::Level::Debug);
            for msg in messages {
                source.send(msg);
            }
        });
        let sender = MsgSender::new(writer);
        Ok(Fileio::new(link, sender))
    }
}

struct ReadProgress {
    inner: Option<ProgressBar>,
}

impl ReadProgress {
    fn file() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(1000));
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("[{elapsed_precise}] {bytes} ({bytes_per_sec}) {msg}")
                .expect("failed to configure progress bar"),
        );
        Self { inner: Some(pb) }
    }

    fn stdout() -> Self {
        Self { inner: None }
    }

    fn inc(&self, n: u64) {
        if let Some(pb) = &self.inner {
            pb.inc(n);
        }
    }

    fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        if let Some(pb) = &self.inner {
            pb.set_message(msg);
        }
    }

    fn finish_with_message(&self, msg: impl Into<Cow<'static, str>>) {
        if let Some(pb) = &self.inner {
            pb.abandon_with_message(msg);
        }
    }
}

/// Connect via a serial port or tcp
fn discover(host: String, opts: ConnectionOpts) -> Result<Connection> {
    match File::open(&host) {
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Err(e.into()),
        Ok(_) => {
            log::debug!("connecting via serial");
            Ok(Connection::serial(host, opts.baudrate, opts.flow_control))
        }
        Err(_) => {
            log::debug!("connecting via tcp");
            Connection::tcp(host, opts.port)
        }
    }
}
