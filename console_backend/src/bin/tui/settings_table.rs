use std::cell::RefCell;

use tui::{
    buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState},
};

use console_backend::settings_tab::{Settings, SettingsEntry};

use crate::terminal::Frame;
use crate::Drawable;

pub struct SettingsTable {
    settings: Settings,
    state: RefCell<TableState>,
}

impl SettingsTable {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            state: RefCell::new(Default::default()),
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn find(&self, idx: usize) -> Option<&SettingsEntry> {
        self.settings
            .iter()
            .flat_map(|(_, g)| g.iter())
            .nth(idx)
            .map(|(_, s)| s)
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.borrow().selected()
    }

    pub fn next(&mut self) {
        let mut state = self.state.borrow_mut();
        let i = state.selected().map_or(0, |i| {
            if i >= self.settings.len() - 1 {
                0
            } else {
                i + 1
            }
        });
        state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let mut state = self.state.borrow_mut();
        let i = state.selected().map_or(0, |i| {
            if i == 0 {
                self.settings.len() - 1
            } else {
                i - 1
            }
        });
        state.select(Some(i));
    }

    pub fn reset(&mut self, settings: Settings) {
        self.settings = settings;
        self.state = RefCell::new(Default::default());
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.settings
            .groups()
            .into_iter()
            .flat_map(|s| {
                s.into_iter()
                    .map(|(s, v)| vec![s.name.clone(), v.to_string()])
            })
            .collect()
    }
}

impl StatefulWidget for &SettingsTable {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut buffer::Buffer, state: &mut Self::State) {
        let rows = self.to_rows().into_iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(c.clone()));
            Row::new(cells).height(height as u16)
        });
        let table = Table::new(rows)
            .header(
                Row::new(
                    ["name", "value"]
                        .iter()
                        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red))),
                )
                .style(Style::default().bg(Color::Blue))
                .height(1)
                .bottom_margin(1),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("device settings"),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Max(10),
            ]);
        StatefulWidget::render(table, area, buf, state);
    }
}

impl Drawable for SettingsTable {
    fn draw(&self, f: &mut Frame, rect: Rect) {
        let mut state = self.state.borrow_mut();
        let state = &mut (*state);
        f.render_stateful_widget(self, rect, state);
    }
}
