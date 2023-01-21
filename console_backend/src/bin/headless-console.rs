// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
    if opt.serial.serial.is_none() && opt.tcp.tcp.is_none() && opt.file.file.is_none() {
        eprintln!(
            r#"
Running in headless mode only command line options work.
Help:
./headless-console --help
Usage:
./headless-console --tcp piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com --port=55555
"#
        );
        return Ok(());
    }
    let (client_send_, client_recv) = channel::unbounded::<Vec<u8>>();
    let (_server_send, server_recv) = channel::unbounded::<Vec<u8>>();
    let client_send = ChannelSender::boxed(client_send_);
    let shared_state = SharedState::new();
    let conn_manager = ConnectionManager::new(client_send.clone(), shared_state.clone());
    handle_cli(opt, &conn_manager, shared_state.clone());
    setup_logging(client_send.clone(), shared_state.clone());
    refresh_connection_frontend(&client_send, &shared_state);
    refresh_loggingbar(&client_send, &shared_state);
    server_recv_thread(conn_manager, client_send, server_recv, shared_state);

    let mut msg_count: usize = 0;
    while client_recv.recv().is_ok() {
        msg_count += 1;
        if msg_count % 100 == 0 {
            let local: DateTime<Local> = Local::now();
            eprintln!("{local} :: Messages received: {msg_count}");
        }
    }
    Ok(())
}
