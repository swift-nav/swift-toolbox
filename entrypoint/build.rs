#[cfg(target_os = "windows")]
use winres::WindowsResource;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()>{
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
    Ok(())
}
