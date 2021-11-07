#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use pyo3::prelude::*;

fn attach_console() {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Console::AttachConsole;
        unsafe {
            AttachConsole(u32::MAX).as_bool();
        }
    }
}

fn main() -> Result<()> {
    attach_console();

    let current_exe = std::env::current_exe()?;
    let parent = current_exe.parent().ok_or("no parent directory")?;

    std::env::set_var("SWIFTNAV_CONSOLE_FROZEN", parent);

    std::env::set_var("PYTHONHOME", parent);
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");

    let exit_code = Python::with_gil(|py| {
        let snav = py.import("swiftnav_console.main")?;
        snav.call_method("main", (), None)?.extract::<i32>()
    })?;

    std::process::exit(exit_code);
}
