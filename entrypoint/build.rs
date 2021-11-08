fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=crypt");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
    }
}
