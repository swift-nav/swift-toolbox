use crate::common_constants::{ApplicationState, SbpLogging};
use crate::constants::{
    APPLICATION_NAME, APPLICATION_ORGANIZATION, APPLICATION_QUALIFIER, CONNECTION_HISTORY_FILENAME,
    DEFAULT_LOG_DIRECTORY, MAX_CONNECTION_HISTORY, MPS,
};
use crate::errors::{CONVERT_TO_STR_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE};
use crate::log_panel::LogLevel;
use crate::output::{CsvLogging, CsvSerializer};
use crate::settings_tab;
use crate::solution_tab::LatLonUnits;
use crate::types::CapnProtoSender;
use crate::update_tab::UpdateTabUpdate;
use crate::utils::send_app_state;
use crate::watch::{WatchReceiver, Watched};
use anyhow::{Context, Result as AHResult};
use chrono::{DateTime, Utc};
use crossbeam::channel::Sender;
use directories::{ProjectDirs, UserDirs};
use indexmap::set::IndexSet;
use lazy_static::lazy_static;
use log::error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
    fs,
    hash::Hash,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
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
    pub fn app_state(&self) -> ApplicationState {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).app_state.get()
    }
    pub fn watch_app_state(&self) -> WatchReceiver<ApplicationState> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).app_state.watch()
    }
    pub fn set_app_state<S>(&self, set_to: ApplicationState, client_send: &mut S)
    where
        S: CapnProtoSender,
    {
        eprintln!("SET APP STATE {}", set_to);
        {
            let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
            (*shared_data).app_state.send(set_to);
        }
        send_app_state(set_to, client_send);
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
    pub fn watch_settings_state(&self) -> WatchReceiver<SettingsTabState> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.watch()
    }
    pub fn set_settings_state(&self, state: SettingsTabState) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab.send(state);
    }
    pub fn stop_settings_thd(&self) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.settings_tab = Watched::new(SettingsTabState::new());
    }
    pub fn set_settings_refresh(&self, refresh: bool) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { refresh, ..data });
    }
    pub fn set_settings_save(&self, save: bool) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { save, ..data });
    }
    pub fn set_settings_reset(&self, reset: bool) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { reset, ..data });
    }
    pub fn set_settings_auto_survey_request(&self, auto_survey_request: bool) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data.settings_tab.send(SettingsTabState {
            auto_survey_request,
            ..data
        });
    }
    pub fn set_settings_confirm_ins_change(&self, confirm_ins_change: bool) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data.settings_tab.send(SettingsTabState {
            confirm_ins_change,
            ..data
        });
    }
    pub fn set_export_settings(&self, export: Option<PathBuf>) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { export, ..data });
    }
    pub fn set_import_settings(&self, import: Option<PathBuf>) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { import, ..data });
    }
    pub fn set_write_setting(&self, write: Option<settings_tab::SaveRequest>) {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let data = shared_data.settings_tab.get();
        shared_data
            .settings_tab
            .send(SettingsTabState { write, ..data });
    }
    pub fn console_version(&self) -> String {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).console_version.clone()
    }
    pub fn set_firmware_version(&self, firmware_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_version = Some(firmware_version);
    }
    pub fn firmware_version(&self) -> Option<String> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_version.take()
    }
    pub fn dgnss_enabled(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.dgnss_enabled
    }
    pub fn set_dgnss_enabled(&self, dgnss_solution_mode: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.dgnss_enabled = dgnss_solution_mode != "No DGNSS";
    }
    pub fn set_reset_device(&self, reset_device: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.reset_device = reset_device;
    }
    pub fn set_advanced_networking_update(&self, update: AdvancedNetworkingState) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.advanced_networking_update = Some(update);
    }
    pub fn advanced_networking_update(&self) -> Option<AdvancedNetworkingState> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.advanced_networking_update.take()
    }
    pub fn auto_survey_requested(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).auto_survey_data.requested
    }
    pub fn set_auto_survey_result(&self, lat: f64, lon: f64, alt: f64) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).auto_survey_data.lat = Some(lat);
        (*shared_data).auto_survey_data.lon = Some(lon);
        (*shared_data).auto_survey_data.alt = Some(alt);
        (*shared_data).auto_survey_data.requested = false;
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
    pub(crate) connection_history: ConnectionHistory,
    pub(crate) app_state: Watched<ApplicationState>,
    pub(crate) debug: bool,
    pub(crate) server_running: bool,
    pub(crate) solution_tab: SolutionTabState,
    pub(crate) baseline_tab: BaselineTabState,
    pub(crate) advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState,
    pub(crate) update_tab_sender: Option<Sender<Option<UpdateTabUpdate>>>,
    pub(crate) settings_tab: Watched<SettingsTabState>,
    pub(crate) console_version: String,
    pub(crate) firmware_version: Option<String>,
    pub(crate) dgnss_enabled: bool,
    pub(crate) reset_device: bool,
    pub(crate) advanced_networking_update: Option<AdvancedNetworkingState>,
    pub(crate) auto_survey_data: AutoSurveyData,
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
            debug: false,
            connection_history,
            app_state: Watched::new(ApplicationState::DISCONNECTED),
            server_running: true,
            solution_tab: SolutionTabState::new(),
            baseline_tab: BaselineTabState::new(),
            advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTabState::new(),
            update_tab_sender: None,
            settings_tab: Watched::new(SettingsTabState::new()),
            console_version: String::from(include_str!("version.txt").trim()),
            firmware_version: None,
            dgnss_enabled: false,
            reset_device: false,
            advanced_networking_update: None,
            auto_survey_data: AutoSurveyData::new(),
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
