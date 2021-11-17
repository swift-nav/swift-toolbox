use std::io::Write;

use tabwriter::TabWriter;
use tui::{
    buffer,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, Widget},
};

use console_backend::settings_tab::SettingsEntry;

use crate::{terminal::Frame, Drawable};

pub struct SettingsPane {
    list: List<'static>,
}

impl SettingsPane {
    pub fn new(SettingsEntry { setting, value }: &SettingsEntry) -> Self {
        let items = [
            ("name", setting.name.clone()),
            (
                "value",
                value.as_ref().map_or_else(String::new, ToString::to_string),
            ),
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

    pub fn empty() -> Self {
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
