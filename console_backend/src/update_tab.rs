use anyhow::anyhow;
use capnp::message::Builder;
use crossbeam::thread;
use log::{debug, error};
use std::thread::sleep;
use std::{
    fs::read,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::errors::{
    CONVERT_TO_STR_FAILURE, CROSSBEAM_SCOPE_UNWRAP_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE,
    THREAD_JOIN_FAILURE,
};
use crate::types::{ArcBool, CapnProtoSender, SharedState};
use crate::update_downloader::UpdateDownloader;
use crate::utils::{compare_semvers, serialize_capnproto_builder};

/// UpdateTab struct.
///
/// # Fields
/// - `client_sender`: Client Sender channel for communication from backend to frontend.
/// - `is_running`: ArcBool to indicate if the update tab is running.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `update_shared`: The shared state for update tab.
pub struct UpdateTab<S: CapnProtoSender> {
    client_sender: S,
    is_running: ArcBool,
    shared_state: SharedState,
    update_shared: UpdateShared,
}

impl<S: CapnProtoSender> UpdateTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> UpdateTab<S> {
        let is_running = ArcBool::new_with(true);
        let update_shared = UpdateShared::new();
        let mut update_tab = UpdateTab {
            client_sender,
            is_running,
            shared_state,
            update_shared,
        };
        update_tab.update_tab_thread();
        update_tab
    }

    fn update_tab_thread(&mut self) {
        let mut shared_state = self.shared_state.clone();
        let update_shared = self.update_shared.clone();
        let is_running = self.is_running.clone();

        thread::scope(|s| {
            s.spawn(|inner_scope| {
                let mut update_downloader_thread: Option<thread::ScopedJoinHandle<()>> = None;
                let mut update_upgrader_thread: Option<thread::ScopedJoinHandle<()>> = None;
                loop {
                    if !is_running.get() {
                        break;
                    }
                    // Check for path updates.
                    if let Some(fw_dir) = shared_state.firmware_directory() {
                        update_shared.set_firmware_directory(fw_dir.clone());
                        update_shared.set_firmware_local_filepath(None);
                    }
                    if let Some(fw_local_filepath) = shared_state.firmware_local_filepath() {
                        if let Some(parent_path) = fw_local_filepath.parent() {
                            update_shared.set_firmware_directory(parent_path.to_path_buf());
                        }
                        update_shared.set_firmware_local_filepath(Some(fw_local_filepath.clone()));
                    }
                    if let Some(fw_local_filename) = shared_state.firmware_local_filename() {
                        let fw_local_filepath =
                            update_shared.firmware_directory().join(fw_local_filename);
                        update_shared.set_firmware_local_filepath(Some(fw_local_filepath));
                    }
                    if let Some(fileio_local_filepath) = shared_state.fileio_local_filepath() {
                        update_shared.set_fileio_local_filepath(Some(fileio_local_filepath));
                    }
                    if let Some(fileio_destination_filepath) =
                        shared_state.fileio_destination_filepath()
                    {
                        update_shared
                            .set_fileio_destination_filepath(Some(fileio_destination_filepath));
                    }
                    // Check for button changes.
                    if let Some(buttons) = shared_state.update_buttons() {
                        if buttons.download_latest_firmware && !update_shared.downloading() {
                            if let Some(update_downloader_thread) = update_downloader_thread.take()
                            {
                                update_downloader_thread.join().expect(THREAD_JOIN_FAILURE);
                            }
                            update_downloader_thread = Some(inner_scope.spawn(|_| {
                                self.download_firmware();
                            }));
                        }
                        if buttons.update_firmware && !update_shared.upgrading() {
                            if let Some(update_upgrader_thread) = update_upgrader_thread.take() {
                                update_upgrader_thread.join().expect(THREAD_JOIN_FAILURE);
                            }
                            update_upgrader_thread = Some(inner_scope.spawn(|_| {
                                self.upgrade_firmware();
                            }));
                        }
                    }
                    self.update_frontend();
                    sleep(std::time::Duration::from_millis(250));
                    log::logger().flush();
                    // Check for messages from MSG_LOG?

                    // Send update to frontend?
                }
            })
            .join()
            .expect(THREAD_JOIN_FAILURE);
        })
        .expect(CROSSBEAM_SCOPE_UNWRAP_FAILURE);
    }

    /// Package data into a message buffer and send to frontend.
    fn update_frontend(&self) {
        let mut client_sender = self.client_sender.clone();
        let mut builder = Builder::new_default();
        let mut update_shared = self.update_shared.clone();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let packet = update_shared.packet();
        let mut status = msg.init_update_tab_status();

        status.set_hardware_revision("piksi_multi");
        status.set_fw_version_current("");
        status.set_fw_version_latest(&packet.latest_firmware_version);
        status.set_fw_local_filename(&packet.firmware_filename);
        status
            .set_fileio_local_filepath(&packet.fileio_local_filepath.to_string_lossy().to_string());
        status.set_fileio_destination_filepath(
            &packet
                .fileio_destination_filepath
                .to_string_lossy()
                .to_string(),
        );
        status.set_directory(&packet.firmware_directory.to_string_lossy().to_string());
        status.set_downloading(packet.downloading);
        status.set_upgrading(packet.upgrading);
        status.set_fw_text(&packet.fw_log);

        client_sender.send_data(serialize_capnproto_builder(builder));
    }

    fn download_firmware(&self) {
        let directory = self.update_shared.firmware_directory();
        self.update_shared.set_downloading(true);
        self.update_shared.fw_log_clear();
        let update_shared = self.update_shared.clone();
        let mut update_downloader = self.update_shared.update_downloader();
        let filepath = match update_downloader
            .download_multi_firmware(directory, Some(update_shared.clone()))
        {
            Ok(filepath_) => Some(filepath_),
            Err(e) => {
                error!("{}", e);
                None
            }
        };
        update_shared.set_firmware_local_filepath(filepath);
        update_shared.set_downloading(false);
        log::logger().flush();
    }

    fn upgrade_firmware(&self) {
        self.update_shared.set_upgrading(true);
        self.update_shared.fw_log_clear();
        let mut update_downloader = self.update_shared.update_downloader();
        // let update_shared = self.update_shared.clone();
        // TODO Get current version from settings.
        // TODO Get serial number from settings.
        // TODO Get firmware version from settings.
        let current_version = String::from("v2.4.15");
        let to_upgrade = match update_downloader.latest_firmware_version() {
            Ok(latest_version) => {
                if !current_version.is_empty() {
                    match compare_semvers(current_version.clone(), latest_version.clone()) {
                        Ok(comp) => {
                            if comp {
                                self.update_shared.fw_log_append(format!("Latest firmware version, {}, is newer than current version, {}.", latest_version, current_version));
                            } else {
                                self.update_shared.fw_log_append(format!("Latest firmware version, {}, is not newer than current version, {}.", latest_version, current_version));
                            }
                            comp
                        }
                        Err(err) => {
                            self.update_shared.fw_log_append(String::from(
                                "Unable to compare latest versus current version.",
                            ));
                            self.update_shared.fw_log_append(err.to_string());
                            false
                        }
                    }
                } else {
                    self.update_shared.fw_log_append(String::from(
                        "Current Version needed to compare Firmware before upgrade.",
                    ));
                    false
                }
            }
            Err(err) => {
                self.update_shared.fw_log_append(String::from(
                    "Latest Version needed to compare Firmware before upgrade.",
                ));
                self.update_shared.fw_log_append(err.to_string());
                false
            }
        };
        if to_upgrade {
            if let Err(err) = self.firmware_upgrade() {
                self.update_shared.fw_log_append(err.to_string());
            }
        }
        self.update_shared.set_upgrading(false);
        log::logger().flush();
    }

    fn firmware_upgrade(&self) -> anyhow::Result<()> {
        if let Some(filepath) = self.update_shared.firmware_local_filepath() {
            self.update_shared.fw_log_append(format!(
                "Reading firmware file from path, {}.",
                filepath.display()
            ));
            if !filepath.exists() || !filepath.is_file() {
                return Err(anyhow!(
                    "Firmware filepath is not a file or does not exist."
                ));
            }
            if let Ok(_firmware_blob) = read(filepath.clone()) {
                self.update_shared
                    .fw_log_append(String::from("Transferring image to device."));
            } else {
                return Err(anyhow!("Failed to read firmware file, {:?}.", filepath));
            }
        }
        Ok(())
    }
}

impl<S: CapnProtoSender> Drop for UpdateTab<S> {
    fn drop(&mut self) {
        self.is_running.set(false);
    }
}
pub struct FirmwareUpgradePaneLogger {
    current_log: Vec<String>,
}
impl FirmwareUpgradePaneLogger {
    pub fn new() -> FirmwareUpgradePaneLogger {
        FirmwareUpgradePaneLogger {
            current_log: vec![],
        }
    }
    pub fn log_append(&mut self, log: String) {
        self.current_log.push(log);
    }
    pub fn log_replace_last(&mut self, log: String) {
        self.current_log.pop();
        self.current_log.push(log);
    }
    pub fn clear(&mut self) {
        self.current_log.clear();
    }
    pub fn joined_string(&self) -> String {
        self.current_log.join("\n")
    }
}
impl Default for FirmwareUpgradePaneLogger {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UpdateSharedInner {
    downloading: bool,
    upgrading: bool,
    firmware_directory: PathBuf,
    firmware_local_filepath: Option<PathBuf>,
    fileio_destination_filepath: Option<PathBuf>,
    fileio_local_filepath: Option<PathBuf>,
    update_downloader: UpdateDownloader,
    fw_logger: FirmwareUpgradePaneLogger,
}
impl UpdateSharedInner {
    pub fn new() -> UpdateSharedInner {
        let mut update_downloader = UpdateDownloader::new();
        if let Err(err) = update_downloader.get_index_data() {
            error!("{}", err);
        }
        UpdateSharedInner {
            upgrading: false,
            downloading: false,
            firmware_directory: PathBuf::from(""),
            firmware_local_filepath: None,
            fileio_destination_filepath: None,
            fileio_local_filepath: None,
            update_downloader,
            fw_logger: FirmwareUpgradePaneLogger::new(),
        }
    }
}
impl Default for UpdateSharedInner {
    fn default() -> Self {
        UpdateSharedInner::new()
    }
}
pub struct UpdateShared(Arc<Mutex<UpdateSharedInner>>);

impl UpdateShared {
    pub fn new() -> UpdateShared {
        UpdateShared(Arc::new(Mutex::new(UpdateSharedInner::default())))
    }

    pub fn downloading(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).downloading
    }
    pub fn set_downloading(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).downloading = set_to;
    }
    pub fn upgrading(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrading
    }
    pub fn set_upgrading(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrading = set_to;
    }
    pub fn fileio_destination_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fileio_destination_filepath.clone()
    }
    pub fn set_fileio_destination_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fileio_destination_filepath = filepath;
    }
    pub fn fileio_local_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fileio_local_filepath.clone()
    }
    pub fn set_fileio_local_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fileio_local_filepath = filepath;
    }
    pub fn firmware_local_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_local_filepath.clone()
    }
    pub fn set_firmware_local_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_local_filepath = filepath;
    }
    pub fn firmware_directory(&self) -> PathBuf {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_directory.clone()
    }
    pub fn set_firmware_directory(&self, directory: PathBuf) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_directory = directory;
    }
    pub fn update_downloader(&self) -> UpdateDownloader {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).update_downloader.clone()
    }
    pub fn fw_log_append(&self, log: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        debug!("{}", log);
        (*shared_data).fw_logger.log_append(log);
    }
    pub fn fw_log_replace_last(&self, log: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fw_logger.log_replace_last(log);
    }
    pub fn fw_log_clear(&self) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fw_logger.clear();
    }
    pub fn fw_log(&self) -> String {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).fw_logger.joined_string()
    }
    pub fn packet(&mut self) -> UpdatePacket {
        let fw_log = self.fw_log();
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let latest_firmware_version =
            if let Ok(version) = (*shared_data).update_downloader.latest_firmware_version() {
                version
            } else {
                String::from("")
            };
        let downloading = (*shared_data).downloading;
        let upgrading = (*shared_data).upgrading;
        let firmware_directory = (*shared_data).firmware_directory.clone();
        let fileio_destination_filepath =
            if let Some(filepath) = (*shared_data).fileio_destination_filepath.clone() {
                filepath
            } else {
                PathBuf::from("")
            };
        let fileio_local_filepath =
            if let Some(filepath) = (*shared_data).fileio_local_filepath.clone() {
                filepath
            } else {
                PathBuf::from("")
            };
        let firmware_filename = if let Some(firmware_local_filepath_) =
            (*shared_data).firmware_local_filepath.clone()
        {
            firmware_local_filepath_
                .file_name()
                .expect(CONVERT_TO_STR_FAILURE)
                .to_string_lossy()
                .to_string()
        } else {
            String::new()
        };
        UpdatePacket {
            latest_firmware_version,
            downloading,
            upgrading,
            firmware_directory,
            fileio_destination_filepath,
            fileio_local_filepath,
            firmware_filename,
            fw_log,
        }
    }
}

pub struct UpdatePacket {
    latest_firmware_version: String,
    downloading: bool,
    upgrading: bool,
    firmware_filename: String,
    firmware_directory: PathBuf,
    fileio_destination_filepath: PathBuf,
    fileio_local_filepath: PathBuf,
    fw_log: String,
}

impl Deref for UpdateShared {
    type Target = Mutex<UpdateSharedInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UpdateShared {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for UpdateShared {
    fn clone(&self) -> Self {
        UpdateShared {
            0: Arc::clone(&self.0),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::types::TestSender;
    // #[test]
    // fn handle_age_corrections_test() {
    //     let shared_state = SharedState::new();
    //     let client_send = TestSender { inner: Vec::new() };
    //     let wtr = MsgSender::new(sink());
    //     let mut baseline_tab = UpdateTab::new(shared_state, client_send, wtr);
    //     assert!(baseline_tab.age_corrections.is_none());
    //     let msg = MsgAgeCorrections {
    //         sender_id: Some(1337),
    //         age: 0xFFFF,
    //         tow: 0,
    //     };
    //     baseline_tab.handle_age_corrections(msg);
    //     assert!(baseline_tab.age_corrections.is_none());
    //     let good_age = 0x4DC6;
    //     let msg = MsgAgeCorrections {
    //         sender_id: Some(1337),
    //         age: good_age,
    //         tow: 0,
    //     };
    //     baseline_tab.handle_age_corrections(msg);
    //     assert!(baseline_tab.age_corrections.is_some());
    //     if let Some(age) = baseline_tab.age_corrections {
    //         assert!(f64::abs(age - 1991_f64) <= f64::EPSILON);
    //     }
    // }
}
