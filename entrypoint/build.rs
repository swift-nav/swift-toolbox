fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=crypt");
        println!("cargo:rustc-link-arg=-fuse-ld=lld-13");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/lib");
    }
}
