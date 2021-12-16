use std::borrow::Cow;
use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::anyhow;
use capnp::message::Builder;
use crossbeam::channel::{self, Sender};
use indexmap::IndexMap;
use ini::Ini;
use lazy_static::lazy_static;
use log::{error, warn};
use parking_lot::Mutex;
use sbp::link::LinkSource;
use sbp::messages::piksi::MsgReset;
use sbp::messages::settings::MsgSettingsSave;
use sbp::Sbp;
use sbp_settings::{Client, Context, Setting, SettingKind, SettingValue};

use crate::client_sender::BoxedClientSender;
use crate::shared_state::{SettingsTabState, SharedState};
use crate::types::{Error, MsgSender, Result};
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

pub fn start_thd(tab: &SettingsTab) {
    let mut recv = tab.shared_state.watch_settings_state();
    while let Ok(mut guard) = recv.wait_mut() {
        let state = &mut *guard;
        // taking the state reverts the settings state back to its default value
        // without triggering another update (which would cause this to loop forever)
        let s = std::mem::take(state);
        // drop the guard so the settings state can get updated while`tick` runs (which might take a while)
        drop(guard);
        tick(tab, s);
    }
}

fn tick(settings_tab: &SettingsTab, settings_state: SettingsTabState) {
    if settings_state.refresh {
        settings_tab.refresh();
    }
    if let Some(ref path) = settings_state.export {
        if let Err(e) = settings_tab.export(path) {
            error!("Issue exporting settings, {}", e);
        };
    }
    if let Some(ref path) = settings_state.import {
        if let Err(e) = settings_tab.import(path) {
            error!("Issue importing settings, {}", e);
        };
    }
    if let Some(req) = settings_state.write {
        if let Err(e) = settings_tab.write_setting(&req.group, &req.name, &req.value) {
            error!("Issue writing setting, {}", e);
        };
    }
    if settings_state.reset {
        if let Err(e) = settings_tab.reset(true) {
            error!("Issue resetting settings {}", e);
        };
    }
    if settings_state.save {
        if let Err(e) = settings_tab.save() {
            error!("Issue saving settings, {}", e);
        };
    }
    if settings_state.confirm_ins_change {
        if let Err(e) = settings_tab.confirm_ins_change() {
            error!("Issue confirming INS change, {}", e);
        };
    }
    if settings_state.auto_survey_request {
        if let Err(e) = settings_tab.auto_survey() {
            error!("Issue running auto survey, {}", e);
        };
    }
}

pub struct SettingsTab {
    shared_state: SharedState,
    client_sender: BoxedClientSender,
    msg_sender: MsgSender,
    settings: Mutex<Settings>,
    sbp_client: Mutex<Client<'static>>,
    send: Sender<Sbp>,
}

impl Drop for SettingsTab {
    fn drop(&mut self) {
        self.shared_state.reset_settings_state();
    }
}

impl SettingsTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
        msg_sender: MsgSender,
    ) -> Self {
        let source = LinkSource::new();
        let link = source.link();
        let (send, recv) = channel::unbounded();

        // this thread will join when `SettingsTab` is dropped, because the channel will disconnect
        std::thread::spawn(move || {
            for msg in recv.iter() {
                source.send(&msg);
            }
        });

        SettingsTab {
            shared_state,
            client_sender,
            msg_sender: msg_sender.clone(),
            settings: Mutex::new(Settings::new()),
            sbp_client: Mutex::new(Client::with_link(link.clone(), move |msg| {
                msg_sender.send(msg).map_err(Into::into)
            })),
            send,
        }
    }

    pub fn handle_msg(&self, msg: Sbp) {
        if self.send.send(msg).is_err() {
            warn!("could not forward message to the settings tab");
        };
    }

    pub fn stop(&self) {
        self.shared_state.reset_settings_state();
    }

    fn refresh(&self) {
        (*self.settings.lock()) = Settings::new();
        self.send_table_data();
        self.read_all_settings();
        self.send_table_data();
    }

    fn export(&self, path: &Path) -> Result<()> {
        let mut f = fs::File::create(path)?;
        let settings = self.settings.lock();
        let groups = settings.groups();
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

    fn import(&self, path: &Path) -> Result<()> {
        let mut f = fs::File::open(path)?;
        let conf = Ini::read_from(&mut f)?;
        for (group, prop) in conf.iter() {
            for (name, value) in prop.iter() {
                if let Err(e) = self.write_setting(group.unwrap(), name, value) {
                    match e.downcast_ref::<sbp_settings::Error>() {
                        Some(sbp_settings::Error::WriteError(
                            sbp_settings::error::WriteError::ReadOnly,
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

    fn import_success(&self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut import_response = msg.init_settings_import_response();
        import_response.set_status("success");
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn import_err(&self, err: &Error) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut import_response = msg.init_settings_import_response();
        import_response.set_status(&err.to_string());
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn reset(&self, reset_settings: bool) -> Result<()> {
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

    fn save(&self) -> Result<()> {
        self.msg_sender
            .send(MsgSettingsSave { sender_id: None }.into())?;
        Ok(())
    }

    fn auto_survey(&self) -> Result<()> {
        let (lat, lon, alt) = {
            let shared_data = self.shared_state.lock();
            (
                shared_data.auto_survey_data.lat,
                shared_data.auto_survey_data.lon,
                shared_data.auto_survey_data.alt,
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

    fn confirm_ins_change(&self) -> Result<()> {
        let ins_mode = self
            .settings
            .lock()
            .get("ins", "output_mode")?
            .value
            .clone()
            .ok_or_else(|| anyhow!("setting not found"))?;
        let ins_on = ins_mode != SettingValue::String("Disabled".to_string());
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

    fn get_recommended_ins_setting_changes(&self) -> Result<Vec<(String, String, String, String)>> {
        let mut recommended_changes = vec![];

        for setting in RECOMMENDED_INS_SETTINGS.iter() {
            let value = self
                .sbp_client
                .lock()
                .read_setting(setting.0, setting.1)?
                .ok_or_else(|| anyhow!("setting not found"))?
                .value;
            if value.as_ref() != Some(&setting.2) {
                recommended_changes.push((
                    setting.0.to_string(),
                    setting.1.to_string(),
                    value.map_or_else(String::new, |v| v.to_string()),
                    setting.2.to_string(),
                ));
            }
        }

        Ok(recommended_changes)
    }

    fn send_ins_change_response(&self, output_mode: &str) -> Result<()> {
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

    fn write_setting(&self, group: &str, name: &str, value: &str) -> Result<()> {
        {
            let settings = self.settings.lock();
            if let Ok(e) = settings.get(group, name) {
                let current = e.value.as_ref().map(|v| v.to_string());
                if current.as_deref() == Some(value) {
                    return Ok(());
                }
            }
        }
        let setting = self.sbp_client.lock().write_setting(group, name, value)?;
        if matches!(
            setting.setting.kind,
            SettingKind::Float | SettingKind::Double
        ) {
            let setting = self
                .sbp_client
                .lock()
                .read_setting(group, name)?
                .ok_or_else(|| anyhow!("setting not found"))?;
            self.settings.lock().insert(setting);
        } else {
            self.settings.lock().insert(setting);
        }
        if group == "ins" && name == "output_mode" {
            self.send_ins_change_response(value)?;
        }
        self.send_table_data();
        Ok(())
    }

    fn read_all_settings(&self) {
        const TIMEOUT: Duration = Duration::from_secs(15);

        let (ctx, _handle) = Context::with_timeout(TIMEOUT);
        let (settings, errors) = self.sbp_client.lock().read_all_ctx(ctx);
        for e in errors {
            warn!("{}", e);
        }
        for entry in settings {
            if let Some(ref value) = entry.value {
                if FIRMWARE_VERSION_SETTING_KEY == entry.setting.name {
                    self.shared_state.set_firmware_version(value.to_string());
                }
                if DGNSS_SOLUTION_MODE_SETTING_KEY == entry.setting.name {
                    self.shared_state.set_dgnss_enabled(value.to_string());
                }
            }
            self.settings.lock().insert(entry);
        }
    }

    /// Package settings table data into a message buffer and send to frontend.
    fn send_table_data(&self) {
        let settings = self.settings.lock();
        let groups = settings.groups();
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
}

#[derive(Debug, Clone)]
pub struct SaveRequest {
    pub group: String,
    pub name: String,
    pub value: String,
}

struct Settings {
    inner: IndexMap<String, IndexMap<String, SettingsEntry>>,
}

impl Settings {
    fn new() -> Self {
        Self {
            inner: Setting::all()
                .iter()
                .fold(IndexMap::new(), |mut settings, setting| {
                    (*settings.entry(setting.group.clone()).or_default())
                        .insert(setting.name.clone(), SettingsEntry::new(setting));
                    settings
                }),
        }
    }

    fn groups(&self) -> Vec<Vec<(&Setting, &SettingValue)>> {
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

    fn get<'a, 'b>(&'a self, group: &'b str, name: &'b str) -> Result<&'a SettingsEntry> {
        self.inner
            .get(group)
            .map(|g| g.get(name))
            .flatten()
            .ok_or_else(|| anyhow!("unknown setting: group: {} name: {}", group, name))
    }

    fn insert(&mut self, entry: sbp_settings::Entry) {
        self.inner
            .entry(entry.setting.group.clone())
            .or_default()
            .insert(
                entry.setting.name.clone(),
                SettingsEntry {
                    setting: entry.setting,
                    value: entry.value,
                },
            );
    }
}

/// A reference to a particular setting and its value if it has been fetched
#[derive(Debug)]
struct SettingsEntry {
    setting: Cow<'static, Setting>,
    value: Option<SettingValue>,
}

impl SettingsEntry {
    fn new(setting: &'static Setting) -> Self {
        Self {
            setting: Cow::Borrowed(setting),
            value: None,
        }
    }
}
