use anyhow::Result;
use chrono::prelude::*;
use console_backend::{
    cli_options::{handle_cli, CliOptions},
    connection::ConnectionState,
    log_panel::setup_logging,
    server_recv_thread::server_recv_thread,
    shared_state::SharedState,
    types::ClientSender,
    utils::{refresh_loggingbar, refresh_navbar},
};
use crossbeam::channel;
use std::fs::File;
use std::io::{prelude::*, BufWriter};

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
    let client_send = ClientSender::new(client_send_);
    setup_logging(client_send.clone(), true);
    let shared_state = SharedState::new();
    let connection_state = ConnectionState::new(client_send.clone(), shared_state.clone());
    handle_cli(opt, &connection_state, shared_state.clone());
    refresh_navbar(&mut client_send.clone(), shared_state.clone());
    refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
    server_recv_thread(connection_state, client_send, server_recv, shared_state);

    let local: DateTime<Local> = Local::now();
    let mut filename = format!("headless-console-{}.data", local);
    filename.retain(|c| !c.is_whitespace());
    let file_out = File::create(filename)?;
    let buf_out = BufWriter::new(file_out);
    let mut msg_count: usize = 0;
    let mut file_out = snap::write::FrameEncoder::new(buf_out);
    while let Ok(msg) = client_recv.recv() {
        match file_out.write_all(&msg) {
            Ok(_) => {
                msg_count += 1;
                if msg_count % 100 == 0 {
                    println!("Messages received: {}", msg_count);
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    Ok(())
}
