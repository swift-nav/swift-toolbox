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
use crate::types::{ArcBool, CapnProtoSender, MsgSender, Result};
use crate::update_downloader::UpdateDownloader;
use crate::utils::{compare_semvers, serialize_capnproto_builder};

const HARDWARE_REVISION: &str = "piksi_multi";
const FIRMWARE_V2_VERSION: &str = "v2.0.0";
const UPGRADE_FIRMWARE_REMOTE_DESTINATION: &str = "upgrade.image_set.bin";
const UPGRADE_FIRMWARE_TOOL: &str = "upgrade_tool";
const UPGRADE_FIRMWARE_TIMEOUT_SEC: u64 = 600;
const UPDATE_THREAD_SLEEP_MS: u64 = 1000;
const WAIT_FOR_SETTINGS_THREAD_SLEEP_MS: u64 = 500;
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
        update_tab_context.set_current_console_version(shared_state.console_version());
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
    shared_state: SharedState,
    client_sender: S,
    link: Link<'_, ()>,
    msg_sender: MsgSender,
) {
    let is_running = ArcBool::new_with(true);
    thread::scope(|scope| {
        scope.spawn(|_| wait_for_device_settings(is_running.clone(), sender.clone(), update_tab_context.clone(), shared_state.clone()));
        scope
            .spawn(|inner_scope| {
                sender.send(Some(UpdateTabUpdate::new())).unwrap();
                while is_running.get() {
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
                                            if update.update_firmware && !update_tab_context.upgrading() && upgrade_confirmed(update_tab_context.clone(), update.serial_prompt_confirm) {
                                                inner_scope.spawn(|_| {
                                                    let mut fileio = Fileio::new(link.clone(), msg_sender.clone());
                                                    upgrade_firmware(
                                                        update_tab_context.clone(),
                                                        &mut fileio,
                                                        msg_sender.clone(),
                                                    );
                                                });
                                            }
                                            if update.send_file_to_device && !update_tab_context.upgrading() {
                                                inner_scope.spawn(|_| {
                                                    let mut fileio = Fileio::new(link.clone(), msg_sender.clone());
                                                    send_file_to_device(
                                                        update_tab_context.clone(),
                                                        &mut fileio,
                                                    );
                                                });
                                            }
                                        },
                                        None => {
                                            is_running.set(false);
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

fn wait_for_device_settings(
    is_running: ArcBool,
    sender: Sender<Option<UpdateTabUpdate>>,
    update_tab_context: UpdateTabContext,
    shared_state: SharedState,
) -> Result<()> {
    while is_running.get() && !update_tab_context.debug() {
        if let Some(firmware_version) = shared_state.firmware_version() {
            update_tab_context.set_current_firmware_version(firmware_version);
            check_console_outdated(update_tab_context.clone())?;
            check_firmware_outdated(update_tab_context.clone())?;
            check_above_v2(update_tab_context.clone())?;
            sender.send(Some(UpdateTabUpdate {
                firmware_directory: Some(update_tab_context.firmware_directory()),
                ..Default::default()
            }))?;
            break;
        }
        std::thread::sleep(Duration::from_millis(WAIT_FOR_SETTINGS_THREAD_SLEEP_MS));
    }
    Ok(())
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

    status.set_hardware_revision(HARDWARE_REVISION);
    status.set_fw_version_current(&packet.current_firmware_version);
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
    status.set_fw_outdated(packet.fw_outdated);
    status.set_fw_v2_outdated(packet.fw_v2_outdated);
    status.set_serial_prompt(packet.serial_prompt);
    status.set_console_outdated(packet.console_outdated);
    status.set_console_version_current(&packet.console_version_current);
    status.set_console_version_latest(&packet.console_version_latest);

    client_sender.send_data(serialize_capnproto_builder(builder));
}

fn download_firmware(update_tab_context: UpdateTabContext) {
    let directory = update_tab_context.firmware_directory();
    update_tab_context.set_downloading(true);
    update_tab_context.fw_log_clear();
    let mut update_downloader = update_tab_context.update_downloader();
    let filepath = if let Some(current_version) = update_tab_context.current_firmware_version() {
        if let Ok(above_v2) = check_above_v2(update_tab_context.clone()) {
            if !above_v2 {
                update_tab_context.fw_log_append(format!(
                    "Current firmware version, {}, requires upgrading to {} before upgrading to latest version.",
                    current_version, FIRMWARE_V2_VERSION
                ));
                match update_downloader
                    .download_multi_v2_firmware(directory, Some(update_tab_context.clone()))
                {
                    Ok(filepath_) => Some(filepath_),
                    Err(err) => {
                        update_tab_context.fw_log_append(err.to_string());
                        None
                    }
                }
            } else {
                match update_downloader
                    .download_multi_firmware(directory, Some(update_tab_context.clone()))
                {
                    Ok(filepath_) => Some(filepath_),
                    Err(err) => {
                        update_tab_context.fw_log_append(err.to_string());
                        None
                    }
                }
            }
        } else {
            update_tab_context.fw_log_append(String::from(
                "Waiting on settings to load to get current version.",
            ));
            None
        }
    } else {
        update_tab_context.fw_log_append(String::from(
            "Waiting on settings to load to get current version.",
        ));
        None
    };
    update_tab_context.set_firmware_local_filepath(filepath);
    update_tab_context.set_downloading(false);
}

fn check_above_v2(update_tab_context: UpdateTabContext) -> Result<bool> {
    if let Some(current_version) = update_tab_context.current_firmware_version() {
        let above_v2 = compare_semvers(
            String::from(FIRMWARE_V2_VERSION),
            current_version.clone(),
            false,
        )?;
        if !above_v2 {
            update_tab_context.fw_log_append(format!(
                "Checkpoint firmware version, {}, is newer than current version, {}.",
                FIRMWARE_V2_VERSION, current_version
            ));
        }
        update_tab_context.set_firmware_v2_outdated(!above_v2);
        Ok(above_v2)
    } else {
        anyhow::bail!("Unable to get current firmware version.")
    }
}

fn check_firmware_outdated(update_tab_context: UpdateTabContext) -> Result<bool> {
    let mut update_downloader = update_tab_context.update_downloader();
    let latest_version = update_downloader.latest_firmware_version()?;
    if let Some(current_version) = update_tab_context.current_firmware_version() {
        let outdated = compare_semvers(current_version.clone(), latest_version.clone(), true)?;
        if outdated {
            update_tab_context.fw_log_append(format!(
                "Latest firmware version, {}, is newer than current version, {}.",
                latest_version, current_version
            ));
        } else {
            update_tab_context.fw_log_append(format!(
                "Latest firmware version, {}, is not newer than current version, {}.",
                latest_version, current_version
            ));
        }
        update_tab_context.set_firmware_outdated(outdated);
        Ok(outdated)
    } else {
        anyhow::bail!("Unable to get current firmware version.")
    }
}

fn check_console_outdated(update_tab_context: UpdateTabContext) -> Result<bool> {
    let mut update_downloader = update_tab_context.update_downloader();
    let latest_version = update_downloader.latest_console_version()?;
    if let Some(current_version) = update_tab_context.current_console_version() {
        let outdated = compare_semvers(current_version.clone(), latest_version.clone(), true)?;
        if outdated {
            update_tab_context.fw_log_append(format!(
                "Latest console version, {}, is newer than current version, {}.",
                latest_version, current_version
            ));
        } else {
            update_tab_context.fw_log_append(format!(
                "Latest console version, {}, is not newer than current version, {}.",
                latest_version, current_version
            ));
        }
        update_tab_context.set_console_outdated(outdated);
        update_tab_context.set_current_console_version(current_version);
        update_tab_context.set_latest_console_version(latest_version);
        Ok(outdated)
    } else {
        anyhow::bail!("Unable to get current console version.")
    }
}

pub fn upgrade_confirmed(
    update_tab_context: UpdateTabContext,
    serial_prompt_confirm: bool,
) -> bool {
    if !serial_prompt_confirm && update_tab_context.serial_prompt() {
        update_tab_context.fw_log_clear();
        update_tab_context.fw_log_append(String::from(
"-----------------------------------------------
USB Flashdrive Upgrade Procedure
-----------------------------------------------
1.  Insert the USB flash drive provided with your Piksi Multi into your computer.
    Select the flash drive root directory as the firmware download destination using the directory chooser above.
    Press the \"Download Latest Firmware\" button. This will download the latest Piksi Multi firmware file onto
    the USB flashdrive.
2.  Eject the drive from your computer and plug it into the USB Host port of the Piksi Multi evaluation board.
3.  Reset your Piksi Multi and it will upgrade to the version on the USB flash drive.
    This should take less than 5 minutes.
4.  When the upgrade completes you will be prompted to remove the USB flash drive and reset your Piksi Multi.
5.  Verify that the firmware version has upgraded via inspection of the Current Firmware Version box
    on the Update Tab of the Swift Console."
        ));
        false
    } else {
        true
    }
}

fn upgrade_firmware(
    update_tab_context: UpdateTabContext,
    fileio: &mut Fileio,
    msg_sender: MsgSender,
) {
    update_tab_context.set_upgrading(true);
    update_tab_context.fw_log_clear();
    let to_upgrade = match check_above_v2(update_tab_context.clone()) {
        Ok(above_v2) => {
            if !above_v2 {
                true
            } else {
                match check_firmware_outdated(update_tab_context.clone()) {
                    Ok(outdated) => outdated,
                    Err(err) => {
                        update_tab_context.fw_log_append(String::from(
                            "Unable to compare latest and current version.",
                        ));
                        update_tab_context.fw_log_append(err.to_string());
                        false
                    }
                }
            }
        }
        Err(_) => {
            update_tab_context.fw_log_append(String::from(
                "Waiting on settings to load to get current version.",
            ));
            false
        }
    };
    if to_upgrade {
        if let Err(err) = firmware_upgrade(update_tab_context.clone(), fileio, msg_sender) {
            update_tab_context.fw_log_append(err.to_string());
        }
    }
    update_tab_context.set_upgrading(false);
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
            let size = firmware_blob.metadata()?.len() as usize;
            let mut bytes_written = 0;
            update_tab_context.fw_log_replace_last("Writing 0.0%...".to_string());
            match fileio.overwrite_with_progress(
                String::from(UPGRADE_FIRMWARE_REMOTE_DESTINATION),
                firmware_blob,
                |n| {
                    bytes_written += n;
                    let progress = (bytes_written as f64) / (size as f64) * 100.0;
                    update_tab_context.fw_log_replace_last(format!("Writing {:.2}%...", progress));
                },
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
) -> Result<i32> {
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

fn send_file_to_device(update_tab_context: UpdateTabContext, fileio: &mut Fileio) {
    // TODO (john-michaelburke) - [CPP-313] - Fix fileio hang on sending arbitrary file.
    // For now we will not block other downloading and upgrading operations.
    // update_tab_context.set_upgrading(true);
    update_tab_context.fw_log_clear();
    if let Err(err) = send_file(update_tab_context.clone(), fileio) {
        update_tab_context.fw_log_append(err.to_string());
    }
    // update_tab_context.set_upgrading(false);
}

fn send_file(update_tab_context: UpdateTabContext, fileio: &mut Fileio) -> anyhow::Result<()> {
    if let Some(filepath) = update_tab_context.fileio_local_filepath() {
        update_tab_context
            .fw_log_append(format!("Reading file from path, {}.", filepath.display()));
        if !filepath.exists() || !filepath.is_file() {
            return Err(anyhow!("Path provided is not a file or does not exist."));
        }
        if let Some(destination_) = update_tab_context.fileio_destination_filepath() {
            let destination = destination_.to_string_lossy();
            if let Ok(file_blob) = std::fs::File::open(filepath.clone()) {
                update_tab_context.fw_log_append(format!(
                    "Transferring file to device location {}",
                    destination.clone()
                ));
                update_tab_context.fw_log_append(String::from(""));
                let size = file_blob.metadata()?.len() as usize;
                let mut bytes_written = 0;
                update_tab_context.fw_log_replace_last("Writing 0.0%...".to_string());
                match fileio.overwrite_with_progress(destination.to_string(), file_blob, |n| {
                    bytes_written += n;
                    let progress = (bytes_written as f64) / (size as f64) * 100.0;
                    update_tab_context.fw_log_replace_last(format!("Writing {:.2}%...", progress));
                }) {
                    Ok(_) => {
                        update_tab_context.fw_log_append(String::from("File transfer complete."));
                    }
                    Err(err) => {
                        update_tab_context.fw_log_append(String::from("File transfer failed."));
                        update_tab_context.fw_log_append(err.to_string());
                        return Err(err);
                    }
                }
            } else {
                return Err(anyhow!("Failed to read file, {:?}.", filepath));
            }
        } else {
            return Err(anyhow!("No destination filepath provided."));
        }
    }
    Ok(())
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
    pub serial_prompt_confirm: bool,
}

impl UpdateTabUpdate {
    fn new() -> UpdateTabUpdate {
        UpdateTabUpdate {
            download_latest_firmware: false,
            update_firmware: false,
            send_file_to_device: false,
            serial_prompt_confirm: false,
            firmware_directory: Some(LOG_DIRECTORY.path()),
            firmware_local_filepath: None,
            firmware_local_filename: None,
            fileio_local_filepath: None,
            fileio_destination_filepath: None,
        }
    }
}
impl Default for UpdateTabUpdate {
    fn default() -> Self {
        Self::new()
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
    current_firmware_version: Option<String>,
    current_console_version: Option<String>,
    latest_console_version: Option<String>,
    console_outdated: bool,
    firmware_outdated: bool,
    firmware_v2_outdated: bool,
    serial_prompt: bool,
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
            current_firmware_version: None,
            current_console_version: None,
            latest_console_version: None,
            console_outdated: false,
            firmware_outdated: false,
            firmware_v2_outdated: false,
            serial_prompt: false,
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
    pub fn current_firmware_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).current_firmware_version.clone()
    }
    pub fn set_current_firmware_version(&self, current_firmware_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).current_firmware_version = Some(current_firmware_version);
    }
    pub fn current_console_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).current_console_version.clone()
    }
    pub fn set_current_console_version(&self, current_console_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).current_console_version = Some(current_console_version);
    }
    pub fn console_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).console_outdated
    }
    pub fn set_console_outdated(&self, console_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).console_outdated = console_outdated;
    }
    pub fn firmware_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_outdated
    }
    pub fn set_firmware_outdated(&self, firmware_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_outdated = firmware_outdated;
    }
    pub fn firmware_v2_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_v2_outdated
    }
    pub fn set_firmware_v2_outdated(&self, firmware_v2_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).firmware_v2_outdated = firmware_v2_outdated;
    }
    pub fn serial_prompt(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).serial_prompt
    }
    pub fn set_serial_prompt(&self, serial_prompt: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).serial_prompt = serial_prompt;
    }
    pub fn set_latest_console_version(&self, latest_console_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).latest_console_version = Some(latest_console_version);
    }
    pub fn latest_console_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).latest_console_version.clone()
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
        let current_firmware_version = (*shared_data).current_firmware_version.clone().unwrap_or_default();
        let serial_prompt = (*shared_data).serial_prompt;
        let console_outdated = (*shared_data).console_outdated;
        let fw_outdated = (*shared_data).firmware_outdated;
        let fw_v2_outdated = (*shared_data).firmware_v2_outdated;
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
        let console_version_current =
            if let Some(version) = (*shared_data).current_console_version.clone() {
                version
            } else {
                String::from("")
            };
        let console_version_latest =
            if let Some(version) = (*shared_data).latest_console_version.clone() {
                version
            } else {
                String::from("")
            };
        UpdatePacket {
            current_firmware_version,
            latest_firmware_version,
            downloading,
            upgrading,
            firmware_directory,
            fileio_destination_filepath,
            fileio_local_filepath,
            firmware_filename,
            fw_log,
            serial_prompt,
            console_outdated,
            fw_outdated,
            fw_v2_outdated,
            console_version_current,
            console_version_latest,
        }
    }
}

pub struct UpdatePacket {
    pub current_firmware_version: String,
    latest_firmware_version: String,
    downloading: bool,
    upgrading: bool,
    firmware_filename: String,
    firmware_directory: PathBuf,
    fileio_destination_filepath: PathBuf,
    fileio_local_filepath: PathBuf,
    fw_log: String,
    fw_outdated: bool,
    fw_v2_outdated: bool,
    serial_prompt: bool,
    console_outdated: bool,
    console_version_current: String,
    console_version_latest: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod update_tab {
        use super::*;

        #[test]
        fn handle_log_msg_test() {
            let shared_state = SharedState::new();
            let update_tab = UpdateTab::new(shared_state);
            let ctx = update_tab.clone_update_tab_context();
            ctx.set_upgrading(true);

            let sender_id = Some(1337);
            let level = 1;

            let good_log_messages = vec![
                "ok\r",
                "writing\r",
                "erasing\r",
                "100 % complete\r",
                "Error\r",
                "error\r",
                "Image\r",
                "upgrade\r",
                "Warning:\r",
                "install\r",
                "upgrade completed successfully\r",
            ];

            for log_message in good_log_messages {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                update_tab.handle_log_msg(msg);
                let ctx_ = ctx.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                assert_eq!(
                    (*ctx_).fw_logger.current_log[(*ctx_).fw_logger.current_log.len() - 1],
                    log_message.trim().to_string()
                );
            }

            let good_log_messages_one_line = vec![
                "ok",
                "writing",
                "erasing",
                "99 % complete",
                "Error",
                "error",
                "Image",
                "upgrade",
                "Warning",
                "install",
                "upgrade completed successfully",
            ];

            for log_message in good_log_messages_one_line {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                update_tab.handle_log_msg(msg);
                let ctx_ = ctx.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                assert_eq!(
                    (*ctx_).fw_logger.current_log[(*ctx_).fw_logger.current_log.len() - 1],
                    log_message.trim().to_string()
                );
            }

            let bad_log_messages = vec![
                "o1k",
                "wr2iting",
                "era3sing",
                "99 %4 complete",
                "Er5ror",
                "er6ror",
                "Im7age",
                "up8grade",
                "Wa9rning",
                "in10stall",
                "upgr11ade completed successfully",
            ];

            for log_message in bad_log_messages {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                update_tab.handle_log_msg(msg);
                let ctx_ = ctx.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                assert_ne!(
                    (*ctx_).fw_logger.current_log[(*ctx_).fw_logger.current_log.len() - 1],
                    log_message.trim().to_string()
                );
            }
        }
        #[test]
        fn handle_command_resp_test() {
            let shared_state = SharedState::new();
            let mut update_tab = UpdateTab::new(shared_state);

            let mut ctx = update_tab.clone_update_tab_context();
            assert!(ctx.upgrade_sequence().is_none());
            ctx.set_upgrading(true);
            let code = 1337;
            let sender_id = Some(1337);
            let correct_sequence = new_sequence();
            let incorrect_sequence = new_sequence();
            ctx.set_upgrade_sequence(Some(correct_sequence));
            let msg = MsgCommandResp {
                sender_id,
                sequence: incorrect_sequence,
                code,
            };
            update_tab.handle_command_resp(msg);
            assert_eq!(ctx.upgrade_sequence(), Some(correct_sequence));
            assert!(ctx.upgrade_ret().is_none());
            assert!(ctx.upgrading());
            let msg = MsgCommandResp {
                sender_id,
                sequence: correct_sequence,
                code,
            };
            update_tab.handle_command_resp(msg);
            assert!(ctx.upgrade_sequence().is_none());
            assert_eq!(ctx.upgrade_ret(), Some(code));
            assert!(!ctx.upgrading());
        }
    }
    mod update_thread {
        use super::*;
        use crate::types::TestSender;
        use glob::glob;
        use sbp::link::LinkSource;
        use std::io::sink;
        use tempfile::TempDir;

        #[test]
        fn thread_test() {
            let shared_state = SharedState::new();
            let update_tab = UpdateTab::new(shared_state.clone());
            let ctx = update_tab.clone_update_tab_context();
            let client_send = TestSender { inner: Vec::new() };
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let wtr = sink();
            let msg_sender = MsgSender::new(wtr);
            crossbeam::scope(|scope| {
                let handle = scope.spawn(|_| {
                    update_tab_thread(
                        update_tab_tx.clone(),
                        update_tab_rx,
                        ctx,
                        shared_state,
                        client_send,
                        link,
                        msg_sender.clone(),
                    );
                });
                update_tab_tx.send(None).unwrap();
                assert!(handle.join().is_ok());
            })
            .unwrap();
        }

        #[test]
        #[ignore]
        fn download_firmware_test() {
            let shared_state = SharedState::new();
            let update_tab = UpdateTab::new(shared_state.clone());
            let ctx = update_tab.clone_update_tab_context();
            let client_send = TestSender { inner: Vec::new() };
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let wtr = sink();
            let msg_sender = MsgSender::new(wtr);
            let tmp_dir = TempDir::new().unwrap();
            let tmp_dir = tmp_dir.path().to_path_buf();
            ctx.set_firmware_directory(tmp_dir.clone());
            shared_state.set_firmware_version(String::from("v2.0.0"));
            crossbeam::scope(|scope| {
                let handle = scope.spawn(|_| {
                    update_tab_thread(
                        update_tab_tx.clone(),
                        update_tab_rx,
                        ctx.clone(),
                        shared_state,
                        client_send,
                        link,
                        msg_sender.clone(),
                    );
                });
                assert!(!ctx.downloading());
                update_tab_tx
                    .send(Some(UpdateTabUpdate {
                        firmware_directory: None,
                        firmware_local_filename: None,
                        firmware_local_filepath: None,
                        fileio_local_filepath: None,
                        fileio_destination_filepath: None,
                        download_latest_firmware: true,
                        update_firmware: false,
                        send_file_to_device: false,
                        serial_prompt_confirm: false,
                    }))
                    .unwrap();
                let start_time = Instant::now();
                while !ctx.downloading() && start_time.elapsed() < Duration::from_secs(5) {
                    std::thread::sleep(Duration::from_millis(100));
                }
                assert!(ctx.downloading());
                let start_time = Instant::now();
                while ctx.downloading() && start_time.elapsed() < Duration::from_secs(60) {
                    std::thread::sleep(Duration::from_millis(100));
                }
                assert!(!ctx.downloading());
                let pattern = tmp_dir.join("PiksiMulti-*");
                let found_filepath = glob(&pattern.to_string_lossy())
                    .unwrap()
                    .next()
                    .unwrap()
                    .unwrap();
                assert!(found_filepath.is_file());
                update_tab_tx.send(None).unwrap();
                assert!(handle.join().is_ok());
            })
            .unwrap();
        }

        #[test]
        fn new_firmware_directory_test() {
            let shared_state = SharedState::new();
            let update_tab = UpdateTab::new(shared_state.clone());
            let ctx = update_tab.clone_update_tab_context();
            let client_send = TestSender { inner: Vec::new() };
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let wtr = sink();
            let msg_sender = MsgSender::new(wtr);
            let tmp_dir = TempDir::new().unwrap();
            let tmp_dir = tmp_dir.path().to_path_buf();
            crossbeam::scope(|scope| {
                let handle = scope.spawn(|_| {
                    update_tab_thread(
                        update_tab_tx.clone(),
                        update_tab_rx,
                        ctx.clone(),
                        shared_state.clone(),
                        client_send,
                        link,
                        msg_sender.clone(),
                    );
                });
                let start_time = Instant::now();
                while start_time.elapsed() < Duration::from_secs(5) {
                    std::thread::sleep(Duration::from_millis(100));
                }
                assert_eq!(ctx.firmware_directory(), LOG_DIRECTORY.path());
                update_tab_tx
                    .send(Some(UpdateTabUpdate {
                        firmware_directory: Some(tmp_dir.clone()),
                        firmware_local_filename: None,
                        firmware_local_filepath: None,
                        fileio_local_filepath: None,
                        fileio_destination_filepath: None,
                        download_latest_firmware: false,
                        update_firmware: false,
                        send_file_to_device: false,
                        serial_prompt_confirm: false,
                    }))
                    .unwrap();
                let start_time = Instant::now();
                while start_time.elapsed() < Duration::from_secs(5) {
                    std::thread::sleep(Duration::from_millis(100));
                }
                assert_eq!(ctx.firmware_directory(), tmp_dir.clone());
                update_tab_tx.send(None).unwrap();
                assert!(handle.join().is_ok());
            })
            .unwrap();
        }
    }
}
