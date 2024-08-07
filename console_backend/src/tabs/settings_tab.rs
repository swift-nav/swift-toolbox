// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

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
const PRODUCT_ID_SETTING_KEY: &str = "product_id";
const ALLOWED_PRODUCT_ID_SUBSTRINGS: [&str; 2] = ["Piksi Multi", "Duro"];
const DGNSS_SOLUTION_MODE_SETTING_KEY: &str = "dgnss_solution_mode";

const ETHERNET_SETTING_GROUP: &str = "ethernet";
const ETHERNET_INTERFACE_MODE_SETTING_KEY: &str = "interface_mode";

const NTRIP_SETTING_GROUP: &str = "ntrip";
const NTRIP_ENABLE_SETTING_KEY: &str = "enable";

const SETTINGS_READ_WRITE_TIMEOUT: Duration = Duration::from_millis(1000);

// Timeout used to read all settings
const GLOBAL_TIMEOUT: Duration = Duration::from_secs(15);

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
            settings_tab.send_notification(format!("Issue writing setting {}, {}", &req.name, e));
        };
    }
    if settings_state.reset {
        if let Err(e) = settings_tab.reset(true) {
            error!("Issue resetting settings {}", e);
        };
    }
    if settings_state.reboot {
        if let Err(e) = settings_tab.reset(false) {
            error!("Issue rebooting device {}", e);
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
    pub(crate) shared_state: SharedState,
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
                source.send(msg);
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

    pub fn get(&self, group: &str, name: &str) -> Result<SettingsEntry> {
        self.settings.lock().get(group, name).cloned()
    }

    pub fn group(&self, group: &str) -> Result<Vec<SettingsEntry>> {
        let group = self
            .settings
            .lock()
            .group(group)?
            .map(Clone::clone)
            .collect();
        Ok(group)
    }

    pub fn refresh(&self) {
        (*self.settings.lock()) = Settings::new();
        self.send_table_data();
        self.read_all_settings();
        self.send_table_data();
    }

    pub fn export(&self, path: &Path) -> Result<()> {
        if path.to_string_lossy() == "-" {
            self.export_impl(std::io::stdout())
        } else {
            self.export_impl(fs::File::create(path)?)
        }
    }

    fn export_impl<T: std::io::Write>(&self, mut f: T) -> Result<()> {
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

    pub fn import(&self, path: &Path) -> Result<()> {
        if path.to_string_lossy() == "-" {
            self.import_impl(std::io::stdin())
        } else {
            self.import_impl(fs::File::open(path)?)
        }
    }

    pub fn import_impl<T: std::io::Read>(&self, mut f: T) -> Result<()> {
        let conf = Ini::read_from(&mut f)?;
        let old_ethernet = self.set_if_group_changes(
            &conf,
            ETHERNET_SETTING_GROUP,
            ETHERNET_INTERFACE_MODE_SETTING_KEY,
            "Config",
        )?;
        let old_ntrip = self.set_if_group_changes(
            &conf,
            NTRIP_SETTING_GROUP,
            NTRIP_ENABLE_SETTING_KEY,
            "False",
        )?;
        for (group, prop) in sort_import_groups(&conf) {
            for (name, value) in sort_import_group(group, prop) {
                if let Err(e) = self.write_setting(group, name, value) {
                    match e.downcast_ref::<sbp_settings::Error>() {
                        Some(sbp_settings::Error::WriteError(
                            sbp_settings::error::WriteError::ReadOnly,
                        )) => {}
                        _ => {
                            self.import_err(&e, group, name, value);
                            return Err(e);
                        }
                    }
                }
            }
        }
        if let Some(v) = old_ethernet {
            self.write_setting(
                ETHERNET_SETTING_GROUP,
                ETHERNET_INTERFACE_MODE_SETTING_KEY,
                &v.to_string(),
            )?;
        }
        if let Some(v) = old_ntrip {
            self.write_setting(
                NTRIP_SETTING_GROUP,
                NTRIP_ENABLE_SETTING_KEY,
                &v.to_string(),
            )?;
        }
        self.import_success();
        Ok(())
    }

    fn import_success(&self) {
        send_settings_import_response(&self.client_sender, "success");
    }

    fn import_err(&self, err: &Error, group: &str, name: &str, value: &str) {
        send_settings_import_response(
            &self.client_sender,
            &format!("{err}\n\nWhile writing \"{value}\" to {group} -> {name}"),
        )
    }

    pub fn reset(&self, reset_settings: bool) -> Result<()> {
        self.msg_sender.send(MsgReset {
            flags: u32::from(reset_settings),
            sender_id: None,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.msg_sender.send(MsgSettingsSave { sender_id: None })
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
            for (group, name, _, value) in recommended_settings {
                self.write_setting(&group, &name, &value)?;
            }
        }
        self.save()?;
        self.reset(false)?;
        Ok(())
    }

    fn get_recommended_ins_setting_changes(&self) -> Result<Vec<(String, String, String, String)>> {
        let mut recommended_changes = vec![];

        for &(group, name, ref recommended_value) in RECOMMENDED_INS_SETTINGS.iter() {
            let value = self
                .sbp_client
                .lock()
                .read_setting_with_timeout(group, name, SETTINGS_READ_WRITE_TIMEOUT)?
                .ok_or_else(|| anyhow!("setting not found"))?
                .value;
            if value.as_ref() != Some(recommended_value) {
                recommended_changes.push((
                    group.to_owned(),
                    name.to_owned(),
                    value.map_or_else(String::new, |v| v.to_string()),
                    recommended_value.to_string(),
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

            for (i, (group, name, curr_val, rec_val)) in recommendations.iter().enumerate() {
                let mut entry = recommended_entries.reborrow().get(i as u32);
                entry.set_setting_group(group);
                entry.set_setting_name(name);
                entry.set_current_value(curr_val);
                entry.set_recommended_value(rec_val);
            }
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));

        Ok(())
    }

    fn send_notification(&self, message: String) {
        error!("{}", message);
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut status = msg.init_settings_notification();
        status.set_message(&message);

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    pub fn write_setting(&self, group: &str, name: &str, value: &str) -> Result<()> {
        {
            // check if current value is identical
            let settings = self.settings.lock();
            if let Ok(e) = settings.get(group, name) {
                let current = e.value.as_ref().map(|v| v.to_string());
                log::debug!("current {group}.{name}={current:?}");
                if current.as_deref() == Some(value) {
                    return Ok(());
                }
            }
        }
        log::info!("writing {group} {name} {value}");
        let setting = self.sbp_client.lock().write_setting_with_timeout(
            group,
            name,
            value,
            SETTINGS_READ_WRITE_TIMEOUT,
        )?;
        if matches!(
            setting.setting.kind,
            SettingKind::Float | SettingKind::Double
        ) {
            let setting = self
                .sbp_client
                .lock()
                .read_setting_with_timeout(group, name, SETTINGS_READ_WRITE_TIMEOUT)?
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
        let (ctx, handle) = Context::with_timeout(SETTINGS_READ_WRITE_TIMEOUT);

        let mut conn = self.shared_state.watch_connection();
        // this thread runs for at most `GLOBAL_TIMEOUT` seconds so we don't join it to avoid
        // blocking the completion of this function
        let _monitor_handle = std::thread::spawn(move || {
            let t = Instant::now();
            loop {
                let waited = t.elapsed();
                if waited >= GLOBAL_TIMEOUT {
                    break;
                }
                if let Ok(Ok(conn)) = conn.wait_for(GLOBAL_TIMEOUT - waited) {
                    if !conn.is_connected() {
                        break;
                    }
                }
            }
            handle.cancel();
        });

        let (settings, errors) = self.sbp_client.lock().read_all_ctx(ctx);
        for e in errors {
            warn!("{}", e);
        }
        let mut firmware_version = None;
        let mut product_id = None;
        for entry in settings {
            if let Some(ref value) = entry.value {
                match entry.setting.name.as_str() {
                    FIRMWARE_VERSION_SETTING_KEY => firmware_version = Some(value.to_string()),
                    PRODUCT_ID_SETTING_KEY => product_id = Some(value.to_string()),
                    DGNSS_SOLUTION_MODE_SETTING_KEY => {
                        self.shared_state.set_dgnss_enabled(value.to_string())
                    }
                    _ => {}
                }
            }
            self.settings.lock().insert(entry);
        }
        if let Some(product_id) = product_id {
            for substring in ALLOWED_PRODUCT_ID_SUBSTRINGS {
                if product_id.contains(substring) {
                    if let Some(firmware_version) = firmware_version {
                        self.shared_state.set_firmware_version(firmware_version);
                    }
                    break;
                }
            }
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

    // set `group`.`name` = `value` if any settings in `group` will be changed
    // when importing `conf`. returns the original value if `name` does not
    // appear in `conf`.
    fn set_if_group_changes(
        &self,
        conf: &Ini,
        group: &str,
        name: &str,
        value: &str,
    ) -> Result<Option<SettingValue>> {
        if !self.group_changed(conf, group)? {
            return Ok(None);
        }
        let original = self
            .settings
            .lock()
            .get(group, name)
            .map(|x| x.value.clone())
            .unwrap_or_default();
        // if is in new config
        let in_config = conf
            .section(Some(group))
            .and_then(|s| s.get(name))
            .is_some();
        self.write_setting(group, name, value)?;
        if !in_config {
            Ok(original) // revert to original if not in new config
        } else {
            Ok(None)
        }
    }

    // will any of the settings in `group` change when importing `conf`
    fn group_changed(&self, conf: &Ini, group: &str) -> Result<bool> {
        let new_group = match conf.section(Some(group)) {
            Some(s) => s,
            None => return Ok(false), // no change, no need to update
        };
        let guard = self.settings.lock();
        let groups = match guard.get_group(group) {
            Some(v) => v,
            None => return Ok(true), // update since current values are empty
        };
        for (name, new_value) in new_group.iter() {
            let entry: Option<&SettingsEntry> = groups.get(name);
            let old_value = entry.and_then(|e| e.value.as_ref().map(|s| s.to_string()));
            if Some(new_value) != old_value.as_deref() {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

// We need to set the ethernet settings at the very end if the user is changing
// the IP address and also communicating over TCP
fn sort_import_groups(conf: &Ini) -> Vec<(&str, &ini::Properties)> {
    let mut groups: Vec<_> = conf
        .iter()
        .flat_map(|(section, props)| section.zip(Some(props)))
        .collect();
    if let Some(idx) = groups
        .iter()
        .position(|&(s, _)| s == ETHERNET_SETTING_GROUP)
    {
        let ethernet = groups.remove(idx);
        groups.push(ethernet);
    }
    groups
}

// Some settings must be disabled in order to write another setting's value.
// e.g. if `ntrip.enable=True` you can't write to `ntrip.username`. This sorts
// setting groups such that those settings are last. Without this an import might
// enable ntrip and then try to write another value.
fn sort_import_group<'a>(
    group: &str,
    prop: &'a ini::Properties,
) -> Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a> {
    match group {
        NTRIP_SETTING_GROUP => {
            let enable = prop.iter().find(|&(n, _)| n == NTRIP_ENABLE_SETTING_KEY);
            Box::new(
                prop.iter()
                    .filter(|&(n, _)| n != NTRIP_ENABLE_SETTING_KEY)
                    .chain(enable),
            )
        }
        ETHERNET_SETTING_GROUP => {
            let interface_mode = prop
                .iter()
                .find(|&(n, _)| n == ETHERNET_INTERFACE_MODE_SETTING_KEY);
            Box::new(
                prop.iter()
                    .filter(|&(n, _)| n != ETHERNET_INTERFACE_MODE_SETTING_KEY)
                    .chain(interface_mode),
            )
        }
        _ => Box::new(prop.iter()),
    }
}

#[derive(Debug, Clone)]
pub struct SaveRequest {
    pub group: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
struct Settings {
    inner: IndexMap<String, IndexMap<String, SettingsEntry>>,
    default: SettingValue,
}

lazy_static! {
    static ref SETTING_ORDERING: HashMap<(&'static str, &'static str), usize> = {
        Setting::all()
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut settings, (index, setting)| {
                settings.insert((&setting.group, &setting.name), index);
                settings
            })
    };
}

impl Settings {
    fn new() -> Self {
        Self {
            // Keep the settings ordered in the same order as defined in the libsettings settings.yaml file
            inner: Setting::all()
                .iter()
                .fold(IndexMap::new(), |mut settings, setting| {
                    settings.insert(setting.group.clone(), IndexMap::new());
                    settings
                }),
            default: SettingValue::String("".into()),
        }
    }

    fn get_group(&self, group: &str) -> Option<&'_ IndexMap<String, SettingsEntry>> {
        self.inner.get(group)
    }

    fn group(&self, group: &str) -> Result<impl Iterator<Item = &SettingsEntry> + '_> {
        self.get_group(group)
            .map(|group| group.values())
            .ok_or_else(|| anyhow!("unknown setting group: {}", group))
    }

    fn groups(&self) -> Vec<Vec<(&Setting, &SettingValue)>> {
        self.inner.values().fold(Vec::new(), |mut groups, group| {
            let mut group: Vec<_> = group
                .values()
                .map(|setting| {
                    setting.value.as_ref().map_or_else(
                        || (setting.setting.as_ref(), &self.default),
                        |v| (setting.setting.as_ref(), v),
                    )
                })
                .collect();

            // Sort settings within a group by the order in which they're defined within the libsettings settings.yaml file
            group.sort_by(|a, b| {
                let a_index = SETTING_ORDERING
                    .get(&(&a.0.group, &a.0.name))
                    .unwrap_or(&usize::MAX);
                let b_index = SETTING_ORDERING
                    .get(&(&b.0.group, &b.0.name))
                    .unwrap_or(&usize::MAX);
                a_index.cmp(b_index)
            });

            if !group.is_empty() {
                groups.push(group);
            }
            groups
        })
    }

    fn get<'a, 'b>(&'a self, group: &'b str, name: &'b str) -> Result<&'a SettingsEntry> {
        self.get_group(group)
            .and_then(|g| g.get(name))
            .ok_or_else(|| anyhow!("unknown setting: group: {group} name: {name}"))
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
#[derive(Debug, Clone)]
pub struct SettingsEntry {
    pub setting: Cow<'static, Setting>,
    pub value: Option<SettingValue>,
}
