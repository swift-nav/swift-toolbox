use std::{
    convert::Infallible,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Context};
use clap::{Args, Parser};
use sbp::{link::LinkSource, SbpIterExt};

use console_backend::{
    cli_options::is_baudrate,
    connection::{SerialConnection, TcpConnection},
    fileio::Fileio,
    types::{FlowControl, MsgSender, Result},
};

fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::parse();
    if opts.list {
        list(opts.src, opts.conn)
    } else if opts.delete {
        delete(opts.src, opts.conn)
    } else {
        transfer(opts.src, opts.dest, opts.conn)
    }
}

/// Piksi File IO operations.
#[derive(Parser)]
#[clap(name = "fileio", version = include_str!("../version.txt"))]
struct Opts {
    /// The source target
    src: Target,

    /// The optional destination (defaults to stdout)
    #[clap(default_value = "-")]
    dest: Target,

    /// List a directory
    #[clap(long, conflicts_with_all = &["dest", "delete"])]
    list: bool,

    /// Delete a file
    #[clap(long, conflicts_with_all = &["dest", "list"])]
    delete: bool,

    #[clap(flatten)]
    conn: ConnectionOpts,
}

#[derive(Args)]
struct ConnectionOpts {
    /// The port to use when connecting via TCP
    #[clap(long, default_value = "55555")]
    port: u16,

    /// The baudrate for processing packets when connecting via serial
    #[clap(long, default_value = "115200", validator(is_baudrate))]
    baudrate: u32,

    /// The flow control spec to use when connecting via serial
    #[clap(long, default_value = "None")]
    flow_control: FlowControl,
}

fn list(target: Target, conn: ConnectionOpts) -> Result<()> {
    let remote = target
        .into_remote()
        .context("--list flag requires src to be a remote target")?;
    let mut fileio = remote.start(conn)?;
    let files = fileio.readdir(remote.path)?;
    for file in files {
        println!("{file}");
    }
    Ok(())
}

fn delete(target: Target, conn: ConnectionOpts) -> Result<()> {
    let remote = target
        .into_remote()
        .context("--delete flag requires src to be a remote target")?;
    let fileio = remote.start(conn)?;
    fileio.remove(remote.path)?;
    Ok(())
}

fn transfer(src: Target, dest: Target, conn: ConnectionOpts) -> Result<()> {
    match (src, dest) {
        (Target::Remote(src), Target::Local(dest)) => read(src, dest, conn),
        (Target::Local(src), Target::Remote(dest)) => write(src, dest, conn),
        (Target::Local(_), Target::Local(_)) => {
            Err(anyhow!("source and dest cannot both be local paths"))
        }
        (Target::Remote(_), Target::Remote(_)) => {
            Err(anyhow!("source and dest cannot both be remote paths"))
        }
    }
}

fn read(src: Remote, dest: PathBuf, conn: ConnectionOpts) -> Result<()> {
    let dest: Box<dyn Write + Send> = if dest.to_str() == Some("-") {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(dest)?)
    };
    let mut fileio = src.start(conn)?;
    fileio.read(src.path, dest)?;
    Ok(())
}

fn write(src: PathBuf, dest: Remote, conn: ConnectionOpts) -> Result<()> {
    let mut fileio = dest.start(conn)?;
    let file = fs::File::open(src)?;
    let size = file.metadata()?.len() as usize;
    let mut bytes_written = 0;
    print!("\rWriting 0.0%...");
    fileio.overwrite_with_progress(dest.path, file, |n| {
        bytes_written += n;
        let progress = (bytes_written as f64) / (size as f64) * 100.0;
        print!("\rWriting {:.2}%...", progress);
    })?;
    println!("\nFile written successfully ({} bytes).", bytes_written);
    Ok(())
}

#[derive(Debug)]
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
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.find(':') {
            Some(idx) => {
                let (host, path) = s.split_at(idx);
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
#[derive(Debug)]
struct Remote {
    host: String,
    path: String,
}

impl Remote {
    fn connect(
        &self,
        port: u16,
        baudrate: u32,
        flow: FlowControl,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let tcp = TcpConnection::new(self.host.clone(), port).and_then(|conn| {
            let conn = conn.try_connect(None)?;
            Ok(conn)
        });
        if let Ok(rw) = tcp {
            return Ok(rw);
        }
        let serial = SerialConnection::new(self.host.clone(), baudrate, flow).try_connect(None)?;
        Ok(serial)
    }

    fn start(&self, conn: ConnectionOpts) -> Result<Fileio> {
        let (reader, writer) = self.connect(conn.port, conn.baudrate, conn.flow_control)?;
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
