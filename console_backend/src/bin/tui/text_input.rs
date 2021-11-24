use rustyline::error::ReadlineError;
use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::Drawable;

pub struct TextInput {
    title: String,
    buffer: String,
    active: bool,
    // editor: rustyline::Editor<()>,
}

impl TextInput {
    pub fn new(title: impl Into<String>) -> Self {
        TextInput {
            title: title.into(),
            buffer: String::new(),
            active: false,
            // editor: rustyline::Editor::new(),
        }
    }

    pub fn readline(&mut self) -> Result<(), ReadlineError> {
        eprintln!("readline");
        // let line = self.editor.readline(">>")?;
        // self.buffer = line;
        Ok(())
    }

    pub fn push(&mut self, c: char) {
        self.buffer.push(c)
    }

    pub fn pop(&mut self) {
        self.buffer.pop();
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn input(&self) -> &str {
        &self.buffer
    }
}

impl Drawable for TextInput {
    fn draw(&self, f: &mut crate::terminal::Frame, rect: tui::layout::Rect) {
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            &self.title,
            Style::default().add_modifier(if self.active {
                Modifier::BOLD
            } else {
                Modifier::DIM
            }),
        ));
        let lines: Vec<_> = self.buffer.as_str().lines().map(Span::from).collect();
        let spans = Spans::from(lines);
        let pg = Paragraph::new(spans).block(block);
        f.render_widget(pg, rect);
    }
}
