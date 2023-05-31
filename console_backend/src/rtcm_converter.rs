use crate::types::ArcBool;
use crossbeam::channel::{Receiver, Sender};
use crossbeam::{channel, select};
use std::io::{stdout, BufRead, BufReader, Read, Write};
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
        self.running.set(true);
        let mut child = Command::new("sh")
            .args(["-c", "./binaries/mac/rtcm3tosbp"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("rtcm converter process failed");

        let mut child_in = child.stdin.take().unwrap();
        let mut child_out = child.stdout.take().unwrap();
        let in_rx = self.in_rx.clone();

        thread::spawn(move || {
            io::copy(&mut child_out, &mut out).unwrap();
        });
        thread::spawn(move || {
            while let Ok(data) = in_rx.recv() {
                child_in.write_all(&data).unwrap();
            }
            println!("channel closed");
        });
        let running = self.running.clone();
        thread::spawn(move || loop {
            match child.try_wait() {
                Ok(None) => {
                    if !running.get() {
                        child.kill().unwrap();
                        break;
                    }
                }
                _ => break,
            }
        });
    }
    pub fn stop(&mut self) {
        self.running.set(false);
    }
}
