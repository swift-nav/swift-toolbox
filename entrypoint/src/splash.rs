#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{
    io::Cursor,
    path::PathBuf,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use minifb::{Window, WindowOptions};
use winit::{event_loop::EventLoop, monitor::MonitorHandle, window::Window as WinitWindow};

use entrypoint::attach_console;

const TIMEOUT_DURATION: Duration = Duration::from_secs(30);
const TEMP_FILENAME: &str = "swiftnav_console";

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref PID_FILE: PathBuf = {
        let pid = std::process::id();
        std::env::temp_dir()
            .join(format!("{TEMP_FILENAME}.{pid}"))
            .into()
    };
}

fn rgb8_3_to_rgb32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn create_temp_file() -> Result<PathBuf> {
    std::fs::File::create(&*PID_FILE)?;
    Ok(PID_FILE.clone())
}

fn fetch_scale_factor(monitor: &MonitorHandle) -> f64 {
    #[cfg(target_os = "macos")]
    {
        monitor.scale_factor()
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = monitor;
        1.0
    }
}

fn launch_splash() -> Result<()> {
    attach_console();
    let logo = include_bytes!("../../resources/images/splash.jpg");
    let image = image::io::Reader::with_format(
        std::io::BufReader::new(Cursor::new(logo)),
        image::ImageFormat::Jpeg,
    )
    .decode()?;
    let u32_buffer: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| rgb8_3_to_rgb32(v[0], v[1], v[2]))
        .collect();
    let current_monitor = WinitWindow::new(&EventLoop::new())?
        .current_monitor()
        .ok_or_else(|| Into::<Error>::into(String::from("could not get current monitor")))?;
    let size = current_monitor.size();
    let scale_factor = fetch_scale_factor(&current_monitor);
    let pos_x = ((size.width as f64 / scale_factor - image.width() as f64) / 2.0) as isize;
    let pos_y = ((size.height as f64 / scale_factor - image.height() as f64) / 2.0) as isize;

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

    let temp_filename = create_temp_file()?;
    let now = Instant::now();
    while window.is_open() && now.elapsed() < TIMEOUT_DURATION && temp_filename.exists() {
        window.update_with_buffer(&u32_buffer, image.width() as usize, image.height() as usize)?;
        window.set_position(pos_x, pos_y);
    }

    Ok(())
}

fn main() -> Result<()> {
    let result = launch_splash();
    if let Err(ref err) = result {
        eprint!("Error launching splash screen: {err}");
    }
    // Try to remove the file, don't care about the result
    let _result = std::fs::remove_file(&*PID_FILE);
    result
}
