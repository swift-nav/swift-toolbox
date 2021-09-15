use anyhow::anyhow;
use capnp::message::Builder;
use crossbeam::{
    channel::{self, Receiver, Sender},
    thread,
};
use glob::glob;
use log::{debug, error};
use regex::Regex;
use sbp::link::Link;
use sbp::messages::{
    logging::MsgLog,
    piksi::{MsgCommandReq, MsgCommandResp, MsgReset},
};
use sbp::SbpString;
use std::{
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::errors::{
    CONVERT_TO_STR_FAILURE, CROSSBEAM_SCOPE_UNWRAP_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE,
    THREAD_JOIN_FAILURE,
};
use crate::fileio::{new_sequence, Fileio};
use crate::shared_state::{SharedState, LOG_DIRECTORY};
use crate::types::{CapnProtoSender, MsgSender};
use crate::update_downloader::UpdateDownloader;
use crate::utils::{compare_semvers, serialize_capnproto_builder};

const UPGRADE_FIRMWARE_REMOTE_DESTINATION: &str = "upgrade.image_set.bin";
const UPGRADE_FIRMWARE_TOOL: &str = "upgrade_tool";
const UPGRADE_FIRMWARE_TIMEOUT_SEC: u64 = 600;
const UPDATE_THREAD_SLEEP_MS: u64 = 1000;
const UPGRADE_WHITELIST: &[&str] = &[
    "ok",
    "writing.*",
    "erasing.*",
    r"\s*[0-9]* % complete",
    "Error.*",
    "error.*",
    ".*Image.*",
    ".*upgrade.*",
    "Warning:*",
    ".*install.*",
    "upgrade completed successfully",
];

/// UpdateTab struct.
///
/// # Fields
/// - `sender`: A sender to send messages to the update thread.
/// - `receiver`: A receiver to receive messages from the update thread.
/// - `update_tab_context`: The shared state for update tab.
pub struct UpdateTab {
    sender: Sender<Option<UpdateTabUpdate>>,
    receiver: Receiver<Option<UpdateTabUpdate>>,
    update_tab_context: UpdateTabContext,
}
impl UpdateTab {
    pub fn new(shared_state: SharedState) -> UpdateTab {
        let (sender, receiver) = channel::unbounded();
        shared_state.set_update_tab_sender(sender.clone());
        let update_tab_context = UpdateTabContext::new();
        update_tab_context.set_debug(shared_state.debug());
        UpdateTab {
            sender,
            receiver,
            update_tab_context,
        }
    }
    pub fn clone_update_tab_context(&self) -> UpdateTabContext {
        self.update_tab_context.clone()
    }
    pub fn clone_channel(
        &self,
    ) -> (
        Sender<Option<UpdateTabUpdate>>,
        Receiver<Option<UpdateTabUpdate>>,
    ) {
        (self.sender.clone(), self.receiver.clone())
    }
    pub fn handle_log_msg(&self, msg: MsgLog) {
        if self.update_tab_context.upgrading() {
            for regex in UPGRADE_WHITELIST.iter() {
                let text = msg.text.to_string();
                if let Ok(reg) = Regex::new(regex) {
                    if reg.captures(&text).is_some() {
                        let text: String = text
                            .chars()
                            .map(|x| match x {
                                '\r' => '\n',
                                _ => x,
                            })
                            .collect();
                        let text = text.split('\n').collect::<Vec<&str>>();
                        let final_text = if text.len() > 1 {
                            // upgrade tool deliminates lines in stoud with \r, we want penultimate line that is complete to show
                            text[text.len() - 2]
                        } else {
                            // If there is only one line, we show that
                            text[text.len() - 1]
                        };
                        self.update_tab_context
                            .fw_log_replace_last(final_text.to_string());
                    }
                }
            }
        }
    }

    pub fn handle_command_resp(&mut self, msg: MsgCommandResp) {
        if self.update_tab_context.upgrading() {
            if let Some(sequence) = self.update_tab_context.upgrade_sequence() {
                if sequence == msg.sequence {
                    self.update_tab_context.set_upgrade_sequence(None);
                    self.update_tab_context.set_upgrade_ret(Some(msg.code));
                    self.update_tab_context.set_upgrading(false);
                }
            }
        }
    }
}

pub fn check_for_firmware_local_filepath(directory: PathBuf) -> Option<PathBuf> {
    let pattern = format!("{}/*.bin", directory.to_string_lossy());
    if let Ok(mut paths) = glob(&pattern) {
        if let Some(Ok(path)) = paths.next() {
            Some(path)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn update_tab_thread<S: CapnProtoSender>(
    sender: Sender<Option<UpdateTabUpdate>>,
    receiver: Receiver<Option<UpdateTabUpdate>>,
    update_tab_context: UpdateTabContext,
    client_sender: S,
    link: Link<'_, ()>,
    msg_sender: MsgSender,
) {
    thread::scope(|scope| {
        scope
            .spawn(|inner_scope| {
                let mut is_running = true;
                sender.send(Some(UpdateTabUpdate::new())).unwrap();
                loop {
                    if !is_running {
                        break;
                    }
                    channel::select! {
                        recv(receiver) -> msg => {
                            match msg {
                                Ok(res) => {
                                    match res {
                                        Some(update) => {
                                            // Check for path updates.
                                            if let Some(fw_dir) = update.firmware_directory {
                                                update_tab_context.set_firmware_directory(fw_dir.clone());
                                                update_tab_context
                                                    .set_firmware_local_filepath(check_for_firmware_local_filepath(fw_dir));
                                            }
                                            if let Some(fw_local_filepath) = update.firmware_local_filepath {
                                                if let Some(parent_path) = fw_local_filepath.parent() {
                                                    update_tab_context.set_firmware_directory(parent_path.to_path_buf());
                                                }
                                                update_tab_context.set_firmware_local_filepath(Some(fw_local_filepath.clone()));
                                            }
                                            if let Some(fw_local_filename) = update.firmware_local_filename {
                                                let fw_local_filepath =
                                                    update_tab_context.firmware_directory().join(fw_local_filename);
                                                update_tab_context.set_firmware_local_filepath(Some(fw_local_filepath));
                                            }
                                            if let Some(fileio_local_filepath) = update.fileio_local_filepath {
                                                update_tab_context.set_fileio_local_filepath(Some(fileio_local_filepath));
                                            }
                                            if let Some(fileio_destination_filepath) = update.fileio_destination_filepath {
                                                update_tab_context
                                                    .set_fileio_destination_filepath(Some(fileio_destination_filepath));
                                            }
                                            // Check for button changes.
                                            if update.download_latest_firmware && !update_tab_context.downloading() {
                                                inner_scope.spawn(|_| {
                                                    download_firmware(update_tab_context.clone());
                                                });
                                            }
                                            if update.update_firmware && !update_tab_context.upgrading() {
                                                inner_scope.spawn(|_| {
                                                    let mut fileio = Fileio::new(link.clone(), msg_sender.clone());
                                                    upgrade_firmware(
                                                        update_tab_context.clone(),
                                                        &mut fileio,
                                                        msg_sender.clone(),
                                                    );
                                                });
                                            }
                                        },
                                        None => {
                                            is_running = false;
                                        }
                                    }
                                },
                                Err(err) => {
                                    error!("{}", err.to_string());
                                }
                            }
                        },
                        recv(channel::after(Duration::from_millis(UPDATE_THREAD_SLEEP_MS))) -> _ => ()
                    }
                    update_frontend(client_sender.clone(), update_tab_context.clone());
                }
            })
            .join()
            .expect(THREAD_JOIN_FAILURE);
    })
    .expect(CROSSBEAM_SCOPE_UNWRAP_FAILURE);
}

/// Package data into a message buffer and send to frontend.
fn update_frontend<S: CapnProtoSender>(
    mut client_sender: S,
    mut update_tab_context: UpdateTabContext,
) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let packet = update_tab_context.packet();
    let mut status = msg.init_update_tab_status();

    status.set_hardware_revision("piksi_multi");
    status.set_fw_version_current("");
    status.set_fw_version_latest(&packet.latest_firmware_version);
    status.set_fw_local_filename(&packet.firmware_filename);
    status.set_fileio_local_filepath(&packet.fileio_local_filepath.to_string_lossy().to_string());
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

fn download_firmware(update_tab_context: UpdateTabContext) {
    let directory = update_tab_context.firmware_directory();
    update_tab_context.set_downloading(true);
    update_tab_context.fw_log_clear();
    let mut update_downloader = update_tab_context.update_downloader();
    let filepath = match update_downloader
        .download_multi_firmware(directory, Some(update_tab_context.clone()))
    {
        Ok(filepath_) => Some(filepath_),
        Err(e) => {
            error!("{}", e);
            None
        }
    };
    update_tab_context.set_firmware_local_filepath(filepath);
    update_tab_context.set_downloading(false);
    log::logger().flush();
}

fn upgrade_firmware(
    update_tab_context: UpdateTabContext,
    fileio: &mut Fileio,
    msg_sender: MsgSender,
) {
    update_tab_context.set_upgrading(true);
    update_tab_context.fw_log_clear();
    let mut update_downloader = update_tab_context.update_downloader();
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
                            update_tab_context.fw_log_append(format!(
                                "Latest firmware version, {}, is newer than current version, {}.",
                                latest_version, current_version
                            ));
                        } else {
                            update_tab_context.fw_log_append(format!("Latest firmware version, {}, is not newer than current version, {}.", latest_version, current_version));
                        }
                        comp
                    }
                    Err(err) => {
                        update_tab_context.fw_log_append(String::from(
                            "Unable to compare latest versus current version.",
                        ));
                        update_tab_context.fw_log_append(err.to_string());
                        false
                    }
                }
            } else {
                update_tab_context.fw_log_append(String::from(
                    "Current Version needed to compare Firmware before upgrade.",
                ));
                false
            }
        }
        Err(err) => {
            update_tab_context.fw_log_append(String::from(
                "Latest Version needed to compare Firmware before upgrade.",
            ));
            update_tab_context.fw_log_append(err.to_string());
            false
        }
    };
    if to_upgrade {
        if let Err(err) = firmware_upgrade(update_tab_context.clone(), fileio, msg_sender) {
            update_tab_context.fw_log_append(err.to_string());
        }
    }
    update_tab_context.set_upgrading(false);
    log::logger().flush();
}

fn firmware_upgrade(
    update_tab_context: UpdateTabContext,
    fileio: &mut Fileio,
    msg_sender: MsgSender,
) -> anyhow::Result<()> {
    if let Some(filepath) = update_tab_context.firmware_local_filepath() {
        update_tab_context.fw_log_append(format!(
            "Reading firmware file from path, {}.",
            filepath.display()
        ));
        if !filepath.exists() || !filepath.is_file() {
            return Err(anyhow!(
                "Firmware filepath is not a file or does not exist."
            ));
        }
        if let Ok(firmware_blob) = std::fs::File::open(filepath.clone()) {
            update_tab_context.fw_log_append(String::from("Transferring image to device..."));
            update_tab_context.fw_log_append(String::from(""));
            match fileio.overwrite(
                String::from(UPGRADE_FIRMWARE_REMOTE_DESTINATION),
                firmware_blob,
            ) {
                Ok(_) => {
                    update_tab_context.fw_log_append(String::from("Image transfer complete."));
                }
                Err(err) => {
                    update_tab_context.fw_log_append(String::from("Image transfer failed."));
                    update_tab_context.fw_log_append(err.to_string());
                    return Err(err);
                }
            }
            update_tab_context.fw_log_append(String::from("Committing image to flash..."));
            update_tab_context.fw_log_append(String::from(""));
            match firmware_upgrade_commit_to_flash(update_tab_context.clone(), msg_sender.clone()) {
                Ok(code) => {
                    if code == 0 {
                        update_tab_context.fw_log_append(String::from("Upgrade Complete."));
                        update_tab_context.fw_log_append(String::from("Resetting Piksi..."));
                        let msg = MsgReset {
                            sender_id: None,
                            flags: 0,
                        };
                        let msg = sbp::messages::SBP::from(msg);
                        msg_sender.send(msg)?;
                    } else {
                        update_tab_context.fw_log_append(String::from("Image transfer failed."))
                    }
                }
                _ => update_tab_context.fw_log_append(String::from("Image transfer failed.")),
            }
        } else {
            return Err(anyhow!("Failed to read firmware file, {:?}.", filepath));
        }
    }
    Ok(())
}

fn firmware_upgrade_commit_to_flash(
    mut update_tab_context: UpdateTabContext,
    msg_sender: MsgSender,
) -> anyhow::Result<i32> {
    let sequence = new_sequence();
    update_tab_context.set_upgrade_sequence(Some(sequence));
    let msg = MsgCommandReq {
        sender_id: None,
        sequence,
        command: SbpString::from(format!(
            "{} {}",
            UPGRADE_FIRMWARE_TOOL, UPGRADE_FIRMWARE_REMOTE_DESTINATION
        )),
    };
    let msg = sbp::messages::SBP::from(msg);
    msg_sender.send(msg)?;
    let start_time = Instant::now();
    let timeout = Duration::from_secs(UPGRADE_FIRMWARE_TIMEOUT_SEC);
    while update_tab_context.upgrading() && start_time.elapsed() < timeout {
        std::thread::sleep(Duration::from_millis(100));
    }
    let code = if let Some(ret_code) = update_tab_context.upgrade_ret() {
        ret_code
    } else {
        update_tab_context.fw_log_append(String::from("Upgrade process timed out."));
        -255
    };
    Ok(code)
}

#[derive(Debug)]
pub struct UpdateTabUpdate {
    pub firmware_local_filepath: Option<PathBuf>,
    pub firmware_local_filename: Option<PathBuf>,
    pub fileio_local_filepath: Option<PathBuf>,
    pub fileio_destination_filepath: Option<PathBuf>,
    pub firmware_directory: Option<PathBuf>,
    pub download_latest_firmware: bool,
    pub update_firmware: bool,
    pub send_file_to_device: bool,
}

impl UpdateTabUpdate {
    fn new() -> UpdateTabUpdate {
        UpdateTabUpdate {
            download_latest_firmware: false,
            update_firmware: false,
            send_file_to_device: false,
            firmware_directory: Some(LOG_DIRECTORY.path()),
            firmware_local_filepath: None,
            firmware_local_filename: None,
            fileio_local_filepath: None,
            fileio_destination_filepath: None,
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
impl Default for FirmwareUpgradePaneLogger {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UpdateTabContextInner {
    upgrade_ret: Option<i32>,
    upgrade_sequence: Option<u32>,
    downloading: bool,
    upgrading: bool,
    debug: bool,
    firmware_directory: PathBuf,
    firmware_local_filepath: Option<PathBuf>,
    fileio_destination_filepath: Option<PathBuf>,
    fileio_local_filepath: Option<PathBuf>,
    update_downloader: UpdateDownloader,
    fw_logger: FirmwareUpgradePaneLogger,
}
impl UpdateTabContextInner {
    pub fn new() -> UpdateTabContextInner {
        UpdateTabContextInner {
            upgrade_ret: None,
            upgrade_sequence: None,
            upgrading: false,
            downloading: false,
            debug: false,
            firmware_directory: PathBuf::from(""),
            firmware_local_filepath: None,
            fileio_destination_filepath: None,
            fileio_local_filepath: None,
            update_downloader: UpdateDownloader::new(),
            fw_logger: FirmwareUpgradePaneLogger::new(),
        }
    }
}
impl Default for UpdateTabContextInner {
    fn default() -> Self {
        UpdateTabContextInner::new()
    }
}
pub struct UpdateTabContext(Arc<Mutex<UpdateTabContextInner>>);

impl UpdateTabContext {
    pub fn new() -> UpdateTabContext {
        UpdateTabContext(Arc::new(Mutex::new(UpdateTabContextInner::default())))
    }
    pub fn debug(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).debug
    }
    pub fn set_debug(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).debug = set_to;
    }
    pub fn upgrade_ret(&self) -> Option<i32> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrade_ret.take()
    }
    pub fn set_upgrade_ret(&mut self, upgrade_ret: Option<i32>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrade_ret = upgrade_ret;
    }
    pub fn upgrade_sequence(&self) -> Option<u32> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrade_sequence
    }
    pub fn set_upgrade_sequence(&mut self, sequence: Option<u32>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).upgrade_sequence = sequence;
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
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        if let Err(err) = (*shared_data).update_downloader.get_index_data() {
            error!("{}", err);
        }
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
        let latest_firmware_version = if !(*shared_data).debug {
            if let Ok(version) = (*shared_data).update_downloader.latest_firmware_version() {
                version
            } else {
                String::from("")
            }
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

impl Deref for UpdateTabContext {
    type Target = Mutex<UpdateTabContextInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UpdateTabContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for UpdateTabContext {
    fn clone(&self) -> Self {
        UpdateTabContext {
            0: Arc::clone(&self.0),
        }
    }
}
