use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::anyhow;
use capnp::message::Builder;
use ini::Ini;
use lazy_static::lazy_static;
use log::{debug, error, warn};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use sbp::link::Link;
use sbp::messages::piksi::MsgReset;
use sbp::messages::settings::MsgSettingsSave;
use sbp_settings::{Client, SettingKind, SettingValue};

use crate::shared_state::SharedState;
use crate::types::{CapnProtoSender, Error, MsgSender, Result};
use crate::utils::*;

const FIRMWARE_VERSION_SETTING_KEY: &str = "firmware_version";
const DGNSS_SOLUTION_MODE_SETTING_KEY: &str = "dgnss_solution_mode";

lazy_static! {
    static ref RECOMMENDED_INS_SETTINGS: [(&'static str, &'static str, SettingValue); 4] = [
        ("imu", "imu_raw_output", SettingValue::Boolean(true)),
        ("imu", "gyro_range", SettingValue::String("125".to_string())),
        ("imu", "acc_range", SettingValue::String("8g".to_string())),
        ("imu", "imu_rate", SettingValue::String("100".to_string())),
    ];
}

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
        if let Some(ref path) = settings_state.export {
            if let Err(e) = self.export(path) {
                error!("Issue exporting settings, {}", e);
            };
        }
        if let Some(ref path) = settings_state.import {
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
            if let Err(e) = self.reset(true) {
                error!("Issue resetting settings {}", e);
            };
        }
        if settings_state.save {
            if let Err(e) = self.save() {
                error!("Issue saving settings, {}", e);
            };
        }
        if settings_state.confirm_ins_change {
            if let Err(e) = self.confirm_ins_change() {
                error!("Issue confirming INS change, {}", e);
            };
        }

        if settings_state.auto_survey_request {
            if let Err(e) = self.auto_survey() {
                error!("Issue running auto survey, {}", e);
            };
        }
    }

    pub fn refresh(&mut self) {
        self.read_all_settings();
        self.send_table_data();
    }

    pub fn export(&self, path: &Path) -> Result<()> {
        let mut f = fs::File::create(path)?;
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

    pub fn import(&mut self, path: &Path) -> Result<()> {
        let mut f = fs::File::open(path)?;
        let conf = Ini::read_from(&mut f)?;
        for (group, prop) in conf.iter() {
            for (name, value) in prop.iter() {
                if let Err(e) = self.write_setting(group.unwrap(), name, value) {
                    match e.downcast_ref::<sbp_settings::Error<sbp_settings::WriteSettingError>>() {
                        Some(sbp_settings::Error::Err(
                            sbp_settings::WriteSettingError::ReadOnly,
                        )) => {}
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

    pub fn reset(&self, reset_settings: bool) -> Result<()> {
        let flags = if reset_settings { 1 } else { 0 };

        self.msg_sender.send(
            MsgReset {
                flags,
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

    pub fn auto_survey(&mut self) -> Result<()> {
        let (lat, lon, alt) = {
            let shared_data = self.shared_state.lock().unwrap();
            (
                (*shared_data).auto_survey_data.lat,
                (*shared_data).auto_survey_data.lon,
                (*shared_data).auto_survey_data.alt,
            )
        };

        match (lat, lon, alt) {
            (Some(lat), Some(lon), Some(alt)) => {
                self.write_setting("surveyed_position", "surveyed_lat", &lat.to_string())?;
                self.write_setting("surveyed_position", "surveyed_lon", &lon.to_string())?;
                self.write_setting("surveyed_position", "surveyed_alt", &alt.to_string())?;
            }
            _ => {
                error!("Auto survey failed due to unknown lat, lon or alt")
            }
        }
        Ok(())
    }

    pub fn confirm_ins_change(&mut self) -> Result<()> {
        let ins_mode = self
            .settings
            .get("ins", "output_mode")?
            .value
            .as_ref()
            .ok_or_else(|| anyhow!("setting not found"))?;

        let ins_on = ins_mode != &SettingValue::String("Disabled".to_string());

        if ins_on {
            let recommended_settings = self.get_recommended_ins_setting_changes()?;
            for recommendation in recommended_settings {
                self.write_setting(&recommendation.0, &recommendation.1, &recommendation.3)?;
            }
        }

        self.save()?;
        self.reset(false)?;

        Ok(())
    }

    pub fn get_recommended_ins_setting_changes(
        &self,
    ) -> Result<Vec<(String, String, String, String)>> {
        let client = self.client();

        let mut recommended_changes = vec![];

        for setting in RECOMMENDED_INS_SETTINGS.iter() {
            let value = client
                .read_setting(setting.0, setting.1)
                .ok_or_else(|| anyhow!("setting not found"))??;
            if value != setting.2 {
                recommended_changes.push((
                    setting.0.to_string(),
                    setting.1.to_string(),
                    value.to_string(),
                    setting.2.to_string(),
                ));
            }
        }

        Ok(recommended_changes)
    }

    pub fn send_ins_change_response(&mut self, output_mode: &str) -> Result<()> {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut ins_resp = msg.init_ins_settings_change_response();

        if output_mode != "Disabled" {
            let recommendations = self.get_recommended_ins_setting_changes()?;
            let mut recommended_entries = ins_resp
                .reborrow()
                .init_recommended_settings(recommendations.len() as u32);

            for (i, recommendation) in recommendations.iter().enumerate() {
                let mut entry = recommended_entries.reborrow().get(i as u32);
                entry.set_setting_group(&recommendation.0);
                entry.set_setting_name(&recommendation.1);
                entry.set_current_value(&recommendation.2);
                entry.set_recommended_value(&recommendation.3);
            }
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));

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

        if group == "ins" && name == "output_mode" {
            self.send_ins_change_response(value)?;
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

            let current_setting = match self.settings.get_mut(&setting.group, &setting.name) {
                Ok(setting) => setting,
                Err(_) => {
                    warn!(
                        "No settings documentation entry or name: {} in group: {}",
                        setting.name, setting.group
                    );
                    continue;
                }
            };
            if FIRMWARE_VERSION_SETTING_KEY == setting.name {
                self.shared_state
                    .set_firmware_version(setting.clone().value);
            }
            if DGNSS_SOLUTION_MODE_SETTING_KEY == setting.name {
                self.shared_state.set_dgnss_enabled(setting.clone().value);
            }

            // update possible enum values with the possible values returned by the device
            if !setting.fmt_type.is_empty() && current_setting.setting.kind == SettingKind::Enum {
                let mut parts = setting.fmt_type.splitn(2, ':');
                let ty = parts.next();
                if !matches!(ty, Some("enum")) {
                    warn!(
                        "the type for setting {} in group {} was marked as enum but the device returned {:?}",
                        setting.name, setting.group, ty,
                    );
                    continue;
                }
                let possible_values = match parts.next() {
                    Some(possible_values) => possible_values,
                    None => {
                        warn!(
                            "setting {} in group {} was marked as enum but had no enumerated_possible_values",
                            setting.name, setting.group,
                        );
                        continue;
                    }
                };
                current_setting.setting.to_mut().enumerated_possible_values =
                    Some(possible_values.to_string());
            }

            current_setting.value = Some(sbp_settings::SettingValue::String(setting.value));
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
        let mut settings: Vec<_> = sbp_settings::settings().iter().collect();
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

    fn groups(&self) -> Vec<Vec<(&sbp_settings::Setting, &SettingValue)>> {
        self.inner.values().fold(Vec::new(), |mut groups, group| {
            let group: Vec<_> = group
                .values()
                .filter_map(|setting| {
                    setting
                        .value
                        .as_ref()
                        .map(|v| (setting.setting.as_ref(), v))
                })
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

    fn get_mut<'a, 'b>(&'a mut self, group: &'b str, name: &'b str) -> Result<&'a mut Setting> {
        self.inner
            .get_mut(group)
            .map(|g| g.get_mut(name))
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
    setting: Cow<'static, sbp_settings::Setting>,
    value: Option<sbp_settings::SettingValue>,
}

impl Setting {
    fn new(setting: &'static sbp_settings::Setting) -> Self {
        Self {
            setting: Cow::Borrowed(setting),
            value: None,
        }
    }
}
