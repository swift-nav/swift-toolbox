use std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
    fs,
    hash::Hash,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
    thread::JoinHandle,
    time::Instant,
};

use anyhow::{Context, Result as AHResult};
use chrono::{DateTime, Utc};
use crossbeam::channel::Sender;
use directories::{ProjectDirs, UserDirs};
use indexmap::set::IndexSet;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use log::error;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serialport::FlowControl;

use crate::constants::{
    APPLICATION_NAME, APPLICATION_ORGANIZATION, APPLICATION_QUALIFIER, CONNECTION_HISTORY_FILENAME,
    DEFAULT_IP_ADDRESS, DEFAULT_LOG_DIRECTORY, DEFAULT_PORT, MAX_CONNECTION_HISTORY, MPS,
};
use crate::errors::CONVERT_TO_STR_FAILURE;
use crate::log_panel::LogLevel;
use crate::ntrip_tab::{NtripOptions, NtripState};
use crate::output::{CsvLogging, CsvSerializer};
use crate::process_messages::StopToken;
use crate::settings_tab;
use crate::solution_tab::LatLonUnits;
use crate::update_tab::UpdateTabUpdate;
use crate::utils::send_conn_state;
use crate::watch::{WatchReceiver, Watched};
use crate::{client_sender::BoxedClientSender, main_tab::logging_stats_thread};
use crate::{common_constants::ConnectionType, connection::Connection};
use crate::{
    common_constants::{self as cc, SbpLogging},
    status_bar::Heartbeat,
};

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;
pub type UtcDateTime = DateTime<Utc>;

#[derive(Debug)]
pub struct SharedState(Arc<Mutex<SharedStateInner>>);

impl SharedState {
    pub fn new() -> SharedState {
        SharedState(Arc::new(Mutex::new(SharedStateInner::default())))
    }
    pub fn connection(&self) -> ConnectionState {
        self.lock().conn.get()
    }
    pub fn connection_dialog_visible(&self) -> bool {
        self.lock().connection_dialog_visible
    }
    pub fn watch_connection(&self) -> WatchReceiver<ConnectionState> {
        self.lock().conn.watch()
    }
    pub fn set_connection(&self, conn: ConnectionState, client_sender: &BoxedClientSender) {
        {
            let shared_data = self.lock();
            if let ConnectionState::Connected { stop_token, .. } = shared_data.conn.get() {
                stop_token.stop();
            }
            shared_data.conn.send(conn.clone());
        }
        send_conn_state(conn, client_sender);
    }
    pub fn debug(&self) -> bool {
        self.lock().debug
    }
    pub fn set_debug(&self, set_to: bool) {
        self.lock().debug = set_to;
    }
    pub fn logging_directory(&self) -> PathBuf {
        let mut folders = self.lock().connection_history.folders();
        if let Some(folder) = folders.pop() {
            PathBuf::from(folder)
        } else {
            LOG_DIRECTORY.path()
        }
    }
    pub fn log_level(&self) -> LogLevel {
        self.lock().log_panel.level.clone()
    }
    pub fn set_log_level(&self, log_level: LogLevel) {
        let filter = log_level.level_filter();
        self.lock().log_panel.level = log_level;
        log::set_max_level(filter);
    }
    pub fn log_filename(&self) -> Option<PathBuf> {
        self.lock().log_panel.filename.clone()
    }
    pub fn set_log_filename(&self, filename: Option<PathBuf>) {
        self.lock().log_panel.filename = filename;
    }
    pub fn reset_logging(&self) {
        let mut guard = self.lock();
        guard.logging_bar.sbp_logging = false;
        guard.logging_bar.sbp_logging_format = SbpLogging::SBP_JSON;
        guard.logging_bar.sbp_logging_filepath = None;
        guard.logging_bar.csv_logging = CsvLogging::OFF;
    }
    pub fn sbp_logging(&self) -> bool {
        self.lock().logging_bar.sbp_logging
    }
    pub fn set_sbp_logging(&self, running: bool, client_sender: BoxedClientSender) {
        let mut guard = self.lock();
        guard.logging_bar.sbp_logging = running;
        if running && guard.logging_bar.handle.is_none() {
            let handle = logging_stats_thread(self.clone(), client_sender);
            guard.logging_bar.handle = Some(handle);
        }
    }
    pub fn sbp_logging_format(&self) -> SbpLogging {
        self.lock().logging_bar.sbp_logging_format.clone()
    }
    pub fn set_sbp_logging_format(&self, logging: SbpLogging) {
        self.lock().logging_bar.sbp_logging_format = logging;
    }
    pub fn sbp_logging_filename(&self) -> Option<PathBuf> {
        self.lock().logging_bar.sbp_logging_filename.clone()
    }
    pub fn set_sbp_logging_filename(&self, sbp_logging_filename: Option<PathBuf>) {
        self.lock().logging_bar.sbp_logging_filename = sbp_logging_filename;
    }
    pub fn sbp_logging_filepath(&self) -> Option<PathBuf> {
        self.lock().logging_bar.sbp_logging_filepath.clone()
    }
    pub fn set_sbp_logging_filepath(&self, sbp_logging_filepath: Option<PathBuf>) {
        self.lock().logging_bar.sbp_logging_filepath = sbp_logging_filepath;
    }
    pub fn csv_logging(&self) -> CsvLogging {
        self.lock().logging_bar.csv_logging.clone()
    }
    pub fn set_logging_directory(&self, directory: PathBuf) {
        let directory = if directory.starts_with("~/") {
            if let Ok(dir) = directory.strip_prefix("~/") {
                user_directory().join(dir)
            } else {
                directory
            }
        } else {
            directory
        };
        self.lock().logging_bar.logging_directory = directory;
    }
    pub fn set_csv_logging(&self, logging: CsvLogging) {
        self.lock().logging_bar.csv_logging = logging;
    }
    pub fn folder_history(&self) -> IndexSet<String> {
        self.lock().connection_history.folders()
    }
    pub fn file_history(&self) -> IndexSet<String> {
        self.lock().connection_history.files()
    }
    pub fn address_history(&self) -> IndexSet<Address> {
        self.lock().connection_history.addresses()
    }
    pub fn serial_history(&self) -> IndexMap<String, SerialConfig> {
        self.lock().connection_history.serial_configs()
    }
    pub fn connection_type_history(&self) -> ConnectionType {
        self.lock().connection_history.last_connection_type.clone()
    }
    pub fn update_folder_history(&self, folder: PathBuf) {
        let folder = String::from(folder.to_str().expect(CONVERT_TO_STR_FAILURE));
        self.lock().connection_history.record_folder(folder);
    }
    pub fn update_file_history(&self, filename: String) {
        self.lock().connection_history.record_file(filename);
    }
    pub fn update_tcp_history(&self, host: String, port: u16) {
        self.lock().connection_history.record_address(host, port);
    }
    pub fn update_serial_history(&self, device: String, baud: u32, flow: FlowControl) {
        self.lock()
            .connection_history
            .record_serial(device, baud, flow);
    }
    pub fn start_vel_log(&self, path: &Path) {
        self.lock().solution_tab.velocity_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {}, error, {}", path.display(), e);
                None
            }
        }
    }
    pub fn end_vel_log(&self) -> Result<()> {
        if let Some(ref mut log) = self.lock().solution_tab.velocity_tab.log_file {
            log.flush()?;
        }
        self.lock().solution_tab.velocity_tab.log_file = None;
        Ok(())
    }
    pub fn start_pos_log(&self, path: &Path) {
        self.lock().solution_tab.position_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {}, error, {}", path.display(), e);
                None
            }
        }
    }
    pub fn end_pos_log(&self) -> Result<()> {
        if let Some(ref mut log) = self.lock().solution_tab.position_tab.log_file {
            log.flush()?;
        }
        self.lock().solution_tab.position_tab.log_file = None;
        Ok(())
    }
    pub fn start_baseline_log(&self, path: &Path) {
        self.lock().baseline_tab.log_file = match CsvSerializer::new(path) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {}, error, {}", path.display(), e);
                None
            }
        }
    }
    pub fn end_baseline_log(&self) -> Result<()> {
        if let Some(ref mut log) = self.lock().baseline_tab.log_file {
            log.flush()?;
        }
        self.lock().baseline_tab.log_file = None;
        Ok(())
    }
    pub fn update_tab_sender(&self) -> Option<Sender<Option<UpdateTabUpdate>>> {
        self.lock().update_tab_sender.clone()
    }
    pub fn set_update_tab_sender(&self, sender: Sender<Option<UpdateTabUpdate>>) {
        self.lock().update_tab_sender = Some(sender);
    }
    pub fn watch_settings_state(&self) -> WatchReceiver<SettingsTabState> {
        self.lock().settings_tab.watch()
    }
    pub fn set_settings_state(&self, state: SettingsTabState) {
        self.lock().settings_tab.send(state);
    }
    pub fn reset_settings_state(&self) {
        self.lock().settings_tab = Watched::new(SettingsTabState::new());
    }
    pub fn set_settings_refresh(&self, refresh: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard
            .settings_tab
            .send(SettingsTabState { refresh, ..data });
    }
    pub fn set_settings_save(&self, save: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { save, ..data });
    }
    pub fn set_settings_reset(&self, reset: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { reset, ..data });
    }
    pub fn set_device_reboot(&self, reboot: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { reboot, ..data });
    }
    pub fn set_settings_auto_survey_request(&self, auto_survey_request: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState {
            auto_survey_request,
            ..data
        });
    }
    pub fn set_settings_confirm_ins_change(&self, confirm_ins_change: bool) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState {
            confirm_ins_change,
            ..data
        });
    }
    pub fn set_export_settings(&self, export: Option<PathBuf>) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { export, ..data });
    }
    pub fn set_import_settings(&self, import: Option<PathBuf>) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { import, ..data });
    }
    pub fn set_write_setting(&self, write: Option<settings_tab::SaveRequest>) {
        let guard = self.lock();
        let data = guard.settings_tab.get();
        guard.settings_tab.send(SettingsTabState { write, ..data });
    }
    pub fn console_version(&self) -> String {
        self.lock().console_version.clone()
    }
    pub fn set_firmware_version(&self, firmware_version: String) {
        self.lock().firmware_version = Some(firmware_version);
    }
    pub fn firmware_version(&self) -> Option<String> {
        self.lock().firmware_version.take()
    }
    pub fn set_dgnss_enabled(&self, dgnss_solution_mode: String) {
        self.lock()
            .heartbeat_data
            .set_dgnss_enabled(dgnss_solution_mode != "No DGNSS");
    }
    pub fn set_advanced_networking_update(&self, update: AdvancedNetworkingState) {
        self.lock().advanced_networking_update = Some(update);
    }
    pub fn advanced_networking_update(&self) -> Option<AdvancedNetworkingState> {
        self.lock().advanced_networking_update.take()
    }
    pub fn auto_survey_requested(&self) -> bool {
        self.lock().auto_survey_data.requested
    }
    pub fn set_auto_survey_result(&self, lat: f64, lon: f64, alt: f64) {
        let mut guard = self.lock();
        guard.auto_survey_data.lat = Some(lat);
        guard.auto_survey_data.lon = Some(lon);
        guard.auto_survey_data.alt = Some(alt);
        guard.auto_survey_data.requested = false;
    }
    pub fn heartbeat_data(&self) -> Heartbeat {
        self.lock().heartbeat_data.clone()
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
        SharedState(Arc::clone(&self.0))
    }
}

#[derive(Debug)]
pub struct SharedStateInner {
    pub(crate) logging_bar: LoggingBarState,
    pub(crate) log_panel: LogPanelState,
    pub(crate) ntrip_tab: NtripState,
    pub(crate) tracking_tab: TrackingTabState,
    pub(crate) connection_history: ConnectionHistory,
    pub(crate) conn: Watched<ConnectionState>,
    pub(crate) connection_dialog_visible: bool,
    pub(crate) debug: bool,
    pub(crate) solution_tab: SolutionTabState,
    pub(crate) baseline_tab: BaselineTabState,
    pub(crate) advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState,
    pub(crate) update_tab_sender: Option<Sender<Option<UpdateTabUpdate>>>,
    pub(crate) settings_tab: Watched<SettingsTabState>,
    pub(crate) console_version: String,
    pub(crate) firmware_version: Option<String>,
    pub(crate) advanced_networking_update: Option<AdvancedNetworkingState>,
    pub(crate) auto_survey_data: AutoSurveyData,
    pub(crate) heartbeat_data: Heartbeat,
}
impl SharedStateInner {
    pub fn new() -> SharedStateInner {
        let connection_history = ConnectionHistory::new();
        let log_directory = connection_history.folders().pop();
        let console_version = String::from(include_str!("version.txt").trim());
        let heartbeat_data = Heartbeat::new();
        heartbeat_data.set_version(console_version.clone());
        SharedStateInner {
            logging_bar: LoggingBarState::new(log_directory),
            log_panel: LogPanelState::new(),
            ntrip_tab: NtripState::default(),
            tracking_tab: TrackingTabState::new(),
            debug: false,
            connection_history,
            conn: Watched::new(ConnectionState::Disconnected),
            connection_dialog_visible: true,
            solution_tab: SolutionTabState::new(),
            baseline_tab: BaselineTabState::new(),
            advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState::new(),
            update_tab_sender: None,
            settings_tab: Watched::new(SettingsTabState::new()),
            console_version,
            firmware_version: None,
            advanced_networking_update: None,
            auto_survey_data: AutoSurveyData::new(),
            heartbeat_data,
        }
    }
}
impl Default for SharedStateInner {
    fn default() -> Self {
        SharedStateInner::new()
    }
}

#[derive(Debug, Default)]
pub struct AdvancedNetworkingState {
    pub ip_address: Option<String>,
    pub port: Option<u16>,
    pub all_messages: Option<bool>,
    pub refresh: bool,
    pub start: bool,
    pub stop: bool,
}

#[derive(Debug)]
pub struct LoggingBarState {
    pub sbp_logging: bool,
    pub sbp_logging_format: SbpLogging,
    /// User-supplied SBP log filename to use instead of the autogenerated one
    pub sbp_logging_filename: Option<PathBuf>,
    /// Full path to the current SBP log
    pub sbp_logging_filepath: Option<PathBuf>,
    pub csv_logging: CsvLogging,
    pub logging_directory: PathBuf,
    pub handle: Option<JoinHandle<()>>,
}

impl LoggingBarState {
    fn new(log_directory: Option<String>) -> LoggingBarState {
        let logging_directory = if let Some(dir) = log_directory {
            PathBuf::from(dir)
        } else {
            LOG_DIRECTORY.path()
        };
        if let Err(err) = fs::create_dir_all(&logging_directory) {
            error!(
                "Unable to create directory, {}, {}.",
                logging_directory.display(),
                err
            );
        }
        LoggingBarState {
            sbp_logging: false,
            sbp_logging_format: SbpLogging::SBP_JSON,
            sbp_logging_filename: None,
            sbp_logging_filepath: None,
            csv_logging: CsvLogging::OFF,
            logging_directory,
            handle: None,
        }
    }
}

#[derive(Debug)]
pub struct LogPanelState {
    pub level: LogLevel,
    pub filename: Option<PathBuf>,
}

impl LogPanelState {
    fn new() -> LogPanelState {
        LogPanelState {
            level: LogLevel::WARNING,
            filename: None,
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
    pub tracked_sv_labels: Vec<String>,
}

impl TrackingSignalsTabState {
    fn new() -> TrackingSignalsTabState {
        TrackingSignalsTabState {
            check_visibility: vec![],
            tracked_sv_labels: vec![],
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
    pub unit: Option<LatLonUnits>,
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
            unit: None,
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
pub struct AutoSurveyData {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub alt: Option<f64>,
    pub requested: bool,
}

impl AutoSurveyData {
    fn new() -> AutoSurveyData {
        AutoSurveyData {
            lat: None,
            lon: None,
            alt: None,
            requested: false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SettingsTabState {
    pub reboot: bool,
    pub refresh: bool,
    pub reset: bool,
    pub save: bool,
    pub confirm_ins_change: bool,
    pub auto_survey_request: bool,
    pub export: Option<PathBuf>,
    pub import: Option<PathBuf>,
    pub write: Option<settings_tab::SaveRequest>,
}

impl SettingsTabState {
    fn new() -> Self {
        Default::default()
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SerialConfig {
    pub baud: u32,
    #[serde(with = "FlowControlRemote")]
    pub flow: FlowControl,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "FlowControl")]
enum FlowControlRemote {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Software")]
    Software,
    #[serde(rename = "Hardware")]
    Hardware,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionHistory {
    addresses: IndexSet<Address>,
    files: IndexSet<String>,
    folders: IndexSet<String>,
    serial_configs: IndexMap<String, SerialConfig>,
    #[serde(with = "ConnectionTypeDef")]
    last_connection_type: ConnectionType,
}

// https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "ConnectionType")]
enum ConnectionTypeDef {
    Tcp,
    File,
    Serial,
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
        if let Ok(file) = fs::File::open(filename) {
            if let Ok(conn_yaml) = serde_yaml::from_reader(file) {
                return conn_yaml;
            }
        }
        let mut folders = IndexSet::new();
        if let Ok(default_path) = LOG_DIRECTORY.path().into_os_string().into_string() {
            folders.insert(default_path);
        }
        let mut addresses = IndexSet::new();
        addresses.insert(Address {
            host: DEFAULT_IP_ADDRESS.to_string(),
            port: DEFAULT_PORT,
        });
        ConnectionHistory {
            addresses,
            files: IndexSet::new(),
            folders,
            serial_configs: IndexMap::new(),
            last_connection_type: ConnectionType::Serial,
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
    /// Returns a clone of the previous serial configs, in order of last use.
    pub fn serial_configs(&self) -> IndexMap<String, SerialConfig> {
        self.serial_configs.clone()
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
        self.last_connection_type = ConnectionType::Tcp;

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
        self.last_connection_type = ConnectionType::File;

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

    /// Attempt to add a new serial configuration if not the most recent entry.
    ///
    /// # Parameters
    /// - `device`: The path to the serial device.
    /// - `baud`: The chosen baud rate
    /// - `flow`: The chosen flow control
    pub fn record_serial(&mut self, device: String, baud: u32, flow: FlowControl) {
        let serial = SerialConfig { baud, flow };
        self.serial_configs.shift_remove(&device);
        self.serial_configs.insert(device, serial);
        let diff = i32::max(0, self.serial_configs.len() as i32 - MAX_CONNECTION_HISTORY);
        self.serial_configs = self.serial_configs.split_off(diff as usize);
        self.last_connection_type = ConnectionType::Serial;

        if let Err(e) = self.save() {
            error!("Unable to save connection history, {}.", e);
        }
    }

    /// Save the history to the expected filepath.
    fn save(&self) -> Result<()> {
        serde_yaml::to_writer(fs::File::create(self.filename())?, self)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    /// App is shut down
    Closed,

    /// Running but disconnected
    Disconnected,

    /// Running and connected
    Connected {
        conn: Connection,
        stop_token: StopToken,
    },

    /// Attempting to connect
    Connecting,
}

impl ConnectionState {
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    pub fn is_disconnected(&self) -> bool {
        matches!(self, Self::Disconnected)
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }

    pub fn is_connecting(&self) -> bool {
        matches!(self, Self::Connecting)
    }

    pub fn name(&self) -> String {
        match self {
            ConnectionState::Closed => "closed".into(),
            ConnectionState::Disconnected => "disconnected".into(),
            ConnectionState::Connecting => "connecting".into(),
            ConnectionState::Connected { conn, .. } => conn.name(),
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            ConnectionState::Closed => false,
            ConnectionState::Disconnected => false,
            ConnectionState::Connecting => false,
            ConnectionState::Connected { conn, .. } => conn.is_file(),
        }
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Closed => write!(f, "{}", cc::ConnectionState::CLOSED),
            ConnectionState::Disconnected => write!(f, "{}", cc::ConnectionState::DISCONNECTED),
            ConnectionState::Connecting => write!(f, "{}", cc::ConnectionState::CONNECTING),
            ConnectionState::Connected { .. } => {
                write!(f, "{}", cc::ConnectionState::CONNECTED)
            }
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
        let host1 = String::from(DEFAULT_IP_ADDRESS);
        let host2 = String::from("host2");
        let port = DEFAULT_PORT;

        conn_history.record_address(host1.clone(), port);
        let addresses = conn_history.addresses();
        let addresses_len = addresses.len();
        let first_addy = addresses.first().unwrap();
        assert_eq!(host1, first_addy.host);
        assert_eq!(port, first_addy.port);
        assert_eq!(ConnectionType::Tcp, conn_history.last_connection_type);

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
        assert_eq!(ConnectionType::File, conn_history.last_connection_type);

        for ele in 0..MAX_CONNECTION_HISTORY {
            conn_history.record_file(ele.to_string());
        }
        assert_eq!(conn_history.files().len(), MAX_CONNECTION_HISTORY as usize);
        restore_backup_file(bfilename);
    }

    #[test]
    #[serial]
    fn connection_history_serial_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());

        let mut conn_history = ConnectionHistory::new();
        assert_eq!(ConnectionType::Serial, conn_history.last_connection_type);

        let configs = conn_history.serial_configs();
        assert_eq!(configs.keys().len(), 0);

        conn_history.record_serial("/dev/ttyUSB0".to_string(), 115200, FlowControl::Software);
        conn_history.record_serial("/dev/ttyUSB0".to_string(), 115200, FlowControl::None);

        assert_eq!(ConnectionType::Serial, conn_history.last_connection_type);

        let configs = conn_history.serial_configs();

        // Should only store one serial record despite the settings changing
        assert_eq!(configs.keys().len(), 1);

        // Settings should be as recorded for the last change
        let config = configs.get("/dev/ttyUSB0").unwrap();
        assert_eq!(config.baud, 115200);
        assert_eq!(config.flow, FlowControl::None);

        conn_history.record_serial("/dev/ttyUSB1".to_string(), 115200, FlowControl::None);

        // The most recent entry should be stored last
        let configs = conn_history.serial_configs();
        assert_eq!(configs.keys().len(), 2);
        assert_eq!(configs.keys().nth(1).unwrap(), "/dev/ttyUSB1");

        conn_history.record_serial("/dev/ttyUSB0".to_string(), 115200, FlowControl::None);

        // But the most recent entry should be updated if a previous device is used again
        let configs = conn_history.serial_configs();
        assert_eq!(configs.keys().len(), 2);
        assert_eq!(configs.keys().nth(1).unwrap(), "/dev/ttyUSB0");

        let mut conn_history = ConnectionHistory::new();

        for ele in 0..MAX_CONNECTION_HISTORY {
            conn_history.record_serial(
                format!("/dev/ttyUSB{:?}", ele),
                9600,
                FlowControl::Hardware,
            );
        }
        let configs = conn_history.serial_configs();
        configs.get("/dev/ttyUSB0").unwrap();

        // Adding a new device should push out the oldest
        conn_history.record_serial("/dev/mynewserial".to_string(), 115200, FlowControl::None);
        let configs = conn_history.serial_configs();
        let config = configs.get("/dev/ttyUSB0");
        assert_eq!(config, None);

        restore_backup_file(bfilename);
    }
}
