use anyhow::Result;
use chrono::prelude::*;
use console_backend::{
    cli_options::{handle_cli, CliOptions},
    client_sender::ChannelSender,
    connection::ConnectionManager,
    log_panel::setup_logging,
    server_recv_thread::server_recv_thread,
    shared_state::SharedState,
    utils::{refresh_connection_frontend, refresh_loggingbar},
};
use crossbeam::channel;

fn main() -> Result<()> {
    let opt = CliOptions::from_filtered_cli();
    if opt.input.is_none() {
        eprintln!(
            r#"
Running in headless mode only command line options work.
Help:
./headless-console --help
Usage:
./headless-console tcp piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com --port=55555
"#
        );
        return Ok(());
    }
    let (client_send_, client_recv) = channel::unbounded::<Vec<u8>>();
    let (_server_send, server_recv) = channel::unbounded::<Vec<u8>>();
    let client_send = ChannelSender::boxed(client_send_);
    let shared_state = SharedState::new();
    let conn_manager = ConnectionManager::new(client_send.clone(), shared_state.clone());
    handle_cli(opt, &conn_manager, shared_state.clone(), &client_send);
    setup_logging(client_send.clone(), shared_state.clone());
    refresh_connection_frontend(&client_send, &shared_state);
    refresh_loggingbar(&client_send, &shared_state);
    server_recv_thread(conn_manager, client_send, server_recv, shared_state);

    let mut msg_count: usize = 0;
    while client_recv.recv().is_ok() {
        msg_count += 1;
        if msg_count % 100 == 0 {
            let local: DateTime<Local> = Local::now();
            eprintln!("{} :: Messages received: {}", local, msg_count);
        }
    }
    Ok(())
}
