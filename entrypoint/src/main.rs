#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn attach_console() {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Console::AttachConsole;
        unsafe {
            AttachConsole(u32::MAX).as_bool();
        }
    }
}

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

    std::env::set_var("SWIFTNAV_CONSOLE_FROZEN", parent);

    std::env::set_var("PYTHONHOME", parent);
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");
    let args: Vec<_> = std::env::args().collect();
    let exit_code = Python::with_gil(|py| {
        let args = PyTuple::new(py, args);
        let snav = py.import("swiftnav_console.main")?;
        snav.call_method("main", (args,), None)?.extract::<i32>()
    })?;

    std::process::exit(exit_code);
}
