use console_backend::{
    cli_options::{handle_cli, CliOptions},
    connection::ConnectionState,
    log_panel::setup_logging,
    shared_state::{backend_recv_thread, SharedState},
    types::{ClientSender, Result},
    utils::{refresh_loggingbar, refresh_navbar},
};
use crossbeam::{channel, scope};
fn main() -> Result<()> {
    scope(|s| {
        let (client_send_, client_recv) = channel::unbounded::<Vec<u8>>();
        let (_server_send, server_recv) = channel::unbounded::<Vec<u8>>();
        let client_send = ClientSender::new(client_send_);
        setup_logging(client_send.clone(), true);
        let opt = CliOptions::from_filtered_cli();
        let shared_state = SharedState::new();
        let connection_state = ConnectionState::new(client_send.clone(), shared_state.clone());
        if opt.input.is_none() {
            eprintln!("\nRunning in headless mode only command line options work.");
            eprintln!("Help:");
            eprintln!("\tcargo make headless-run -- --help");
            eprintln!("Usage:");
            eprintln!("\tcargo make headless-run -- tcp piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com --port=55555\n");
        } else {
            handle_cli(opt, &connection_state, shared_state.clone());
            refresh_navbar(&mut client_send.clone(), shared_state.clone());
            refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
            backend_recv_thread(connection_state, client_send, server_recv, shared_state);
            s.spawn(move |_| {
                loop {
                    if client_recv.recv().is_err() {
                        break;
                    }
                }
            })
            .join().unwrap();
        }
    }).unwrap();
    Ok(())
}
