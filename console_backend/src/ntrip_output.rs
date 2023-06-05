use crate::ntrip_tab::OutputType;
use crate::types::{ArcBool, Result};
use anyhow::Context;
use crossbeam::channel::Receiver;
use log::error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{io, thread};

pub struct MessageConverter {
    in_rx: Receiver<Vec<u8>>,
    running: ArcBool,
    output_type: OutputType,
}

impl MessageConverter {
    pub fn new(in_rx: Receiver<Vec<u8>>, output_type: OutputType) -> Self {
        Self {
            in_rx,
            running: ArcBool::new(),
            output_type,
        }
    }

    pub fn start<W: Write + Send + 'static>(&mut self, out: W) -> Result<()> {
        self.running.set(true);
        match self.output_type {
            OutputType::RTCM => self.output_rtcm(out),
            OutputType::SBP => self.output_sbp(out),
        }
    }

    /// Just redirects directly to writer
    fn output_rtcm<W: Write + Send + 'static>(&mut self, mut out: W) -> Result<()> {
        let in_rx = self.in_rx.clone();
        let running = self.running.clone();
        thread::spawn(move || loop {
            if !running.get() {
                break;
            }
            if let Ok(data) = in_rx.try_recv() {
                if let Err(e) = out.write(&data) {
                    error!("failed to write to device {e}");
                }
            }
        });
        Ok(())
    }

    /// Runs rtcm3tosbp converter
    fn output_sbp<W: Write + Send + 'static>(&mut self, mut out: W) -> Result<()> {
        let mut cmd = Command::new("sh");
        let _spawned = cmd.args(["-c", "ls"]).spawn()?;
        let mut child = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", "./binaries/win/rtcm3tosbp.exe"]);
            cmd
        } else if cfg!(target_os = "macos") {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "./binaries/mac/rtcm3tosbp"]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "./binaries/linux/rtcm3tosbp"]);
            cmd
        };
        let mut child = child
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("rtcm converter process failed")?;

        let mut child_in = child.stdin.take().context("rtcm3tosbp stdin missing")?;
        let mut child_out = child.stdout.take().context("rtcm3tosbp stdout missing")?;
        let in_rx = self.in_rx.clone();

        thread::spawn(move || {
            if let Err(e) = io::copy(&mut child_out, &mut out) {
                error!("failed to write to device {e}");
            }
        });
        thread::spawn(move || {
            while let Ok(data) = in_rx.recv() {
                if let Err(e) = child_in.write_all(&data) {
                    error!("failed to write to rtcm3tosbp {e}")
                }
            }
        });
        let running = self.running.clone();
        thread::spawn(move || {
            while let Ok(None) = child.try_wait() {
                if !running.get() {
                    let _ = child.kill();
                    break;
                }
            }
        });
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running.set(false);
    }
}
