use clap::Clap;
use crossbeam::{channel, scope};
use sbp::{messages::piksi::MsgSpecan, sbp_tools::SBPTools};

use console_backend::{
    broadcaster::Broadcaster, connection::Connection, fft_monitor::FftMonitor, types::Result,
};

#[derive(Clap, Debug)]
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
    pub fn into_conn(self) -> Connection {
        Connection::tcp(self.host, self.port)
    }
}

#[cfg(feature = "fft")]
fn main() -> Result<()> {
    use serde_pickle::ser;
    use std::{fs::File, thread::sleep, time::Duration};

    let bc = Broadcaster::new();

    let (done_tx, done_rx) = channel::bounded(0);

    let bc_source = bc.clone();
    let (specan_msg, _) = bc_source.clone().subscribe::<MsgSpecan>();
    let run = move |rdr| {
        let messages = sbp::iter_messages(rdr).log_errors(log::Level::Debug);
        for msg in messages {
            bc_source.clone().send(&msg);
            if done_rx.try_recv().is_ok() {
                break;
            }
        }
    };
    let opts = CliFftMonitor::parse();
    let filename = format!("{}.pickle", opts.output);
    println!("Writing to file: {}", &filename);
    let channel = opts.channel;
    let num_ffts = opts.num_ffts;
    let (rdr, _) = opts.into_conn().try_connect(/*shared_state=*/ None)?;

    scope(|s| {
        s.spawn(|_| run(rdr));
        let mut fftmonitor = FftMonitor::new();
        fftmonitor.enable_channel(Some(channel));
        while let Some(n) = fftmonitor.num_ffts(channel) {
            if n >= num_ffts as usize {
                break;
            }
            if let Ok(msg) = specan_msg.try_recv() {
                if let Err(err) = fftmonitor.capture_fft(msg) {
                    eprintln!("error capturing fft, {}", err);
                }
            }
            sleep(Duration::from_millis(1));
        }
        if let Some(fft) = fftmonitor.get_ffts(channel) {
            let mut file = File::create(filename)?;
            ser::to_writer(&mut file, &fft, false)?;
        }

        println!("File written successfully.");
        done_tx.send(true).unwrap();
        Result::Ok(())
    })
    .unwrap()?;

    Ok(())
}
