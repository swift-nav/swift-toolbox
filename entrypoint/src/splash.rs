#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{
    io::Cursor,
    path::PathBuf,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use minifb::{Window, WindowOptions};

const STARTUP_TIMEOUT_DURATION: Duration = Duration::from_secs(1);
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
    static ref SPLASH_IMAGE: image::DynamicImage = {
        let splash_image_buf = include_bytes!("../../resources/images/splash.jpg");
        image::io::Reader::with_format(
            std::io::BufReader::new(Cursor::new(splash_image_buf)),
            image::ImageFormat::Jpeg,
        )
        .decode()
        .expect("could not decode splash image")
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

fn splash_position() -> Result<(isize, isize)> {
    let monitor = {
        let init_pos = winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(0.0, 0.0));
        let init_size = winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(1.0, 1.0));
        let event_loop = winit::event_loop::EventLoop::new();
        winit::window::WindowBuilder::new()
            .with_visible(false)
            .with_decorations(false)
            .with_inner_size(init_size)
            .with_position(init_pos)
            .build(&event_loop)?
            .current_monitor()
            .or(event_loop.primary_monitor())
            .or(event_loop.available_monitors().take(1).next())
    };
    let image = &SPLASH_IMAGE;
    let (pos_x, pos_y) = if let Some(monitor) = monitor {
        let size = monitor.size();
        let (width, height) = if cfg!(target_os = "macos") {
            let size = size.to_logical::<f64>(monitor.scale_factor());
            (size.width, size.height)
        } else {
            (size.width as f64, size.height as f64)
        };
        let pos_x = ((width - image.width() as f64) / 2.0) as isize;
        let pos_y = ((height - image.height() as f64) / 2.0) as isize;
        (pos_x, pos_y)
    } else {
        (20, 20)
    };
    Ok((pos_x, pos_y))
}

fn launch_splash(pos_x: isize, pos_y: isize) -> Result<()> {
    let image = &SPLASH_IMAGE;
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
        window.update_with_buffer(&image_buffer, image.width() as usize, image.height() as usize)?;
        window.set_position(pos_x, pos_y);
    }
    Ok(())
}

pub fn spawn() -> Result<()> {
    let now = Instant::now();
    while !marker_exists() {
        std::thread::sleep(Duration::from_millis(50));
        if now.elapsed() > STARTUP_TIMEOUT_DURATION {
            return Ok(());
        }
    }
    let (pos_x, pos_y) = splash_position()?;
    std::thread::spawn(move || {
       let result = launch_splash(pos_x, pos_y);
        if let Err(ref err) = result {
            eprint!("Error launching splash screen: {err}");
        }
        // Try to remove the file, don't care about the result
        let _result = std::fs::remove_file(&*PID_FILE);
        if let Err(e) = result {
            eprintln!("error launching splash: {e}");
        }
    });
    Ok(())
}