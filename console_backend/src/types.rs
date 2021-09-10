use crate::broadcaster::Event;
use crate::common_constants::{self as cc, SbpLogging};
use crate::constants::*;
use crate::errors::*;
use crate::log_panel::LogLevel;
use crate::output::{CsvLogging, CsvSerializer};
use crate::piksi_tools_constants::*;
use crate::settings_tab;
use crate::update_tab::UpdateTabUpdate;
use crate::utils::{mm_to_m, ms_to_sec, set_connected_frontend};

use anyhow::{Context, Result as AHResult};
use chrono::{DateTime, Utc};
use crossbeam::channel::{self, Sender};
use directories::{ProjectDirs, UserDirs};
use indexmap::set::IndexSet;
use lazy_static::lazy_static;
use log::error;
use ordered_float::OrderedFloat;
use sbp::codec::dencode::{FramedWrite, IterSinkExt};
use sbp::codec::sbp::SbpEncoder;
use sbp::messages::piksi::{MsgSpecan, MsgSpecanDep};
use sbp::messages::{
    navigation::{
        MsgBaselineNED, MsgBaselineNEDDepA, MsgDops, MsgDopsDepA, MsgGPSTime, MsgGPSTimeDepA,
        MsgPosLLH, MsgPosLLHDepA, MsgVelNED, MsgVelNEDDepA,
    },
    observation::{
        MsgObs, MsgObsDepB, MsgObsDepC, MsgOsr, PackedObsContent, PackedObsContentDepB,
        PackedObsContentDepC, PackedOsrContent,
    },
    SBP,
};
use sbp::messages::{ConcreteMessage, SBPMessage};
use serde::{Deserialize, Serialize};
use serialport::FlowControl as SPFlowControl;
use std::io;
use std::path::Path;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    fmt,
    fmt::Debug,
    fs,
    hash::Hash,
    ops::Deref,
    path::PathBuf,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc, Mutex,
    },
    time::Instant,
};
pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;
pub type UtcDateTime = DateTime<Utc>;

/// Sends SBP messages to the connected device
pub struct MsgSender {
    inner: Arc<Mutex<FramedWrite<Box<dyn io::Write + Send>, SbpEncoder>>>,
}

impl MsgSender {
    /// 42 is the conventional sender ID intended for messages sent from the host to the device
    const SENDER_ID: u16 = 42;
    const LOCK_FAILURE: &'static str = "failed to aquire sender lock";

    pub fn new<W>(wtr: W) -> Self
    where
        W: io::Write + Send + 'static,
    {
        Self {
            inner: Arc::new(Mutex::new(FramedWrite::new(
                Box::new(wtr),
                SbpEncoder::new(),
            ))),
        }
    }

    pub fn send(&self, mut msg: SBP) -> Result<()> {
        if msg.get_sender_id().is_none() {
            msg.set_sender_id(Self::SENDER_ID);
        }
        let mut framed = self.inner.lock().expect(Self::LOCK_FAILURE);
        framed.send(msg).context("while sending a message")?;
        Ok(())
    }
}

impl Clone for MsgSender {
    fn clone(&self) -> Self {
        MsgSender {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deque<T> {
    d: Vec<T>,
    capacity: usize,
}
impl<T: Clone> Deque<T> {
    pub fn with_size_limit(capacity: usize, fill_value: Option<T>) -> Deque<T> {
        let d = if let Some(val) = fill_value {
            vec![val; capacity]
        } else {
            vec![]
        };
        Deque { d, capacity }
    }
    pub fn add(&mut self, ele: T) {
        if self.d.len() == self.capacity {
            self.d.remove(0);
        }
        self.d.push(ele);
    }
    pub fn get(&self) -> &Vec<T> {
        &self.d
    }
    pub fn clear(&mut self) {
        self.d.clear();
    }
}

pub trait CapnProtoSender: Debug + Clone + Send + Sync + 'static {
    fn send_data(&mut self, msg_bytes: Vec<u8>);
}

#[derive(Debug, Clone)]
pub struct ClientSender {
    pub inner: channel::Sender<Vec<u8>>,
    pub connected: ArcBool,
}
impl ClientSender {
    pub fn new(inner: channel::Sender<Vec<u8>>) -> Self {
        Self {
            inner,
            connected: ArcBool::new_with(true),
        }
    }
}
impl CapnProtoSender for ClientSender {
    fn send_data(&mut self, msg_bytes: Vec<u8>) {
        self.inner.send(msg_bytes).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct TestSender {
    pub inner: Vec<Vec<u8>>,
}
impl CapnProtoSender for TestSender {
    fn send_data(&mut self, msg: Vec<u8>) {
        self.inner.push(msg)
    }
}

#[derive(Debug, Default)]
pub struct ArcBool(Arc<AtomicBool>);
impl ArcBool {
    pub fn new() -> ArcBool {
        ArcBool(Arc::new(AtomicBool::new(false)))
    }
    pub fn new_with(value: bool) -> ArcBool {
        ArcBool(Arc::new(AtomicBool::new(value)))
    }
    pub fn get(&self) -> bool {
        self.load(Acquire)
    }
    pub fn set(&self, set_to: bool) {
        self.store(set_to, Release);
    }
}

impl Deref for ArcBool {
    type Target = AtomicBool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for ArcBool {
    fn clone(&self) -> Self {
        ArcBool {
            0: Arc::clone(&self.0),
        }
    }
}

#[derive(Debug)]
pub struct SharedState(Arc<Mutex<SharedStateInner>>);

impl SharedState {
    pub fn new() -> SharedState {
        SharedState(Arc::new(Mutex::new(SharedStateInner::default())))
    }
    pub fn is_running(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).running
    }
    pub fn set_running<S>(&self, set_to: bool, mut client_send: S)
    where
        S: CapnProtoSender,
    {
        if set_to {
            set_connected_frontend(cc::ApplicationStates::CONNECTED, &mut client_send);
        } else {
            set_connected_frontend(cc::ApplicationStates::DISCONNECTED, &mut client_send);
            self.set_current_connection(EMPTY_STR.to_string());
        }
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).running = set_to;
    }
    pub fn is_server_running(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).server_running
    }
    pub fn stop_server_running(&self) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).running = false;
        (*shared_data).server_running = false;
    }
    pub fn is_paused(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).paused
    }
    pub fn set_paused(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).paused = set_to;
    }
    pub fn debug(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).debug
    }
    pub fn set_debug(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).debug = set_to;
    }
    pub fn current_connection(&self) -> String {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).status_bar.current_connection.clone()
    }
    pub fn set_current_connection(&self, current_connection: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).status_bar.current_connection = current_connection;
    }
    pub fn logging_directory(&self) -> PathBuf {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let mut folders = (*shared_data).connection_history.folders();
        if let Some(folder) = folders.pop() {
            PathBuf::from(folder)
        } else {
            LOG_DIRECTORY.path()
        }
    }
    pub fn set_log_level(&self, log_level: LogLevel) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).log_panel.log_level = log_level.clone();
        log::set_max_level(log_level.level_filter());
    }
    pub fn log_level(&self) -> LogLevel {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).log_panel.log_level.clone()
    }
    pub fn sbp_logging(&self) -> SbpLogging {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).logging_bar.sbp_logging.clone()
    }
    pub fn csv_logging(&self) -> CsvLogging {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).logging_bar.csv_logging.clone()
    }
    pub fn set_logging_directory(&self, directory: PathBuf) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let directory = if directory.starts_with("~/") {
            if let Ok(dir) = directory.strip_prefix("~/") {
                user_directory().join(dir)
            } else {
                directory
            }
        } else {
            directory
        };
        (*shared_data).logging_bar.logging_directory = directory;
    }
    pub fn set_sbp_logging(&self, logging: SbpLogging) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).logging_bar.sbp_logging = logging;
    }
    pub fn set_csv_logging(&self, logging: CsvLogging) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).logging_bar.csv_logging = logging;
    }
    pub fn folder_history(&self) -> IndexSet<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.folders()
    }
    pub fn file_history(&self) -> IndexSet<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.files()
    }
    pub fn address_history(&self) -> IndexSet<Address> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.addresses()
    }
    pub fn update_folder_history(&self, folder: PathBuf) {
        let folder = String::from(folder.to_str().expect(CONVERT_TO_STR_FAILURE));
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.record_folder(folder);
    }
    pub fn update_file_history(&self, filename: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.record_file(filename);
    }
    pub fn update_tcp_history(&self, host: String, port: u16) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).connection_history.record_address(host, port);
    }
    pub fn start_vel_log(&self, path: &Path) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).solution_tab.velocity_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", path, e);
                None
            }
        }
    }
    pub fn end_vel_log(&self) -> Result<()> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        if let Some(ref mut log) = (*shared_data).solution_tab.velocity_tab.log_file {
            log.flush()?;
        }
        (*shared_data).solution_tab.velocity_tab.log_file = None;
        Ok(())
    }
    pub fn start_pos_log(&self, path: &Path) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).solution_tab.position_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", path, e);
                None
            }
        }
    }
    pub fn end_pos_log(&self) -> Result<()> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        if let Some(ref mut log) = (*shared_data).solution_tab.position_tab.log_file {
            log.flush()?;
        }
        (*shared_data).solution_tab.position_tab.log_file = None;
        Ok(())
    }
    pub fn start_baseline_log(&self, path: &Path) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).baseline_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", path, e);
                None
            }
        }
    }
    pub fn end_baseline_log(&self) -> Result<()> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        if let Some(ref mut log) = (*shared_data).baseline_tab.log_file {
            log.flush()?;
        }
        (*shared_data).baseline_tab.log_file = None;
        Ok(())
    }
    pub fn update_tab_sender(&self) -> Option<Sender<Option<UpdateTabUpdate>>> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).update_tab_sender.clone()
    }
    pub fn set_update_tab_sender(&self, sender: Sender<Option<UpdateTabUpdate>>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).update_tab_sender = Some(sender);
    }
    pub fn settings_refresh(&self) -> bool {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .refresh
    }
    pub fn set_settings_refresh(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.refresh = set_to;
    }
    pub fn settings_save(&self) -> bool {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .save
    }
    pub fn set_settings_save(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.save = set_to;
    }
    pub fn settings_reset(&self) -> bool {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .reset
    }
    pub fn set_settings_reset(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.reset = set_to;
    }
    pub fn export_settings(&self) -> Option<String> {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .export
            .clone()
    }
    pub fn set_export_settings(&self, path: Option<String>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.export = path;
    }
    pub fn import_settings(&self) -> Option<String> {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .import
            .clone()
    }
    pub fn set_import_settings(&self, path: Option<String>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.import = path;
    }
    pub fn write_setting(&self) -> Option<settings_tab::SaveRequest> {
        self.lock()
            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE)
            .settings_tab
            .write
            .clone()
    }
    pub fn set_write_setting(&self, setting: Option<settings_tab::SaveRequest>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.write = setting;
    }
}

impl Deref for SharedState {
    type Target = Mutex<SharedStateInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        SharedState {
            0: Arc::clone(&self.0),
        }
    }
}

#[derive(Debug)]
pub struct SharedStateInner {
    pub(crate) status_bar: StatusBarState,
    pub(crate) logging_bar: LoggingBarState,
    pub(crate) log_panel: LogPanelState,
    pub(crate) tracking_tab: TrackingTabState,
    pub(crate) paused: bool,
    pub(crate) connection_history: ConnectionHistory,
    pub(crate) running: bool,
    pub(crate) debug: bool,
    pub(crate) server_running: bool,
    pub(crate) solution_tab: SolutionTabState,
    pub(crate) baseline_tab: BaselineTabState,
    pub(crate) advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState,
    pub(crate) update_tab_sender: Option<Sender<Option<UpdateTabUpdate>>>,
    pub(crate) settings_tab: SettingsTabState,
}
impl SharedStateInner {
    pub fn new() -> SharedStateInner {
        let connection_history = ConnectionHistory::new();
        let log_directory = connection_history.folders().pop();
        SharedStateInner {
            status_bar: StatusBarState::new(),
            logging_bar: LoggingBarState::new(log_directory),
            log_panel: LogPanelState::new(),
            tracking_tab: TrackingTabState::new(),
            paused: false,
            debug: false,
            connection_history,
            running: false,
            server_running: true,
            solution_tab: SolutionTabState::new(),
            baseline_tab: BaselineTabState::new(),
            advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState::new(),
            update_tab_sender: None,
            settings_tab: SettingsTabState::new(),
        }
    }
}
impl Default for SharedStateInner {
    fn default() -> Self {
        SharedStateInner::new()
    }
}

#[derive(Debug)]
pub struct StatusBarState {
    pub current_connection: String,
}

impl StatusBarState {
    fn new() -> StatusBarState {
        StatusBarState {
            current_connection: String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct LoggingBarState {
    pub sbp_logging: SbpLogging,
    pub csv_logging: CsvLogging,
    pub logging_directory: PathBuf,
}

impl LoggingBarState {
    fn new(log_directory: Option<String>) -> LoggingBarState {
        let logging_directory = if let Some(dir) = log_directory {
            PathBuf::from(dir)
        } else {
            LOG_DIRECTORY.path()
        };
        LoggingBarState {
            sbp_logging: SbpLogging::OFF,
            csv_logging: CsvLogging::OFF,
            logging_directory,
        }
    }
}

#[derive(Debug)]
pub struct LogPanelState {
    pub log_level: LogLevel,
}

impl LogPanelState {
    fn new() -> LogPanelState {
        LogPanelState {
            log_level: LogLevel::INFO,
        }
    }
}

#[derive(Debug)]
pub struct TrackingTabState {
    pub signals_tab: TrackingSignalsTabState,
}

impl TrackingTabState {
    fn new() -> TrackingTabState {
        TrackingTabState {
            signals_tab: TrackingSignalsTabState::new(),
        }
    }
}

#[derive(Debug)]
pub struct TrackingSignalsTabState {
    pub check_visibility: Vec<String>,
}

impl TrackingSignalsTabState {
    fn new() -> TrackingSignalsTabState {
        TrackingSignalsTabState {
            check_visibility: vec![],
        }
    }
}

#[derive(Debug)]
pub struct BaselineTabState {
    pub(crate) clear: bool,
    pub(crate) pause: bool,
    pub(crate) reset: bool,
    pub(crate) log_file: Option<CsvSerializer>,
}

impl BaselineTabState {
    fn new() -> BaselineTabState {
        BaselineTabState {
            clear: false,
            pause: false,
            reset: false,
            log_file: None,
        }
    }
}

#[derive(Debug)]
pub struct SolutionTabState {
    pub position_tab: SolutionPositionTabState,
    pub velocity_tab: SolutionVelocityTabState,
}

impl SolutionTabState {
    fn new() -> SolutionTabState {
        SolutionTabState {
            position_tab: SolutionPositionTabState::new(),
            velocity_tab: SolutionVelocityTabState::new(),
        }
    }
}

#[derive(Debug)]
pub struct SolutionPositionTabState {
    pub clear: bool,
    pub ins_status_flags: u32,
    pub last_ins_status_receipt_time: Instant,
    pub last_odo_update_time: Instant,
    pub pause: bool,
    pub unit: String,
    pub log_file: Option<CsvSerializer>,
}

impl SolutionPositionTabState {
    fn new() -> SolutionPositionTabState {
        SolutionPositionTabState {
            clear: false,
            ins_status_flags: 0,
            last_ins_status_receipt_time: Instant::now(),
            last_odo_update_time: Instant::now(),
            pause: false,
            unit: String::from(DEGREES),
            log_file: None,
        }
    }
}

#[derive(Debug)]
pub struct SolutionVelocityTabState {
    pub unit: String,
    pub log_file: Option<CsvSerializer>,
}

impl SolutionVelocityTabState {
    fn new() -> SolutionVelocityTabState {
        SolutionVelocityTabState {
            unit: String::from(MPS),
            log_file: None,
        }
    }
}

#[derive(Debug)]
pub struct AdvancedSpectrumAnalyzerTabState {
    pub channel_idx: u16,
}

impl AdvancedSpectrumAnalyzerTabState {
    fn new() -> AdvancedSpectrumAnalyzerTabState {
        AdvancedSpectrumAnalyzerTabState { channel_idx: 0 }
    }
}

#[derive(Debug)]
pub struct SettingsTabState {
    refresh: bool,
    reset: bool,
    save: bool,
    export: Option<String>,
    import: Option<String>,
    write: Option<settings_tab::SaveRequest>,
}

impl SettingsTabState {
    fn new() -> Self {
        Self {
            refresh: true,
            reset: false,
            save: false,
            export: None,
            import: None,
            write: None,
        }
    }
}

// Main Tab Types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RealtimeDelay {
    On,
    Off,
}

// Navbar Types.

/// Directory struct for storing informating and helpers pertaining to project directory.
///
/// Taken from swift-cli/swift/src/types.rs.
/// impl taken from swift-cli/swift/src/lib.rs.
#[derive(Debug)]
pub struct Directory {
    path: PathBuf,
}
lazy_static! {
    pub static ref DATA_DIRECTORY: Directory = Directory::new_data_directory();
}
lazy_static! {
    pub static ref LOG_DIRECTORY: Directory = Directory::new_log_directory();
}
impl Directory {
    pub fn new_data_directory() -> Directory {
        Directory {
            path: create_data_dir().unwrap(),
        }
    }

    pub fn new_log_directory() -> Directory {
        Directory {
            path: user_directory().join(DEFAULT_LOG_DIRECTORY),
        }
    }
    /// Return a clone to the private path PathBuf.
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
impl Default for Directory {
    fn default() -> Self {
        Directory::new_data_directory()
    }
}

/// Deduce data directory path and create folder.
///
/// Taken from swift-cli/swift/src/lib.rs.
/// # Returns
/// - `Ok`: The PathBuf for the data directory path.
/// - `Err`: Issue deducing path or creating the data directory.
fn create_data_dir() -> AHResult<PathBuf> {
    let proj_dirs = ProjectDirs::from(
        APPLICATION_QUALIFIER,
        APPLICATION_ORGANIZATION,
        APPLICATION_NAME,
    )
    .context("could not discover local project directory")?;
    let path: PathBuf = ProjectDirs::data_local_dir(&proj_dirs).into();
    fs::create_dir_all(path.clone())?;
    Ok(path)
}

/// Create directory.
///
/// Taken from swift-cli/swift/src/lib.rs.
/// # Returns
/// - `Ok`: The PathBuf for the data directory path.
/// - `Err`: Issue deducing path or creating the data directory.
pub fn create_directory(directory: PathBuf) -> Result<PathBuf> {
    fs::create_dir_all(&directory)?;
    Ok(directory)
}

/// Get user directory.
///
/// # Returns
/// - The PathBuf for the user directory path. Otherwise empty pathbuf.
pub fn user_directory() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        PathBuf::from(user_dirs.home_dir())
    } else {
        PathBuf::from("")
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionHistory {
    addresses: IndexSet<Address>,
    files: IndexSet<String>,
    folders: IndexSet<String>,
}

impl Default for ConnectionHistory {
    fn default() -> Self {
        ConnectionHistory::new()
    }
}

impl ConnectionHistory {
    /// Attempts to create a new ConnectionHistory from expected filepath otherwise empty.
    pub fn new() -> ConnectionHistory {
        let filename = DATA_DIRECTORY.path().join(CONNECTION_HISTORY_FILENAME);
        if let Ok(file) = fs::File::open(&filename) {
            if let Ok(conn_yaml) = serde_yaml::from_reader(file) {
                return conn_yaml;
            }
        }
        let mut folders = IndexSet::new();
        if let Ok(default_path) = LOG_DIRECTORY.path().into_os_string().into_string() {
            folders.insert(default_path);
        }
        ConnectionHistory {
            addresses: IndexSet::new(),
            files: IndexSet::new(),
            folders,
        }
    }
    /// Return the filename of the saved connection history file.
    fn filename(&self) -> PathBuf {
        DATA_DIRECTORY.path().join(CONNECTION_HISTORY_FILENAME)
    }
    /// Returns a clone of the private addresses vec.
    pub fn addresses(&self) -> IndexSet<Address> {
        self.addresses.clone()
    }
    /// Returns a clone of the private files vec.
    pub fn files(&self) -> IndexSet<String> {
        self.files.clone()
    }
    /// Returns a clone of the private folders vec.
    pub fn folders(&self) -> IndexSet<String> {
        self.folders.clone()
    }
    /// Attempt to add a new host and port if not the most recent entries.
    ///
    /// # Parameters
    /// - `host`: The TCP host to add to the history.
    /// - `port`: The TCP port to add to the history.
    pub fn record_address(&mut self, host: String, port: u16) {
        let address = Address { host, port };
        self.addresses.shift_remove(&address);
        self.addresses.insert(address);
        let diff = i32::max(0, self.addresses.len() as i32 - MAX_CONNECTION_HISTORY);
        self.addresses = self.addresses.split_off(diff as usize);

        if let Err(e) = self.save() {
            error!("Unable to save connection history, {}.", e);
        }
    }
    /// Attempt to add a new filepath if not the most recent entry.
    ///
    /// # Parameters
    /// - `filename`: The path to the file to add to history.
    pub fn record_file(&mut self, filename: String) {
        self.files.shift_remove(&filename);
        self.files.insert(filename);
        let diff = i32::max(0, self.files.len() as i32 - MAX_CONNECTION_HISTORY);
        self.files = self.files.split_off(diff as usize);

        if let Err(e) = self.save() {
            error!("Unable to save connection history, {}.", e);
        }
    }
    /// Attempt to add a new folder if not the most recent entry.
    ///
    /// # Parameters
    /// - `folder`: The path to the folder to add to history.
    pub fn record_folder(&mut self, folder: String) {
        self.folders.shift_remove(&folder);
        self.folders.insert(folder);
        let diff = i32::max(0, self.folders.len() as i32 - MAX_CONNECTION_HISTORY);
        self.folders = self.folders.split_off(diff as usize);

        if let Err(e) = self.save() {
            error!("Unable to save connection history, {}.", e);
        }
    }

    /// Save the history to the expected filepath.
    fn save(&self) -> Result<()> {
        serde_yaml::to_writer(fs::File::create(&self.filename())?, self)?;
        Ok(())
    }
}

// Enum wrapping around various Vel NED Message types.
#[derive(Debug, Clone)]
pub struct FlowControl(SPFlowControl);

impl FromStr for FlowControl {
    type Err = String;

    /// Convert flow control string slice to expected serialport FlowControl variant.
    ///
    /// # Parameters
    ///
    /// - `sat_str`: The signal code.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            FLOW_CONTROL_NONE => Ok(FlowControl(SPFlowControl::None)),
            FLOW_CONTROL_SOFTWARE => Ok(FlowControl(SPFlowControl::Software)),
            FLOW_CONTROL_HARDWARE => Ok(FlowControl(SPFlowControl::Hardware)),
            _ => Err(format!(
                "Not a valid flow control option. Choose from [\"{}\", \"{}\", \"{}\"]",
                FLOW_CONTROL_NONE, FLOW_CONTROL_SOFTWARE, FLOW_CONTROL_HARDWARE
            )),
        }
    }
}

impl Deref for FlowControl {
    type Target = SPFlowControl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Tracking Signals Tab Types.
pub type Cn0Dict = HashMap<(SignalCodes, i16), Deque<(OrderedFloat<f64>, f64)>>;
pub type Cn0Age = HashMap<(SignalCodes, i16), f64>;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalCodes {
    CodeGpsL1Ca = 0,
    CodeGpsL2Cm = 1,
    CodeGpsL2Cl = 7,
    CodeGpsL2Cx = 8,
    CodeGpsL1P = 5,
    CodeGpsL2P = 6,
    CodeGpsL5I = 9,
    CodeGpsL5Q = 10,
    CodeGpsL5X = 11,
    CodeGpsL1Ci = 56,
    CodeGpsL1Cq = 57,
    CodeGpsL1Cx = 58,
    CodeAuxGps = 59,

    CodeGloL1Of = 3,
    CodeGloL2Of = 4,
    CodeGloL1P = 29,
    CodeGloL2P = 30,

    CodeSbasL1Ca = 2,
    CodeSbasL5I = 41,
    CodeSbasL5Q = 42,
    CodeSbasL5X = 43,
    CodeAuxSbas = 60,

    CodeBds2B1 = 12,
    CodeBds2B2 = 13,
    CodeBds3B1Ci = 44,
    CodeBds3B1Cq = 45,
    CodeBds3B1Cx = 46,
    CodeBds3B5I = 47,
    CodeBds3B5Q = 48,
    CodeBds3B5X = 49,
    CodeBds3B7I = 50,
    CodeBds3B7Q = 51,
    CodeBds3B7X = 52,
    CodeBds3B3I = 53,
    CodeBds3B3Q = 54,
    CodeBds3B3X = 55,

    CodeGalE1B = 14,
    CodeGalE1C = 15,
    CodeGalE1X = 16,
    CodeGalE6B = 17,
    CodeGalE6C = 18,
    CodeGalE6X = 19,
    CodeGalE7I = 20,
    CodeGalE7Q = 21,
    CodeGalE7X = 22,
    CodeGalE8I = 23,
    CodeGalE8Q = 24,
    CodeGalE8X = 25,
    CodeGalE5I = 26,
    CodeGalE5Q = 27,
    CodeGalE5X = 28,
    CodeAuxGal = 61,

    CodeQzsL1Ca = 31,
    CodeQzsL1Ci = 32,
    CodeQzsL1Cq = 33,
    CodeQzsL1Cx = 34,
    CodeQzsL2Cm = 35,
    CodeQzsL2Cl = 36,
    CodeQzsL2Cx = 37,
    CodeQzsL5I = 38,
    CodeQzsL5Q = 39,
    CodeQzsL5X = 40,
    CodeAuxQzs = 62,
    NotAvailable = u8::MAX,
}

impl SignalCodes {
    pub fn code_is_gps(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGpsL1Ca
                | SignalCodes::CodeGpsL2Cm
                | SignalCodes::CodeGpsL2Cl
                | SignalCodes::CodeGpsL2Cx
                | SignalCodes::CodeGpsL1P
                | SignalCodes::CodeGpsL2P
                | SignalCodes::CodeGpsL5I
                | SignalCodes::CodeGpsL5Q
                | SignalCodes::CodeGpsL5X
                | SignalCodes::CodeAuxGps
        )
    }
    pub fn code_is_glo(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGloL1Of
                | SignalCodes::CodeGloL2Of
                | SignalCodes::CodeGloL1P
                | SignalCodes::CodeGloL2P
        )
    }
    pub fn code_is_sbas(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeSbasL1Ca
                | SignalCodes::CodeSbasL5I
                | SignalCodes::CodeSbasL5Q
                | SignalCodes::CodeSbasL5X
                | SignalCodes::CodeAuxSbas
        )
    }
    pub fn code_is_bds(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeBds2B1
                | SignalCodes::CodeBds2B2
                | SignalCodes::CodeBds3B1Ci
                | SignalCodes::CodeBds3B1Cq
                | SignalCodes::CodeBds3B1Cx
                | SignalCodes::CodeBds3B5I
                | SignalCodes::CodeBds3B5Q
                | SignalCodes::CodeBds3B5X
                | SignalCodes::CodeBds3B3I
                | SignalCodes::CodeBds3B3Q
                | SignalCodes::CodeBds3B3X
                | SignalCodes::CodeBds3B7I
                | SignalCodes::CodeBds3B7Q
                | SignalCodes::CodeBds3B7X
        )
    }

    pub fn code_is_galileo(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGalE1B
                | SignalCodes::CodeGalE1C
                | SignalCodes::CodeGalE1X
                | SignalCodes::CodeGalE6B
                | SignalCodes::CodeGalE6C
                | SignalCodes::CodeGalE6X
                | SignalCodes::CodeGalE7I
                | SignalCodes::CodeGalE7Q
                | SignalCodes::CodeGalE7X
                | SignalCodes::CodeGalE8I
                | SignalCodes::CodeGalE8Q
                | SignalCodes::CodeGalE8X
                | SignalCodes::CodeGalE5I
                | SignalCodes::CodeGalE5Q
                | SignalCodes::CodeGalE5X
                | SignalCodes::CodeAuxGal
        )
    }

    pub fn code_is_qzss(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeQzsL1Ca
                | SignalCodes::CodeQzsL2Cm
                | SignalCodes::CodeQzsL2Cl
                | SignalCodes::CodeQzsL2Cx
                | SignalCodes::CodeQzsL5I
                | SignalCodes::CodeQzsL5Q
                | SignalCodes::CodeQzsL5X
                | SignalCodes::CodeAuxQzs
        )
    }

    pub fn filters(&self) -> Option<String> {
        match self {
            SignalCodes::CodeGpsL1Ca => Some(GPS_L1CA_STR.to_string()),
            SignalCodes::CodeGpsL2Cm => Some(GPS_L2CM_STR.to_string()),
            SignalCodes::CodeGloL1Of => Some(GLO_L1OF_STR.to_string()),
            SignalCodes::CodeGloL2Of => Some(GLO_L2OF_STR.to_string()),
            SignalCodes::CodeBds2B1 => Some(BDS2_B1_STR.to_string()),
            SignalCodes::CodeBds2B2 => Some(BDS2_B2_STR.to_string()),
            SignalCodes::CodeGalE1B => Some(GAL_E1B_STR.to_string()),
            SignalCodes::CodeGalE7I => Some(GAL_E7I_STR.to_string()),
            SignalCodes::CodeQzsL1Ca => Some(QZS_L1CA_STR.to_string()),
            SignalCodes::CodeQzsL2Cm => Some(QZS_L2CM_STR.to_string()),
            SignalCodes::CodeSbasL1Ca => Some(SBAS_L1_STR.to_string()),
            _ => None,
        }
    }
}

impl From<u8> for SignalCodes {
    fn from(s: u8) -> Self {
        match s {
            0 => SignalCodes::CodeGpsL1Ca,
            1 => SignalCodes::CodeGpsL2Cm,
            7 => SignalCodes::CodeGpsL2Cl,
            8 => SignalCodes::CodeGpsL2Cx,
            5 => SignalCodes::CodeGpsL1P,
            6 => SignalCodes::CodeGpsL2P,
            9 => SignalCodes::CodeGpsL5I,
            10 => SignalCodes::CodeGpsL5Q,
            11 => SignalCodes::CodeGpsL5X,
            56 => SignalCodes::CodeGpsL1Ci,
            57 => SignalCodes::CodeGpsL1Cq,
            58 => SignalCodes::CodeGpsL1Cx,
            59 => SignalCodes::CodeAuxGps,

            3 => SignalCodes::CodeGloL1Of,
            4 => SignalCodes::CodeGloL2Of,
            29 => SignalCodes::CodeGloL1P,
            30 => SignalCodes::CodeGloL2P,

            2 => SignalCodes::CodeSbasL1Ca,
            41 => SignalCodes::CodeSbasL5I,
            42 => SignalCodes::CodeSbasL5Q,
            43 => SignalCodes::CodeSbasL5X,
            60 => SignalCodes::CodeAuxSbas,

            12 => SignalCodes::CodeBds2B1,
            13 => SignalCodes::CodeBds2B2,
            44 => SignalCodes::CodeBds3B1Ci,
            45 => SignalCodes::CodeBds3B1Cq,
            46 => SignalCodes::CodeBds3B1Cx,
            47 => SignalCodes::CodeBds3B5I,
            48 => SignalCodes::CodeBds3B5Q,
            49 => SignalCodes::CodeBds3B5X,
            50 => SignalCodes::CodeBds3B7I,
            51 => SignalCodes::CodeBds3B7Q,
            52 => SignalCodes::CodeBds3B7X,
            53 => SignalCodes::CodeBds3B3I,
            54 => SignalCodes::CodeBds3B3Q,
            55 => SignalCodes::CodeBds3B3X,

            14 => SignalCodes::CodeGalE1B,
            15 => SignalCodes::CodeGalE1C,
            16 => SignalCodes::CodeGalE1X,
            17 => SignalCodes::CodeGalE6B,
            18 => SignalCodes::CodeGalE6C,
            19 => SignalCodes::CodeGalE6X,
            20 => SignalCodes::CodeGalE7I,
            21 => SignalCodes::CodeGalE7Q,
            22 => SignalCodes::CodeGalE7X,
            23 => SignalCodes::CodeGalE8I,
            24 => SignalCodes::CodeGalE8Q,
            25 => SignalCodes::CodeGalE8X,
            26 => SignalCodes::CodeGalE5I,
            27 => SignalCodes::CodeGalE5Q,
            28 => SignalCodes::CodeGalE5X,
            61 => SignalCodes::CodeAuxGal,

            31 => SignalCodes::CodeQzsL1Ca,
            32 => SignalCodes::CodeQzsL1Ci,
            33 => SignalCodes::CodeQzsL1Cq,
            34 => SignalCodes::CodeQzsL1Cx,
            35 => SignalCodes::CodeQzsL2Cm,
            36 => SignalCodes::CodeQzsL2Cl,
            37 => SignalCodes::CodeQzsL2Cx,
            38 => SignalCodes::CodeQzsL5I,
            39 => SignalCodes::CodeQzsL5Q,
            40 => SignalCodes::CodeQzsL5X,
            62 => SignalCodes::CodeAuxQzs,
            _ => SignalCodes::NotAvailable,
        }
    }
}

impl std::str::FromStr for SignalCodes {
    type Err = Error;

    /// Retrieve the signal constellation, band and code based off provided string.
    ///
    /// # Parameters
    ///
    /// - `sat_str`: The signal code.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            GPS_L1CA_STR => Ok(SignalCodes::CodeGpsL1Ca),
            GPS_L2CM_STR => Ok(SignalCodes::CodeGpsL2Cm),
            GPS_L2CL_STR => Ok(SignalCodes::CodeGpsL2Cl),
            GPS_L2CX_STR => Ok(SignalCodes::CodeGpsL2Cx),
            GPS_L5I_STR => Ok(SignalCodes::CodeGpsL5I),
            GPS_L5Q_STR => Ok(SignalCodes::CodeGpsL5Q),
            GPS_L5X_STR => Ok(SignalCodes::CodeGpsL5X),
            GPS_L1P_STR => Ok(SignalCodes::CodeGpsL1P),
            GPS_L2P_STR => Ok(SignalCodes::CodeGpsL2P),
            GPS_AUX_STR => Ok(SignalCodes::CodeAuxGps),

            SBAS_L1_STR => Ok(SignalCodes::CodeSbasL1Ca),
            SBAS_L5I_STR => Ok(SignalCodes::CodeSbasL5I),
            SBAS_L5Q_STR => Ok(SignalCodes::CodeSbasL5Q),
            SBAS_L5X_STR => Ok(SignalCodes::CodeSbasL5X),
            SBAS_AUX_STR => Ok(SignalCodes::CodeAuxSbas),

            GLO_L1OF_STR => Ok(SignalCodes::CodeGloL1Of),
            GLO_L2OF_STR => Ok(SignalCodes::CodeGloL2Of),
            GLO_L1P_STR => Ok(SignalCodes::CodeGloL1P),
            GLO_L2P_STR => Ok(SignalCodes::CodeGloL2P),

            BDS2_B1_STR => Ok(SignalCodes::CodeBds2B1),
            BDS2_B2_STR => Ok(SignalCodes::CodeBds2B2),
            BDS3_B1CI_STR => Ok(SignalCodes::CodeBds3B1Ci),
            BDS3_B1CQ_STR => Ok(SignalCodes::CodeBds3B1Cq),
            BDS3_B1CX_STR => Ok(SignalCodes::CodeBds3B1Cx),
            BDS3_B5I_STR => Ok(SignalCodes::CodeBds3B5I),
            BDS3_B5Q_STR => Ok(SignalCodes::CodeBds3B5Q),
            BDS3_B5X_STR => Ok(SignalCodes::CodeBds3B5X),
            BDS3_B7I_STR => Ok(SignalCodes::CodeBds3B7I),
            BDS3_B7Q_STR => Ok(SignalCodes::CodeBds3B7Q),
            BDS3_B7X_STR => Ok(SignalCodes::CodeBds3B7X),
            BDS3_B3I_STR => Ok(SignalCodes::CodeBds3B3I),
            BDS3_B3Q_STR => Ok(SignalCodes::CodeBds3B3Q),
            BDS3_B3X_STR => Ok(SignalCodes::CodeBds3B3X),

            GAL_E1B_STR => Ok(SignalCodes::CodeGalE1B),
            GAL_E1C_STR => Ok(SignalCodes::CodeGalE1C),
            GAL_E1X_STR => Ok(SignalCodes::CodeGalE1X),
            GAL_E5I_STR => Ok(SignalCodes::CodeGalE5I),
            GAL_E5Q_STR => Ok(SignalCodes::CodeGalE5Q),
            GAL_E5X_STR => Ok(SignalCodes::CodeGalE5X),
            GAL_E6B_STR => Ok(SignalCodes::CodeGalE6B),
            GAL_E6C_STR => Ok(SignalCodes::CodeGalE6C),
            GAL_E6X_STR => Ok(SignalCodes::CodeGalE6X),
            GAL_E7I_STR => Ok(SignalCodes::CodeGalE7I),
            GAL_E7Q_STR => Ok(SignalCodes::CodeGalE7Q),
            GAL_E7X_STR => Ok(SignalCodes::CodeGalE7X),
            GAL_E8I_STR => Ok(SignalCodes::CodeGalE8I),
            GAL_E8Q_STR => Ok(SignalCodes::CodeGalE8Q),
            GAL_E8X_STR => Ok(SignalCodes::CodeGalE8X),
            GAL_AUX_STR => Ok(SignalCodes::CodeAuxGal),

            QZS_L1CA_STR => Ok(SignalCodes::CodeQzsL1Ca),
            QZS_L2CM_STR => Ok(SignalCodes::CodeQzsL2Cm),
            QZS_L2CL_STR => Ok(SignalCodes::CodeQzsL2Cl),
            QZS_L2CX_STR => Ok(SignalCodes::CodeQzsL2Cx),
            QZS_L5I_STR => Ok(SignalCodes::CodeQzsL5I),
            QZS_L5Q_STR => Ok(SignalCodes::CodeQzsL5Q),
            QZS_L5X_STR => Ok(SignalCodes::CodeQzsL5X),
            QZS_AUX_STR => Ok(SignalCodes::CodeAuxQzs),
            _ => Ok(SignalCodes::NotAvailable),
        }
    }
}

impl fmt::Display for SignalCodes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sat_code_str = match self {
            SignalCodes::CodeGpsL1Ca => GPS_L1CA_STR,
            SignalCodes::CodeGpsL2Cm => GPS_L2CM_STR,
            SignalCodes::CodeGpsL2Cl => GPS_L2CL_STR,
            SignalCodes::CodeGpsL2Cx => GPS_L2CX_STR,
            SignalCodes::CodeGpsL1P => GPS_L1P_STR,
            SignalCodes::CodeGpsL2P => GPS_L2P_STR,
            SignalCodes::CodeGpsL5I => GPS_L5I_STR,
            SignalCodes::CodeGpsL5Q => GPS_L5Q_STR,
            SignalCodes::CodeGpsL5X => GPS_L5X_STR,
            SignalCodes::CodeAuxGps => GPS_AUX_STR,

            SignalCodes::CodeGloL1Of => GLO_L1OF_STR,
            SignalCodes::CodeGloL2Of => GLO_L2OF_STR,
            SignalCodes::CodeGloL1P => GLO_L1P_STR,
            SignalCodes::CodeGloL2P => GLO_L2P_STR,

            SignalCodes::CodeSbasL1Ca => SBAS_L1_STR,
            SignalCodes::CodeSbasL5I => SBAS_L5I_STR,
            SignalCodes::CodeSbasL5Q => SBAS_L5Q_STR,
            SignalCodes::CodeSbasL5X => SBAS_L5X_STR,
            SignalCodes::CodeAuxSbas => SBAS_AUX_STR,

            SignalCodes::CodeBds2B1 => BDS2_B1_STR,
            SignalCodes::CodeBds2B2 => BDS2_B2_STR,
            SignalCodes::CodeBds3B1Ci => BDS3_B1CI_STR,
            SignalCodes::CodeBds3B1Cq => BDS3_B1CQ_STR,
            SignalCodes::CodeBds3B1Cx => BDS3_B1CX_STR,
            SignalCodes::CodeBds3B5I => BDS3_B5I_STR,
            SignalCodes::CodeBds3B5Q => BDS3_B5Q_STR,
            SignalCodes::CodeBds3B5X => BDS3_B5X_STR,
            SignalCodes::CodeBds3B7I => BDS3_B7I_STR,
            SignalCodes::CodeBds3B7Q => BDS3_B7Q_STR,
            SignalCodes::CodeBds3B7X => BDS3_B7X_STR,
            SignalCodes::CodeBds3B3I => BDS3_B3I_STR,
            SignalCodes::CodeBds3B3Q => BDS3_B3Q_STR,
            SignalCodes::CodeBds3B3X => BDS3_B3X_STR,

            SignalCodes::CodeGalE1B => GAL_E1B_STR,
            SignalCodes::CodeGalE1C => GAL_E1C_STR,
            SignalCodes::CodeGalE1X => GAL_E1X_STR,
            SignalCodes::CodeGalE6B => GAL_E6B_STR,
            SignalCodes::CodeGalE6C => GAL_E6C_STR,
            SignalCodes::CodeGalE6X => GAL_E6X_STR,
            SignalCodes::CodeGalE7I => GAL_E7I_STR,
            SignalCodes::CodeGalE7Q => GAL_E7Q_STR,
            SignalCodes::CodeGalE7X => GAL_E7X_STR,
            SignalCodes::CodeGalE8I => GAL_E8I_STR,
            SignalCodes::CodeGalE8Q => GAL_E8Q_STR,
            SignalCodes::CodeGalE8X => GAL_E8X_STR,
            SignalCodes::CodeGalE5I => GAL_E5I_STR,
            SignalCodes::CodeGalE5Q => GAL_E5Q_STR,
            SignalCodes::CodeGalE5X => GAL_E5X_STR,
            SignalCodes::CodeAuxGal => GAL_AUX_STR,

            SignalCodes::CodeQzsL1Ca => QZS_L1CA_STR,
            SignalCodes::CodeQzsL2Cm => QZS_L2CM_STR,
            SignalCodes::CodeQzsL2Cl => QZS_L2CL_STR,
            SignalCodes::CodeQzsL2Cx => QZS_L2CX_STR,
            SignalCodes::CodeQzsL5I => QZS_L5I_STR,
            SignalCodes::CodeQzsL5Q => QZS_L5Q_STR,
            SignalCodes::CodeQzsL5X => QZS_L5X_STR,
            _ => CODE_NOT_AVAILABLE,
        };
        write!(f, "{}", sat_code_str)
    }
}

// Struct with shared fields for various Observation Message types.
pub struct ObservationMsgFields {
    pub n_obs: u8,
    pub tow: f64,
    pub wn: u16,
    pub ns_residual: i32,
    pub states: Vec<Observations>,
    pub sender_id: Option<u16>,
}

// Enum wrapping around various Observation Message types.
#[derive(Debug, Clone)]
pub enum ObservationMsg {
    MsgObs(MsgObs),
    // MsgObsDepA(MsgObsDepA),
    MsgObsDepB(MsgObsDepB),
    MsgObsDepC(MsgObsDepC),
    MsgOsr(MsgOsr),
}

impl ObservationMsg {
    pub fn fields(&self) -> ObservationMsgFields {
        let (n_obs, tow, wn, ns_residual, states, sender_id) = match &self {
            ObservationMsg::MsgObs(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContent)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    obs.header.t.ns_residual,
                    states,
                    obs.sender_id,
                )
            }
            // ObservationMsg::MsgObsDepA(obs)
            ObservationMsg::MsgObsDepB(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContentDepB)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    0_i32,
                    states,
                    obs.sender_id,
                )
            }
            ObservationMsg::MsgObsDepC(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContentDepC)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    0_i32,
                    states,
                    obs.sender_id,
                )
            }

            ObservationMsg::MsgOsr(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedOsrContent)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    obs.header.t.ns_residual,
                    states,
                    obs.sender_id,
                )
            }
        };
        ObservationMsgFields {
            n_obs,
            tow,
            wn,
            ns_residual,
            states,
            sender_id,
        }
    }
}

impl Event for ObservationMsg {
    const MESSAGE_TYPES: &'static [u16] = &[
        MsgObs::MESSAGE_TYPE,
        MsgObsDepB::MESSAGE_TYPE,
        MsgObsDepC::MESSAGE_TYPE,
        MsgOsr::MESSAGE_TYPE,
    ];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgObs(m) => ObservationMsg::MsgObs(m),
            SBP::MsgObsDepB(m) => ObservationMsg::MsgObsDepB(m),
            SBP::MsgObsDepC(m) => ObservationMsg::MsgObsDepC(m),
            SBP::MsgOsr(m) => ObservationMsg::MsgOsr(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Observation Contents types.
pub struct ObservationFields {
    pub is_deprecated_msg_type: bool,
    pub code: SignalCodes,
    pub sat: i16,
    pub pseudo_range: f64,
    pub carrier_phase: f64,
    pub cn0: f64,
    pub measured_doppler: f64,
    pub computed_doppler: f64,
    pub lock: u16,
    pub flags: u8,
}

// Enum wrapping around various Observation Contents observation types.
pub enum Observations {
    PackedObsContent(PackedObsContent),
    // PackedObsContentDepA(PackedObsContentDepA),
    PackedObsContentDepB(PackedObsContentDepB),
    PackedObsContentDepC(PackedObsContentDepC),
    PackedOsrContent(PackedOsrContent),
}

impl Observations {
    pub fn fields(&self) -> ObservationFields {
        // DEP_B and DEP_A obs had different pseudorange scaling
        let divisor = match self {
            Observations::PackedObsContentDepB(_) => 1e2,
            _ => 5e1,
        };
        let (
            is_deprecated_msg_type,
            code,
            sat,
            pseudo_range,
            carrier_phase_cycles,
            carrier_phase_fractional,
            cn0,
            measured_doppler,
            lock,
            flags,
        ) = match self {
            Observations::PackedObsContentDepB(obs) => {
                let mut sat_ = obs.sid.sat as i16;
                let signal_code = SignalCodes::from(obs.sid.code);
                if signal_code.code_is_gps() {
                    sat_ += 1;
                }
                (
                    true,
                    signal_code,
                    sat_,
                    obs.P as f64 / divisor,
                    obs.L.i,
                    obs.L.f,
                    obs.cn0 as f64,
                    0_f64, // obs.D
                    obs.lock,
                    0_u8,
                )
            }
            Observations::PackedObsContentDepC(obs) => {
                let mut sat_ = obs.sid.sat as i16;
                let signal_code = SignalCodes::from(obs.sid.code);
                if signal_code.code_is_gps() {
                    sat_ += 1;
                }
                (
                    true,
                    signal_code,
                    sat_,
                    obs.P as f64 / divisor,
                    obs.L.i,
                    obs.L.f,
                    obs.cn0 as f64,
                    0_f64, // obs.D
                    obs.lock,
                    0_u8,
                )
            }
            Observations::PackedObsContent(obs) => (
                false,
                SignalCodes::from(obs.sid.code),
                obs.sid.sat as i16,
                obs.P as f64 / divisor,
                obs.L.i,
                obs.L.f,
                obs.cn0 as f64,
                obs.D.i as f64 + obs.D.f as f64 / ((1 << 8) as f64),
                obs.lock as u16,
                obs.flags,
            ),
            Observations::PackedOsrContent(obs) => (
                false,
                SignalCodes::from(obs.sid.code),
                obs.sid.sat as i16,
                obs.P as f64 / divisor,
                obs.L.i,
                obs.L.f,
                0_f64, // cn0
                0_f64, // obs.D
                obs.lock as u16,
                obs.flags,
            ),
        };
        let carrier_phase =
            carrier_phase_cycles as f64 + carrier_phase_fractional as f64 / ((1 << 8) as f64);
        ObservationFields {
            is_deprecated_msg_type,
            code,
            sat,
            pseudo_range,
            carrier_phase,
            cn0,
            measured_doppler,
            computed_doppler: 0_f64,
            lock,
            flags,
        }
    }
}

// Solution Tab Types.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum GnssModes {
    NoFix = 0,
    Spp = 1,
    Dgnss = 2,
    Float = 3,
    Fixed = 4,
    Dr = 5,
    Sbas = 6,
}
impl From<u8> for GnssModes {
    fn from(s: u8) -> Self {
        match s {
            0 => GnssModes::NoFix,
            1 => GnssModes::Spp,
            2 => GnssModes::Dgnss,
            3 => GnssModes::Float,
            4 => GnssModes::Fixed,
            5 => GnssModes::Dr,
            6 => GnssModes::Sbas,
            _ => panic!("this u8 does not convert to a gnss mode"),
        }
    }
}
impl fmt::Display for GnssModes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gnss_mode_str = match self {
            GnssModes::NoFix => NO_FIX,
            GnssModes::Spp => SPP,
            GnssModes::Dgnss => DGNSS,
            GnssModes::Float => FLOAT,
            GnssModes::Fixed => FIXED,
            GnssModes::Dr => DR,
            GnssModes::Sbas => SBAS,
        };
        write!(f, "{}", gnss_mode_str)
    }
}
impl GnssModes {
    pub fn label(&self) -> String {
        let gnss_mode_label = match self {
            GnssModes::NoFix => NO_FIX_LABEL,
            GnssModes::Spp => SPP_LABEL,
            GnssModes::Dgnss => DGNSS_LABEL,
            GnssModes::Float => FLOAT_LABEL,
            GnssModes::Fixed => FIXED_LABEL,
            GnssModes::Dr => DR_LABEL,
            GnssModes::Sbas => SBAS_LABEL,
        };
        String::from(gnss_mode_label)
    }
    pub fn color(&self) -> String {
        let gnss_mode_color = match self {
            GnssModes::NoFix => NO_FIX_COLOR,
            GnssModes::Spp => SPP_COLOR,
            GnssModes::Dgnss => DGNSS_COLOR,
            GnssModes::Float => FLOAT_COLOR,
            GnssModes::Fixed => FIXED_COLOR,
            GnssModes::Dr => DR_COLOR,
            GnssModes::Sbas => SBAS_COLOR,
        };
        String::from(gnss_mode_color)
    }
    pub fn pos_mode(&self) -> String {
        let gnss_pos_mode = match self {
            GnssModes::NoFix => NO_FIX_LABEL,
            GnssModes::Spp => SPP,
            GnssModes::Dgnss => DGNSS,
            GnssModes::Float => RTK,
            GnssModes::Fixed => RTK,
            GnssModes::Dr => DR_LABEL,
            GnssModes::Sbas => SBAS,
        };
        String::from(gnss_pos_mode)
    }
}

// Struct with shared fields for various PosLLH Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct PosLLHFields {
    pub flags: u8,
    pub h_accuracy: f64,
    pub v_accuracy: f64,
    pub tow: f64,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
    pub n_sats: u8,
}

// Enum wrapping around various PosLLH Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum PosLLH {
    MsgPosLLH(MsgPosLLH),
    MsgPosLLHDepA(MsgPosLLHDepA),
}

impl PosLLH {
    pub fn fields(&self) -> PosLLHFields {
        match self {
            PosLLH::MsgPosLLH(MsgPosLLH {
                flags,
                h_accuracy,
                v_accuracy,
                tow,
                lat,
                lon,
                height,
                n_sats,
                ..
            })
            | PosLLH::MsgPosLLHDepA(MsgPosLLHDepA {
                flags,
                h_accuracy,
                v_accuracy,
                tow,
                lat,
                lon,
                height,
                n_sats,
                ..
            }) => PosLLHFields {
                flags: *flags,
                h_accuracy: mm_to_m(*h_accuracy as f64),
                v_accuracy: mm_to_m(*v_accuracy as f64),
                tow: *tow as f64,
                lat: *lat,
                lon: *lon,
                height: *height,
                n_sats: *n_sats,
            },
        }
    }
    pub fn mode(&self) -> u8 {
        match self {
            PosLLH::MsgPosLLH(msg) => msg.flags & 0x7,
            PosLLH::MsgPosLLHDepA(msg) => {
                let mode = msg.flags & 0x7;
                match mode {
                    0 => 1,
                    1 => 4,
                    2 => 3,
                    _ => mode,
                }
            }
        }
    }
}

impl Event for PosLLH {
    const MESSAGE_TYPES: &'static [u16] = &[MsgPosLLH::MESSAGE_TYPE, MsgPosLLHDepA::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgPosLLH(m) => PosLLH::MsgPosLLH(m),
            SBP::MsgPosLLHDepA(m) => PosLLH::MsgPosLLHDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Dops Message types.
pub struct DopsFields {
    pub pdop: u16,
    pub gdop: u16,
    pub tdop: u16,
    pub hdop: u16,
    pub vdop: u16,
    pub flags: u8,
}
// Enum wrapping around various Dops Message types.
#[derive(Debug, Clone)]
pub enum Dops {
    MsgDops(MsgDops),
    MsgDopsDepA(MsgDopsDepA),
}

impl Dops {
    pub fn fields(self) -> DopsFields {
        let (pdop, gdop, tdop, hdop, vdop, flags) = match self {
            Dops::MsgDops(msg_) => (
                msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, msg_.flags,
            ),
            Dops::MsgDopsDepA(msg_) => {
                (msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, 1_u8)
            }
        };
        DopsFields {
            pdop,
            gdop,
            tdop,
            hdop,
            vdop,
            flags,
        }
    }
}

impl Event for Dops {
    const MESSAGE_TYPES: &'static [u16] = &[MsgDops::MESSAGE_TYPE, MsgDopsDepA::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgDops(m) => Dops::MsgDops(m),
            SBP::MsgDopsDepA(m) => Dops::MsgDopsDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various GpsTime Message types.
pub struct GpsTimeFields {
    pub wn: u16,
    pub ns_residual: i32,
    pub flags: u8,
}
// Enum wrapping around various GpsTime Message types.
#[derive(Debug, Clone)]
pub enum GpsTime {
    MsgGpsTime(MsgGPSTime),
    MsgGpsTimeDepA(MsgGPSTimeDepA),
}

impl GpsTime {
    pub fn fields(self) -> GpsTimeFields {
        let (wn, ns_residual, flags) = match self {
            GpsTime::MsgGpsTime(msg_) => (msg_.wn, msg_.ns_residual, msg_.flags),
            GpsTime::MsgGpsTimeDepA(msg_) => (msg_.wn, msg_.ns_residual, msg_.flags),
        };
        GpsTimeFields {
            wn,
            ns_residual,
            flags,
        }
    }
}

impl Event for GpsTime {
    const MESSAGE_TYPES: &'static [u16] = &[MsgGPSTime::MESSAGE_TYPE, MsgGPSTimeDepA::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgGPSTime(m) => GpsTime::MsgGpsTime(m),
            SBP::MsgGPSTimeDepA(m) => GpsTime::MsgGpsTimeDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Specan Message types.
pub struct SpecanFields {
    pub wn: u16,
    pub tow: u32,
    pub ns_residual: i32,
    pub amplitude_value: Vec<u8>,
    pub freq_ref: f32,
    pub freq_step: f32,
    pub amplitude_ref: f32,
    pub amplitude_unit: f32,
    pub channel_tag: u16,
}

// Enum wrapping around various Specan Message types.
#[derive(Debug, Clone)]
pub enum Specan {
    MsgSpecan(MsgSpecan),
    MsgSpecanDep(MsgSpecanDep),
}

impl Specan {
    pub fn fields(self) -> SpecanFields {
        let (
            wn,
            tow,
            ns_residual,
            amplitude_value,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            channel_tag,
        ) = match self {
            Specan::MsgSpecan(msg_) => (
                msg_.t.wn,
                msg_.t.tow,
                msg_.t.ns_residual,
                msg_.amplitude_value,
                msg_.freq_ref,
                msg_.freq_step,
                msg_.amplitude_ref,
                msg_.amplitude_unit,
                msg_.channel_tag,
            ),
            Specan::MsgSpecanDep(msg_) => (
                msg_.t.wn,
                msg_.t.tow,
                /*msg_.t.ns_residual*/ 0,
                msg_.amplitude_value,
                msg_.freq_ref,
                msg_.freq_step,
                msg_.amplitude_ref,
                msg_.amplitude_unit,
                msg_.channel_tag,
            ),
        };
        SpecanFields {
            wn,
            tow,
            ns_residual,
            amplitude_value,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            channel_tag,
        }
    }
}

impl Event for Specan {
    const MESSAGE_TYPES: &'static [u16] = &[MsgSpecan::MESSAGE_TYPE, MsgSpecanDep::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgSpecan(m) => Specan::MsgSpecan(m),
            SBP::MsgSpecanDep(m) => Specan::MsgSpecanDep(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various VelNED Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct VelNEDFields {
    pub flags: u8,
    pub tow: f64,
    pub n: i32,
    pub e: i32,
    pub d: i32,
    pub n_sats: u8,
}

// Enum wrapping around various Vel NED Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum VelNED {
    MsgVelNED(MsgVelNED),
    MsgVelNEDDepA(MsgVelNEDDepA),
}

impl VelNED {
    pub fn fields(self) -> VelNEDFields {
        let (flags, tow, n, e, d, n_sats) = match self {
            VelNED::MsgVelNED(msg) => (msg.flags, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
            VelNED::MsgVelNEDDepA(msg) => (1, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
        };
        VelNEDFields {
            flags,
            tow,
            n,
            e,
            d,
            n_sats,
        }
    }
}

impl Event for VelNED {
    const MESSAGE_TYPES: &'static [u16] = &[MsgVelNED::MESSAGE_TYPE, MsgVelNEDDepA::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgVelNED(m) => VelNED::MsgVelNED(m),
            SBP::MsgVelNEDDepA(m) => VelNED::MsgVelNEDDepA(m),
            _ => unreachable!(),
        }
    }
}

// Baseline Tab Types.

// Struct with shared fields for various BaselineNED Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct BaselineNEDFields {
    pub flags: u8,
    pub tow: f64,
    pub n: i32,
    pub e: i32,
    pub d: i32,
    pub h_accuracy: u16,
    pub v_accuracy: u16,
    pub n_sats: u8,
}
// Enum wrapping around various Baseline NED Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum BaselineNED {
    MsgBaselineNED(MsgBaselineNED),
    MsgBaselineNEDDepA(MsgBaselineNEDDepA),
}

impl BaselineNED {
    pub fn fields(&self) -> BaselineNEDFields {
        let (flags, tow, n, e, d, h_accuracy, v_accuracy, n_sats) = match self {
            BaselineNED::MsgBaselineNED(msg) => (
                msg.flags,
                msg.tow as f64,
                msg.n,
                msg.e,
                msg.d,
                msg.h_accuracy,
                msg.v_accuracy,
                msg.n_sats,
            ),
            BaselineNED::MsgBaselineNEDDepA(msg) => (
                1,
                msg.tow as f64,
                msg.n,
                msg.e,
                msg.d,
                msg.h_accuracy,
                msg.v_accuracy,
                msg.n_sats,
            ),
        };
        BaselineNEDFields {
            flags,
            tow,
            n,
            e,
            d,
            h_accuracy,
            v_accuracy,
            n_sats,
        }
    }
    pub fn mode(&self) -> u8 {
        match self {
            BaselineNED::MsgBaselineNED(MsgBaselineNED { flags, .. })
            | BaselineNED::MsgBaselineNEDDepA(MsgBaselineNEDDepA { flags, .. }) => *flags & 0x7,
        }
    }
}

impl Event for BaselineNED {
    const MESSAGE_TYPES: &'static [u16] = &[
        MsgBaselineNED::MESSAGE_TYPE,
        MsgBaselineNEDDepA::MESSAGE_TYPE,
    ];

    fn from_sbp(msg: SBP) -> Self {
        match msg {
            SBP::MsgBaselineNED(m) => BaselineNED::MsgBaselineNED(m),
            SBP::MsgBaselineNEDDepA(m) => BaselineNED::MsgBaselineNEDDepA(m),
            _ => unreachable!(),
        }
    }
}

// Solution Velocity Tab Types.
#[derive(Debug, Clone, PartialEq)]
pub enum VelocityUnits {
    Mps,
    Mph,
    Kph,
}

impl VelocityUnits {
    /// Retrieve the velocity unit as string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            VelocityUnits::Mps => MPS,
            VelocityUnits::Mph => MPH,
            VelocityUnits::Kph => KPH,
        }
    }
    pub fn get_multiplier(&self) -> f64 {
        match self {
            VelocityUnits::Mps => 1.0,
            VelocityUnits::Mph => MPS2MPH,
            VelocityUnits::Kph => MPS2KPH,
        }
    }
}

impl std::str::FromStr for VelocityUnits {
    type Err = Error;
    /// Retrieve the velocity unit from string slice.
    ///
    /// # Parameters
    ///
    /// - `s`: The string slice to convert to VelocityUnits.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            MPS => Ok(VelocityUnits::Mps),
            MPH => Ok(VelocityUnits::Mph),
            KPH => Ok(VelocityUnits::Kph),
            _ => panic!("unable to convert to VelocityUnits"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{backup_file, data_directories, filename, restore_backup_file};
    use serial_test::serial;

    #[test]
    fn create_data_dir_test() {
        create_data_dir().unwrap();
        let user_dirs = UserDirs::new().unwrap();
        let home_dir = user_dirs.home_dir();
        #[cfg(target_os = "linux")]
        {
            assert!(home_dir.join(data_directories::LINUX).exists());
        }

        #[cfg(target_os = "macos")]
        {
            assert!(home_dir.join(data_directories::MACOS).exists());
        }
        #[cfg(target_os = "windows")]
        {
            assert!(home_dir.join(data_directories::WINDOWS).exists());
        }
    }

    #[test]
    #[serial]
    fn connection_history_save_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());
        let conn_history = ConnectionHistory::new();
        conn_history.save().unwrap();
        assert!(bfilename.exists());
        restore_backup_file(bfilename);
    }

    #[test]
    #[serial]
    fn connection_history_additions_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());

        let mut conn_history = ConnectionHistory::new();
        let host1 = String::from("host1");
        let host2 = String::from("host2");
        let port = 100;

        conn_history.record_address(host1.clone(), port);
        let addresses = conn_history.addresses();
        let addresses_len = addresses.len();
        let first_addy = addresses.first().unwrap();
        assert_eq!(host1, first_addy.host);
        assert_eq!(port, first_addy.port);

        conn_history.record_address(host2.clone(), port);
        let addresses = conn_history.addresses();
        let first_addy = addresses.first().unwrap();
        let second_addy = addresses.get_index(1).unwrap();
        assert_eq!(host1, first_addy.host);
        assert_eq!(port, first_addy.port);
        assert_eq!(host2, second_addy.host);
        assert_eq!(port, second_addy.port);
        assert_eq!(addresses_len + 1, addresses.len());

        conn_history.record_address(host1.clone(), port);
        let addresses = conn_history.addresses();
        let first_addy = addresses.first().unwrap();
        let second_addy = addresses.get_index(1).unwrap();
        assert_eq!(host2, first_addy.host);
        assert_eq!(port, first_addy.port);
        assert_eq!(host1, second_addy.host);
        assert_eq!(port, second_addy.port);
        assert_eq!(addresses_len + 1, addresses.len());

        let filename1 = String::from("filename1");
        let filename2 = String::from("filename2");
        conn_history.record_file(filename1.clone());
        conn_history.record_file(filename2.clone());
        conn_history.record_file(filename1.clone());
        let files = conn_history.files();
        assert_eq!(filename1, files[1]);
        assert_eq!(filename2, files[0]);

        for ele in 0..MAX_CONNECTION_HISTORY {
            conn_history.record_file(ele.to_string());
        }
        assert_eq!(conn_history.files().len(), MAX_CONNECTION_HISTORY as usize);
        restore_backup_file(bfilename);
    }
}
