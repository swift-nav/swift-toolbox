use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::{io, thread};

pub struct RtcmConverter {
    input: Sender<Vec<u8>>,
}

impl RtcmConverter {
    pub fn start<W: Write + Send + 'static>(mut out: W) -> Self {
        println!("starting rtcm3tosbp");
        let mut child = Command::new("./binaries/mac/rtcm3tosbp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("rtcm converter process failed");

        let (in_tx, in_rx) = channel::unbounded::<Vec<u8>>();
        let mut child_in = child.stdin.take().unwrap();
        thread::spawn(move || {
            io::copy(child.stderr.as_mut().unwrap(), &mut out).unwrap();
        });
        thread::spawn(move || loop {
            if let Ok(data) = in_rx.recv() {
                child_in.write_all(&data).unwrap();
            }
        });
        Self { input: in_tx }
    }

    pub fn send_data(&self, data: Vec<u8>) -> anyhow::Result<()> {
        self.input.send(data)?;
        Ok(())
    }
}
