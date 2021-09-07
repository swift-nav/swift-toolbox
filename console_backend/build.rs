extern crate capnpc;

fn main() {
    let output_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=console_backend.capnp");
    ::capnpc::CompilerCommand::new()
        .file("console_backend.capnp")
        .output_path(output_dir)
        .run()
        .unwrap();
}
