use crate::tabs::advanced_tab::ntrip_tab::OutputType;
use crate::types::Result;
use crate::utils::pythonhome_dir;
use anyhow::Context;
use crossbeam::channel::Receiver;
use log::{error, info};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{io, thread};

pub struct MessageConverter {
    in_rx: Receiver<Vec<u8>>,
    output_type: OutputType,
}

impl MessageConverter {
    pub fn new(in_rx: Receiver<Vec<u8>>, output_type: OutputType) -> Self {
        Self { in_rx, output_type }
    }

    pub fn start<W: Write + Send + 'static>(&mut self, out: W) -> Result<()> {
        match self.output_type {
            OutputType::RTCM => self.output_rtcm(out),
            OutputType::SBP => self.output_sbp(out),
        }
    }

    /// Just redirects directly to writer
    fn output_rtcm<W: Write + Send + 'static>(&mut self, mut out: W) -> Result<()> {
        let in_rx = self.in_rx.clone();
        thread::spawn(move || loop {
            if let Ok(data) = in_rx.recv() {
                if let Err(e) = out.write(&data) {
                    error!("failed to write to device {e}");
                }
            }
        });
        Ok(())
    }

    /// Runs rtcm3tosbp converter
    fn output_sbp<W: Write + Send + 'static>(&mut self, mut out: W) -> Result<()> {
        let mut child = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            let rtcm = pythonhome_dir()?
                .join("binaries")
                .join("windows")
                .join("rtcm3tosbp.exe")
                .to_string_lossy()
                .to_string();
            info!("running rtcm3tosbp from \"{}\"", rtcm);
            cmd.args(["/C", &rtcm]);
            cmd
        } else if cfg!(target_os = "macos") {
            let mut cmd = Command::new("sh");
            let rtcm = pythonhome_dir()?
                .join("binaries")
                .join("mac")
                .join("rtcm3tosbp")
                .to_string_lossy()
                .to_string();
            info!("running rtcm3tosbp from \"{}\"", rtcm);
            cmd.args(["-c", &rtcm]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            let rtcm = pythonhome_dir()?
                .join("binaries")
                .join("linux")
                .join("rtcm3tosbp")
                .to_string_lossy()
                .to_string();
            info!("running rtcm3tosbp from \"{}\"", rtcm);
            cmd.args(["-c", &rtcm]);
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
        thread::spawn(move || child.wait());
        Ok(())
    }
}
