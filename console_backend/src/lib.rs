
pub mod server;
pub mod icbins_msg_utils;
pub mod console_backend_capnp {
  include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}