fn main() {
    println!("cargo:rustc-link-search=py39-dist/lib");
    println!("cargo:rustc-link-lib=static=python3.9");
}
