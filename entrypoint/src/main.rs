#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
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

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use entrypoint::attach_console;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn handle_wayland() {
    #[cfg(target_os = "linux")]
    {
        const QT_QPA_PLATFORM: &str = "QT_QPA_PLATFORM";
        const XDG_SESSION_TYPE: &str = "XDG_SESSION_TYPE";
        const WAYLAND: &str = "wayland";

        if std::env::var(XDG_SESSION_TYPE).as_deref() == Ok(WAYLAND) {
            // Only override QT_QPA_PLATFORM if it's not already set
            if std::env::var(QT_QPA_PLATFORM).is_err() {
                std::env::set_var(QT_QPA_PLATFORM, WAYLAND);
            }
        }
    }
}

fn handle_splash() {
    #[cfg(feature = "splash")]
    {
        std::env::set_var(
            "SWIFTNAV_CONSOLE_SPLASH",
            entrypoint::splash::marker_filepath(),
        );
        entrypoint::splash::spawn();
    }
}

fn handle_debug() {
    if std::env::var("SWIFTNAV_CONSOLE_DEBUG").is_ok() {
        for (key, value) in std::env::vars() {
            eprintln!("{key}={value}");
        }
    }
}

fn app_dir() -> Result<PathBuf> {
    let current_exe = std::env::current_exe()?;
    current_exe
        .parent()
        .ok_or_else(|| "no parent directory".into())
        .map(Path::to_path_buf)
}

fn pythonhome_dir() -> Result<PathBuf> {
    let app_dir = app_dir()?;
    if cfg!(target_os = "macos") {
        if let Some(parent) = app_dir.parent() {
            let resources = parent.join("Resources/lib");
            if resources.exists() {
                Ok(parent.join("Resources"))
            } else {
                Ok(app_dir)
            }
        } else {
            Ok(app_dir)
        }
    } else {
        Ok(app_dir)
    }
}

fn webengine_dir() -> Result<PathBuf> {
    let app_dir = pythonhome_dir()?;
    if cfg!(target_os = "macos") {
        Ok(app_dir.join("lib/python3.11/site-packages/PySide6/Qt/lib/QtWebEngineCore.framework/Versions/A/Helpers/QtWebEngineProcess.app/Contents/MacOS/QtWebEngineProcess"))
    } else if cfg!(target_os = "windows") {
        Ok(app_dir.join("Lib/site-packages/PySide6/QtWebEngineProcess.exe"))
    } else {
        Ok(app_dir.join("lib/python3.11/site-packages/PySide6/Qt/libexec/QtWebEngineProcess"))
    }
}

fn main() -> Result<()> {
    attach_console();
    handle_wayland();
    handle_debug();
    let args: Vec<_> = std::env::args().collect();

    std::env::set_var("SWIFTNAV_CONSOLE_FROZEN", app_dir()?);
    std::env::set_var("PYTHONHOME", pythonhome_dir()?);
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");

    std::env::set_var("QTWEBENGINEPROCESS_PATH", webengine_dir()?);
    std::env::set_var("QTWEBENGINE_DISABLE_SANDBOX", "1");

    if cfg!(target_os = "windows") {
        let app_dir = pythonhome_dir()?;
        std::env::set_var(
            "QTWEBENGINE_LOCALES_PATH",
            app_dir.join("Lib/site-packages/PySide6/translations/qtwebengine_locales"),
        );

        std::env::set_var(
            "QTWEBENGINE_RESOURCES_PATH",
            app_dir.join("Lib/site-packages/PySide6/resources"),
        );
    }

    handle_splash();
    let exit_code = Python::with_gil(|py| {
        let args = PyTuple::new(py, args);
        let snav = py.import("swiftnav_console.main")?;
        snav.call_method("main", (args,), None)?.extract::<i32>()
    })?;
    std::process::exit(exit_code);
}
