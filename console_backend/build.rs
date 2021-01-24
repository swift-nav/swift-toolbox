extern crate capnpc;

fn main() {

    let output_dir = std::env::var("OUT_DIR").unwrap();
    let input_path = format!("{}/console_backend.capnp", output_dir);

    std::fs::copy("../src/main/resources/base/console_backend.capnp", &input_path).unwrap();

    ::capnpc::CompilerCommand::new()
        .file(input_path)
        .output_path(output_dir)
        .run()
        .unwrap();
}