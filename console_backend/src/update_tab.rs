use anyhow::anyhow;
use capnp::message::Builder;
use log::{debug, error};
use std::{
    fs::read,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{errors::{CONVERT_TO_STR_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE, THREAD_JOIN_FAILURE}, fileio::Fileio};
use crate::types::{ArcBool, CapnProtoSender, SharedState};
use crate::update_downloader::UpdateDownloader;
use crate::utils::{compare_semvers, serialize_capnproto_builder};

/// UpdateTab struct.
///
/// # Fields
/// - `client_sender`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
pub struct UpdateTab<'a, S: CapnProtoSender> {
    client_sender: S,
    is_running: ArcBool,
    shared_state: SharedState,
    update_shared: UpdateShared,
    fileio: Fileio<'a>,
    handler: Option<thread::JoinHandle<()>>,
}

<<<<<<< HEAD
impl<S: CapnProtoSender> UpdateTab<S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
    ) -> UpdateTab<S> {
=======
impl<'a, S: CapnProtoSender> UpdateTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S, fileio: Fileio<'a>) -> UpdateTab<S> {
>>>>>>> Update Tab Frontend.
        let is_running = ArcBool::new_with(true);
        let update_shared = UpdateShared::new();
        let mut update_tab = UpdateTab {
            client_sender,
            is_running,
            shared_state,
            update_shared,
            handler: None,
            fileio,
        };
        update_tab.update_tab_thread();
        update_tab
        
    }

    fn update_tab_thread(&mut self) {
        let client_sender = self.client_sender.clone();
        let mut shared_state = self.shared_state.clone();
        let update_shared = self.update_shared.clone();
        let is_running = self.is_running.clone();
        let mut update_downloader_thread: Option<JoinHandle<()>> = None;
        let mut update_upgrader_thread: Option<JoinHandle<()>> = None;
        self.handler = Some(thread::spawn(move || loop {
            if !is_running.get() {
                break;
            }
            // Check for button changes.
            let directory = shared_state.firmware_directory();
            if let Some(buttons) = shared_state.update_buttons() {
                if buttons.download_latest_firmware && !update_shared.downloading() {
                    if let Some(update_downloader_thread) = update_downloader_thread.take() {
                        update_downloader_thread.join().expect(THREAD_JOIN_FAILURE);
                    }
                    update_downloader_thread = Some(UpdateTab::<S>::download_firmware(
                        update_shared.clone(),
                        directory.clone(),
                    ));
                }
                if buttons.update_firmware && !update_shared.upgrading() {
                    if let Some(update_upgrader_thread) = update_upgrader_thread.take() {
                        update_upgrader_thread.join().expect(THREAD_JOIN_FAILURE);
                    }
                    update_upgrader_thread = Some(UpdateTab::<S>::upgrade_firmware(
                        update_shared.clone(),
                        directory.clone(),
                    ));
                }
            }
            UpdateTab::update_frontend(update_shared.clone(), client_sender.clone(), directory);
            thread::sleep(std::time::Duration::from_millis(250));
            log::logger().flush();
            // Check for messages from MSG_LOG?

            // Send update to frontend?
        }));
    }

    /// Package data into a message buffer and send to frontend.
    fn update_frontend(mut update_shared: UpdateShared, mut client_sender: S, directory: PathBuf) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let packet = update_shared.packet();
        let mut status = msg.init_update_tab_status();
        status.set_hardware_revision("piksi_multi");
        status.set_fw_version_current("");
        status.set_fw_version_latest(&packet.latest_firmware_version);
        status.set_fw_local_filename(&packet.firmware_filename);
        status.set_directory(&directory.to_string_lossy().to_string());
        status.set_downloading(packet.downloading);
        status.set_upgrading(packet.upgrading);
        status.set_fw_text(&packet.fw_log);

        client_sender.send_data(serialize_capnproto_builder(builder));
    }

    fn download_firmware(update_shared: UpdateShared, directory: PathBuf) -> JoinHandle<()> {
        update_shared.set_downloading(true);
        update_shared.fw_log_clear();
        let mut update_downloader = update_shared.update_downloader();
        std::thread::spawn(move || {
            let filepath = match update_downloader.download_multi_firmware(directory.clone(), Some(update_shared.clone())) {
                Ok(filepath_) => {
                    Some(filepath_)
                },
                Err(e) => {
                    error!("{}", e);
                    None
                }
            };
            update_shared.set_firmware_filepath(filepath);
            update_shared.set_downloading(false);
            log::logger().flush();
        })
    }

    fn upgrade_firmware(update_shared: UpdateShared, directory: PathBuf) -> JoinHandle<()> {
        update_shared.set_upgrading(true);
        update_shared.fw_log_clear();
        let mut update_downloader = update_shared.update_downloader();
        std::thread::spawn(move || {
            // TODO Get current version from settings.
            // TODO Get serial number from settings.
            // TODO Get firmware version from settings.
            let current_version = String::from("v2.4.16");
            let to_upgrade = match update_downloader.latest_firmware_version() {
                Ok(latest_version) => {
                    if !current_version.is_empty() {
                        if let Ok(comp) = compare_semvers(current_version.clone(), latest_version.clone()) {
                            update_shared.fw_log_append(format!("Latest firmware version, {}, is newer than current version, {}.", latest_version, current_version));
                            comp
                        } else {
                            update_shared.fw_log_append(format!("Latest firmware version, {}, deemed older than current version, {}.", latest_version, current_version));
                            false
                        }
                    } else {
                        update_shared.fw_log_append(String::from("Current Version needed to compare Firmware before upgrade."));
                        false
                    }
                }
                Err(err) => {
                    update_shared.fw_log_append(String::from("Latest Version needed to compare Firmware before upgrade."));
                    update_shared.fw_log_append(err.to_string());
                    false
                }
            };
            if to_upgrade {

            }
            update_shared.set_upgrading(false);
            log::logger().flush();
        })
    }

    fn firmware_upgrade(update_shared: UpdateShared) -> anyhow::Result<()> {
        
        if let Some(filepath) = update_shared.firmware_filepath() {
            
            update_shared.fw_log_append(format!("Reading firmware file from path, {}.", filepath.display()));
            if !filepath.exists() || !filepath.is_file()  {
                return Err(anyhow!("Firmware filepath is not a file or does not exist."));
            }
            if let Ok(firmware_blob) = read(filepath.clone()) {
                update_shared.fw_log_append(String::from("Transferring image to device."));
            } else {
                return Err(anyhow!("Failed to read firmware file, {:?}.", filepath));
            }
        }
        Ok(())
    }
    /*
    def manage_multi_firmware_update(self):
        self.blob_size = float(len(self.stm_fw.blob))
        self.pcent_complete = 0
        # Set up progress dialog and transfer file to Piksi using SBP FileIO
        self._clear_stream()
        self._write("Transferring image to device...\n\n00.0 of {:2.1f} MB trasnferred".format(
            self.blob_size * 1e-6))
        try:
            FileIO(self.link).write(
                b"upgrade.image_set.bin",
                self.stm_fw.blob,
                progress_cb=self.file_transfer_progress_cb)
        except Exception as e:
            self._write("Failed to transfer image file to Piksi: %s\n" % e)
            self._write("Upgrade Aborted.")
            import traceback
            print(traceback.format_exc())
            return -1

        self.stream.scrollback_write(
            "Image transfer complete: {:2.1f} MB transferred.\n".format(self.blob_size * 1e-6))
        # Setup up pulsed progress dialog and commit to flash
        self._write("Committing file to Flash...\n")
        self.link.add_callback(self.log_cb, SBP_MSG_LOG)
        code = shell_command(
            self.link,
            b"upgrade_tool upgrade.image_set.bin",
            200)
        self.link.remove_callback(self.log_cb, SBP_MSG_LOG)

        if code != 0:
            self._write('Failed to perform upgrade (code = %d)' % code)
            if code == -255:
                self._write('Shell command timed out.  Please try again.')
            return
        self._write("Upgrade Complete.")
        self._write('Resetting Piksi...')
        self.link(MsgReset(flags=0))
    */

}

impl<'a, S: CapnProtoSender> Drop for UpdateTab<'a, S> {
    fn drop(&mut self) {
        self.is_running.set(false);
        if let Some(handle) = self.handler.take() {
            handle.join().expect(THREAD_JOIN_FAILURE);
        }
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

pub struct UpdateSharedInner{
    downloading: bool,
    upgrading: bool,
    firmware_filepath: Option<PathBuf>,
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
            firmware_filepath: None,
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
pub struct UpdateShared (Arc<Mutex<UpdateSharedInner>>);

pub struct UpdatePacket {
    latest_firmware_version: String,
    downloading: bool,
    upgrading: bool,
    firmware_filename: String,
    fw_log: String,
}

impl UpdateShared  {
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
    pub fn firmware_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_filepath.clone()
    }
    pub fn set_firmware_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_filepath = filepath;
    }
    pub fn update_downloader(&self) -> UpdateDownloader {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).update_downloader.clone()
    }
    pub fn fw_log_append(&self, log: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        debug!("{}", log.clone());
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
        let firmware_filename =
            if let Some(firmware_filepath_) = (*shared_data).firmware_filepath.clone() {
                firmware_filepath_
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
            firmware_filename,
            fw_log,
        }
    }
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
