mod app;
mod command;
mod event;
mod settings_view;
mod split_view;
mod terminal;
mod text_input;

use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use crossbeam::channel;
use tui::layout::Rect;

use console_backend::{
    client_sender::ChannelSender, connection::ConnectionManager, shared_state::SharedState,
};

use crate::app::App;
use crate::event::Events;
use crate::terminal::Frame;

fn main() -> Result<()> {
    // init_logging()?;
    let terminal = terminal::init()?;
    let (mut conn_manager, shared_state) = start_app()?;
    shared_state.set_log_to_std(false);
    let tabs = conn_manager.wait_for_connection();
    let settings_tab = tabs
        .settings
        .as_ref()
        .context("settings are not supported on this device")?;
    let mut app = App::new(terminal, shared_state, settings_tab);
    let mut events = Events::new();
    while let Some(event) = events.next() {
        // eprintln!("{:?}", event);
        // events.set_mode();
        app.handle_event(event)?;
        // eprintln!("h");
        app.draw()?;
        // eprintln!("d");
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Opts {
    /// The TCP host to connect to.
    #[clap(long, default_value = "10.1.54.2")]
    host: String,

    /// The port to use when connecting via TCP.
    #[clap(long, default_value = "55555")]
    port: u16,
}

fn start_app() -> Result<(ConnectionManager, SharedState)> {
    let (client_send, _) = channel::unbounded();
    let client_sender = ChannelSender::boxed(client_send);
    let shared_state = SharedState::new();
    let conn_manager = ConnectionManager::new(client_sender, shared_state.clone());
    let opt = Opts::parse();
    conn_manager.connect_to_host(opt.host, opt.port)?;
    Ok((conn_manager, shared_state))
}

// TODO: show the user where to find the log file, save it on crash
fn init_logging() -> io::Result<()> {
    let log_file = tempfile::tempfile()?;
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    Ok(())
}

trait Drawable {
    fn draw(&self, f: &mut Frame, rect: Rect);
}
