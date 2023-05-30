use crate::types::ArcBool;
use crossbeam::channel::{Receiver, Sender};
use crossbeam::{channel, select};
use std::io::{stdout, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{io, thread};

pub struct RtcmConverter {
    in_rx: Receiver<Vec<u8>>,
    running: ArcBool,
}

impl RtcmConverter {
    pub fn new(in_rx: Receiver<Vec<u8>>) -> Self {
        Self {
            in_rx,
            running: ArcBool::new(),
        }
    }

    pub fn start<W: Write + Send + 'static>(&mut self, mut out: W) {
        println!("starting rtcm3tosbp");
        self.running.set(true);
        let mut child = Command::new("./binaries/mac/rtcm3tosbp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("rtcm converter process failed");

        let mut child_in = child.stdin.take().unwrap();
        let mut child_err = child.stderr.take().unwrap();
        let in_rx = self.in_rx.clone();

        thread::spawn(move || {
            println!("io open");
            io::copy(&mut child_err, &mut out).unwrap();
            println!("io closed");
        });
        // should be terminated when sender is dropped
        thread::spawn(move || {
            while let Ok(data) = in_rx.recv() {
                child_in.write_all(&data).unwrap();
            }
            println!("channel closed");
        });
        let running = self.running.clone();
        thread::spawn(move || loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    println!("rtcm3tosbp exited with: {status}");
                    break;
                }
                Ok(None) => {
                    if !running.get() {
                        child.kill().unwrap();
                        break;
                    }
                }
                Err(e) => {
                    println!("rtcm3tosbp error attempting to wait: {e}");
                    break;
                }
            }
        });
    }
    pub fn stop(&mut self) {
        self.running.set(false);
    }
}
