use std::str::FromStr;

use capnp::message::Builder;
use capnp::serialize;

use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgGPSTime, MsgUtcTime},
    system::{MsgInsStatus, MsgInsUpdates},
};

use std::{
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::piksi_tools_constants::{DR_MODE, rtk_mode_dict, RTK_MODES};
use crate::types::{BaselineNED, GnssModes, MessageSender, PosLLH, SharedState};
use crate::utils::decisec_to_sec;

/// StatusBar struct.
///
/// # Fields:
///
/// - `age_corrections`: Stored age corrections to be checked for validity.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
#[derive(Debug)]
pub struct StatusBar<S: MessageSender> {
    age_of_corrections: String,
    age_corrections: Option<f64>,
    client_sender: S,
    current_time: Instant,
    last_age_corr_receipt_time: Option<Instant>,
    last_stime_update: Option<Instant>,
    shared_state: SharedState,
    heartbeat_data: Heartbeat,
    heartbeat_handler: Option<JoinHandle<()>>,
}

impl<S: MessageSender> StatusBar<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> StatusBar<S> {
        let mut status_bar = StatusBar {
            age_of_corrections: String::from(EMPTY_STR),
            age_corrections: None,
            current_time: Instant::now(),
            client_sender,
            last_age_corr_receipt_time: None,
            last_stime_update: None,
            shared_state: shared_state.clone(),
            heartbeat_data: Heartbeat::new(),
            heartbeat_handler: None,
        };
        let heartbeat_data = status_bar.heartbeat_data.clone();
        let handle = spawn(move || loop {
            if !shared_state.is_running() {
                break;
            }
            heartbeat_data.heartbeat();
            sleep(Duration::from_millis(200));
        });
        status_bar.heartbeat_handler = Some(handle);
        status_bar
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
        let mut shared_data = self.heartbeat_data.lock().unwrap();
        (*shared_data).age_corrections = age_corrections;
        (*shared_data).last_age_corr_receipt_time = Some(Instant::now());
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

        let mut shared_data = self.heartbeat_data.lock().unwrap();
        (*shared_data).llh_solution_mode = llh_solution_mode;
        (*shared_data).last_stime_update = last_stime_update;
        if llh_solution_mode > 0 {
            (*shared_data).llh_num_sats = llh_num_sats;
        }
        (*shared_data).ins_used = ins_used;
    }

    pub fn handle_baseline_ned(&mut self, msg: BaselineNED) {
        let baseline_solution_mode = msg.mode();
        let mut last_btime_update = None;
        if baseline_solution_mode > 0 {
            last_btime_update = Some(Instant::now());
        }
        let baseline_display_mode = if let Some(bsoln_mode) = rtk_mode_dict.get(baseline_solution_mode) {
            bsoln_mode
        } else {
            EMPTY_STR
        };
    }
}

use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Heartbeat(Arc<Mutex<HeartbeatInner>>);

impl Heartbeat {
    pub fn new() -> Heartbeat {
        Heartbeat(Arc::new(Mutex::new(HeartbeatInner::default())))
    }
    pub fn heartbeat(&self) {
        let mut shared_data = self.lock().unwrap();
        (*shared_data).age_of_corrections_update();
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

#[derive(Debug)]
pub struct HeartbeatInner {
    age_of_corrections: String,
    age_corrections: Option<f64>,
    current_time: Instant,
    ins_used: bool,
    last_age_corr_receipt_time: Option<Instant>,
    last_stime_update: Option<Instant>,
    llh_display_mode: String,
    llh_is_rtk: bool,
    llh_num_sats: u8,
    llh_solution_mode: u8,
}
impl HeartbeatInner {
    pub fn new() -> HeartbeatInner {
        HeartbeatInner {
            age_of_corrections: String::from(EMPTY_STR),
            age_corrections: None,
            current_time: Instant::now(),
            ins_used: false,
            last_age_corr_receipt_time: None,
            last_stime_update: None,
            llh_display_mode: String::from("None"),
            llh_is_rtk: false,
            llh_num_sats: 0,
            llh_solution_mode: 0,
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

    pub fn pos_llh_update(&mut self) {
        if let Some(last_stime_update) = self.last_stime_update {
            if (self.current_time - last_stime_update).as_secs_f64() < UPDATE_TOLERANCE_SECONDS {
                let llh_display_mode = GnssModes::from(self.llh_solution_mode);
                let llh_display_mode = llh_display_mode.to_string();
                self.llh_is_rtk = RTK_MODES.contains(&(self.llh_solution_mode as i32));
                if self.ins_used && (self.llh_solution_mode as i32) != DR_MODE {
                    self.llh_display_mode = format!("{}+INS", llh_display_mode);
                }
            }
        }
    }
}
impl Default for HeartbeatInner {
    fn default() -> Self {
        HeartbeatInner::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{MPS2KPH, MPS2MPH};
    use crate::types::TestSender;

    // #[test]
    // fn handle_vel_ned_test() {
    //     let shared_state = SharedState::new();
    //     let client_send = TestSender { inner: Vec::new() };
    //     let mut solution_velocity_tab = StatusBar::new(shared_state, client_send);

    //     let msg: MsgVelNED = MsgVelNED {
    //         sender_id: Some(5),
    //         n: 6,
    //         e: 66,
    //         d: 666,
    //         tow: 1001_u32,
    //         h_accuracy: 0,
    //         v_accuracy: 0,
    //         flags: 1,
    //         n_sats: 1,
    //     };

    //     solution_velocity_tab.handle_vel_ned(msg);
    //     assert_eq!(solution_velocity_tab.points.len(), 2);
    //     let hpoints = solution_velocity_tab.points[0].get();
    //     let vpoints = solution_velocity_tab.points[1].get();
    //     assert_eq!(hpoints.len(), 1);
    //     assert_eq!(vpoints.len(), 1);
    //     assert!((*hpoints[0].1 - 0.06627216610312357) <= f64::EPSILON);
    //     assert!((*vpoints[0].1 - (-0.666)) <= f64::EPSILON);
    //     let msg = MsgVelNED {
    //         sender_id: Some(5),
    //         n: 1,
    //         e: 133,
    //         d: 1337,
    //         tow: 1002_u32,
    //         h_accuracy: 0,
    //         v_accuracy: 0,
    //         flags: 1,
    //         n_sats: 1,
    //     };
    //     solution_velocity_tab.handle_vel_ned(msg);
    //     let hpoints = solution_velocity_tab.points[0].get();
    //     let vpoints = solution_velocity_tab.points[1].get();
    //     assert_eq!(hpoints.len(), 2);
    //     assert_eq!(vpoints.len(), 2);
    //     assert!(f64::abs(*hpoints[1].1 - 0.13300375934536587) <= f64::EPSILON);
    //     assert!(f64::abs(*vpoints[1].1 - (-1.337)) <= f64::EPSILON);
    //     let msg = MsgVelNED {
    //         sender_id: Some(5),
    //         n: 7,
    //         e: 67,
    //         d: 667,
    //         tow: 1003_u32,
    //         h_accuracy: 0,
    //         v_accuracy: 0,
    //         flags: 1,
    //         n_sats: 1,
    //     };
    //     solution_velocity_tab.handle_vel_ned(msg);
    //     let hpoints = solution_velocity_tab.points[0].get();
    //     let vpoints = solution_velocity_tab.points[1].get();
    //     assert_eq!(hpoints.len(), 3);
    //     assert_eq!(vpoints.len(), 3);
    //     assert!(f64::abs(*hpoints[1].1 - solution_velocity_tab.max) <= f64::EPSILON);
    //     assert!(f64::abs(*vpoints[1].1 - solution_velocity_tab.min) <= f64::EPSILON);
    // }
}
