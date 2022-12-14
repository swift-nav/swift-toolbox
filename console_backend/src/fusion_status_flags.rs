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

use capnp::message::Builder;
use crossbeam::channel::{after, select, unbounded, Receiver, Sender, TrySendError};
use log::error;
use sbp::messages::system::MsgInsUpdates;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    time::Duration,
};

use crate::client_sender::BoxedClientSender;
use crate::common_constants as cc;
use crate::errors::{
    THREAD_JOIN_FAILURE, UNABLE_TO_SEND_INS_UPDATE_FAILURE, UNABLE_TO_STOP_TIMER_THREAD_FAILURE,
    UPDATE_STATUS_LOCK_MUTEX_FAILURE,
};
use crate::types::ArcBool;
use crate::utils::serialize_capnproto_builder;

const STATUS_PERIOD: f64 = 1.0;
const SET_STATUS_THREAD_SLEEP_SEC: f64 = 0.25;

#[derive(Debug)]
pub struct FusionStatus(Arc<Mutex<FusionStatusInner>>);
impl FusionStatus {
    fn new(update_status: FusionStatusInner) -> FusionStatus {
        FusionStatus(Arc::new(Mutex::new(update_status)))
    }
    fn get(&mut self) -> FusionStatusInner {
        let update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        (*update_status).clone()
    }
    fn set(&mut self, status: FusionStatusInner) {
        let mut update_status = self.lock().expect(UPDATE_STATUS_LOCK_MUTEX_FAILURE);
        (*update_status) = status;
    }
}

impl Deref for FusionStatus {
    type Target = Mutex<FusionStatusInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for FusionStatus {
    fn default() -> Self {
        Self::new(FusionStatusInner::UNKNOWN)
    }
}

impl Clone for FusionStatus {
    fn clone(&self) -> Self {
        FusionStatus(Arc::clone(&self.0))
    }
}

pub type FusionStatusInner = cc::FusionStatus;
impl FusionStatusInner {
    fn from(status: u8) -> FusionStatusInner {
        if status & 0x0f != 0 {
            FusionStatusInner::WARNING
        } else if status & 0xf0 != 0 {
            FusionStatusInner::OK
        } else {
            FusionStatusInner::UNKNOWN
        }
    }
}

struct StatusTimer {
    is_running: ArcBool,
    running_sender: Option<Sender<bool>>,
    handle: Option<JoinHandle<()>>,
}
impl StatusTimer {
    fn new() -> StatusTimer {
        StatusTimer {
            handle: None,
            is_running: ArcBool::new(),
            running_sender: None,
        }
    }
    fn restart(
        &mut self,
        msg_sender: Sender<Option<FusionStatusInner>>,
        value: FusionStatusInner,
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
        is_running: ArcBool,
        running_receiver: Receiver<bool>,
        msg_sender: Sender<Option<FusionStatusInner>>,
        value: FusionStatusInner,
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
                    _ => error!("Issue cancelling timer, {err}."),
                }
            }
        }
        if let Some(handle) = self.handle.take() {
            handle.join().expect(UNABLE_TO_STOP_TIMER_THREAD_FAILURE);
        }
    }
}

#[derive(Debug)]
struct FusionStatusFlag {
    status: FusionStatus,
    sender: Sender<Option<FusionStatusInner>>,
    handle: Option<JoinHandle<()>>,
}

impl FusionStatusFlag {
    fn new() -> FusionStatusFlag {
        let (sender, receiver) = unbounded();
        let status = FusionStatus::default();
        FusionStatusFlag {
            status: status.clone(),
            sender: sender.clone(),
            handle: Some(FusionStatusFlag::set_status_thread(
                status, sender, receiver,
            )),
        }
    }

    fn set_status_thread(
        mut status: FusionStatus,
        sender: Sender<Option<FusionStatusInner>>,
        receiver: Receiver<Option<FusionStatusInner>>,
    ) -> JoinHandle<()> {
        spawn(move || {
            let mut last_status = FusionStatus::default();
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
                            last_status.set(new_status.clone());

                            match new_status {
                                FusionStatusInner::WARNING => {
                                    status.set(new_status.clone());
                                    warning_timer.restart(
                                        sender_clone.clone(),
                                        new_status,
                                        STATUS_PERIOD,
                                    );
                                    unknown_timer.restart(
                                        sender_clone,
                                        FusionStatusInner::UNKNOWN,
                                        STATUS_PERIOD,
                                    );
                                }
                                FusionStatusInner::UNKNOWN => {
                                    if warning_timer.active() {
                                        warning_timer.cancel();
                                        sender_clone
                                            .try_send(Some(last_status.get()))
                                            .expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
                                    }
                                    status.set(new_status);
                                }
                                FusionStatusInner::OK => {
                                    if !warning_timer.active() {
                                        status.set(new_status);
                                    }
                                    unknown_timer.restart(
                                        sender_clone,
                                        FusionStatusInner::UNKNOWN,
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
    fn status(&mut self) -> FusionStatusInner {
        self.status.get()
    }

    fn stop(&mut self) {
        if let Err(err) = self.sender.try_send(None) {
            match err {
                TrySendError::Disconnected(_) => (),
                _ => error!("Issue stopping timer, {}.", err),
            }
        }
        if let Some(handle) = self.handle.take() {
            handle.join().expect(THREAD_JOIN_FAILURE);
        }
    }

    fn update_status(&mut self, status: u8) {
        let status = FusionStatusInner::from(status);
        match status {
            FusionStatusInner::OK | FusionStatusInner::WARNING => {
                self.sender
                    .try_send(Some(status))
                    .expect(UNABLE_TO_SEND_INS_UPDATE_FAILURE);
            }
            _ => {}
        }
    }
}

impl Drop for FusionStatusFlag {
    fn drop(&mut self) {
        self.stop();
    }
}

/// FusionStatusFlags struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `gnsspos`: Storage for the GNSS Position status.
/// - `gnssvel`: Storage for the GNSS Velocity status.
/// - `wheelticks`: Storage for the wheel ticks status.
/// - `speed`: Storage for the wheel speed status.
/// - `nhc`: Storage for the non-holonomic constraints model status.
/// - `zerovel`: Storage for the zero velocity status.
#[derive(Debug)]
pub struct FusionStatusFlags {
    client_sender: BoxedClientSender,
    gnsspos: FusionStatusFlag,
    gnssvel: FusionStatusFlag,
    wheelticks: FusionStatusFlag,
    speed: FusionStatusFlag,
    nhc: FusionStatusFlag,
    zerovel: FusionStatusFlag,
}

impl FusionStatusFlags {
    pub fn new(client_sender: BoxedClientSender) -> FusionStatusFlags {
        FusionStatusFlags {
            client_sender,
            gnsspos: FusionStatusFlag::new(),
            gnssvel: FusionStatusFlag::new(),
            wheelticks: FusionStatusFlag::new(),
            speed: FusionStatusFlag::new(),
            nhc: FusionStatusFlag::new(),
            zerovel: FusionStatusFlag::new(),
        }
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
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_fusion_status_flags_status();

        tab_status.set_gnsspos(&self.gnsspos.status().to_string());
        tab_status.set_gnssvel(&self.gnssvel.status().to_string());
        tab_status.set_wheelticks(&self.wheelticks.status().to_string());
        tab_status.set_speed(&self.speed.status().to_string());
        tab_status.set_nhc(&self.nhc.status().to_string());
        tab_status.set_zerovel(&self.zerovel.status().to_string());

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
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
            FusionStatusInner::from(0b00000000),
            FusionStatusInner::UNKNOWN
        );
        assert_eq!(
            FusionStatusInner::from(0b00000001),
            FusionStatusInner::WARNING
        );
        assert_eq!(
            FusionStatusInner::from(0b00000010),
            FusionStatusInner::WARNING
        );
        assert_eq!(
            FusionStatusInner::from(0b00000100),
            FusionStatusInner::WARNING
        );
        assert_eq!(
            FusionStatusInner::from(0b00001000),
            FusionStatusInner::WARNING
        );
        assert_eq!(
            FusionStatusInner::from(0b00001001),
            FusionStatusInner::WARNING
        );
        assert_eq!(FusionStatusInner::from(0b00010000), FusionStatusInner::OK);
        assert_eq!(FusionStatusInner::from(0b00100000), FusionStatusInner::OK);
        assert_eq!(FusionStatusInner::from(0b01000000), FusionStatusInner::OK);
        assert_eq!(FusionStatusInner::from(0b10000000), FusionStatusInner::OK);
        assert_eq!(FusionStatusInner::from(0b01010000), FusionStatusInner::OK);
    }

    #[test]
    fn status_timer_new_test() {
        let mut status_timer = StatusTimer::new();

        let (sender, receiver) = unbounded();
        let update_status_inner = FusionStatusInner::WARNING;

        assert!(!status_timer.active());
        assert!(receiver.is_empty());
        status_timer.restart(sender, update_status_inner.clone(), STATUS_PERIOD);

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

        let update_status_inner = FusionStatusInner::WARNING;

        assert!(!status_timer.active());
        assert!(receiver.is_empty());
        status_timer.restart(sender.clone(), update_status_inner.clone(), STATUS_PERIOD);
        assert!(status_timer.active());
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.5));
        status_timer.restart(sender, update_status_inner.clone(), STATUS_PERIOD);
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.75));
        assert!(receiver.is_empty());
        sleep(Duration::from_secs_f64(STATUS_PERIOD as f64 * 0.26));
        assert_eq!(receiver.recv().unwrap().unwrap(), update_status_inner);
    }

    #[test]
    fn flag_status_update_status_test() {
        let mut flag_status = FusionStatusFlag::new();
        let ok_status = 0b00010000;
        assert!(flag_status.sender.is_empty());
        flag_status.update_status(ok_status);
        assert!(!flag_status.sender.is_empty());
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert!(flag_status.sender.is_empty());
    }

    #[test]
    fn flag_status_stop_test() {
        let mut flag_status = FusionStatusFlag::new();
        assert!(flag_status.handle.is_some());
        {
            flag_status.stop();
        }
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert!(flag_status.handle.is_none());
    }

    #[test]
    fn flag_status_warning_test() {
        let mut flag_status = FusionStatusFlag::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        // Let warning message initiate and expire to go back to unknown.
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::UNKNOWN);
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        // Update_status for an unknown should not change the result.
        flag_status.update_status(unknown_status);
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::WARNING);
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        let now = Instant::now();
        while flag_status.status.get() != FusionStatusInner::UNKNOWN {
            sleep(Duration::from_millis(10));
            if (Instant::now() - now).as_secs_f64() > TIMEOUT_SWITCHING_TO_UNKNOWN_AFTER_TIMEOUT_SEC
            {
                break;
            }
        }
        assert_eq!(flag_status.status.get(), FusionStatusInner::UNKNOWN);
    }

    #[test]
    fn flag_status_ok_test() {
        let mut flag_status = FusionStatusFlag::new();
        let unknown_status = 0b00000000;
        let warning_status = 0b00000001;
        let ok_status = 0b00010000;

        // Ok message then initiate warning, send another ok which should not take, then unknown kicks in.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::OK);
        flag_status.update_status(warning_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::WARNING);
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(SET_STATUS_THREAD_SLEEP_SEC));
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::WARNING);
        sleep(Duration::from_secs_f64(STATUS_PERIOD));
        let now = Instant::now();
        while flag_status.status.get() != FusionStatusInner::UNKNOWN {
            sleep(Duration::from_millis(10));
            if (Instant::now() - now).as_secs_f64() > TIMEOUT_SWITCHING_TO_UNKNOWN_AFTER_TIMEOUT_SEC
            {
                break;
            }
        }

        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::UNKNOWN);

        // Ok message with no warning active should set status ok then after expires goes unknown.
        flag_status.update_status(ok_status);
        sleep(Duration::from_secs_f64(
            SET_STATUS_THREAD_SLEEP_SEC * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.clone().get(), FusionStatusInner::OK);
        flag_status.update_status(unknown_status);
        sleep(Duration::from_secs_f64(
            STATUS_PERIOD * SLEEP_BUFFER_MULT_WITH_ERROR_MARGIN,
        ));
        assert_eq!(flag_status.status.get(), FusionStatusInner::UNKNOWN);
    }
}
