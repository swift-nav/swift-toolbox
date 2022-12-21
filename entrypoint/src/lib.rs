// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
    if std::env::var("SWIFTNAV_CONSOLE_DEBUG").is_ok() {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::System::Console::AttachConsole;
            unsafe {
                AttachConsole(u32::MAX).as_bool();
            }
        }
    }
}
