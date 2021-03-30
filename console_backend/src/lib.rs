pub mod console_backend_capnp {
    include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}
pub mod constants;
pub mod process_messages;
pub mod server;
pub mod tracking_tab;
pub mod types;
