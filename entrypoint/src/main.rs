#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use entrypoint::attach_console;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

fn main() -> Result<()> {
    attach_console();
    handle_wayland();
    let current_exe = std::env::current_exe()?;
    let parent = current_exe.parent().ok_or("no parent directory")?;
    let args: Vec<_> = std::env::args().collect();
    std::env::set_var("SWIFT_CONSOLE_SPLASH", entrypoint::splash::marker_filepath());
    std::env::set_var("SWIFTNAV_CONSOLE_FROZEN", parent);
    std::env::set_var("PYTHONHOME", parent);
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");
    entrypoint::splash::spawn();
    let exit_code = Python::with_gil(|py| {
        let args = PyTuple::new(py, args);
        let snav = py.import("swiftnav_console.main")?;
        snav.call_method("main", (args,), None)?.extract::<i32>()
    })?;
    std::process::exit(exit_code);
}
