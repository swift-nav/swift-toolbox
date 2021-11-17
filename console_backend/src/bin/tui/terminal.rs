use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::error;
use tui::terminal::{CompletedFrame, Terminal};

pub type Backend = tui::backend::CrosstermBackend<io::Stdout>;
pub type Frame<'a> = tui::terminal::Frame<'a, Backend>;

pub fn init() -> io::Result<Term> {
    let backend = Backend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(Term(terminal))
}

pub struct Term(Terminal<Backend>);

impl Term {
    pub fn draw<F: FnOnce(&mut Frame)>(&mut self, f: F) -> io::Result<CompletedFrame> {
        self.0.draw(f)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.0.clear()
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if let Err(e) = self.clear() {
            error!("error clearing screen: {}", e);
        };
        if let Err(e) = disable_raw_mode() {
            error!("error disabling raw_mode: {}", e);
        };
    }
}
