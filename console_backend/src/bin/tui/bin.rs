mod settings_pane;
mod settings_table;
mod terminal;

use std::{io, thread, time::Duration};

use anyhow::Result;
use clap::Parser;
use crossbeam::channel::{self, Receiver};
use crossterm::event::{self, KeyCode};
use log::error;
use tui::layout::{Constraint, Direction, Layout, Rect};

use console_backend::{
    client_sender::ChannelSender, connection::ConnectionManager, shared_state::SharedState,
};

use crate::settings_pane::SettingsPane;
use crate::settings_table::SettingsTable;
use crate::terminal::Frame;

fn main() -> Result<()> {
    init_logging()?;
    let mut terminal = terminal::init()?;
    let (mut conn_manager, shared_state) = start_app()?;
    let (tabs, settings) = conn_manager.wait_for_connection();
    let settings_tab = settings.unwrap();

    let events = Events::new();

    let mut settings_table = SettingsTable::new(settings_tab.settings().clone());
    let mut settings_pane = SettingsPane::empty();

    // let mut split = Split::new(30, settings_table, settings_pane);

    for event in events {
        match event {
            Event::Tick => {
                if (&*settings_tab.settings()) != settings_table.settings() {
                    settings_table.reset(settings_tab.settings().clone());
                }
            }
            Event::Input(KeyCode::Up) => settings_table.previous(),
            Event::Input(KeyCode::Down) => settings_table.next(),
            // Event::Input(KeyCode::Char('+')) => settings_pane.bigger(),
            // Event::Input(KeyCode::Char('-')) => settings_pane.smaller(),
            Event::Input(KeyCode::Esc | KeyCode::Char('q')) => break,
            _ => continue,
        }

        terminal.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(settings_pane.width()),
                    Constraint::Min(0),
                ])
                .split(f.size());

            settings_table.draw(f, rects[0]);

            if let Some(idx) = settings_table.selected() {
                let entry = settings_table.find(idx).unwrap();
                settings_pane = SettingsPane::new(entry);
            } else {
                settings_pane = SettingsPane::empty()
            };
            settings_pane.draw(f, rects[1]);
        })?;
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

fn init_logging() -> io::Result<()> {
    let log_file = tempfile::tempfile()?;
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    Ok(())
}

struct Split<T, U> {
    direction: Direction,
    at: u16,
    a: T,
    b: U,
}

impl<T, U> Split<T, U> {
    fn new(direction: Direction, at: u16, a: T, b: U) -> Self {
        Self {
            direction,
            at,
            a,
            b,
        }
    }

    fn shift(&mut self) {
        self.at = (self.at + 10).max(100);
    }

    fn unshift(&mut self) {
        self.at = self.at.saturating_sub(10);
    }
}

impl<T: Drawable, U: Drawable> Drawable for Split<T, U> {
    fn draw(&self, f: &mut Frame, rect: Rect) {
        let rects = Layout::default()
            .direction(self.direction)
            .constraints([Constraint::Percentage(self.at), Constraint::Min(0)])
            .split(f.size());
        self.a.draw(f, rects[0]);
        self.b.draw(f, rects[1]);
    }
}

trait Drawable {
    fn draw(&self, f: &mut Frame, rect: Rect);
}

#[derive(Debug)]
enum Event {
    Input(KeyCode),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
#[derive(Debug)]
struct Events {
    rx: Receiver<Event>,
}

#[derive(Debug, Clone, Copy)]
struct Config {
    tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(1000),
        }
    }
}

impl Events {
    fn new() -> Events {
        Events::with_config(Config::default())
    }

    fn with_config(config: Config) -> Events {
        let (tx, rx) = channel::unbounded();
        thread::spawn({
            let tx = tx.clone();
            move || {
                while let Ok(evt) = event::read() {
                    if let event::Event::Key(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key.code)) {
                            error!("{}", err);
                            return;
                        }
                    }
                }
            }
        });
        thread::spawn(move || loop {
            if let Err(err) = tx.send(Event::Tick) {
                error!("{}", err);
                break;
            }
            thread::sleep(config.tick_rate);
        });
        Events { rx }
    }
}

impl IntoIterator for Events {
    type Item = Event;
    type IntoIter = channel::IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.into_iter()
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}
