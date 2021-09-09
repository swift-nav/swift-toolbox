use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use capnp::message::Builder;
use ini::Ini;
use libsettings::{SettingKind, SettingValue};
use log::{debug, error, warn};
use sbp::messages::piksi::MsgReset;
use sbp::messages::settings::MsgSettingsSave;

use crate::broadcaster::Link;
use crate::types::{CapnProtoSender, Error, MsgSender, Result, SharedState};
use crate::utils::*;

use client::Client;

pub struct SettingsTab<'link, S> {
    client_sender: S,
    shared_state: SharedState,
    settings: Settings,
    client: Client<'link>,
    msg_sender: MsgSender,
}

impl<'link, S: CapnProtoSender> SettingsTab<'link, S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
        msg_sender: MsgSender,
        link: Link<'link>,
    ) -> SettingsTab<'link, S> {
        SettingsTab {
            settings: Settings::new(),
            client: Client::new(link, msg_sender.clone()),
            client_sender,
            shared_state,
            msg_sender,
        }
    }

    pub fn tick(&mut self) {
        if self.shared_state.settings_refresh() {
            self.refresh();
            self.shared_state.set_settings_refresh(false);
        }
        if let Some(path) = self.shared_state.export_settings() {
            if let Err(e) = self.export(path) {
                error!("Issue exporting settings, {}", e);
            };
            self.shared_state.set_export_settings(None);
        }
        if let Some(path) = self.shared_state.import_settings() {
            if let Err(e) = self.import(path) {
                error!("Issue importing settings, {}", e);
            };
            self.shared_state.set_import_settings(None);
        }
        if let Some(req) = self.shared_state.write_setting() {
            if let Err(e) = self.write_setting(&req.group, &req.name, &req.value) {
                error!("Issue saving setting, {}", e);
            };
            self.shared_state.set_write_setting(None);
        }
        if self.shared_state.settings_reset() {
            if let Err(e) = self.reset() {
                error!("Issue reseting setting, {}", e);
            };
            self.shared_state.set_settings_reset(false);
        }
        if self.shared_state.settings_save() {
            if let Err(e) = self.save() {
                error!("Issue reseting setting, {}", e);
            };
            self.shared_state.set_settings_save(false);
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
                    match e.downcast_ref::<client::WriteError>() {
                        Some(client::WriteError::ReadOnly { .. }) => {}
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
        let setting = self.settings.get_mut(group, name)?;

        if let Some(ref v) = setting.value {
            if v.to_string() == value {
                debug!("skipping write because setting has not changed");
                return Ok(());
            }
        }

        self.client.write_setting(group, name, value)?;

        if matches!(
            setting.setting.kind,
            SettingKind::Float | SettingKind::Double
        ) {
            let new_setting = self.client.read_setting(group, name)?;
            setting.value = Some(new_setting);
        } else {
            setting.value = Some(SettingValue::String(value.to_string()));
        }

        Ok(())
    }

    pub fn read_all_settings(&mut self) {
        for setting in self.client.read_all() {
            let current = self.settings.get_mut(&setting.group, &setting.name);
            if let Ok(current) = current {
                current.value = Some(libsettings::SettingValue::String(setting.value))
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

    fn get_mut<'a, 'b>(&'a mut self, group: &'b str, name: &'b str) -> Result<&'a mut Setting> {
        self.inner
            .get_mut(group)
            .map(|g| g.get_mut(name))
            .flatten()
            .ok_or_else(|| anyhow!("unknown setting: group: {} name: {}", group, name))
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

mod client {
    use std::{
        convert::TryInto,
        ffi::{CStr, CString},
        os::raw::{c_char, c_ulong, c_void},
        ptr, slice,
        time::Duration,
    };

    use anyhow::anyhow;
    use libsettings::{settings, SettingKind, SettingValue};
    use libsettings_sys::{
        sbp_msg_callback_t, sbp_msg_callbacks_node_t, settings_api_t, settings_create,
        settings_destroy, settings_read_bool, settings_read_by_idx, settings_read_float,
        settings_read_int, settings_read_str, settings_t,
        settings_write_res_e_SETTINGS_WR_MODIFY_DISABLED, settings_write_res_e_SETTINGS_WR_OK,
        settings_write_res_e_SETTINGS_WR_PARSE_FAILED, settings_write_res_e_SETTINGS_WR_READ_ONLY,
        settings_write_res_e_SETTINGS_WR_SERVICE_FAILED,
        settings_write_res_e_SETTINGS_WR_SETTING_REJECTED,
        settings_write_res_e_SETTINGS_WR_TIMEOUT, settings_write_res_e_SETTINGS_WR_VALUE_REJECTED,
        settings_write_str,
    };
    use log::{debug, error};
    use sbp::{
        messages::{SBPMessage, SBP},
        serialize::SbpSerialize,
    };

    use crate::{
        broadcaster::{Key, Link},
        types::{MsgSender, Result},
    };

    const SENDER_ID: u16 = 66;

    #[derive(Debug)]
    pub struct ReadByIdxResult {
        pub group: String,
        pub name: String,
        pub value: String,
        pub fmt_type: String,
    }

    pub struct Client<'a>(Box<ClientInner<'a>>);

    impl<'a> Client<'a> {
        pub fn new(link: Link<'a>, sender: MsgSender) -> Client<'a> {
            let context = Box::new(Context {
                link,
                sender,
                callbacks: Vec::new(),
                lock: Lock::new(),
                event: None,
            });

            let api = Box::new(settings_api_t {
                ctx: ptr::null_mut(),
                send: Some(send),
                send_from: Some(send_from),
                wait_init: Some(libsettings_wait_init),
                wait: Some(libsettings_wait),
                wait_deinit: None,
                signal: Some(libsettings_signal),
                wait_thd: Some(libsettings_wait_thd),
                signal_thd: Some(libsettings_signal_thd),
                lock: Some(libsettings_lock),
                unlock: Some(libsettings_unlock),
                register_cb: Some(register_cb),
                unregister_cb: Some(unregister_cb),
                log: None,
                log_preformat: false,
            });

            let mut inner = Box::new(ClientInner {
                context: ptr::null_mut(),
                api: Box::into_raw(api),
                ctx: ptr::null_mut(),
            });

            let context_raw = Box::into_raw(context);
            inner.context = context_raw;

            // Safety: inner.api was just created via Box::into_raw so it is
            // properly aligned and non-null
            unsafe {
                (*inner.api).ctx = context_raw as *mut c_void;
            }

            inner.ctx = unsafe { settings_create(SENDER_ID, inner.api) };
            assert!(!inner.ctx.is_null());

            Client(inner)
        }

        pub fn read_all(&mut self) -> Vec<ReadByIdxResult> {
            const WORKERS: u16 = 10;

            let read_one = |i| self.read_by_index(i);
            let mut settings = Vec::new();
            let mut idx = 0;

            // TODO: probably should reuse the same worker threads each loop
            'outer: loop {
                let results = crossbeam::scope(|scope| {
                    #[allow(clippy::needless_collect)]
                    let handles = (0..WORKERS)
                        .map(|i| scope.spawn(move |_| read_one(idx + i)))
                        .collect::<Vec<_>>();
                    handles
                        .into_iter()
                        .map(|h| h.join().unwrap())
                        .collect::<Vec<_>>()
                })
                .unwrap();

                for result in results {
                    if let Ok(Some(result)) = result {
                        settings.push(result);
                    } else {
                        break 'outer;
                    }
                }

                idx += WORKERS;
            }

            debug!("settings count {}", settings.len());

            settings
        }

        pub fn read_by_index(&self, idx: u16) -> Result<Option<ReadByIdxResult>> {
            const BUF_SIZE: usize = 255;

            let mut section = Vec::<c_char>::with_capacity(BUF_SIZE);
            let mut name = Vec::<c_char>::with_capacity(BUF_SIZE);
            let mut value = Vec::<c_char>::with_capacity(BUF_SIZE);
            let mut fmt_type = Vec::<c_char>::with_capacity(BUF_SIZE);
            let mut event = Event::new();

            let status = unsafe {
                settings_read_by_idx(
                    self.0.ctx,
                    &mut event as *mut Event as *mut _,
                    idx,
                    section.as_mut_ptr(),
                    BUF_SIZE as c_ulong,
                    name.as_mut_ptr(),
                    BUF_SIZE as c_ulong,
                    value.as_mut_ptr(),
                    BUF_SIZE as c_ulong,
                    fmt_type.as_mut_ptr(),
                    BUF_SIZE as c_ulong,
                )
            };

            if status < 0 {
                return Err(anyhow!("settings read failed with status code {}", status));
            }

            if status > 0 {
                return Ok(None);
            }

            let res = unsafe {
                ReadByIdxResult {
                    group: CStr::from_ptr(section.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    name: CStr::from_ptr(name.as_ptr()).to_string_lossy().into_owned(),
                    value: CStr::from_ptr(value.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    fmt_type: CStr::from_ptr(fmt_type.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                }
            };

            Ok(Some(res))
        }

        pub fn read_setting(&self, group: &str, name: &str) -> Result<SettingValue> {
            let setting = settings()
                .iter()
                .find(|s| s.group == group && s.name == name)
                .ok_or_else(|| anyhow!("setting not found"))?;

            let c_group = CString::new(group)?;
            let c_name = CString::new(name)?;

            let (res, status) = match setting.kind {
                SettingKind::Integer => unsafe {
                    let mut value: i32 = 0;
                    let status = settings_read_int(
                        self.0.ctx,
                        c_group.as_ptr(),
                        c_name.as_ptr(),
                        &mut value,
                    );
                    (SettingValue::Integer(value), status)
                },
                SettingKind::Boolean => unsafe {
                    let mut value: bool = false;
                    let status = settings_read_bool(
                        self.0.ctx,
                        c_group.as_ptr(),
                        c_name.as_ptr(),
                        &mut value,
                    );
                    (SettingValue::Boolean(value), status)
                },
                SettingKind::Float | SettingKind::Double => unsafe {
                    let mut value: f32 = 0.0;
                    let status = settings_read_float(
                        self.0.ctx,
                        c_group.as_ptr(),
                        c_name.as_ptr(),
                        &mut value,
                    );
                    (SettingValue::Float(value), status)
                },
                SettingKind::String | SettingKind::Enum | SettingKind::PackedBitfield => unsafe {
                    let mut buf = Vec::<c_char>::with_capacity(1024);
                    let status = settings_read_str(
                        self.0.ctx,
                        c_group.as_ptr(),
                        c_name.as_ptr(),
                        buf.as_mut_ptr(),
                        buf.capacity().try_into().unwrap(),
                    );
                    (
                        SettingValue::String(
                            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned(),
                        ),
                        status,
                    )
                },
            };

            debug!("{} {} {:?} {}", setting.group, setting.name, res, status);

            if status == 0 {
                Ok(res)
            } else {
                Err(anyhow!("settings read failed with status code {}", status))
            }
        }

        pub fn write_setting(&self, group: &str, name: &str, value: &str) -> Result<()> {
            let cgroup = CString::new(group)?;
            let cname = CString::new(name)?;
            let cvalue = CString::new(value)?;
            let mut event = Event::new();

            let result = unsafe {
                settings_write_str(
                    self.0.ctx,
                    &mut event as *mut Event as *mut _,
                    cgroup.as_ptr(),
                    cname.as_ptr(),
                    cvalue.as_ptr(),
                )
            };

            #[allow(non_upper_case_globals)]
            match result {
                settings_write_res_e_SETTINGS_WR_OK => Ok(()),
                code => Err(WriteError::new(code, group, name, value).into()),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum WriteError {
        ValueRejected {
            name: String,
            group: String,
            value: String,
        },
        SettingRejected {
            name: String,
            group: String,
        },
        ParseFailed {
            name: String,
            group: String,
            value: String,
        },
        ReadOnly {
            name: String,
            group: String,
        },
        ModifyDisabled {
            name: String,
            group: String,
        },
        ServiceFailed,
        Timeout,
        Unknown(u32),
    }

    impl WriteError {
        fn new(code: u32, name: &str, group: &str, value: &str) -> WriteError {
            #[allow(non_upper_case_globals)]
            match code {
                settings_write_res_e_SETTINGS_WR_VALUE_REJECTED => WriteError::ValueRejected {
                    name: name.to_string(),
                    group: group.to_string(),
                    value: value.to_string(),
                },
                settings_write_res_e_SETTINGS_WR_SETTING_REJECTED => WriteError::SettingRejected {
                    name: name.to_string(),
                    group: group.to_string(),
                },
                settings_write_res_e_SETTINGS_WR_PARSE_FAILED => WriteError::ParseFailed {
                    name: name.to_string(),
                    group: group.to_string(),
                    value: value.to_string(),
                },
                settings_write_res_e_SETTINGS_WR_READ_ONLY => WriteError::ReadOnly {
                    name: name.to_string(),
                    group: group.to_string(),
                },
                settings_write_res_e_SETTINGS_WR_MODIFY_DISABLED => WriteError::ModifyDisabled {
                    name: name.to_string(),
                    group: group.to_string(),
                },
                settings_write_res_e_SETTINGS_WR_SERVICE_FAILED => WriteError::ServiceFailed,
                settings_write_res_e_SETTINGS_WR_TIMEOUT => WriteError::Timeout,
                code => WriteError::Unknown(code),
            }
        }
    }

    impl std::fmt::Display for WriteError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WriteError::ValueRejected { value, name, group } => write!(
                    f,
                    "setting value {} invalid for setting {} in group {}",
                    value, name, group
                ),
                WriteError::SettingRejected { name, group } => {
                    write!(f, "setting {} in group {} does not exist", name, group)
                }
                WriteError::ParseFailed { name, group, value } => {
                    write!(
                        f,
                        "could not parse setting value {} for setting {} in group {}",
                        value, name, group
                    )
                }
                WriteError::ReadOnly { name, group } => {
                    write!(f, "setting {} in group {} is read only", name, group)
                }
                WriteError::ModifyDisabled { name, group } => {
                    write!(f, "setting {} in group {} is not modifiable", name, group)
                }
                WriteError::ServiceFailed => write!(f, "system failure during setting"),
                WriteError::Timeout => write!(f, "request wasn't replied in time"),
                WriteError::Unknown(code) => write!(f, "unknown settings write response: {}", code),
            }
        }
    }

    impl std::error::Error for WriteError {}

    struct ClientInner<'a> {
        context: *mut Context<'a>,
        api: *mut settings_api_t,
        ctx: *mut settings_t,
    }

    impl Drop for ClientInner<'_> {
        fn drop(&mut self) {
            // Safety: These were created via Box::into_raw and are not
            // freed by the c library. We don't need to free event because
            // it is pointer to the event held inside context so it gets dropped
            unsafe {
                let _ = Box::from_raw(self.context);
                let _ = Box::from_raw(self.api);
                settings_destroy(&mut self.ctx);
            }
        }
    }

    unsafe impl Send for ClientInner<'_> {}
    unsafe impl Sync for ClientInner<'_> {}

    #[repr(C)]
    struct Context<'a> {
        link: Link<'a>,
        sender: MsgSender,
        callbacks: Vec<Callback>,
        event: Option<Event>,
        lock: Lock,
    }

    impl<'a> Context<'a> {
        fn callback_broker(&self, msg: SBP) {
            let cb_data = {
                let _guard = self.lock.lock();
                let idx = match self
                    .callbacks
                    .iter()
                    .position(|cb| cb.msg_type == msg.get_message_type())
                {
                    Some(idx) => idx,
                    None => panic!(
                        "callback not registered for message type {}",
                        msg.get_message_type()
                    ),
                };
                self.callbacks[idx]
            };

            let cb = cb_data.cb.expect("callback was None");
            let cb_context = cb_data.cb_context;

            let mut payload = Vec::with_capacity(256);
            msg.append_to_sbp_buffer(&mut payload);
            let payload_ptr = payload.as_mut_ptr();

            unsafe {
                cb(
                    msg.get_sender_id().unwrap_or(0),
                    msg.sbp_size() as u8,
                    payload_ptr,
                    cb_context,
                )
            };
        }
    }

    unsafe impl Send for Context<'_> {}
    unsafe impl Sync for Context<'_> {}

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Callback {
        node: usize,
        msg_type: u16,
        cb: sbp_msg_callback_t,
        cb_context: *mut c_void,
        key: Key,
    }

    #[repr(C)]
    struct Event {
        condvar: parking_lot::Condvar,
        lock: parking_lot::Mutex<()>,
    }

    impl Event {
        fn new() -> Self {
            Self {
                condvar: parking_lot::Condvar::new(),
                lock: parking_lot::Mutex::new(()),
            }
        }

        fn wait_timeout(&self, ms: i32) -> bool {
            let mut started = self.lock.lock();
            let result = self
                .condvar
                .wait_for(&mut started, Duration::from_millis(ms as u64));
            !result.timed_out()
        }

        fn set(&self) {
            let _ = self.lock.lock();
            if !self.condvar.notify_one() {
                eprintln!("event set did not notify anything");
            }
        }
    }

    #[repr(C)]
    struct Lock(parking_lot::Mutex<()>);

    impl Lock {
        fn new() -> Self {
            Self(parking_lot::Mutex::new(()))
        }

        fn lock(&self) -> parking_lot::MutexGuard<()> {
            self.0.lock()
        }

        fn acquire(&self) {
            std::mem::forget(self.0.lock());
        }

        fn release(&self) {
            // Safety: Only called via libsettings_unlock after libsettings_lock was called
            unsafe { self.0.force_unlock() }
        }
    }

    struct CtxPtr(*mut c_void);

    unsafe impl Sync for CtxPtr {}
    unsafe impl Send for CtxPtr {}

    #[no_mangle]
    unsafe extern "C" fn register_cb(
        ctx: *mut c_void,
        msg_type: u16,
        cb: sbp_msg_callback_t,
        cb_context: *mut c_void,
        node: *mut *mut sbp_msg_callbacks_node_t,
    ) -> i32 {
        let context: &mut Context = &mut *(ctx as *mut _);
        let _guard = context.lock.lock();
        let ctx_ptr = CtxPtr(ctx);
        let key = context.link.register_cb_by_id(msg_type, move |msg: SBP| {
            let context: &mut Context = &mut *(ctx_ptr.0 as *mut _);
            context.callback_broker(msg)
        });
        context.callbacks.push(Callback {
            node: node as usize,
            msg_type,
            cb,
            cb_context,
            key,
        });
        0
    }

    #[no_mangle]
    unsafe extern "C" fn unregister_cb(
        ctx: *mut c_void,
        node: *mut *mut sbp_msg_callbacks_node_t,
    ) -> i32 {
        let context: &mut Context = &mut *(ctx as *mut _);
        let _guard = context.lock.lock();
        if (node as i32) != 0 {
            let key = {
                let idx = context
                    .callbacks
                    .iter()
                    .position(|cb| cb.node == node as usize)
                    .unwrap();
                context.callbacks.remove(idx).key
            };
            context.link.unregister_cb(key);
            0
        } else {
            -127
        }
    }

    #[no_mangle]
    unsafe extern "C" fn send(ctx: *mut c_void, msg_type: u16, len: u8, payload: *mut u8) -> i32 {
        send_from(ctx, msg_type, len, payload, 0)
    }

    #[no_mangle]
    unsafe extern "C" fn send_from(
        ctx: *mut ::std::os::raw::c_void,
        msg_type: u16,
        len: u8,
        payload: *mut u8,
        sender_id: u16,
    ) -> i32 {
        let context: &mut Context = &mut *(ctx as *mut _);
        let mut buf = slice::from_raw_parts(payload, len as usize);
        let msg = match SBP::parse(msg_type, sender_id, &mut buf) {
            Ok(msg) => msg,
            Err(err) => {
                error!("parse error: {}", err);
                return -1;
            }
        };

        if let Err(err) = context.sender.send(msg) {
            error!("failed to send message: {}", err);
            return -1;
        };

        0
    }

    #[no_mangle]
    extern "C" fn libsettings_wait(ctx: *mut c_void, timeout_ms: i32) -> i32 {
        assert!(timeout_ms > 0);
        let context: &mut Context = unsafe { &mut *(ctx as *mut _) };
        if context.event.as_ref().unwrap().wait_timeout(timeout_ms) {
            0
        } else {
            -1
        }
    }

    #[no_mangle]
    extern "C" fn libsettings_wait_thd(event: *mut c_void, timeout_ms: i32) -> i32 {
        assert!(timeout_ms > 0);
        let event: &Event = unsafe { &*(event as *const _) };
        if event.wait_timeout(timeout_ms) {
            0
        } else {
            -1
        }
    }

    #[no_mangle]
    extern "C" fn libsettings_signal_thd(event: *mut c_void) {
        let event: &Event = unsafe { &*(event as *const _) };
        event.set();
    }

    #[no_mangle]
    extern "C" fn libsettings_signal(ctx: *mut c_void) {
        let context: &Context = unsafe { &*(ctx as *const _) };
        context.event.as_ref().unwrap().set();
    }

    #[no_mangle]
    extern "C" fn libsettings_lock(ctx: *mut c_void) {
        let context: &Context = unsafe { &*(ctx as *const _) };
        context.lock.acquire();
    }

    #[no_mangle]
    extern "C" fn libsettings_unlock(ctx: *mut c_void) {
        let context: &Context = unsafe { &*(ctx as *const _) };
        context.lock.release();
    }

    #[no_mangle]
    extern "C" fn libsettings_wait_init(ctx: *mut c_void) -> i32 {
        let context: &mut Context = unsafe { &mut *(ctx as *mut _) };
        context.event = Some(Event::new());
        0
    }
}
