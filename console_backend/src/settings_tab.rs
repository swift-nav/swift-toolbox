use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use capnp::message::Builder;
use ini::Ini;
use libsettings::{Client, SettingKind, SettingValue};
use log::{debug, error, warn};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use sbp::link::Link;
use sbp::messages::piksi::MsgReset;
use sbp::messages::settings::MsgSettingsSave;

use crate::types::{CapnProtoSender, Error, MsgSender, Result, SharedState};
use crate::utils::*;

pub struct SettingsTab<'link, S> {
    client_sender: S,
    shared_state: SharedState,
    settings: Settings,
    client: Mutex<Option<Client<'link>>>,
    link: Link<'link, ()>,
    msg_sender: MsgSender,
}

impl<'link, S: CapnProtoSender> SettingsTab<'link, S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
        msg_sender: MsgSender,
        link: Link<'link, ()>,
    ) -> SettingsTab<'link, S> {
        SettingsTab {
            settings: Settings::new(),
            client: Mutex::new(None),
            client_sender,
            shared_state,
            link,
            msg_sender,
        }
    }

    pub fn tick(&mut self) {
        let settings_state = self.shared_state.take_settings_state();
        if settings_state.refresh {
            self.refresh();
        }
        if let Some(path) = settings_state.export {
            if let Err(e) = self.export(path) {
                error!("Issue exporting settings, {}", e);
            };
        }
        if let Some(path) = settings_state.import {
            if let Err(e) = self.import(path) {
                error!("Issue importing settings, {}", e);
            };
        }
        if let Some(req) = settings_state.write {
            if let Err(e) = self.write_setting(&req.group, &req.name, &req.value) {
                error!("Issue writing setting, {}", e);
            };
        }
        if settings_state.reset {
            if let Err(e) = self.reset() {
                error!("Issue resetting settings {}", e);
            };
        }
        if settings_state.save {
            if let Err(e) = self.save() {
                error!("Issue saving settings, {}", e);
            };
        }
    }

    pub fn refresh(&mut self) {
        self.read_all_settings();
        self.send_table_data();
    }

    pub fn export(&self, path: String) -> Result<()> {
        let mut f = {
            let path = PathBuf::from(path.trim_start_matches("file://"));
            if let Some(p) = path.parent() {
                fs::create_dir_all(p)?;
            }
            fs::File::create(&path)?
        };
        let groups = self.settings.groups();
        let mut conf = Ini::new();
        for group in groups.iter() {
            let mut section = conf.with_section(Some(&group[0].0.group));
            let mut s = &mut section;
            for (setting, value) in group.iter() {
                s = s.set(&setting.name, value.to_string());
            }
        }
        conf.write_to(&mut f)?;
        Ok(())
    }

    pub fn import(&mut self, path: String) -> Result<()> {
        let mut f = {
            let path = PathBuf::from(path.trim_start_matches("file://"));
            fs::File::open(&path)?
        };
        let conf = Ini::read_from(&mut f)?;
        for (group, prop) in conf.iter() {
            for (name, value) in prop.iter() {
                if let Err(e) = self.write_setting(group.unwrap(), name, value) {
                    match e.downcast_ref::<libsettings::Error<libsettings::WriteSettingError>>() {
                        Some(libsettings::Error::Err(libsettings::WriteSettingError::ReadOnly)) => {
                        }
                        _ => {
                            self.import_err(&e);
                            return Err(e);
                        }
                    }
                }
            }
        }
        self.import_success();
        Ok(())
    }

    pub fn import_success(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut import_response = msg.init_settings_import_response();
        import_response.set_status("success");
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    pub fn import_err(&mut self, err: &Error) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut import_response = msg.init_settings_import_response();
        import_response.set_status(&err.to_string());
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    pub fn reset(&self) -> Result<()> {
        self.msg_sender.send(
            MsgReset {
                flags: 1,
                sender_id: None,
            }
            .into(),
        )?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.msg_sender
            .send(MsgSettingsSave { sender_id: None }.into())?;
        Ok(())
    }

    pub fn write_setting(&mut self, group: &str, name: &str, value: &str) -> Result<()> {
        let setting = self.settings.get(group, name)?;

        if let Some(ref v) = setting.value {
            if v.to_string() == value {
                debug!("skipping write because setting has not changed");
                return Ok(());
            }
        }

        self.client().write_setting(group, name, value)?;

        if matches!(
            setting.setting.kind,
            SettingKind::Float | SettingKind::Double
        ) {
            let new_setting = self
                .client()
                .read_setting(group, name)
                .ok_or_else(|| anyhow!("settting not found"))??;
            self.settings.set(group, name, new_setting)?;
        } else {
            self.settings
                .set(group, name, SettingValue::String(value.to_string()))?;
        }

        self.send_table_data();

        Ok(())
    }

    pub fn read_all_settings(&mut self) {
        let results = self.client().read_all();
        for result in results {
            let setting = match result {
                Ok(setting) => setting,
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            };
            if self.settings.get(&setting.group, &setting.name).is_ok() {
                self.settings
                    .set(
                        &setting.group,
                        &setting.name,
                        libsettings::SettingValue::String(setting.value),
                    )
                    .unwrap();
            } else {
                warn!(
                    "No settings documentation entry or name: {} in group: {}",
                    setting.name, setting.group
                );
            }
        }
    }

    /// Package settings table data into a message buffer and send to frontend.
    pub fn send_table_data(&mut self) {
        let groups = self.settings.groups();
        if groups.is_empty() {
            return;
        }
        let num_settings: usize = groups.iter().map(|group| group.len()).sum();
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut settings_table_status = msg.init_settings_table_status();
        let mut table_entries = settings_table_status
            .reborrow()
            .init_data((num_settings + groups.len()) as u32);

        let mut i: u32 = 0;
        for group in groups.iter() {
            let mut entry = table_entries.reborrow().get(i);
            entry.set_group(&group[0].0.group);
            i += 1;
            for (setting, value) in group.iter() {
                let mut entry = table_entries.reborrow().get(i).init_setting();
                entry.set_name(&setting.name);
                entry.set_group(&setting.group);
                entry.set_type(setting.kind.to_str());
                entry.set_expert(setting.expert);
                entry.set_readonly(setting.readonly);
                {
                    let entry = entry.reborrow();
                    if let Some(ref description) = setting.description {
                        entry.get_description().set_description(description);
                    } else {
                        entry.get_description().set_no_description(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    if let Some(ref default_value) = setting.default_value {
                        entry.get_default_value().set_default_value(default_value);
                    } else {
                        entry.get_default_value().set_no_default_value(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    if let Some(ref notes) = setting.notes {
                        entry.get_notes().set_notes(notes);
                    } else {
                        entry.get_notes().set_no_notes(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    if let Some(ref units) = setting.units {
                        entry.get_units().set_units(units);
                    } else {
                        entry.get_units().set_no_units(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    if let Some(ref enumerated_possible_values) = setting.enumerated_possible_values
                    {
                        entry
                            .get_enumerated_possible_values()
                            .set_enumerated_possible_values(enumerated_possible_values);
                    } else {
                        entry
                            .get_enumerated_possible_values()
                            .set_no_enumerated_possible_values(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    if let Some(ref digits) = setting.digits {
                        entry.get_digits().set_digits(digits);
                    } else {
                        entry.get_digits().set_no_digits(());
                    }
                }
                {
                    let entry = entry.reborrow();
                    entry
                        .get_value_on_device()
                        .set_value_on_device(&value.to_string());
                }
                i += 1;
            }
        }
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn client<'a>(&'a self) -> MappedMutexGuard<'a, Client<'link>> {
        let mut client = self.client.lock();
        if client.is_some() {
            return MutexGuard::map(client, |c| c.as_mut().unwrap());
        }
        let sender = self.msg_sender.clone();
        *client = Some(Client::new(self.link.clone(), move |msg| {
            sender.send(msg).map_err(Into::into)
        }));
        MutexGuard::map(client, |c| c.as_mut().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct SaveRequest {
    pub group: String,
    pub name: String,
    pub value: String,
}

struct Settings {
    inner: BTreeMap<&'static str, BTreeMap<&'static str, Setting>>,
}

impl Settings {
    fn new() -> Self {
        let mut settings: Vec<_> = libsettings::settings().iter().collect();
        settings.sort_by_key(|s| &s.group);
        Self {
            inner: settings
                .into_iter()
                .fold(BTreeMap::new(), |mut settings, setting| {
                    (*settings.entry(&setting.group).or_default())
                        .insert(&setting.name, Setting::new(setting));
                    settings
                }),
        }
    }

    fn groups(&self) -> Vec<Vec<(&libsettings::Setting, &SettingValue)>> {
        self.inner.values().fold(Vec::new(), |mut groups, group| {
            let group: Vec<_> = group
                .values()
                .filter_map(|setting| setting.value.as_ref().map(|v| (setting.setting, v)))
                .collect();
            if !group.is_empty() {
                groups.push(group);
            }
            groups
        })
    }

    fn get<'a, 'b>(&'a self, group: &'b str, name: &'b str) -> Result<&'a Setting> {
        self.inner
            .get(group)
            .map(|g| g.get(name))
            .flatten()
            .ok_or_else(|| anyhow!("unknown setting: group: {} name: {}", group, name))
    }

    fn set<'a, 'b>(&'a mut self, group: &'b str, name: &'b str, value: SettingValue) -> Result<()> {
        let setting = self
            .inner
            .get_mut(group)
            .map(|g| g.get_mut(name))
            .flatten()
            .ok_or_else(|| anyhow!("unknown setting: group: {} name: {}", group, name))?;
        setting.value = Some(value);
        Ok(())
    }
}

impl std::ops::Deref for Settings {
    type Target = BTreeMap<&'static str, BTreeMap<&'static str, Setting>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A reference to a particular setting and its value if it has been fetched
#[derive(Debug)]
struct Setting {
    setting: &'static libsettings::Setting,
    value: Option<libsettings::SettingValue>,
}

impl Setting {
    fn new(setting: &'static libsettings::Setting) -> Self {
        Self {
            setting,
            value: None,
        }
    }
}
