#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use minifb::{Window, WindowOptions};
use std::{
    io::Cursor,
    path::PathBuf,
    time::{Duration, Instant},
};
use winit::{event_loop::EventLoop, window::Window as WinitWindow};
const TIMEOUT_DURATION: Duration = Duration::from_secs(15);
const TEMP_FILENAME: &str = "swiftnav_console";

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn rgb8_3_to_rgb32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn create_temp_file() -> Result<PathBuf> {
    let pid = std::process::id();
    let temp_filename = std::env::temp_dir().join(format!("{TEMP_FILENAME}.{pid}"));
    std::fs::File::create(&temp_filename)?;
    Ok(temp_filename)
}

fn main() {
    let logo = include_bytes!("../../resources/images/splash.jpg");
    let image = image::io::Reader::with_format(
        std::io::BufReader::new(Cursor::new(logo)),
        image::ImageFormat::Jpeg,
    )
    .decode()
    .unwrap();
    let u32_buffer: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| rgb8_3_to_rgb32(v[0], v[1], v[2]))
        .collect();
    let current_monitor = WinitWindow::new(&EventLoop::new())
        .unwrap()
        .current_monitor()
        .unwrap();
    let size = current_monitor.size();
    let pos_x = ((size.width as f64 / current_monitor.scale_factor() - image.width() as f64) / 2.0)
        as isize;
    let pos_y = ((size.height as f64 / current_monitor.scale_factor() - image.height() as f64)
        / 2.0) as isize;

    let mut window = Window::new(
        "",
        image.width() as usize,
        image.height() as usize,
        WindowOptions {
            title: false,
            borderless: true,
            topmost: true,
            none: true,
            ..WindowOptions::default()
        },
    )
    .expect("unable to open window");

    let temp_filename = create_temp_file().unwrap();
    let now = Instant::now();
    while window.is_open() && now.elapsed() < TIMEOUT_DURATION && temp_filename.exists() {
        window
            .update_with_buffer(&u32_buffer, image.width() as usize, image.height() as usize)
            .unwrap();
        window.set_position(pos_x, pos_y);
    }
}
