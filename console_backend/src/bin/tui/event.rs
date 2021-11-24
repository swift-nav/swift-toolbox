use std::thread;
use std::time::Duration;

use crossbeam::channel::{self, Receiver};
use crossterm::event::{self, KeyCode};
use log::error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Input(KeyCode),
    Tick,
}

#[derive(Debug)]
pub struct Events {
    rx: Receiver<Event>,
}

impl Events {
    const TICK: Duration = Duration::from_secs(1);

    pub fn new() -> Events {
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
            thread::sleep(Events::TICK);
        });
        Events { rx }
    }

    pub fn set_mode(&mut self) {}
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}
