use crate::types::{ArcBool, Result};
use crate::utils::pythonhome_dir;
use anyhow::Context;
use crossbeam::channel::Receiver;
use log::error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs, io, thread};

pub struct MessageConverter {
    in_rx: Receiver<Vec<u8>>,
    running: ArcBool,
    binary_path: String,
}

impl MessageConverter {
    pub fn new(in_rx: Receiver<Vec<u8>>, binary_path: String) -> Self {
        Self {
            in_rx,
            running: ArcBool::new(),
            binary_path,
        }
    }

    pub fn start<W: Write + Send + 'static>(&mut self, out: W) -> Result<()> {
        self.running.set(true);
        if self.binary_path.is_empty() {
            return self.output_rtcm(out);
        }
        let path = self.binary_path.clone();
        let meta = fs::metadata(&path);
        match meta {
            Ok(meta) => {
                if meta.is_file() {
                    self.output_sbp(out, &path)
                } else {
                    error!("error rtcm converter path is not a file, defaulting to send RTCM");
                    self.output_rtcm(out)
                }
            }
            _ => {
                error!("error in opening rtcm converter path, defaulting to send RTCM");
                self.output_rtcm(out)
            }
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
    fn output_sbp<W: Write + Send + 'static>(&mut self, mut out: W, path: &str) -> Result<()> {
        let mut child = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", path]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", path]);
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
