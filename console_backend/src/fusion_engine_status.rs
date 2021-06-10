use capnp::message::Builder;
use capnp::serialize;
use sbp::messages::system::MsgInsUpdates;
use std::{
    fmt,
    ops::Deref,
    sync::{Arc, Mutex},
    thread::{self, sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use crate::console_backend_capnp as m;
use crate::errors::{
    CAP_N_PROTO_SERIALIZATION_FAILURE, GET_FUSION_ENGINE_STATUS_FAILURE,
    UPDATE_STATUS_LOCK_MUTEX_FAILURE,
};
use crate::types::{IsRunning, MessageSender, SharedState};

const STATUS_PERIOD: f64 = 1.0;
const SET_STATUS_THREAD_SLEEP_SEC: f64 = 0.05;

// No updates have been attempted in the past `STATUS_PERIOD`
const UNKNOWN: &str = "\u{2B1B}"; // Unicode Character “⬛” (U+2B1B)
                                  // There have been attempted updates in the past `STATUS_PERIOD` but at least one was rejected
const WARNING: &str = "\u{26A0}"; // Unicode Character “⚠” (U+26A0)
                                  // There have been updates in the past `STATUS_PERIOD` and none were rejected
const OK: &str = "\u{26AB}"; // Unicode Character “⚫” (U+26AB)

#[derive(Debug)]
pub struct UpdateStatus(Arc<Mutex<Option<UpdateStatusInner>>>);
impl UpdateStatus {
    fn new(update_status: Option<UpdateStatusInner>) -> UpdateStatus {
        UpdateStatus(Arc::new(Mutex::new(update_status)))
    }
    fn get(&mut self) -> Option<UpdateStatusInner> {
        let update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        *update_status
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
impl fmt::Display for UpdateStatusInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_emoji = match self {
            UpdateStatusInner::Unknown => UNKNOWN,
            UpdateStatusInner::Warning => WARNING,
            UpdateStatusInner::Ok => OK,
        };
        write!(f, "{}", status_emoji)
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
    fn restart(&mut self, storage: UpdateStatus, value: UpdateStatusInner, delay: f64) {
        self.cancel();
        sleep(Duration::from_millis(5));
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
        delay: f64,
    ) -> JoinHandle<()> {
        let start_time = Instant::now();
        let mut storage = storage;
        is_running.set(true);
        spawn(move || {
            let mut expired = false;
            while is_running.get() {
                if (Instant::now() - start_time).as_secs_f64() > delay {
                    expired = true;
                    break;
                }
                sleep(Duration::from_millis(5));
            }
            if expired {
                storage.set(value);
            }
        })
    }
    fn active(&mut self) -> bool {
        self.is_running.get()
    }

    fn cancel(&mut self) {
        self.is_running.set(false);
        if let Some(handle) = self.handle.take() {
            drop(handle);
        }
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
            thread::sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        });
        self.handle = Some(handle);
    }
    fn status(&mut self) -> UpdateStatusInner {
        self.status.get().expect(GET_FUSION_ENGINE_STATUS_FAILURE)
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
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `gnsspos`: Storage for the Imu configuration.
/// - `gnssvel`: Storage for the Imu configuration.
/// - `wheelticks`: Storage for the Imu configuration.
/// - `speed`: Storage for the Imu configuration.
/// - `nhc`: Storage for the Imu configuration.
/// - `zerovel`: Storage for the Imu configuration.
#[derive(Debug)]
pub struct FusionEngineStatus<S: MessageSender> {
    client_sender: S,
    shared_state: SharedState,
    gnsspos: FlagStatus,
    gnssvel: FlagStatus,
    wheelticks: FlagStatus,
    speed: FlagStatus,
    nhc: FlagStatus,
    zerovel: FlagStatus,
}

impl<S: MessageSender> FusionEngineStatus<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> FusionEngineStatus<S> {
        let mut fusion_engine_status = FusionEngineStatus {
            client_sender,
            shared_state,
            gnsspos: FlagStatus::new(),
            gnssvel: FlagStatus::new(),
            wheelticks: FlagStatus::new(),
            speed: FlagStatus::new(),
            nhc: FlagStatus::new(),
            zerovel: FlagStatus::new(),
        };
        fusion_engine_status.send_data();
        fusion_engine_status
    }

    /// Handle INS Updates messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsUpdates to extract data from.
    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        self.gnsspos.update_status(msg.gnsspos);
        self.gnssvel.update_status(msg.gnssvel);
        self.wheelticks.update_status(msg.wheelticks);
        self.speed.update_status(msg.speed);
        self.nhc.update_status(msg.nhc);
        self.zerovel.update_status(msg.zerovel);
        self.send_data();
    }

    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let mut tab_status = msg.init_fusion_engine_status();

        tab_status.set_gnsspos(&self.gnsspos.status().to_string());
        tab_status.set_gnssvel(&self.gnssvel.status().to_string());
        tab_status.set_wheelticks(&self.wheelticks.status().to_string());
        tab_status.set_speed(&self.speed.status().to_string());
        tab_status.set_nhc(&self.nhc.status().to_string());
        tab_status.set_zerovel(&self.zerovel.status().to_string());

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder)
            .expect(CAP_N_PROTO_SERIALIZATION_FAILURE);
        self.client_sender.send_data(msg_bytes);
    }
}

impl<S: MessageSender> Drop for FusionEngineStatus<S> {
    fn drop(&mut self) {
        self.gnsspos.stop();
        self.gnssvel.stop();
        self.wheelticks.stop();
        self.speed.stop();
        self.nhc.stop();
        self.zerovel.stop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{thread::sleep, time::Duration};
    const DELAY: f64 = 1.0;

    #[test]
    fn update_status_inner_test() {
        assert_eq!(
            UpdateStatusInner::from(0b00000000),
            UpdateStatusInner::Unknown
        );
        assert_eq!(
            UpdateStatusInner::from(0b00000001),
            UpdateStatusInner::Warning
        );
        assert_eq!(
            UpdateStatusInner::from(0b00000010),
            UpdateStatusInner::Warning
        );
        assert_eq!(
            UpdateStatusInner::from(0b00000100),
            UpdateStatusInner::Warning
        );
        assert_eq!(
            UpdateStatusInner::from(0b00001000),
            UpdateStatusInner::Warning
        );
        assert_eq!(
            UpdateStatusInner::from(0b00001001),
            UpdateStatusInner::Warning
        );
        assert_eq!(UpdateStatusInner::from(0b00010000), UpdateStatusInner::Ok);
        assert_eq!(UpdateStatusInner::from(0b00100000), UpdateStatusInner::Ok);
        assert_eq!(UpdateStatusInner::from(0b01000000), UpdateStatusInner::Ok);
        assert_eq!(UpdateStatusInner::from(0b10000000), UpdateStatusInner::Ok);
        assert_eq!(UpdateStatusInner::from(0b01010000), UpdateStatusInner::Ok);
    }

    #[test]
    fn status_timer_new_test() {
        let mut status_timer = StatusTimer::new();

        let mut update_status = UpdateStatus::default();

        let update_status_inner = UpdateStatusInner::Warning;

        assert!(!status_timer.active());
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        status_timer.restart(update_status.clone(), update_status_inner, DELAY);
        assert!(status_timer.active());
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        sleep(Duration::from_secs_f64(DELAY as f64 * 1.01_f64));
        assert!(!status_timer.active());
        assert_eq!(update_status.get().unwrap(), update_status_inner);
    }

    #[test]
    fn status_timer_restart_active_test() {
        let mut status_timer = StatusTimer::new();

        let mut update_status = UpdateStatus::default();

        let update_status_inner = UpdateStatusInner::Warning;

        assert!(!status_timer.active());
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        status_timer.restart(update_status.clone(), update_status_inner, DELAY);
        assert!(status_timer.active());
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        sleep(Duration::from_secs_f64(DELAY as f64 * 0.5));
        status_timer.restart(update_status.clone(), update_status_inner, DELAY);
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        sleep(Duration::from_secs_f64(DELAY as f64 * 0.75));
        assert_eq!(
            update_status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        sleep(Duration::from_secs_f64(DELAY as f64 * 0.26));
        assert_eq!(update_status.get().unwrap(), update_status_inner);
    }

    #[test]
    fn flag_status_update_status_test() {
        let mut flag_status = FlagStatus::new();
        let ok_status = 0b00010000;
        assert_eq!(flag_status.incoming.clone().get(), None);
        flag_status.update_status(ok_status);
        assert_eq!(flag_status.incoming.get().unwrap(), UpdateStatusInner::Ok);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert_eq!(flag_status.incoming.get(), None);
    }

    #[test]
    fn flag_status_stop_test() {
        let mut flag_status = FlagStatus::new();
        assert!(flag_status.is_running.get());
        {
            flag_status.stop();
        }
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert!(!flag_status.is_running.get());
    }

    #[test]
    fn flag_status_warning_test() {
        let mut flag_status = FlagStatus::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        // Let warning message initiate and expire to go back to unknown.
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        // Update_status for an unknown should not change the result.
        flag_status.update_status(unknown_status);
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Warning
        );
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        assert_eq!(
            flag_status.status.get().unwrap(),
            UpdateStatusInner::Unknown
        );
    }

    #[test]
    fn flag_status_ok_test() {
        let mut flag_status = FlagStatus::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        let ok_status = 0b00010000;

        // Ok message then initiate warning, send another ok which should not take, then unknown kicks in.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Ok
        );
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Warning
        );
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Warning
        );
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Unknown
        );

        // Ok message with no warning active should set status ok then after expires goes unknown.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC * 1.1));
        assert_eq!(
            flag_status.status.clone().get().unwrap(),
            UpdateStatusInner::Ok
        );
        flag_status.update_status(unknown_status);
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        assert_eq!(
            flag_status.status.get().unwrap(),
            UpdateStatusInner::Unknown
        );
    }
}
