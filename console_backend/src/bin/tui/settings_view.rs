use std::{
    cell::RefCell,
    io::Write,
    iter,
    ops::{Deref, DerefMut},
};

use sbp_settings::{Setting, SettingValue};
use tabwriter::TabWriter;
use tui::{
    buffer,
    layout::{Constraint, Direction, Rect},
    style::{Modifier, Style},
    widgets::{
        Block, Borders, Cell, List, ListItem, Row, StatefulWidget, Table, TableState, Widget,
    },
};

use console_backend::settings_tab::{Settings, SettingsTab};

use crate::split_view::SplitView;
use crate::terminal::Frame;
use crate::Drawable;

pub struct SettingsView {
    view: SplitView<SettingsTable, SettingsPane>,
    active: bool,
}

impl SettingsView {
    pub fn new(settings_tab: &SettingsTab) -> Self {
        let settings_table = SettingsTable::new(settings_tab.settings().clone());
        let settings_pane = SettingsPane::empty();
        Self {
            view: SplitView::new(settings_table, settings_pane, Direction::Horizontal, 30),
            active: false,
        }
    }

    pub fn selected(&self) -> Option<&(Setting, SettingValue)> {
        self.view.a.selected()
    }

    pub fn next(&mut self) {
        self.view.a.next();
        if let Some((setting, value)) = self.view.a.selected() {
            self.view.b = SettingsPane::new(setting, value);
        } else {
            self.view.b = SettingsPane::empty()
        };
    }

    pub fn previous(&mut self) {
        self.view.a.previous();
        if let Some((setting, value)) = self.view.a.selected() {
            self.view.b = SettingsPane::new(setting, value);
        } else {
            self.view.b = SettingsPane::empty()
        };
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        self.view.draw(f, rect);
    }

    pub fn shift(&mut self) {
        self.view.shift();
    }

    pub fn unshift(&mut self) {
        self.view.unshift();
    }

    pub fn settings(&self) -> &Settings {
        &self.view.a.settings
    }

    pub fn reset(&mut self, settings: Settings) {
        self.view.a.reset(settings);
        self.view.b = SettingsPane::empty();
    }
}

struct SettingsTable {
    settings: Settings,
    groups: Vec<Vec<(Setting, SettingValue)>>,
    header_indexes: Vec<usize>,
    state: RefCell<TableState>,
}

impl SettingsTable {
    fn new(settings: Settings) -> Self {
        let groups = get_groups(&settings);
        Self {
            header_indexes: calc_header_indexes(&groups),
            groups,
            settings,
            state: Default::default(),
        }
    }

    fn select(&self, idx: usize) {
        self.state
            .borrow_mut()
            .select(Some(idx + self.num_headers(idx)));
    }

    fn selected(&self) -> Option<&(Setting, SettingValue)> {
        let selected = self.state.borrow().selected();
        selected.and_then(|idx| {
            let mut n = idx - self.num_headers(idx);
            if n != 0 {
                n -= 1;
            }
            self.groups.iter().flat_map(|g| g.iter()).nth(n)
        })
    }

    fn next(&mut self) {
        let mut state = self.state.borrow_mut();
        match state.selected() {
            Some(idx) => {
                let mut idx = if idx > self.num_settings() + self.header_indexes.len() - 1 {
                    0
                } else {
                    idx + 1
                };
                if self.header_indexes.contains(&idx) {
                    idx += 1;
                }
                state.select(Some(idx));
            }
            None => state.select(Some(1)),
        };
    }

    fn previous(&mut self) {
        let mut state = self.state.borrow_mut();
        match state.selected() {
            Some(idx) => {
                let mut idx = if idx == 1 {
                    self.num_settings() + self.header_indexes.len()
                } else {
                    idx - 1
                };
                if self.header_indexes.contains(&idx) {
                    idx -= 1;
                }
                state.select(Some(idx));
            }
            None => {
                let var_name = self.num_settings() + self.header_indexes.len();
                state.select(Some(var_name))
            }
        };
    }

    fn reset(&mut self, settings: Settings) {
        self.groups = get_groups(&settings);
        self.header_indexes = calc_header_indexes(&self.groups);
        self.settings = settings;
        self.state = Default::default();
    }

    fn num_settings(&self) -> usize {
        self.groups.iter().map(Vec::len).sum()
    }

    fn num_headers(&self, idx: usize) -> usize {
        self.header_indexes.iter().take_while(|o| idx < **o).count()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.groups
            .iter()
            .enumerate()
            .flat_map(|(idx, group)| {
                let header = if idx == 0 {
                    vec![group[0].0.group.to_owned()]
                } else {
                    vec!["\n".to_owned() + &group[0].0.group]
                };
                let rows = group
                    .iter()
                    .map(|(s, v)| vec![s.name.clone(), v.to_string()]);
                iter::once(header).chain(rows)
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
            let cells: Vec<_> = item.iter().map(|c| Cell::from(c.clone())).collect();
            if cells.len() == 1 {
                // header
                Row::new(cells)
                    .height(height as u16)
                    .style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                // row
                Row::new(cells).height(height as u16)
            }
        });
        let table = Table::new(rows)
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

struct SettingsPane {
    list: List<'static>,
}

impl SettingsPane {
    fn new(setting: &Setting, value: &SettingValue) -> Self {
        let items = [
            ("name", setting.name.clone()),
            ("value", value.to_string()),
            ("units", setting.units.clone().unwrap_or_default()),
            ("kind", setting.kind.to_str().to_owned()),
            (
                "default_value",
                setting.default_value.clone().unwrap_or_default(),
            ),
            (
                "description",
                setting.description.clone().unwrap_or_default(),
            ),
            ("notes", setting.notes.clone().unwrap_or_default()),
        ];
        let mut tw = TabWriter::new(vec![]);
        for (h, v) in items {
            writeln!(&mut tw, "{}\t{}", h, v).unwrap();
        }
        tw.flush().unwrap();
        let r = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        let items: Vec<_> = r
            .lines()
            .map(ToOwned::to_owned)
            .map(ListItem::new)
            .collect();
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{} -> {}", setting.group, setting.name)),
        );
        Self { list }
    }

    fn empty() -> Self {
        let list = List::new(Vec::new()).block(Block::default().borders(Borders::ALL));
        Self { list }
    }
}

impl Widget for &SettingsPane {
    fn render(self, area: Rect, buf: &mut buffer::Buffer) {
        Widget::render(self.list.clone(), area, buf);
    }
}

impl Drawable for SettingsPane {
    fn draw(&self, f: &mut Frame, rect: Rect) {
        f.render_widget(self, rect);
    }
}

fn calc_header_indexes(groups: &[Vec<(Setting, SettingValue)>]) -> Vec<usize> {
    let mut indexes = vec![0];
    if groups.is_empty() {
        return indexes;
    }
    for group in groups[0..groups.len() - 1].iter() {
        let last = indexes[indexes.len() - 1];
        indexes.push(last + group.len() + 1);
    }
    indexes
}

fn get_groups(settings: &Settings) -> Vec<Vec<(Setting, SettingValue)>> {
    settings
        .groups()
        .into_iter()
        .map(|g| {
            g.into_iter()
                .map(|(s, v)| (s.to_owned(), v.to_owned()))
                .collect()
        })
        .collect()
}
