use capnp::message::Builder;
use capnp::serialize;

use sbp::messages::{
    navigation::MsgAgeCorrections,
    system::{MsgInsStatus, MsgInsUpdates},
};

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::piksi_tools_constants::{
    ins_error_dict, ins_mode_dict, ins_type_dict, rtk_mode_dict, DR_MODE, EMPTY_STR, RTK_MODES,
};
use crate::types::{BaselineNED, GnssModes, MessageSender, PosLLH, SharedState};
use crate::utils::decisec_to_sec;

#[derive(Debug, Clone)]
pub struct StatusBarUpdate {
    age_of_corrections: String,
    ins_status: String,
    num_sats: String,
    pos_mode: String,
    rtk_mode: String,
}
impl StatusBarUpdate {
    pub fn connection_dropped() -> StatusBarUpdate {
        StatusBarUpdate {
            age_of_corrections: String::from(EMPTY_STR),
            ins_status: String::from(EMPTY_STR),
            num_sats: String::from(EMPTY_STR),
            pos_mode: String::from(EMPTY_STR),
            rtk_mode: String::from(EMPTY_STR),
        }
    }
}

/// StatusBar struct.
///
/// # Fields:
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `heartbeat_data`: The shared object for storing and accessing relevant status bar data.
/// - `heartbeat_handler`: The handler to store the running heartbeat thread.
/// - `port`: The string corresponding to the current connection.
#[derive(Debug)]
pub struct StatusBar<S: MessageSender> {
    client_sender: S,
    shared_state: SharedState,
    heartbeat_data: Heartbeat,
    heartbeat_handler: Option<JoinHandle<()>>,
    port: String,
}
impl<S: MessageSender> StatusBar<S> {
    /// Create a new StatusBar.
    ///
    /// # Parameters:
    /// - `client_send`: Client Sender channel for communication from backend to frontend.
    /// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
    pub fn new(shared_state: SharedState, client_sender: S) -> StatusBar<S> {
        let heartbeat_data = Heartbeat::new();
        StatusBar {
            client_sender,
            shared_state: shared_state.clone(),
            heartbeat_data: heartbeat_data.clone(),
            port: shared_state.current_connection(),
            heartbeat_handler: {
                let mut last_time = Instant::now();
                Some(spawn(move || loop {
                    if !shared_state.is_running() {
                        break;
                    }
                    heartbeat_data.heartbeat();
                    let new_time = Instant::now();
                    let time_diff = (new_time - last_time).as_secs_f64();
                    let delay_time = UPDATE_TOLERANCE_SECONDS - time_diff;
                    if delay_time > 0_f64 {
                        sleep(Duration::from_secs_f64(delay_time));
                    }
                    last_time = Instant::now();
                }))
            },
        }
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let data_rate = self.shared_state.data_rate();
        let sb_update;
        {
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            sb_update = (*shared_data).new_update.clone();
            (*shared_data).new_update = None;
            (*shared_data).data_rate = data_rate;
        }
        let sb_update = if let Some(sb_update_) = sb_update {
            sb_update_
        } else {
            return;
        };

        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let mut status_bar_status = msg.init_status_bar_status();
        status_bar_status.set_port(&self.port);
        status_bar_status.set_pos(&sb_update.pos_mode);
        status_bar_status.set_rtk(&sb_update.rtk_mode);
        status_bar_status.set_sats(&sb_update.num_sats);
        status_bar_status.set_corr_age(&sb_update.age_of_corrections);
        status_bar_status.set_ins(&sb_update.ins_status);
        status_bar_status.set_data_rate(&format!("{:.2}", data_rate));

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        self.client_sender.send_data(msg_bytes);
    }

    pub fn handle_heartbeat(&mut self) {
        let mut shared_data = self.heartbeat_data.lock().unwrap();
        (*shared_data).heartbeat_count += 1;
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

        let mut last_stime_update = None;
        if llh_solution_mode != 0 {
            last_stime_update = Some(Instant::now());
        }
        {
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            (*shared_data).llh_solution_mode = llh_solution_mode;
            (*shared_data).last_stime_update = last_stime_update;
            if llh_solution_mode > 0 {
                (*shared_data).llh_num_sats = llh_num_sats;
            }
            (*shared_data).ins_used = ins_used;
        }

        self.send_data();
    }

    /// Handle BaselineNED and BaselineNEDDepA messages.
    pub fn handle_baseline_ned(&mut self, msg: BaselineNED) {
        let baseline_solution_mode = msg.mode();
        let mut last_btime_update = None;
        if baseline_solution_mode > 0 {
            last_btime_update = Some(Instant::now());
        }
        {
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            (*shared_data).baseline_solution_mode = baseline_solution_mode;
            (*shared_data).last_btime_update = last_btime_update;
        }
        self.send_data();
    }

    /// Handle INS Updates messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsUpdates to extract data from.
    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        let tic = msg.wheelticks;
        if ((tic & 0xF0) >> 4) > (tic & 0x0F) {
            let last_odo_update_time = Instant::now();
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            (*shared_data).last_odo_update_time = Some(last_odo_update_time);
        }
        self.send_data();
    }

    /// Handle INS Status messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsStatus to extract data from.
    pub fn handle_ins_status(&mut self, msg: MsgInsStatus) {
        let ins_status_flags = msg.flags;
        let last_ins_status_receipt_time = Some(Instant::now());
        {
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            (*shared_data).ins_status_flags = ins_status_flags;
            (*shared_data).last_ins_status_receipt_time = last_ins_status_receipt_time;
        }
        self.send_data();
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
            let mut shared_data = self.heartbeat_data.lock().unwrap();
            (*shared_data).age_corrections = age_corrections;
            (*shared_data).last_age_corr_receipt_time = Some(Instant::now());
        }
        self.send_data();
    }
}

#[derive(Debug)]
pub struct HeartbeatInner {
    age_of_corrections: String,
    age_corrections: Option<f64>,
    baseline_display_mode: String,
    baseline_solution_mode: u8,
    current_time: Instant,
    data_rate: f64,
    heartbeat_count: usize,
    ins_status_flags: u32,
    ins_status_string: String,
    ins_used: bool,
    last_age_corr_receipt_time: Option<Instant>,
    last_btime_update: Option<Instant>,
    last_bytes_read: usize,
    last_heartbeat_count: usize,
    last_ins_status_receipt_time: Option<Instant>,
    last_odo_update_time: Option<Instant>,
    last_stime_update: Option<Instant>,
    llh_display_mode: String,
    llh_is_rtk: bool,
    llh_num_sats: u8,
    llh_solution_mode: u8,
    new_update: Option<StatusBarUpdate>,
    solid_connection: bool,
    total_bytes_read: usize,
}
impl HeartbeatInner {
    pub fn new() -> HeartbeatInner {
        HeartbeatInner {
            age_of_corrections: String::from(EMPTY_STR),
            age_corrections: None,
            baseline_display_mode: String::from(EMPTY_STR),
            baseline_solution_mode: 0,
            current_time: Instant::now(),
            data_rate: 0.0,
            heartbeat_count: 0,
            ins_status_flags: 0,
            ins_status_string: String::from(EMPTY_STR),
            ins_used: false,
            last_age_corr_receipt_time: None,
            last_btime_update: None,
            last_bytes_read: 0,
            last_heartbeat_count: 0,
            last_ins_status_receipt_time: None,
            last_odo_update_time: None,
            last_stime_update: None,
            llh_display_mode: String::from(EMPTY_STR),
            llh_is_rtk: false,
            llh_num_sats: 0,
            llh_solution_mode: 0,
            new_update: None,
            solid_connection: false,
            total_bytes_read: 0,
        }
    }

    pub fn check_heartbeat(&mut self) -> bool {
        self.solid_connection = if (self.heartbeat_count == self.last_heartbeat_count
            && self.heartbeat_count != 0)
            || (self.data_rate <= f64::EPSILON)
        {
            self.new_update = Some(StatusBarUpdate::connection_dropped());
            false
        } else {
            true
        };
        self.last_heartbeat_count = self.heartbeat_count;

        if self.solid_connection {
            self.current_time = Instant::now();
        }
        self.solid_connection
    }

    pub fn pos_llh_update(&mut self) {
        if let Some(last_stime_update) = self.last_stime_update {
            if (self.current_time - last_stime_update).as_secs_f64() < UPDATE_TOLERANCE_SECONDS {
                let llh_display_mode = GnssModes::from(self.llh_solution_mode);
                self.llh_display_mode = llh_display_mode.pos_mode();
                self.llh_is_rtk = RTK_MODES.contains(&(self.llh_solution_mode as i32));
                if self.ins_used && (self.llh_solution_mode as i32) != DR_MODE {
                    self.llh_display_mode = format!("{}{}", llh_display_mode, INS_POSTFIX);
                }
            }
        }
    }

    pub fn baseline_ned_update(&mut self) {
        if let Some(last_btime_update) = self.last_btime_update {
            //TODO(@john-michaelburke) [CPP-172] Add missing dgnss_enabled logic.
            if (self.current_time - last_btime_update).as_secs_f64() < UPDATE_TOLERANCE_SECONDS {
                self.baseline_display_mode = if let Some(bsoln_mode) =
                    rtk_mode_dict.get(&(self.baseline_solution_mode as i32))
                {
                    String::from(*bsoln_mode)
                } else {
                    String::from(EMPTY_STR)
                };
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
                            type_string
                        } else {
                            UNKNOWN_ERROR_SHORT
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
                        format!("{}-{}{}", ins_type_string, ins_mode_string, odo_str);
                }
            }
        }
    }

    pub fn age_of_corrections_update(&mut self) {
        self.age_of_corrections = String::from(EMPTY_STR);
        if let Some(age_corr) = self.age_corrections {
            if let Some(last_age_corr_time) = self.last_age_corr_receipt_time {
                if (self.current_time - last_age_corr_time).as_secs_f64() < UPDATE_TOLERANCE_SECONDS
                {
                    self.age_of_corrections = format!("{} s", age_corr);
                }
            }
        }
    }

    pub fn prepare_update_packet(&mut self) {
        let sb_update = StatusBarUpdate {
            age_of_corrections: self.age_of_corrections.clone(),
            ins_status: self.ins_status_string.clone(),
            num_sats: self.llh_num_sats.to_string(),
            pos_mode: self.llh_display_mode.clone(),
            rtk_mode: self.baseline_display_mode.clone(),
        };
        self.new_update = Some(sb_update);
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
    pub fn heartbeat(&self) {
        let mut shared_data = self.lock().unwrap();
        let good_heartbeat: bool = (*shared_data).check_heartbeat();
        if !good_heartbeat {
            return;
        }
        (*shared_data).pos_llh_update();
        (*shared_data).baseline_ned_update();
        (*shared_data).ins_update();
        (*shared_data).age_of_corrections_update();
        (*shared_data).prepare_update_packet();
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
        Heartbeat {
            0: Arc::clone(&self.0),
        }
    }
}
