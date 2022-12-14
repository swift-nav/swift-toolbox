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

#[cfg(target_os = "windows")]
use winres::WindowsResource;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=crypt");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
    }

    #[cfg(target_os = "windows")]
    {
        WindowsResource::new()
            .set_icon("../resources/images/icon.ico")
            .compile()?;
    }

    if std::fs::metadata("../resources/images/splash-version.jpg").is_ok() {
        println!("cargo:rustc-env=CONSOLE_SPLASH_IMAGE=resources/images/splash-version.jpg");
        println!("cargo:rerun-if-changed=../resources/images/splash-version.jpg");
    } else {
        println!("cargo:rustc-env=CONSOLE_SPLASH_IMAGE=resources/images/splash.jpg");
        println!("cargo:rerun-if-changed=../resources/images/splash.jpg");
    }

    Ok(())
}
