
use capnp::message::Builder;
use log::error;
use std::{ops::Deref, path::PathBuf, sync::{Arc, Mutex}, thread::JoinHandle};

use crate::errors::{CONVERT_TO_STR_FAILURE, SHARED_STATE_LOCK_MUTEX_FAILURE, THREAD_JOIN_FAILURE};
use crate::types::{
    ArcBool, CapnProtoSender, SharedState, UpdateTabButtons
};
use crate::update_downloader::UpdateDownloader;
use crate::utils::serialize_capnproto_builder;



/// UpdateTab struct.
///
/// # Fields
/// - `client_sender`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
pub struct UpdateTab<S: CapnProtoSender> {
    client_sender: S,
    is_running: ArcBool,
    shared_state: SharedState,
    update_shared: UpdateShared,
    handler: Option<JoinHandle<()>>,
}

impl<S: CapnProtoSender> UpdateTab<S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
    ) -> UpdateTab<S> {
        let is_running = ArcBool::new_with(true);
        let update_shared = UpdateShared::new();
        let update_tab_thread = UpdateTab::<S>::update_tab_thread(client_sender.clone(), shared_state.clone(), update_shared.clone(), is_running.clone());
        UpdateTab {
            client_sender,
            is_running,
            shared_state,
            update_shared,
            handler: Some(update_tab_thread),
        }
    }

    fn update_tab_thread(client_sender: S, shared_state: SharedState, update_shared: UpdateShared, is_running: ArcBool) -> JoinHandle<()> {
        let mut update_downloader_thread: Option<JoinHandle<()>> = None;
        let mut shared_state = shared_state.clone();
        std::thread::spawn(move || loop {
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
                    
                    update_downloader_thread = Some(UpdateTab::<S>::download_firmware(update_shared.clone(), directory.clone()));
                }
            }
            UpdateTab::<S>::update_frontend(update_shared.clone(), client_sender.clone(), directory);

            // Check for messages from MSG_LOG?

            // Send update to frontend?
            
        })
    }

    /// Package data into a message buffer and send to frontend.
    fn update_frontend(update_shared: UpdateShared, client_sender: S, directory: PathBuf) {
        

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

        client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn download_firmware(update_shared: UpdateShared, directory: PathBuf) -> JoinHandle<()> {
        
        update_shared.set_downloading(true);
        let mut update_downloader = update_shared.update_downloader();
        std::thread::spawn(move || {
            let filepath = match update_downloader.download_multi_firmware(directory) {
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
        })
    }

    
}

impl<S: CapnProtoSender> Drop for UpdateTab<S> {
    fn drop(&mut self) {
        self.is_running.set(false);
        if let Some(handle) = self.handler.take() {
            handle.join().expect(THREAD_JOIN_FAILURE);
        }
    }
}

pub struct UpdateSharedInner {
    downloading: bool,
    firmware_filepath: Option<PathBuf>,
    update_downloader: UpdateDownloader,
}
impl UpdateSharedInner {
    pub fn new() -> UpdateSharedInner {
        let mut update_downloader = UpdateDownloader::new();
        if let Err(err) = update_downloader.get_index_data() {
            error!("{}", err);
        }
        UpdateSharedInner {
            downloading: false,
            firmware_filepath: None,
            update_downloader,
        }
    }
    
}
impl Default for UpdateSharedInner {
    fn default() -> Self {
        UpdateSharedInner::new()
    }
}
pub struct UpdateShared(Arc<Mutex<UpdateSharedInner>>);

struct UpdatePacket {
    latest_firmware_version: String,
    downloading: bool,
    firmware_filename: String,
}

impl UpdateShared {
    pub fn new() -> UpdateShared {
        UpdateShared(Arc::new(Mutex::new(UpdateSharedInner::default())))
    }

    pub fn downloading(&self) -> bool {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).downloading
    }
    pub fn set_downloading(&self, set_to: bool){
        let mut shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        (*shared_data).downloading = set_to;
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
    pub fn packet(&self) -> UpdatePacket {
        let shared_data = self.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
        let latest_firmware_version = if let Ok(version) = (*shared_data).update_downloader.latest_firmware_version() {
            version
        } else {
            String::from("")
        };
        let downloading = (*shared_data).downloading;
        let firmware_filename = if let Some(firmware_filepath_) = (*shared_data).firmware_filepath.clone() {
            firmware_filepath_.file_name().expect(CONVERT_TO_STR_FAILURE).to_string_lossy().to_string()
        } else {
            String::new()
        };
        UpdatePacket {
            latest_firmware_version,
            downloading,
            firmware_filename,
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
    use super::*;
    use crate::types::TestSender;
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
