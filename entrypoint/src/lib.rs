use std::io::Cursor;

use lazy_static::lazy_static;

#[cfg(feature = "splash")]
pub mod splash;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    pub static ref SPLASH_IMAGE: image::DynamicImage = {
        let splash_image_buf = include_bytes!(concat!("../../", env!("CONSOLE_SPLASH_IMAGE")));
        image::io::Reader::with_format(
            std::io::BufReader::new(Cursor::new(splash_image_buf)),
            image::ImageFormat::Jpeg,
        )
        .decode()
        .expect("could not decode splash image")
    };
}

#[cfg(feature = "entrypoint")]
pub fn attach_console() {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Console::AttachConsole;
        unsafe {
            AttachConsole(u32::MAX).as_bool();
        }
    }
}
