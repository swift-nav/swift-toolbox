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

use anyhow::anyhow;
use capnp::message::Builder;
use crossbeam::{
    channel::{self, Receiver, Sender},
    thread,
};
use glob::glob;
use log::{debug, error};

use sbp::link::Link;

use std::{
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::errors::{
    CONVERT_TO_STR_FAILURE, CROSSBEAM_SCOPE_UNWRAP_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE,
    THREAD_JOIN_FAILURE,
};
use crate::fileio::Fileio;
use crate::shared_state::{SharedState, LOG_DIRECTORY};
use crate::types::{ArcBool, MsgSender, Result};
use crate::updater::{
    firmware_update::{firmware_update, LogOverwriteBehavior},
    swift_version::SwiftVersion,
    update_downloader::UpdateDownloader,
};
use crate::utils::serialize_capnproto_builder;
use crate::{
    client_sender::BoxedClientSender,
    constants::{FIRMWARE_V2, FIRMWARE_V2_VERSION, HARDWARE_REVISION},
};

const UPDATE_THREAD_SLEEP_MS: u64 = 1000;
const WAIT_FOR_SETTINGS_THREAD_SLEEP_MS: u64 = 500;

pub struct UpdateTab {
    /// A sender to send messages to the update thread.
    sender: Sender<Option<UpdateTabUpdate>>,
    /// A receiver to receive messages from the update thread.
    receiver: Receiver<Option<UpdateTabUpdate>>,
    /// The shared state for update tab.
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

pub fn update_tab_thread(
    sender: Sender<Option<UpdateTabUpdate>>,
    receiver: Receiver<Option<UpdateTabUpdate>>,
    update_tab_context: UpdateTabContext,
    shared_state: SharedState,
    client_sender: BoxedClientSender,
    link: Link<'static, ()>,
    msg_sender: MsgSender,
) {
    let is_running = ArcBool::new_with(true);
    let mut app_started = false;
    thread::scope(|scope| {
        scope.spawn(|_| {
            if let Err(err) = wait_for_device_settings(is_running.clone(), update_tab_context.clone(), shared_state.clone()) {
                error!("{}", err.to_string());
            }
        });
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
                                                if app_started {
                                                    update_tab_context.set_firmware_local_filepath(check_for_firmware_local_filepath(fw_dir));
                                                }
                                                app_started = true;
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
                                                    if let Err(err) = upgrade_firmware(
                                                        update_tab_context.clone(),
                                                        link.clone(),
                                                        msg_sender.clone(),
                                                    ) {
                                                        update_tab_context.fw_log_append(format!("Error starting upgrade: {err}"));
                                                    }
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

                                            if update.check_for_updates {
                                                inner_scope.spawn(|_| {
                                                    if let Err(err) = check_console_outdated(update_tab_context.clone()) {
                                                        error!("{}", err);
                                                    }
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
    update_tab_context: UpdateTabContext,
    shared_state: SharedState,
) -> Result<()> {
    update_tab_context.fw_log_append(
        "Warning: Settings received from Piksi do not contain firmware version information. \
        Unable to determine software update status."
            .to_string(),
    );
    while is_running.get() && !update_tab_context.debug() {
        if let Some(firmware_version) = shared_state.firmware_version() {
            update_tab_context.set_current_firmware_version(firmware_version);
            update_tab_context.fw_log_clear();
            check_console_outdated(update_tab_context.clone())?;
            check_firmware_outdated(update_tab_context.clone())?;
            check_above_v2(update_tab_context)?;
            break;
        }
        std::thread::sleep(Duration::from_millis(WAIT_FOR_SETTINGS_THREAD_SLEEP_MS));
    }
    Ok(())
}

/// Package data into a message buffer and send to frontend.
fn update_frontend(client_sender: BoxedClientSender, mut update_tab_context: UpdateTabContext) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let packet = update_tab_context.packet();
    let mut status = msg.init_update_tab_status();

    status.set_hardware_revision(HARDWARE_REVISION);
    status.set_fw_version_current(&packet.current_firmware_version);
    status.set_fw_version_latest(&packet.latest_firmware_version);
    status.set_fw_local_filename(&packet.firmware_filename);
    status.set_fileio_local_filepath(&packet.fileio_local_filepath.to_string_lossy());
    status.set_fileio_destination_filepath(&packet.fileio_destination_filepath.to_string_lossy());
    status.set_directory(&packet.firmware_directory.to_string_lossy());
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
    let filepath = update_tab_context
        .current_firmware_version()
        .ok_or_else(||anyhow!("Waiting on settings to load to get current version."))
        .and_then(|current_version| {
            if let Ok(above_v2) = check_above_v2(update_tab_context.clone()) {
                return if !above_v2 {
                    update_tab_context.fw_log_append(format!(
                        "Current firmware version, {current_version}, requires upgrading to {FIRMWARE_V2_VERSION} before upgrading to latest version."
                    ));
                    update_downloader.download_multi_v2_firmware(directory, Some(update_tab_context.clone()))
                } else {
                    update_downloader.download_multi_firmware(directory, Some(update_tab_context.clone()))
                }
            }
            Err(anyhow!("Waiting on settings to load to get current version."))
        });

    let filepath = match filepath {
        Ok(path) => Some(path),
        Err(err) => {
            update_tab_context.fw_log_append(err.to_string());
            None
        }
    };
    update_tab_context.set_firmware_local_filepath(filepath);
    update_tab_context.set_downloading(false);
}

fn check_above_v2(update_tab_context: UpdateTabContext) -> Result<bool> {
    if let Some(current_version) = update_tab_context.current_firmware_version() {
        let current = SwiftVersion::parse(&current_version)?;
        let above_v2 = current.is_dev() || current > *FIRMWARE_V2;
        if !above_v2 {
            update_tab_context.fw_log_append(format!(
                "Checkpoint firmware version, {FIRMWARE_V2_VERSION}, is newer than current version, {current_version}." 
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
        let current = SwiftVersion::parse(&current_version)?;
        let latest = SwiftVersion::parse(&latest_version)?;
        let outdated = latest > current;
        if outdated {
            update_tab_context.fw_log_append(format!(
                "Latest firmware version, {latest_version}, does not match current version, {current_version}."
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
        let current = SwiftVersion::parse(&current_version)?;
        let latest = SwiftVersion::parse(&latest_version)?;
        let outdated = latest > current;
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
    link: Link<'static, ()>,
    msg_sender: MsgSender,
) -> anyhow::Result<()> {
    let raw_version = update_tab_context
        .current_firmware_version()
        .ok_or_else(|| anyhow!("Could not get current filename!"))?;
    let current_version = SwiftVersion::parse(&raw_version)?;
    let filepath = update_tab_context
        .firmware_local_filepath()
        .ok_or_else(|| anyhow!("Could not get firmware filepath!"))?;

    update_tab_context.set_upgrading(true);
    update_tab_context.fw_log_clear();

    let log_callback_ctx = update_tab_context.clone();
    let progress_callback_ctx = update_tab_context.clone();

    if let Err(err) = firmware_update(
        link,
        msg_sender,
        &filepath,
        &current_version,
        move |msg, overwrite| match overwrite {
            LogOverwriteBehavior::DontOverwrite => log_callback_ctx.fw_log_append(msg),
            LogOverwriteBehavior::Overwrite => log_callback_ctx.fw_log_replace_last(msg),
        },
        move |progress| {
            progress_callback_ctx
                .fw_log_replace_last(format!("Uploading image to device {progress:.2}%..."));
        },
    ) {
        update_tab_context.fw_log_append(err.to_string());
    }

    update_tab_context.set_upgrading(false);
    Ok(())
}

fn send_file_to_device(update_tab_context: UpdateTabContext, fileio: &mut Fileio) {
    update_tab_context.set_upgrading(true);
    update_tab_context.fw_log_clear();
    if let Err(err) = send_file(update_tab_context.clone(), fileio) {
        update_tab_context.fw_log_append(err.to_string());
    }
    update_tab_context.set_upgrading(false);
}

fn send_file(update_tab_context: UpdateTabContext, fileio: &mut Fileio) -> anyhow::Result<()> {
    if let Some(filepath) = update_tab_context.fileio_local_filepath() {
        update_tab_context
            .fw_log_append(format!("Reading file from path, {}.", filepath.display()));
        if !filepath.exists() || !filepath.is_file() {
            return Err(anyhow!("Path provided is not a file or does not exist."));
        }
        let destination = String::from(
            update_tab_context
                .fileio_destination_filepath()
                .ok_or_else(|| anyhow!("No destination filepath provided."))?
                .to_string_lossy(),
        );
        let file_blob = std::fs::File::open(&filepath)
            .map_err(|e| anyhow!("Failed to read file {}: {}", filepath.display(), e))?;

        update_tab_context.fw_log_append(format!(
            "Transferring file to device location {destination}"
        ));
        update_tab_context.fw_log_append(String::from(""));
        let size = file_blob.metadata()?.len() as usize;
        let mut bytes_written = 0;
        update_tab_context.fw_log_replace_last("Writing 0.0%...".to_string());
        match fileio.overwrite_with_progress(destination, file_blob, |n| {
            bytes_written += n;
            let progress = (bytes_written as f64) / (size as f64) * 100.0;
            update_tab_context.fw_log_replace_last(format!("Writing {progress:.2}%..."));
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
    }
    Ok(())
}

#[derive(Debug, Default)]
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
    pub check_for_updates: bool,
}

impl UpdateTabUpdate {
    fn new() -> UpdateTabUpdate {
        UpdateTabUpdate {
            firmware_directory: Some(LOG_DIRECTORY.path()),
            ..Default::default()
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

#[derive(Default)]
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
            firmware_directory: PathBuf::from(""),
            update_downloader: UpdateDownloader::new(),
            fw_logger: FirmwareUpgradePaneLogger::new(),
            ..Default::default()
        }
    }
}

pub struct UpdateTabContext(Arc<Mutex<UpdateTabContextInner>>);

impl UpdateTabContext {
    pub fn new() -> UpdateTabContext {
        UpdateTabContext(Arc::new(Mutex::new(UpdateTabContextInner::default())))
    }
    pub fn debug(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.debug
    }
    pub fn set_debug(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.debug = set_to;
    }
    pub fn upgrade_ret(&self) -> Option<i32> {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrade_ret.take()
    }
    pub fn set_upgrade_ret(&mut self, upgrade_ret: Option<i32>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrade_ret = upgrade_ret;
    }
    pub fn upgrade_sequence(&self) -> Option<u32> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrade_sequence
    }
    pub fn set_upgrade_sequence(&mut self, sequence: Option<u32>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrade_sequence = sequence;
    }
    pub fn downloading(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.downloading
    }
    pub fn set_downloading(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.downloading = set_to;
    }
    pub fn upgrading(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrading
    }
    pub fn set_upgrading(&self, set_to: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.upgrading = set_to;
    }
    pub fn fileio_destination_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fileio_destination_filepath.clone()
    }
    pub fn set_fileio_destination_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fileio_destination_filepath = filepath;
    }
    pub fn fileio_local_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fileio_local_filepath.clone()
    }
    pub fn set_fileio_local_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fileio_local_filepath = filepath;
    }
    pub fn firmware_local_filepath(&self) -> Option<PathBuf> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_local_filepath.clone()
    }
    pub fn set_firmware_local_filepath(&self, filepath: Option<PathBuf>) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_local_filepath = filepath;
    }
    pub fn firmware_directory(&self) -> PathBuf {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_directory.clone()
    }
    pub fn set_firmware_directory(&self, directory: PathBuf) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_directory = directory;
    }
    pub fn update_downloader(&self) -> UpdateDownloader {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        if let Err(err) = shared_data.update_downloader.get_index_data() {
            error!("{}", err);
        }
        shared_data.update_downloader.clone()
    }
    pub fn fw_log_append(&self, log: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        debug!("{}", log);
        shared_data.fw_logger.log_append(log);
    }
    pub fn fw_log_replace_last(&self, log: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fw_logger.log_replace_last(log);
    }
    pub fn fw_log_clear(&self) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fw_logger.clear();
    }
    pub fn fw_log(&self) -> String {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.fw_logger.joined_string()
    }
    pub fn current_firmware_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.current_firmware_version.clone()
    }
    pub fn set_current_firmware_version(&self, current_firmware_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.current_firmware_version = Some(current_firmware_version);
    }
    pub fn current_console_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.current_console_version.clone()
    }
    pub fn set_current_console_version(&self, current_console_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.current_console_version = Some(current_console_version);
    }
    pub fn console_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.console_outdated
    }
    pub fn set_console_outdated(&self, console_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.console_outdated = console_outdated;
    }
    pub fn firmware_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_outdated
    }
    pub fn set_firmware_outdated(&self, firmware_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_outdated = firmware_outdated;
    }
    pub fn firmware_v2_outdated(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_v2_outdated
    }
    pub fn set_firmware_v2_outdated(&self, firmware_v2_outdated: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.firmware_v2_outdated = firmware_v2_outdated;
    }
    pub fn serial_prompt(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.serial_prompt
    }
    pub fn set_serial_prompt(&self, serial_prompt: bool) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.serial_prompt = serial_prompt;
    }
    pub fn set_latest_console_version(&self, latest_console_version: String) {
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.latest_console_version = Some(latest_console_version);
    }
    pub fn latest_console_version(&self) -> Option<String> {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        shared_data.latest_console_version.clone()
    }
    pub fn packet(&mut self) -> UpdatePacket {
        let fw_log = self.fw_log();
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let latest_firmware_version = if !shared_data.debug {
            shared_data
                .update_downloader
                .latest_firmware_version()
                .unwrap_or_default()
        } else {
            String::from("")
        };
        let current_firmware_version = shared_data
            .current_firmware_version
            .clone()
            .unwrap_or_default();
        let serial_prompt = shared_data.serial_prompt;
        let console_outdated = shared_data.console_outdated;
        let fw_outdated = shared_data.firmware_outdated;
        let fw_v2_outdated = shared_data.firmware_v2_outdated;
        let downloading = shared_data.downloading;
        let upgrading = shared_data.upgrading;
        let firmware_directory = shared_data.firmware_directory.clone();
        let fileio_destination_filepath = shared_data
            .fileio_destination_filepath
            .clone()
            .unwrap_or_default();
        let fileio_local_filepath = shared_data
            .fileio_local_filepath
            .clone()
            .unwrap_or_default();
        let firmware_filename =
            if let Some(firmware_local_filepath_) = shared_data.firmware_local_filepath.clone() {
                firmware_local_filepath_
                    .file_name()
                    .expect(CONVERT_TO_STR_FAILURE)
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };
        let console_version_current = shared_data
            .current_console_version
            .clone()
            .unwrap_or_default();
        let console_version_latest = shared_data
            .latest_console_version
            .clone()
            .unwrap_or_default();
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
        UpdateTabContext(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod update_thread {
        use super::*;
        use crate::client_sender::TestSender;
        use glob::glob;
        use sbp::link::LinkSource;
        use std::{io::sink, time::Instant};
        use tempfile::TempDir;

        #[test]
        fn thread_test() {
            let shared_state = SharedState::new();
            let update_tab = UpdateTab::new(shared_state.clone());
            let ctx = update_tab.clone_update_tab_context();
            let client_send = TestSender::boxed();
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let writer = sink();
            let msg_sender = MsgSender::new(writer);
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
            let client_send = TestSender::boxed();
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let writer = sink();
            let msg_sender = MsgSender::new(writer);
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
                        download_latest_firmware: true,
                        ..Default::default()
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
            let client_send = TestSender::boxed();
            let (update_tab_tx, update_tab_rx) = update_tab.clone_channel();
            let source: LinkSource<()> = LinkSource::new();
            let link = source.link();
            let writer = sink();
            let msg_sender = MsgSender::new(writer);
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
                        ..Default::default()
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
