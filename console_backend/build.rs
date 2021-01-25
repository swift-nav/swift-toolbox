extern crate capnpc;

fn main() {

    let output_dir = std::env::var("OUT_DIR").unwrap();

    ::capnpc::CompilerCommand::new()
        .file("console_backend.capnp")
        .output_path(output_dir)
        .run()
        .unwrap();
}