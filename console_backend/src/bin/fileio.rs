use std::{
    fs,
    io::{self, Write},
};

use clap::Clap;
use crossbeam::{channel, scope};
use sbp::sbp_tools::SBPTools;

use console_backend::{
    broadcaster::Broadcaster,
    cli_options::Input,
    fileio::Fileio,
    types::{MsgSender, Result},
};

#[derive(Clap, Debug)]
#[clap(about = "Fileio operations.")]
pub enum Opts {
    /// Write a file from local source to remote destination dest.
    Write {
        source: String,
        dest: String,
        #[clap(subcommand)]
        input: Input,
    },

    /// Read a file from remote source to local dest. If no dest is provided, file is read to stdout.
    Read {
        source: String,
        dest: Option<String>,
        #[clap(subcommand)]
        input: Input,
    },

    /// List a directory.
    List {
        path: String,
        #[clap(subcommand)]
        input: Input,
    },

    /// Delete a file.
    Delete {
        path: String,
        #[clap(subcommand)]
        input: Input,
    },
}

fn main() -> Result<()> {
    let bc = Broadcaster::new();

    let (done_tx, done_rx) = channel::bounded(0);

    let bc_source = bc.clone();
    let run = move |rdr| {
        let messages = sbp::iter_messages(rdr).log_errors(log::Level::Debug);
        for msg in messages {
            bc_source.send(&msg);
            if done_rx.try_recv().is_ok() {
                break;
            }
        }
    };

    match Opts::parse() {
        Opts::Write {
            source,
            dest,
            input,
        } => {
            let (rdr, wtr) = input.into_conn()?.into_io();
            let sender = MsgSender::new(wtr);
            scope(|s| {
                s.spawn(|_| run(rdr));
                let mut fileio = Fileio::new(bc, sender);
                let data = fs::read(source)?;
                fileio.write(dest, &data)?;
                done_tx.send(true).unwrap();
                eprintln!("file written successfully.");
                Result::Ok(())
            })
            .unwrap()
        }
        Opts::Read {
            source,
            dest,
            input,
        } => {
            let (rdr, wtr) = input.into_conn()?.into_io();
            let sender = MsgSender::new(wtr);
            scope(|s| {
                s.spawn(|_| run(rdr));
                let mut fileio = Fileio::new(bc, sender);
                let data = fileio.read(source)?;
                let mut dest: Box<dyn Write> = match dest {
                    Some(path) => Box::new(fs::File::create(path)?),
                    None => Box::new(io::stdout()),
                };
                dest.write_all(&data)?;
                done_tx.send(true).unwrap();
                Result::Ok(())
            })
            .unwrap()
        }
        Opts::List { path, input } => {
            let (rdr, wtr) = input.into_conn()?.into_io();
            let sender = MsgSender::new(wtr);
            scope(|s| {
                s.spawn(|_| run(rdr));
                let mut fileio = Fileio::new(bc, sender);
                let files = fileio.readdir(path)?;
                eprintln!("{:#?}", files);
                done_tx.send(true).unwrap();
                Result::Ok(())
            })
            .unwrap()
        }
        Opts::Delete { path, input } => {
            let (rdr, wtr) = input.into_conn()?.into_io();
            let sender = MsgSender::new(wtr);
            scope(|s| {
                s.spawn(|_| run(rdr));
                let fileio = Fileio::new(bc, sender);
                fileio.remove(path)?;
                done_tx.send(true).unwrap();
                eprintln!("file deleted.");
                Result::Ok(())
            })
            .unwrap()
        }
    }
}
