use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(target_os = "windows")]
fn find_py(dir: &Path) -> PathBuf {
    dir.join("pythonw.exe")
}

#[cfg(not(target_os = "windows"))]
fn find_py(dir: &Path) -> PathBuf {
    let py3 = dir.join("python3");
    if py3.exists() {
        py3
    } else {
        dir.join("python")
    }
}

#[cfg(target_os = "windows")]
fn start(py: &Path) -> Result<()> {
    let mut child = Command::new(py)
        .arg("-m")
        .arg("swiftnav_console.main")
        .arg("--")
        .args(env::args().skip(1))
        .spawn()?;
    child.wait()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn start(py: &Path) -> Result<()> {
    use std::os::unix::prelude::CommandExt;
    let err = Command::new(py)
        .arg("-m")
        .arg("swiftnav_console.main")
        .arg("--")
        .args(env::args().skip(1))
        .exec();
    Err(err.into())
}

fn main() -> Result<()> {
    let me = env::current_exe()?;
    let parent = me.parent().ok_or("no parent directory")?;
    let py = find_py(parent);
    start(&py)
}
