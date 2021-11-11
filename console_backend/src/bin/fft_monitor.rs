use std::{fs::File, thread::sleep, time::Duration};

use anyhow::Context;
use clap::Parser;
use crossbeam::{channel, scope};
use parking_lot::Mutex;
use sbp::messages::piksi::MsgSpecan;
use sbp::{link::LinkSource, SbpIterExt};
use serde_pickle::{ser, SerOptions};

use console_backend::{
    connection::Connection,
    fft_monitor::FftMonitor,
    types::{Result, Specan},
};

#[derive(Parser, Debug)]
#[clap(name = "FFT Monitor Example")]
pub struct CliFftMonitor {
    /// The TCP host to connect to.
    host: String,

    /// The port to use when connecting via TCP.
    #[clap(long, default_value = "55555")]
    port: u16,

    /// Specify the number of FFTs to capture.
    #[clap(long = "num-ffts", default_value = "5")]
    num_ffts: u16,

    /// Specify the channel to monitor.
    #[clap(long, default_value = "1")]
    channel: u16,

    /// The output filename prefix.
    #[clap(long, default_value = "fftmonitor")]
    output: String,
}

impl CliFftMonitor {
    pub fn into_conn(self) -> Result<Connection> {
        Connection::tcp(self.host, self.port)
    }
}

fn main() -> Result<()> {
    let opts = CliFftMonitor::parse();
    let filename = format!("{}.pickle", opts.output);
    println!("Writing to file: {}", &filename);
    let channel = opts.channel;
    let num_ffts = opts.num_ffts;
    let (rdr, _) = opts
        .into_conn()?
        .try_connect(/*shared_state=*/ None)
        .context("while connecting")?;

    let fftmonitor = Mutex::new(FftMonitor::new());
    fftmonitor.lock().enable_channel(Some(channel));

    let (done_tx, done_rx) = channel::bounded(0);

    let source = LinkSource::new();
    let link = source.link();
    link.register(|msg: MsgSpecan| {
        if let Err(err) = fftmonitor.lock().capture_fft(Specan::MsgSpecan(msg)) {
            eprintln!("error capturing fft, {}", err);
        }
    });

    scope(|s| {
        s.spawn(|_| {
            let messages = sbp::iter_messages(rdr).log_errors(log::Level::Debug);
            for msg in messages {
                source.send(msg);
                if done_rx.try_recv().is_ok() {
                    break;
                }
            }
        });
        while let Some(n) = fftmonitor.lock().num_ffts(channel) {
            if n >= num_ffts as usize {
                break;
            }
            sleep(Duration::from_millis(1));
        }
        if let Some(fft) = fftmonitor.lock().get_ffts(channel) {
            let mut file = File::create(filename)?;
            ser::to_writer(&mut file, &fft, SerOptions::new().proto_v2())?;
        }
        println!("File written successfully.");
        done_tx.send(true).unwrap();
        Result::Ok(())
    })
    .unwrap()?;

    Ok(())
}
