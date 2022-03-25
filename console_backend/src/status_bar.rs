use capnp::message::Builder;

use crossbeam::channel::{self, select, Receiver, Sender};
use sbp::messages::{
    navigation::MsgAgeCorrections,
    system::{MsgHeartbeat, MsgInsStatus, MsgInsUpdates},
};

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    time::{Duration, Instant},
};

use crate::client_sender::BoxedClientSender;
use crate::constants::*;
use crate::errors::*;
use crate::piksi_tools_constants::{
    ins_error_dict, ins_mode_dict, ins_type_dict, rtk_mode_dict, DR_MODE, EMPTY_STR, RTK_MODES,
};
use crate::shared_state::SharedState;
use crate::types::{BaselineNED, GnssModes, PosLLH};
use crate::utils::{bytes_to_kb, decisec_to_sec, serialize_capnproto_builder};

enum AntennaStatus {
    Short,
    Ok,
    Open,
}
impl From<u32> for AntennaStatus {
    fn from(status: u32) -> Self {
        if status & 0x40000000 != 0 {
            AntennaStatus::Short
        } else if status & 0x80000000 != 0 {
            AntennaStatus::Ok
        } else {
            AntennaStatus::Open
        }
    }
}
impl AntennaStatus {
    pub fn label(status: u32) -> String {
        let ant = AntennaStatus::from(status);
        let ant_slice = match ant {
            AntennaStatus::Ok => "Ok",
            AntennaStatus::Short => "Short",
            AntennaStatus::Open => "Open",
        };
        String::from(ant_slice)
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarUpdate {
    age_of_corrections: f64,
    data_rate: f64,
    ins_status: String,
    num_sats: u8,
    pos_mode: String,
    rtk_mode: String,
    solid_connection: bool,
    ant_status: String,
    port: String,
    version: String,
}
impl StatusBarUpdate {
    pub fn new() -> StatusBarUpdate {
        StatusBarUpdate {
            age_of_corrections: 0.0,
            data_rate: 0.0,
            ins_status: String::from(EMPTY_STR),
            num_sats: 0,
            pos_mode: String::from(EMPTY_STR),
            rtk_mode: String::from(EMPTY_STR),
            solid_connection: false,
            ant_status: String::from(EMPTY_STR),
            port: String::from(""),
            version: String::from(""),
        }
    }
}
impl Default for StatusBarUpdate {
    fn default() -> Self {
        StatusBarUpdate::new()
    }
}

/// StatusBar struct.
///
/// # Fields:
/// - `heartbeat_data`: The shared object for storing and accessing relevant status bar data.
#[derive(Debug)]
pub struct StatusBar {
    heartbeat_data: Heartbeat,
}
impl StatusBar {
    /// Create a new StatusBar.
    ///
    /// # Parameters:
    /// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
    pub fn new(shared_state: SharedState) -> StatusBar {
        let mut heartbeat_data = shared_state.heartbeat_data();
        heartbeat_data.reset();
        heartbeat_data.set_port(shared_state.connection().name());
        heartbeat_data.set_version(shared_state.console_version());
        heartbeat_data.set_conn_is_file(shared_state.connection().is_file());
        StatusBar { heartbeat_data }
    }

    /// Thread for handling the consistently repeating heartbeat.
    ///
    /// # Parameters:
    /// - `client_sender`: Client Sender channel for communication from backend to frontend.
    /// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
    pub fn heartbeat_thread(
        client_sender: BoxedClientSender,
        shared_state: SharedState,
    ) -> (Sender<bool>, JoinHandle<()>) {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel::bounded(1);
        let heartbeat_data = shared_state.heartbeat_data();

        let mut delay_time = UPDATE_TOLERANCE_SECONDS;
        (
            tx,
            spawn(move || loop {
                select! {
                    recv(channel::after(Duration::from_secs_f64(delay_time))) -> _ => {
                        let last_time = Instant::now();
                        let sb_update = heartbeat_data.heartbeat();
                        StatusBar::send_data(client_sender.clone(), sb_update);
                        let new_time = Instant::now();
                        let time_diff = (new_time - last_time).as_secs_f64();
                        delay_time = UPDATE_TOLERANCE_SECONDS - time_diff;
                        delay_time = delay_time.max(0.001); // Add buffer if delay is negative.
                    }
                    recv(rx) -> _ => break,
                }
            }),
        )
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        let mut shared_data = self
            .heartbeat_data
            .lock()
            .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
        (*shared_data).total_bytes_read += bytes;
    }

    /// Package data into a message buffer and send to frontend.
    ///
    /// # Parameters:
    /// - `client_sender`: Client Sender channel for communication from backend to frontend.
    /// - `sb_update`: Optional update packet to send to frontend.
    pub fn send_data(client_sender: BoxedClientSender, sb_update: StatusBarUpdate) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut status_bar_status = msg.init_status_bar_status();
        status_bar_status.set_antenna_status(&sb_update.ant_status);
        status_bar_status.set_pos(&sb_update.pos_mode);
        status_bar_status.set_rtk(&sb_update.rtk_mode);
        status_bar_status.set_sats(sb_update.num_sats);
        status_bar_status.set_corr_age(sb_update.age_of_corrections);
        status_bar_status.set_ins(&sb_update.ins_status);
        status_bar_status.set_data_rate(sb_update.data_rate);
        status_bar_status.set_solid_connection(sb_update.solid_connection);
        status_bar_status.set_title(&format!(
            "{} Swift Console {}",
            sb_update.port, sb_update.version
        ));
        client_sender.send_data(serialize_capnproto_builder(builder));
    }

    pub fn handle_heartbeat(&mut self, msg: MsgHeartbeat) {
        let mut shared_data = self
            .heartbeat_data
            .lock()
            .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
        (*shared_data).heartbeat_count += 1;
        (*shared_data).ant_status = AntennaStatus::label(msg.flags);
    }

    /// Handle PosLLH / PosLLHDepA messages.
    ///
    /// Taken from solution_tab.rs
    /// Need to validate logging.
    pub fn handle_pos_llh(&mut self, msg: PosLLH) {
        let llh_solution_mode = msg.mode();
        let pos_llh_fields = msg.fields();

        let llh_num_sats = pos_llh_fields.n_sats;
        let ins_used = ((pos_llh_fields.flags & 0x8) >> 3) == 1;

        let last_stime_update = if llh_solution_mode != 0 {
            Some(Instant::now())
        } else {
            None
        };
        {
            let mut shared_data = self
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).llh_solution_mode = llh_solution_mode;
            (*shared_data).last_stime_update = last_stime_update;
            if llh_solution_mode > 0 {
                (*shared_data).llh_num_sats = llh_num_sats;
            }
            (*shared_data).ins_used = ins_used;
        }
    }

    /// Handle BaselineNED and BaselineNEDDepA messages.
    pub fn handle_baseline_ned(&mut self, msg: BaselineNED) {
        let baseline_solution_mode = msg.mode();
        let last_btime_update = if baseline_solution_mode > 0 {
            Some(Instant::now())
        } else {
            None
        };
        {
            let mut shared_data = self
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).baseline_solution_mode = baseline_solution_mode;
            (*shared_data).last_btime_update = last_btime_update;
        }
    }

    /// Handle INS Updates messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsUpdates to extract data from.
    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        let tic = msg.wheelticks;
        if ((tic & 0xF0) >> 4) > (tic & 0x0F) {
            let last_odo_update_time = Instant::now();
            let mut shared_data = self
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).last_odo_update_time = Some(last_odo_update_time);
        }
    }

    /// Handle INS Status messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsStatus to extract data from.
    pub fn handle_ins_status(&mut self, msg: MsgInsStatus) {
        let ins_status_flags = msg.flags;
        let last_ins_status_receipt_time = Some(Instant::now());
        {
            let mut shared_data = self
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).ins_status_flags = ins_status_flags;
            (*shared_data).last_ins_status_receipt_time = last_ins_status_receipt_time;
        }
    }

    /// Handle Age Corrections messages.
    ///
    /// Taken from solution_tab.rs.
    /// # Parameters
    /// - `msg`: The MsgAgeCorrections to extract data from.
    pub fn handle_age_corrections(&mut self, msg: MsgAgeCorrections) {
        let age_corrections = if msg.age != 0xFFFF {
            Some(decisec_to_sec(msg.age as f64))
        } else {
            None
        };
        {
            let mut shared_data = self
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).age_corrections = age_corrections;
            (*shared_data).last_age_corr_receipt_time = Some(Instant::now());
        }
    }
}

#[derive(Debug)]
pub struct HeartbeatInner {
    age_of_corrections: f64,
    age_corrections: Option<f64>,
    ant_status: String,
    baseline_display_mode: String,
    baseline_solution_mode: u8,
    conn_is_file: bool,
    current_time: Instant,
    data_rate: f64,
    dgnss_enabled: bool,
    heartbeat_count: usize,
    ins_status_flags: u32,
    ins_status_string: String,
    ins_used: bool,
    last_age_corr_receipt_time: Option<Instant>,
    last_btime_update: Option<Instant>,
    last_heartbeat_count: usize,
    last_ins_status_receipt_time: Option<Instant>,
    last_odo_update_time: Option<Instant>,
    last_stime_update: Option<Instant>,
    llh_display_mode: String,
    llh_is_rtk: bool,
    llh_num_sats: u8,
    llh_solution_mode: u8,
    port: String,
    solid_connection: bool,
    total_bytes_read: usize,
    last_bytes_read: usize,
    last_time_bytes_read: Instant,
    version: String,
}
impl HeartbeatInner {
    pub fn new() -> HeartbeatInner {
        HeartbeatInner {
            age_of_corrections: 0.0,
            age_corrections: None,
            ant_status: String::from(EMPTY_STR),
            baseline_display_mode: String::from(EMPTY_STR),
            baseline_solution_mode: 0,
            conn_is_file: false,
            current_time: Instant::now(),
            data_rate: 0.0,
            dgnss_enabled: false,
            heartbeat_count: 0,
            ins_status_flags: 0,
            ins_status_string: String::from(EMPTY_STR),
            ins_used: false,
            last_age_corr_receipt_time: None,
            last_btime_update: None,
            last_heartbeat_count: 0,
            last_ins_status_receipt_time: None,
            last_odo_update_time: None,
            last_stime_update: None,
            llh_display_mode: String::from(EMPTY_STR),
            llh_is_rtk: false,
            llh_num_sats: 0,
            llh_solution_mode: 0,
            port: String::from(""),
            solid_connection: false,
            total_bytes_read: 0,
            last_bytes_read: 0,
            last_time_bytes_read: Instant::now(),
            version: String::from(""),
        }
    }

    pub fn check_heartbeat(&mut self) -> bool {
        self.solid_connection = if (self.heartbeat_count == self.last_heartbeat_count
            && self.heartbeat_count != 0)
            || (self.data_rate <= f64::EPSILON)
        {
            false
        } else {
            self.current_time = Instant::now();
            true
        };
        self.last_heartbeat_count = self.heartbeat_count;
        self.data_rate_update();
        self.solid_connection
    }

    pub fn data_rate_update(&mut self) {
        let new_bytes_time_read = Instant::now();
        let diff = self.total_bytes_read - self.last_bytes_read;
        self.data_rate = bytes_to_kb(diff as f64)
            / (new_bytes_time_read - self.last_time_bytes_read).as_secs_f64();
        self.last_bytes_read = self.total_bytes_read;
        self.last_time_bytes_read = new_bytes_time_read;
    }

    pub fn pos_llh_update(&mut self) {
        self.llh_display_mode = String::from(EMPTY_STR);
        if let Some(last_stime_update) = self.last_stime_update {
            if (self.current_time - last_stime_update).as_secs_f64() < UPDATE_TOLERANCE_SECONDS {
                self.llh_display_mode = GnssModes::from(self.llh_solution_mode).pos_mode();
                self.llh_is_rtk = RTK_MODES.contains(&(self.llh_solution_mode as i32));
                if self.ins_used && (self.llh_solution_mode as i32) != DR_MODE {
                    self.llh_display_mode = format!("{}{}", self.llh_display_mode, INS_POSTFIX);
                }
            }
        }
    }

    pub fn baseline_ned_update(&mut self) {
        self.baseline_display_mode = String::from(EMPTY_STR);
        if let Some(last_btime_update) = self.last_btime_update {
            if (self.dgnss_enabled || self.conn_is_file)
                && (self.current_time - last_btime_update).as_secs_f64() < UPDATE_TOLERANCE_SECONDS
            {
                if let Some(bsoln_mode) = rtk_mode_dict.get(&(self.baseline_solution_mode as i32)) {
                    self.baseline_display_mode = String::from(*bsoln_mode)
                }
            }
        }
        if rtk_mode_dict
            .get(&(self.baseline_solution_mode as i32))
            .is_none()
            && self.llh_is_rtk
        {
            self.baseline_display_mode =
                if let Some(soln_mode) = rtk_mode_dict.get(&(self.llh_solution_mode as i32)) {
                    String::from(*soln_mode)
                } else {
                    String::from(EMPTY_STR)
                };
        }
    }

    pub fn ins_update(&mut self) {
        if let Some(last_ins_status_receipt_time) = self.last_ins_status_receipt_time {
            if (self.current_time - last_ins_status_receipt_time).as_secs_f64()
                < UPDATE_TOLERANCE_SECONDS
            {
                let ins_mode = self.ins_status_flags & 0x7;
                let ins_type = (self.ins_status_flags >> 29) & 0x7;
                let mut odo_status = (self.ins_status_flags >> 8) & 0x3;
                if odo_status != 1 {
                    if let Some(last_odo_update_time) = self.last_odo_update_time {
                        if (self.current_time - last_odo_update_time).as_secs_f64() < 10_f64 {
                            odo_status = 1;
                        }
                    }
                }
                let ins_error = (self.ins_status_flags >> 4) & 0xF;
                if ins_error != 0 {
                    self.ins_status_string =
                        if let Some(err_string) = ins_error_dict.get(&(ins_error as i32)) {
                            err_string.to_string()
                        } else {
                            UNKNOWN_ERROR.to_string()
                        };
                } else {
                    let ins_type_string =
                        if let Some(type_string) = ins_type_dict.get(&(ins_type as i32)) {
                            String::from(*type_string)
                        } else {
                            format!("{}-", UNKNOWN_ERROR_SHORT)
                        };
                    let ins_mode_string =
                        if let Some(mode_string) = ins_mode_dict.get(&(ins_mode as i32)) {
                            mode_string
                        } else {
                            UNKNOWN_ERROR_SHORT
                        };
                    let mut odo_str = "";
                    if odo_status == 1 {
                        odo_str = ODO_POSTFIX;
                    }
                    self.ins_status_string =
                        format!("{}{}{}", ins_type_string, ins_mode_string, odo_str);
                }
            }
        }
    }

    pub fn age_of_corrections_update(&mut self) {
        self.age_of_corrections = 0.0;
        if let Some(age_corr) = self.age_corrections {
            if let Some(last_age_corr_time) = self.last_age_corr_receipt_time {
                if (self.current_time - last_age_corr_time).as_secs_f64() < UPDATE_TOLERANCE_SECONDS
                {
                    self.age_of_corrections = age_corr;
                }
            }
        }
    }

    pub fn prepare_update_packet(&mut self, good_heartbeat: bool) -> StatusBarUpdate {
        if good_heartbeat {
            StatusBarUpdate {
                age_of_corrections: self.age_of_corrections,
                ant_status: self.ant_status.clone(),
                data_rate: self.data_rate,
                ins_status: self.ins_status_string.clone(),
                num_sats: self.llh_num_sats,
                pos_mode: self.llh_display_mode.clone(),
                rtk_mode: self.baseline_display_mode.clone(),
                solid_connection: self.solid_connection,
                port: self.port.clone(),
                version: self.version.clone(),
            }
        } else {
            let packet = StatusBarUpdate {
                solid_connection: self.solid_connection,
                ant_status: self.ant_status.clone(),
                num_sats: self.llh_num_sats,
                version: self.version.clone(),
                ..Default::default()
            };
            self.llh_num_sats = 0;
            self.ant_status = String::from(EMPTY_STR);
            packet
        }
    }
}
impl Default for HeartbeatInner {
    fn default() -> Self {
        HeartbeatInner::new()
    }
}

#[derive(Debug)]
pub struct Heartbeat(Arc<Mutex<HeartbeatInner>>);

impl Heartbeat {
    pub fn new() -> Heartbeat {
        Heartbeat(Arc::new(Mutex::new(HeartbeatInner::default())))
    }
    pub fn heartbeat(&self) -> StatusBarUpdate {
        let mut shared_data = self.lock().expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
        let good_heartbeat: bool = (*shared_data).check_heartbeat();
        if good_heartbeat {
            (*shared_data).pos_llh_update();
            (*shared_data).baseline_ned_update();
            (*shared_data).ins_update();
            (*shared_data).age_of_corrections_update();
        }
        (*shared_data).prepare_update_packet(good_heartbeat)
    }
    pub fn set_dgnss_enabled(&self, dgnss_enabled: bool) {
        self.lock()
            .expect(HEARTBEAT_LOCK_MUTEX_FAILURE)
            .dgnss_enabled = dgnss_enabled;
    }
    pub fn set_conn_is_file(&self, conn_is_file: bool) {
        self.lock()
            .expect(HEARTBEAT_LOCK_MUTEX_FAILURE)
            .conn_is_file = conn_is_file;
    }
    pub fn set_version(&self, version: String) {
        self.lock().expect(HEARTBEAT_LOCK_MUTEX_FAILURE).version = version;
    }
    pub fn set_port(&self, port: String) {
        self.lock().expect(HEARTBEAT_LOCK_MUTEX_FAILURE).port = port;
    }
    pub fn reset(&mut self) {
        *self.lock().expect(HEARTBEAT_LOCK_MUTEX_FAILURE) = HeartbeatInner::new();
    }
}

impl Deref for Heartbeat {
    type Target = Mutex<HeartbeatInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Heartbeat {
    fn clone(&self) -> Self {
        Heartbeat(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };
    const DELAY_BUFFER_MS: u64 = 10;

    #[test]
    fn status_bar_thread_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let (tx, status_thd): (Sender<bool>, JoinHandle<()>) =
            StatusBar::heartbeat_thread(client_send, shared_state.clone());
        spawn(move || {
            let heartbeat_data = shared_state.heartbeat_data();
            {
                let hb = heartbeat_data.lock().unwrap();
                assert!(!(*hb).solid_connection);
            }
            (0..3).for_each(|_| {
                {
                    let mut hb = heartbeat_data.lock().unwrap();
                    hb.heartbeat_count += 1;
                    hb.total_bytes_read += 1337;
                }
                sleep(Duration::from_secs_f64(UPDATE_TOLERANCE_SECONDS));
            });
            {
                let hb = heartbeat_data.lock().unwrap();
                assert!((*hb).solid_connection);
            }
            sleep(Duration::from_secs_f64(2.0 * UPDATE_TOLERANCE_SECONDS));
            {
                let hb = heartbeat_data.lock().unwrap();
                assert!(!(*hb).solid_connection);
            }
        })
        .join()
        .unwrap();
        tx.send(false).unwrap();
        status_thd.join().unwrap();
    }

    #[test]
    fn handle_age_corrections_test() {
        let shared_state = SharedState::new();
        let mut status_bar = StatusBar::new(shared_state);
        let age_corrections = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).age_corrections
        };

        assert!(age_corrections.is_none());
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: 0xFFFF,
            tow: 0,
        };
        status_bar.handle_age_corrections(msg);
        let age_corrections = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).age_corrections
        };
        assert!(age_corrections.is_none());
        let good_age = 0x4DC6;
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: good_age,
            tow: 0,
        };
        status_bar.handle_age_corrections(msg);
        let age_corrections = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).age_corrections
        };
        assert!(age_corrections.is_some());
        if let Some(age) = age_corrections {
            assert!(f64::abs(age - 1991_f64) <= f64::EPSILON);
        }
    }

    #[test]
    fn handle_ins_status_test() {
        let shared_state = SharedState::new();
        let mut status_bar = StatusBar::new(shared_state);
        let flags = 0xf0_u32;
        let msg = MsgInsStatus {
            sender_id: Some(1337),
            flags,
        };
        let update_time = Instant::now();
        status_bar.handle_ins_status(msg);
        sleep(Duration::from_millis(DELAY_BUFFER_MS));
        let (last_ins_status_receipt_time, ins_status_flags) = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (
                (*shared_data).last_ins_status_receipt_time,
                (*shared_data).ins_status_flags,
            )
        };
        assert!(
            last_ins_status_receipt_time.unwrap() > update_time,
            "[Flaky] If this test fails
        consider rerunning as it is known to be flaky.
        More info found here: https://swift-nav.atlassian.net/browse/CPP-252"
        );
        assert_eq!(ins_status_flags, flags);
    }

    #[test]
    fn handle_ins_updates_test() {
        let shared_state = SharedState::new();
        let mut status_bar = StatusBar::new(shared_state);
        let msg = MsgInsUpdates {
            sender_id: Some(1337),
            gnsspos: 0,
            gnssvel: 0,
            wheelticks: 0xf0_u8,
            speed: 0,
            nhc: 0,
            zerovel: 0,
            tow: 0,
        };

        let odo_update_time = Instant::now();
        sleep(Duration::from_secs(1));
        status_bar.handle_ins_updates(msg);
        let last_odo_update_time = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).last_odo_update_time
        };
        assert!(last_odo_update_time.unwrap() > odo_update_time);

        let msg = MsgInsUpdates {
            sender_id: Some(1337),
            gnsspos: 4,
            gnssvel: 4,
            wheelticks: 0xff_u8,
            speed: 0,
            nhc: 0,
            zerovel: 0,
            tow: 0,
        };

        let odo_update_time = Instant::now();
        status_bar.handle_ins_updates(msg);
        let last_odo_update_time = {
            let shared_data = status_bar
                .heartbeat_data
                .lock()
                .expect(HEARTBEAT_LOCK_MUTEX_FAILURE);
            (*shared_data).last_odo_update_time
        };

        assert!(last_odo_update_time.unwrap() < odo_update_time);
    }

    // TODO (john-michaelburke@) [CPP-173] Add missing unittests!!
}
