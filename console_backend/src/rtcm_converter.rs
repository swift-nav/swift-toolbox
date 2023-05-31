use crate::types::ArcBool;
use crossbeam::channel::Receiver;
use std::io::Write;
use std::process::{Command, Stdio};
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
        let mut child = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", "resources/binaries/win/rtcm3tosbp.exe"]);
            cmd
        } else if cfg!(target_os = "macos") {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "resources/binaries/mac/rtcm3tosbp"]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "resources/binaries/win/rtcm3tosbp"]);
            cmd
        };
        let mut child = child
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
        thread::spawn(move || {
            while let Ok(None) = child.try_wait() {
                if !running.get() {
                    child.kill().unwrap();
                    break;
                }
            }
        });
    }
    pub fn stop(&mut self) {
        self.running.set(false);
    }
}
