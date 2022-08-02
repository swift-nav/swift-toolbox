#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

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
        .ok_or("no parent directory".into())
        .map(Path::to_path_buf)
}

fn pythonhome_dir() -> Result<PathBuf> {
    let app_dir = app_dir()?.to_path_buf();
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

fn main() -> Result<()> {
    attach_console();
    handle_wayland();
    handle_debug();
    let args: Vec<_> = std::env::args().collect();
    std::env::set_var("SWIFTNAV_CONSOLE_FROZEN", app_dir()?);
    std::env::set_var("PYTHONHOME", pythonhome_dir()?);
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");

    // This enables the Qt Rendering Hardware Interface (RHI), to reduce observed render
    // glitching on Windows. You can disable this feature by setting QSG_RHI=0 in your
    // environment variables.
    #[cfg(target_os = "windows")]
    if std::env::var("QSG_RHI").is_err() {
        std::env::set_var("QSG_RHI", "1");
    }

    handle_splash();
    let exit_code = Python::with_gil(|py| {
        let args = PyTuple::new(py, args);
        let snav = py.import("swiftnav_console.main")?;
        snav.call_method("main", (args,), None)?.extract::<i32>()
    })?;
    std::process::exit(exit_code);
}
