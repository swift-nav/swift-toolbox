#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use minifb::{Window, WindowOptions};
use std::{
    io::{Cursor, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

const TIMEOUT_DURATION: Duration = Duration::from_secs(15);
const TEMP_FILENAME: &str = "swiftnav_console.{pid}"; // don't think a template like this will actually work, but you get the idea
const NAIVE_UPPER_LEFT_X: isize = 20;
const NAIVE_UPPER_LEFT_Y: isize = 20;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn rgb8_3_to_rgb32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn create_temp_file(path: &PathBuf) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    let pid = std::process::id();
    write!(file, "{pid}")?;
    Ok(())
}

fn main() {
    let logo = include_bytes!("../../resources/images/LogoBackground.jpg");
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
    )
    .expect("unable to open window");
    let temp_filename = std::env::temp_dir().join(TEMP_FILENAME);
    create_temp_file(&temp_filename).unwrap();
    let now = Instant::now();
    while window.is_open() && now.elapsed() < TIMEOUT_DURATION && temp_filename.exists() {
        window
            .update_with_buffer(&u32_buffer, image.width() as usize, image.height() as usize)
            .unwrap();
        window.set_position(NAIVE_UPPER_LEFT_X, NAIVE_UPPER_LEFT_Y);
    }
}
