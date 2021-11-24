use std::io;

use anyhow::{bail, Result};
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout};

use console_backend::settings_tab::SettingsTab;
use console_backend::shared_state::SharedState;

use crate::event::Event;
use crate::settings_view::SettingsView;
use crate::terminal::Term;
use crate::text_input::TextInput;
use crate::Drawable;

pub struct App<'a> {
    terminal: Term,
    shared_state: SharedState,
    mode: InputMode,
    command: TextInput,
    settings_tab: &'a SettingsTab<'static>,
    settings_view: SettingsView,
}

impl<'a> App<'a> {
    pub fn new(
        terminal: Term,
        shared_state: SharedState,
        settings_tab: &'a SettingsTab<'static>,
    ) -> Self {
        Self {
            terminal,
            shared_state,
            mode: InputMode::Normal,
            command: TextInput::new("command"),
            settings_tab,
            settings_view: SettingsView::new(settings_tab),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if event == Event::Tick {
            // eprintln!("1");
            let settings = self.settings_tab.settings();
            // eprintln!("2");
            if &settings != self.settings_view.settings() {
                // eprintln!("3");;
                // eprintln!("4");
                self.settings_view.reset(settings);
                // eprintln!("5");
            }
            // eprintln!("4");
            return Ok(());
        }
        match self.mode {
            InputMode::Normal => match event {
                Event::Input(KeyCode::Char('r')) => self.terminal.clear()?, // TODO: remove
                Event::Input(KeyCode::Char('q')) => bail!("quit"),
                Event::Input(KeyCode::Char('e')) => self.shared_state.set_settings_refresh(true), // TODO: remove

                // Event::Input(KeyCode::Esc) => self.mode = InputMode::Command,
                Event::Input(KeyCode::Esc) => {
                    self.mode = InputMode::Command;
                }
                Event::Input(KeyCode::Up) => self.settings_view.previous(),
                Event::Input(KeyCode::Down) => self.settings_view.next(),
                Event::Input(KeyCode::Char('+')) => self.settings_view.shift(),
                Event::Input(KeyCode::Char('-')) => self.settings_view.unshift(),
                _ => {}
            },
            InputMode::Command => match event {
                Event::Input(KeyCode::Enter) => {
                    self.exec(self.settings_view.selected(), self.settings_tab)?;
                    self.command.clear();
                }
                Event::Input(KeyCode::Char(c)) => {
                    self.command.push(c);
                }
                Event::Input(KeyCode::Backspace) => {
                    self.command.pop();
                }
                Event::Input(KeyCode::Esc) => {
                    self.mode = InputMode::Normal;
                }
                _ => {}
            },
        };
        Ok(())
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.settings_view
            .set_active(self.mode == InputMode::Normal);
        self.command.set_active(self.mode == InputMode::Command);
        self.terminal.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(92), Constraint::Min(0)])
                .split(f.size());
            self.settings_view.draw(f, rects[0]);
            self.command.draw(f, rects[1]);
        })?;
        Ok(())
    }

    fn exec(
        &self,
        selected: Option<&(sbp_settings::Setting, sbp_settings::SettingValue)>,
        settings_tab: &SettingsTab<'static>,
    ) -> Result<()> {
        match self.command.input().trim() {
            ":q" | ":Q" | ":quit" => bail!("quit"),

            ":h" | ":help" => {}

            cmd if cmd.starts_with("write") => {
                let args: Vec<_> = cmd.split_ascii_whitespace().skip(1).collect();
                match args.as_slice() {
                    [value] => {
                        if let Some((setting, value)) = selected {
                            if let Err(_err) = settings_tab.write_setting(
                                &setting.group,
                                &setting.name,
                                &value.to_string(),
                            ) {
                                // error
                            }
                        } else {
                            // error too few args
                        }
                    }
                    [group, name, value] => {
                        if let Err(_err) = settings_tab.write_setting(group, name, value) {
                            // error
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }

            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InputMode {
    Normal,
    Command,
}
