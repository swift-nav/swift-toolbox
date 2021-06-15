use capnp::message::Builder;
use capnp::serialize;
use crossbeam::channel::{after, select, unbounded, Receiver, Sender, TrySendError};
use log::error;
use sbp::messages::system::MsgInsUpdates;
use std::{
    fmt,
    ops::Deref,
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    time::Duration,
};

use crate::console_backend_capnp as m;
use crate::errors::{
    CAP_N_PROTO_SERIALIZATION_FAILURE, THREAD_JOIN_FAILURE, UNABLE_TO_SEND_INS_UPDATE_FAILURE,
    UNABLE_TO_STOP_TIMER_THREAD_FAILURE, UPDATE_STATUS_LOCK_MUTEX_FAILURE,
};
use crate::types::IsRunning;
use crate::types::{MessageSender, SharedState};

const STATUS_PERIOD: f64 = 1.0;
const SET_STATUS_THREAD_SLEEP_SEC: f64 = 0.25;

// No updates have been attempted in the past `STATUS_PERIOD`
const UNKNOWN: &str = "\u{2B1B}"; // Unicode Character “⬛” (U+2B1B)
                                  // There have been attempted updates in the past `STATUS_PERIOD` but at least one was rejected
const WARNING: &str = "\u{26A0}"; // Unicode Character “⚠” (U+26A0)
                                  // There have been updates in the past `STATUS_PERIOD` and none were rejected
const OK: &str = "\u{26AB}"; // Unicode Character “⚫” (U+26AB)

#[derive(Debug)]
pub struct UpdateStatus(Arc<Mutex<UpdateStatusInner>>);
impl UpdateStatus {
    fn new(update_status: UpdateStatusInner) -> UpdateStatus {
        UpdateStatus(Arc::new(Mutex::new(update_status)))
    }
    fn get(&mut self) -> UpdateStatusInner {
        let update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        *update_status
    }
    fn set(&mut self, status: UpdateStatusInner) {
        let mut update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        (*update_status) = status;
    }
}

impl Deref for UpdateStatus {
    type Target = Mutex<UpdateStatusInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self::new(UpdateStatusInner::Unknown)
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
    running_sender: Option<Sender<bool>>,
    handle: Option<JoinHandle<()>>,
}
impl StatusTimer {
    fn new() -> StatusTimer {
        StatusTimer {
            handle: None,
            is_running: IsRunning::new(),
            running_sender: None,
        }
    }
    fn restart(
        &mut self,
        msg_sender: Sender<Option<UpdateStatusInner>>,
        value: UpdateStatusInner,
        delay: f64,
    ) {
        self.cancel();
        let (running_sender, running_receiver) = unbounded::<bool>();
        self.running_sender = Some(running_sender);
        self.is_running.set(true);
        self.handle = Some(StatusTimer::timer_thread(
            self.is_running.clone(),
            running_receiver,
            msg_sender,
            value,
            delay,
        ));
    }
    fn timer_thread(
        is_running: IsRunning,
        running_receiver: Receiver<bool>,
        msg_sender: Sender<Option<UpdateStatusInner>>,
        value: UpdateStatusInner,
        delay: f64,
    ) -> JoinHandle<()> {
        spawn(move || {
            select! {
                recv(after(Duration::from_secs_f64(delay))) -> _ => {
                    msg_sender.try_send(Some(value)).expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
                },
                recv(running_receiver) -> _ => (),
            }
            is_running.set(false);
        })
    }
    fn active(&mut self) -> bool {
        self.is_running.get()
    }

    fn cancel(&mut self) {
        if let Some(running_sender) = self.running_sender.take() {
            if let Err(err) = running_sender.try_send(false) {
                match err {
                    TrySendError::Disconnected(_) => (),
                    _ => error!("Issue cancelling timer, {}.", err),
                }
            }
        }
        if let Some(handle) = self.handle.take() {
            handle.join().expect(UNABLE_TO_STOP_TIMER_THREAD_FAILURE);
        }
    }
}

#[derive(Debug)]
struct FlagStatus {
    status: UpdateStatus,
    sender: Sender<Option<UpdateStatusInner>>,
    handle: Option<JoinHandle<()>>,
}

impl FlagStatus {
    fn new() -> FlagStatus {
        let (sender, receiver) = unbounded();
        let status = UpdateStatus::default();
        FlagStatus {
            status: status.clone(),
            sender: sender.clone(),
            handle: Some(FlagStatus::set_status_thread(status, sender, receiver)),
        }
    }

    fn set_status_thread(
        mut status: UpdateStatus,
        sender: Sender<Option<UpdateStatusInner>>,
        receiver: Receiver<Option<UpdateStatusInner>>,
    ) -> JoinHandle<()> {
        spawn(move || {
            let mut last_status = UpdateStatus::default();
            let mut warning_timer = StatusTimer::new();
            let mut unknown_timer = StatusTimer::new();
            let mut is_running = true;
            while is_running {
                if let Ok(status_option) =
                    receiver.recv_timeout(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC))
                {
                    let sender_clone = sender.clone();
                    match status_option {
                        Some(new_status) => {
                            last_status.set(new_status);

                            match new_status {
                                UpdateStatusInner::Warning => {
                                    status.set(new_status);
                                    warning_timer.restart(
                                        sender_clone.clone(),
                                        new_status,
                                        STATUS_PERIOD,
                                    );
                                    unknown_timer.restart(
                                        sender_clone,
                                        UpdateStatusInner::Unknown,
                                        STATUS_PERIOD,
                                    );
                                }
                                UpdateStatusInner::Unknown => {
                                    if warning_timer.active() {
                                        warning_timer.cancel();
                                        sender_clone
                                            .try_send(Some(last_status.get()))
                                            .expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
                                    }
                                    status.set(new_status);
                                }
                                UpdateStatusInner::Ok => {
                                    if !warning_timer.active() {
                                        status.set(new_status);
                                    }
                                    unknown_timer.restart(
                                        sender_clone,
                                        UpdateStatusInner::Unknown,
                                        STATUS_PERIOD,
                                    );
                                }
                            }
                        }
                        None => {
                            is_running = false;
                        }
                    }
                }
            }
        })
    }
    fn status(&mut self) -> UpdateStatusInner {
        self.status.get()
    }

    fn stop(&mut self) {
        self.sender
            .clone()
            .try_send(None)
            .expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
        if let Some(handle) = self.handle.take() {
            handle.join().expect(THREAD_JOIN_FAILURE);
        }
    }

    fn update_status(&mut self, status: u8) {
        let status = UpdateStatusInner::from(status);
        match status {
            UpdateStatusInner::Ok | UpdateStatusInner::Warning => {
                self.sender
                    .try_send(Some(status))
                    .expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
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
/// - `gnsspos`: Storage for the GNSS Position status.
/// - `gnssvel`: Storage for the GNSS Velocity status.
/// - `wheelticks`: Storage for the wheel ticks status.
/// - `speed`: Storage for the wheel speed status.
/// - `nhc`: Storage for the non-holonomic constraints model status.
/// - `zerovel`: Storage for the zero velocity status.
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
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };
    const SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN: f64 = 1.1;
    const TIMEOUT_SWITCHING_TO_UNKNOWN_AFTER_TIMEOUT_SEC: f64 = 5.0;

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

        let (sender, receiver) = unbounded();
        let update_status_inner = UpdateStatusInner::Warning;

        assert!(!status_timer.active());
        assert!(receiver.is_empty());
        status_timer.restart(sender, update_status_inner, STATUS_PERIOD);

        assert!(status_timer.active());
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(
            STATUS_PERIOD * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));

        assert!(!status_timer.active());
        assert_eq!(receiver.recv().unwrap().unwrap(), update_status_inner);
    }

    #[test]
    fn status_timer_restart_active_test() {
        let mut status_timer = StatusTimer::new();

        let (sender, receiver) = unbounded();

        let update_status_inner = UpdateStatusInner::Warning;

        assert!(!status_timer.active());
        assert!(receiver.is_empty());
        status_timer.restart(sender.clone(), update_status_inner, STATUS_PERIOD);
        assert!(status_timer.active());
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.5));
        status_timer.restart(sender, update_status_inner, STATUS_PERIOD);
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.75));
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.26));
        assert_eq!(receiver.recv().unwrap().unwrap(), update_status_inner);
    }

    #[test]
    fn flag_status_update_status_test() {
        let mut flag_status = FlagStatus::new();
        let ok_status = 0b00010000;
        assert!(flag_status.sender.is_empty());
        flag_status.update_status(ok_status);
        assert!(!flag_status.sender.is_empty());
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert!(flag_status.sender.is_empty());
    }

    #[test]
    fn flag_status_stop_test() {
        let mut flag_status = FlagStatus::new();
        assert!(flag_status.handle.is_some());
        {
            flag_status.stop();
        }
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert!(flag_status.handle.is_none());
    }

    #[test]
    fn flag_status_warning_test() {
        let mut flag_status = FlagStatus::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        // Let warning message initiate and expire to go back to unknown.
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Unknown);
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        // Update_status for an unknown should not change the result.
        flag_status.update_status(unknown_status);
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Warning);
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        let now = Instant::now();
        while flag_status.status.get() != UpdateStatusInner::Unknown {
            sleep(Duration::from_millis(10));
            if (Instant::now() - now).as_secs_f64() > TIMEOUT_SWITCHING_TO_UNKNOWN_AFTER_TIMEOUT_SEC
            {
                break;
            }
        }
        assert_eq!(flag_status.status.get(), UpdateStatusInner::Unknown);
    }

    #[test]
    fn flag_status_ok_test() {
        let mut flag_status = FlagStatus::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        let ok_status = 0b00010000;

        // Ok message then initiate warning, send another ok which should not take, then unknown kicks in.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Ok);
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Warning);
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Warning);
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        let now = Instant::now();
        while flag_status.status.get() != UpdateStatusInner::Unknown {
            sleep(Duration::from_millis(10));
            if (Instant::now() - now).as_secs_f64() > TIMEOUT_SWITCHING_TO_UNKNOWN_AFTER_TIMEOUT_SEC
            {
                break;
            }
        }

        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Unknown);

        // Ok message with no warning active should set status ok then after expires goes unknown.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), UpdateStatusInner::Ok);
        flag_status.update_status(unknown_status);
        sleep(Duration::from_secs_f64(
            STATUS_PERIOD * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.get(), UpdateStatusInner::Unknown);
    }
}
