extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new().file("../src/main/resources/base/console_backend.capnp").run().unwrap();
}