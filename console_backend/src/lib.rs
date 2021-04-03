pub mod console_backend_capnp {
    include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}
pub mod constants;
pub mod main_tab;
pub mod process_messages;
pub mod server;
pub mod solution_table;
pub mod solution_velocity_tab;
pub mod tracking_signals_tab;
pub mod types;
