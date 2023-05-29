use crossbeam::channel::{Receiver, Sender};
use crossbeam::{channel, select};
use std::io::{stdout, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::{io, thread};

pub struct RtcmConverter {
    in_rx: Receiver<Vec<u8>>,
}

impl RtcmConverter {
    pub fn new(in_rx: Receiver<Vec<u8>>) -> Self {
        Self { in_rx }
    }

    pub fn start<W: Write + Send + 'static>(&self, mut out: W) {
        println!("starting rtcm3tosbp");
        let mut child = Command::new("./binaries/mac/rtcm3tosbp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("rtcm converter process failed");

        let mut child_in = child.stdin.take().unwrap();
        let mut child_err = child.stderr.take().unwrap();

        thread::spawn(move || {
            io::copy(&mut child_err, &mut out).unwrap();
        });

        let in_rx = self.in_rx.clone();
        // should be terminated when sender is dropped
        thread::spawn(move || {
            while let Ok(data) = in_rx.recv() {
                child_in.write_all(&data).unwrap();
            }
        });
    }
}
