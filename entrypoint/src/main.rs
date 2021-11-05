#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

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

    Python::with_gil(|py| {

        let locals = [
            ("sys", py.import("sys")?),
            ("snav", py.import("swiftnav_console.main")?)
        ].into_py_dict(py);

        let code = "snav.main()";

        py.eval(code, None, Some(locals)).unwrap();

        Ok(())
    })
}
