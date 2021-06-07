use log::error;
use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

use capnp::message::Builder;
use capnp::serialize;
use sbp::messages::system::MsgInsUpdates;
use std::thread;
use std::{
    ops::Deref,
    sync::{mpsc, Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::errors::{
    CAP_N_PROTO_SERIALIZATION_FAILURE, GET_MUT_OBJECT_FAILURE, UPDATE_STATUS_LOCK_MUTEX_FAILURE,
};
use crate::types::{IsRunning, MessageSender, Result, SharedState};

const STATUS_PERIOD: u64 = 1;

#[derive(Debug)]
pub struct UpdateStatus(Arc<Mutex<Option<UpdateStatusInner>>>);
impl UpdateStatus {
    fn new(update_status: Option<UpdateStatusInner>) -> UpdateStatus {
        UpdateStatus(Arc::new(Mutex::new(update_status)))
    }
    fn get(&mut self) -> Option<UpdateStatusInner> {
        let update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        (*update_status).clone()
    }
    fn take(&mut self) -> Option<UpdateStatusInner> {
        let mut update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        let inner = *update_status;
        (*update_status) = None;
        inner
    }
    fn set(&mut self, status: UpdateStatusInner) {
        let mut update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        (*update_status) = Some(status);
    }
}

impl Deref for UpdateStatus {
    type Target = Mutex<Option<UpdateStatusInner>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self::new(Some(UpdateStatusInner::Unknown))
    }
}

impl Clone for UpdateStatus {
    fn clone(&self) -> Self {
        UpdateStatus {
            0: Arc::clone(&self.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateStatusInner {
    Unknown,
    Warning,
    Ok,
}
impl UpdateStatusInner {
    fn from(status: u8) -> UpdateStatusInner {
        if status & 0x0f != 0 {
            UpdateStatusInner::Warning
        } else if status & 0xf0 != 0 {
            UpdateStatusInner::Ok
        } else {
            UpdateStatusInner::Unknown
        }
    }
}

struct StatusTimer {
    is_running: IsRunning,
    handle: Option<JoinHandle<()>>,
}
impl StatusTimer {
    fn new() -> StatusTimer {
        StatusTimer {
            handle: None,
            is_running: IsRunning::new(),
        }
    }
    fn restart(&mut self, storage: UpdateStatus, value: UpdateStatusInner, delay: u64) {
        self.cancel();
        self.is_running.set(true);
        self.handle = Some(StatusTimer::timer_thread(
            self.is_running.clone(),
            storage,
            value,
            delay,
        ));
    }
    fn timer_thread(
        is_running: IsRunning,
        storage: UpdateStatus,
        value: UpdateStatusInner,
        delay: u64,
    ) -> JoinHandle<()> {
        let mut storage = storage.clone();
        spawn(move || {
            sleep(Duration::from_secs(delay));
            storage.set(value);
            is_running.set(false);
        })
    }
    fn active(&mut self) -> bool {
        self.is_running.get()
    }

    fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            drop(handle);
        }
        self.is_running.set(false);
    }
}

#[derive(Debug)]
struct FlagStatus {
    last_status: UpdateStatus,
    status: UpdateStatus,
    incoming: UpdateStatus,
    is_running: IsRunning,
    handle: Option<JoinHandle<()>>,
}

impl FlagStatus {
    fn new() -> FlagStatus {
        let mut flag_status = FlagStatus {
            last_status: UpdateStatus::default(),
            status: UpdateStatus::default(),
            incoming: UpdateStatus::new(None),
            is_running: IsRunning::new(),
            handle: None,
        };
        flag_status.set_status_thread();
        flag_status
    }

    fn set_status_thread(&mut self) {
        self.is_running.set(true);
        let mut incoming_clone = self.incoming.clone();
        let mut last_status_clone = self.last_status.clone();
        let mut status_clone = self.status.clone();
        let mut warning_timer = StatusTimer::new();
        let mut unknown_timer = StatusTimer::new();
        let is_running = self.is_running.clone();

        let handle = spawn(move || loop {
            if !is_running.get() {
                break;
            }
            if let Some(status) = incoming_clone.take() {
                last_status_clone.set(status);

                match status {
                    UpdateStatusInner::Warning => {
                        status_clone.set(status);
                        warning_timer.restart(last_status_clone.clone(), status, STATUS_PERIOD);
                        unknown_timer.restart(
                            incoming_clone.clone(),
                            UpdateStatusInner::Unknown,
                            STATUS_PERIOD,
                        );
                    }
                    UpdateStatusInner::Unknown => {
                        if warning_timer.active() {
                            warning_timer.cancel();
                            if let Some(last_status) = last_status_clone.get() {
                                incoming_clone.set(last_status);
                            }
                        }
                        status_clone.set(status);
                    }
                    UpdateStatusInner::Ok => {
                        if !warning_timer.active() {
                            status_clone.set(status);
                        }
                        unknown_timer.restart(
                            incoming_clone.clone(),
                            UpdateStatusInner::Unknown,
                            STATUS_PERIOD,
                        );
                    }
                }
            }
            thread::sleep(Duration::from_secs_f64(0.05));
            if let Some(sc) = status_clone.get() {
                println!("{:?}", sc);
            }
        });
        self.handle = Some(handle);
    }

    fn stop(&mut self) {
        self.is_running.set(false);
    }

    fn update_status(&mut self, status: u8) {
        let status = UpdateStatusInner::from(status);
        match status {
            UpdateStatusInner::Ok | UpdateStatusInner::Warning => {
                self.incoming.set(status);
            }
            _ => {}
        }
    }
}

/// FusionEngineStatus struct.
///
/// # Fields:
///
/// - `available_units` - The available units of measure to send to frontend for selection.
/// - `colors`: Stored rgb codes for frontend correspond to index of sv_labels.
/// - `max`: Stored maximum measure of unit used for frontend plot.
/// - `min`: Stored minimum measure of unit used for frontend plot.
/// - `multiplier`: The current multiplier used to modify points accounting for unit of measure.
/// - `points`: The horizontal and vertical velocity points of size, NUM_POINTS, to be sent to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `tow`: The GPS Time of Week.
/// - `unit`: Currently displayed and converted to unit of measure.
#[derive(Debug)]
pub struct FusionEngineStatus<S: MessageSender> {
    client_sender: S,
    shared_state: SharedState,
    gnssvel: FlagStatus,
}

impl<S: MessageSender> FusionEngineStatus<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> FusionEngineStatus<S> {
        FusionEngineStatus {
            client_sender,
            shared_state,
            gnssvel: FlagStatus::new(),
        }
    }

    /// Handle INS Updates messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsUpdates to extract data from.
    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        // println!("{:?}", msg);
        self.gnssvel.update_status(msg.gnssvel);
        // let tic = msg.wheelticks;
        // if ((tic & 0xF0) >> 4) > (tic & 0x0F) {
        //     let last_odo_update_time = Instant::now();
        //     let mut shared_data = self
        //         .heartbeat_data
        //         .lock()
        //         .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
        //     (*shared_data).last_odo_update_time = Some(last_odo_update_time);
        // }
        // self.send_data();
    }
}

impl<S: MessageSender> Drop for FusionEngineStatus<S> {
    fn drop(&mut self) {
        self.gnssvel.stop();
    }
}
