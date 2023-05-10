use std::{
    path::PathBuf,
    process::Command,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use minifb::{Window, WindowOptions};

use crate::Result;

const STARTUP_TIMEOUT_DURATION: Duration = Duration::from_secs(2);
const TIMEOUT_DURATION: Duration = Duration::from_secs(30);
const TEMP_FILENAME: &str = "swiftnav_console";

lazy_static! {
    static ref PID_FILE: PathBuf = {
        let pid = std::process::id();
        std::env::temp_dir().join(format!("{TEMP_FILENAME}.{pid}"))
    };
}

fn rgb8_3_to_rgb32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn marker_filepath() -> PathBuf {
    PID_FILE.clone()
}

fn marker_exists() -> bool {
    std::fs::metadata(&*PID_FILE).is_ok()
}

fn ensure_no_marker() {
    let _ = std::fs::remove_file(marker_filepath());
}

fn launch_splash(pos_x: isize, pos_y: isize) -> Result<()> {
    let image = &crate::SPLASH_IMAGE;
    let image_buffer: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| rgb8_3_to_rgb32(v[0], v[1], v[2]))
        .collect();
    let mut window = Window::new(
        "",
        image.width() as usize,
        image.height() as usize,
        WindowOptions {
            title: false,
            borderless: true,
            none: true,
            ..WindowOptions::default()
        },
    )?;
    let now = Instant::now();
    while window.is_open() && now.elapsed() < TIMEOUT_DURATION && marker_exists() {
        window.update_with_buffer(
            &image_buffer,
            image.width() as usize,
            image.height() as usize,
        )?;
        window.set_position(pos_x, pos_y);
    }
    Ok(())
}

fn splash_position() -> Result<(isize, isize)> {
    let current_exe = std::env::current_exe()?;
    let parent = current_exe.parent().ok_or("no parent directory")?;
    let stdout = Command::new(parent.join("windowpos")).output()?.stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    let xy: Vec<&str> = stdout.split_whitespace().collect();
    let (x, y) = (xy[0].parse::<isize>()?, xy[1].parse::<isize>()?);
    Ok((x, y))
}

pub fn spawn() {
    ensure_no_marker();
    std::thread::spawn(|| {
        let now = Instant::now();
        while !marker_exists() {
            std::thread::sleep(Duration::from_millis(100));
            if now.elapsed() > STARTUP_TIMEOUT_DURATION {
                eprintln!("splash: marker never existed, exiting...");
                return;
            }
        }
        eprintln!("splash: launching");
        let (pos_x, pos_y) = match splash_position() {
            Ok((pos_x, pos_y)) => (pos_x, pos_y),
            Err(err) => {
                eprint!("splash: error launching: {err}");
                (20, 20)
            }
        };
        let result = launch_splash(pos_x, pos_y);
        if let Err(ref err) = result {
            eprint!("splash: error launching: {err}");
        }
        // Try to remove the file, don't care about the result
        let _result = std::fs::remove_file(&*PID_FILE);
        if let Err(ref err) = result {
            eprintln!("splash: error launching: {err}");
        }
    });
}
