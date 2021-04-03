use capnp::message::Builder;
use capnp::serialize;

use sbp::messages::navigation::{MsgGPSTime, MsgUtcTime};
use std::collections::HashMap;

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::types::{Deque, MessageSender, SharedState, UtcDateTime, VelNED};

/// SolutionTable struct.
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
pub struct SolutionTable {
    pub alts: Deque<f64>,
    pub lats: Deque<f64>,
    pub lngs: Deque<f64>,
    pub max: f64,
    pub min: f64,
    pub modes: Deque<f64>,
    pub nsec: Option<i32>,
    pub shared_state: SharedState,
    pub slns: HashMap<String, Deque<f64>>,
    pub tows: Deque<f64>,
    pub utc_source: Option<String>,
    pub utc_time: Option<UtcDateTime>,
    pub week: Some(u16),
}

impl SolutionTable {
    pub fn new(shared_state: SharedState) -> SolutionTable {
        SolutionTable {
            alts: Deque::with_size_limit(PLOT_HISTORY_MAX),
            lats: Deque::with_size_limit(PLOT_HISTORY_MAX),
            lngs: Deque::with_size_limit(PLOT_HISTORY_MAX),
            max: 0_f64,
            min: 0_f64,
            modes: Deque::with_size_limit(PLOT_HISTORY_MAX),
            nsec: Some(0),
            shared_state,
            slns: {
                let mut slns_map = HashMap::new();
                for key in SOLUTIONS_KEYS {
                    slns_map.insert(String::from(*key), Deque::with_size_limit(PLOT_HISTORY_MAX));
                }
                slns_map
            },
            tows: Deque::with_size_limit(PLOT_HISTORY_MAX),
            utc_source: None,
            utc_time: None,
            week: None,
        }
    }

    pub fn handle_utc_time(&mut self, msg: MsgUtcTime) {
        if msg.flags & 0x7 == 0 {
            self.utc_time = None;
            self.utc_source = None;
            return;
        }
        self.utc_time = Some(get_utc_time(
            msg.year as i32,
            msg.month as u32,
            msg.day as u32,
            msg.hours as u32,
            msg.minutes as u32,
            msg.seconds as u32,
            msg.ns as u32,
        ));
        self.utc_source = Some(get_utc_source(msg.flags));
    }

    pub fn handle_gps_time(&mut self, msg: MsgGPSTime) {
        if msg.flags == 0 {
            return;
        }
        self.week = Some(msg.wn);
        self.nsec = Some(msg.ns_residual);
    }

    pub fn handle_vel_ned(&mut self, msg: VelNED) {
        let (flags, tow, n, e, d) = match msg {
            VelNED::MsgVelNED(msg) => {
                (msg.flags, msg.tow as f64, msg.n, msg.e, msg.d)
            }
            VelNED::MsgVelNEDDepA(msg) => {
                (1, msg.tow as f64, msg.n, msg.e, msg.d)
            }
        };
        let tow = tow * 1.0e-3_f64;
        if let Some(nsec) = self.nsec {

        }

    }

    // /// Package data into a message buffer and send to frontend.
    // ///
    // /// # Parameters:
    // ///
    // /// - `client_send`: The MessageSender channel to be used to send data to frontend.
    // fn send_data<P: MessageSender>(&mut self, client_send: &mut P) {
    //     let mut builder = Builder::new_default();
    //     let msg = builder.init_root::<m::message::Builder>();

    //     let mut velocity_status = msg.init_solution_velocity_status();
    //     velocity_status.set_min(self.min);
    //     velocity_status.set_max(self.max);

    //     let mut velocity_points = velocity_status
    //         .reborrow()
    //         .init_data(self.points.len() as u32);
    //     for idx in 0..self.points.len() {
    //         let points = self.points.get_mut(idx).unwrap().get();
    //         let mut point_val_idx = velocity_points
    //             .reborrow()
    //             .init(idx as u32, points.len() as u32);
    //         for (i, (x, OrderedFloat(y))) in points.iter().enumerate() {
    //             let mut point_val = point_val_idx.reborrow().get(i as u32);
    //             point_val.set_x(*x);
    //             point_val.set_y(*y);
    //         }
    //     }
    //     let mut available_units = velocity_status
    //         .reborrow()
    //         .init_available_units(self.available_units.len() as u32);

    //     for (i, unit) in self.available_units.iter().enumerate() {
    //         available_units.set(i as u32, *unit);
    //     }
    //     let mut colors = velocity_status
    //         .reborrow()
    //         .init_colors(self.colors.len() as u32);

    //     for (i, color) in self.colors.iter().enumerate() {
    //         colors.set(i as u32, color);
    //     }

    //     let mut msg_bytes: Vec<u8> = vec![];
    //     serialize::write_message(&mut msg_bytes, &builder).unwrap();

    //     client_send.send_data(msg_bytes);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TestSender;
    use chrono::{TimeZone, Utc};

    #[test]
    fn handle_utc_time_test() {
        let shared_state = SharedState::new();
        let mut solution_table = SolutionTable::new(shared_state);
        let year = 2020_u16;
        let month = 3_u8;
        let day = 19_u8;
        let hours = 13_u8;
        let minutes = 3_u8;
        let seconds = 7_u8;
        let ns = 666_u32;
        let bad_flags = 0x00_u8;
        let tow = 1337_u32;
        let msg: MsgUtcTime = MsgUtcTime {
            sender_id: Some(1337),
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            ns,
            flags: bad_flags,
            tow,
        };
        solution_table.utc_time = None;
        solution_table.utc_source = None;
        solution_table.handle_utc_time(msg);
        assert_eq!(solution_table.utc_time, None);
        assert_eq!(solution_table.utc_source, None);
        let good_flags = 0x0f_u8;
        let msg: MsgUtcTime = MsgUtcTime {
            sender_id: Some(1337),
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            ns,
            flags: good_flags,
            tow,
        };
        solution_table.utc_time = None;
        solution_table.utc_source = None;
        solution_table.handle_utc_time(msg);
        let datetime = Utc.ymd(year as i32, month as u32, day as u32).and_hms_nano(
            hours as u32,
            minutes as u32,
            seconds as u32,
            ns as u32,
        );
        assert_eq!(solution_table.utc_time, Some(datetime));
        assert_eq!(
            solution_table.utc_source,
            Some(String::from(NON_VOLATILE_MEMORY))
        );
    }

    #[test]
    fn handle_gps_time_test() {
        let shared_state = SharedState::new();
        let mut solution_table = SolutionTable::new(shared_state);
        let wn = 0_u16;
        let ns_residual = 1337_i32;
        let bad_flags = 0_u8;
        let msg = MsgGPSTime {
            sender_id: Some(1337),
            wn,
            tow: 0,
            ns_residual,
            flags: bad_flags,
        };
        let old_wn = 5_u16;
        let old_nsec = 678_i32;
        solution_table.week = old_wn;
        solution_table.nsec = old_nsec;
        solution_table.handle_gps_time(msg);
        assert_eq!(solution_table.week, old_wn);
        assert_eq!(solution_table.nsec, old_nsec);

        let good_flags = 1_u8;
        let msg = MsgGPSTime {
            sender_id: Some(1337),
            wn,
            tow: 0,
            ns_residual,
            flags: good_flags,
        };
        solution_table.handle_gps_time(msg);
        assert_eq!(solution_table.week, wn);
        assert_eq!(solution_table.nsec, ns_residual);
    }

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
    //     solution_velocity_tab.handle_vel_ned(msg, &mut client_send);
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
    //     solution_velocity_tab.handle_vel_ned(msg, &mut client_send);
    //     let hpoints = solution_velocity_tab.points[0].get();
    //     let vpoints = solution_velocity_tab.points[1].get();
    //     assert_eq!(hpoints.len(), 3);
    //     assert_eq!(vpoints.len(), 3);
    //     assert!(f64::abs(*hpoints[1].1 - solution_velocity_tab.max) <= f64::EPSILON);
    //     assert!(f64::abs(*vpoints[1].1 - solution_velocity_tab.min) <= f64::EPSILON);
    // }

    // #[test]
    // fn test_convert_points() {
    //     let shared_state = SharedState::new();
    //     let mut solution_velocity_tab = SolutionTable::new(shared_state);

    //     let mut msg: MsgVelNED = MsgVelNED {
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

    //     let mut client_send = TestSender { inner: Vec::new() };
    //     solution_velocity_tab.handle_vel_ned(msg, &mut client_send);
    //     msg = MsgVelNED {
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
    //     solution_velocity_tab.handle_vel_ned(msg, &mut client_send);
    //     let hpoints = solution_velocity_tab.points[0].get().clone();
    //     let vpoints = solution_velocity_tab.points[1].get().clone();

    //     let new_unit = VelocityUnits::Mps;
    //     solution_velocity_tab.convert_points(new_unit);
    //     let new_hpoints = solution_velocity_tab.points[0].get();
    //     let new_vpoints = solution_velocity_tab.points[1].get();
    //     for idx in 0..hpoints.len() {
    //         assert!(f64::abs(*hpoints[idx].1 - *new_hpoints[idx].1) <= f64::EPSILON);
    //         assert!(f64::abs(*vpoints[idx].1 - *new_vpoints[idx].1) <= f64::EPSILON);
    //     }

    //     let hpoints = solution_velocity_tab.points[0].get().clone();
    //     let vpoints = solution_velocity_tab.points[1].get().clone();

    //     let new_unit = VelocityUnits::Mph;
    //     solution_velocity_tab.convert_points(new_unit);
    //     let new_hpoints = solution_velocity_tab.points[0].get();
    //     let new_vpoints = solution_velocity_tab.points[1].get();
    //     for idx in 0..hpoints.len() {
    //         assert!(f64::abs((*hpoints[idx].1 * MPS2MPH) - *new_hpoints[idx].1) <= f64::EPSILON);
    //         assert!(f64::abs((*vpoints[idx].1 * MPS2MPH) - *new_vpoints[idx].1) <= f64::EPSILON);
    //     }

    //     let hpoints = solution_velocity_tab.points[0].get().clone();
    //     let vpoints = solution_velocity_tab.points[1].get().clone();
    //     let new_unit = VelocityUnits::Kph;
    //     solution_velocity_tab.convert_points(new_unit);
    //     let new_hpoints = solution_velocity_tab.points[0].get();
    //     let new_vpoints = solution_velocity_tab.points[1].get();

    //     for idx in 0..hpoints.len() {
    //         assert!(
    //             f64::abs(*hpoints[idx].1 * (MPS2KPH / MPS2MPH) - *new_hpoints[idx].1)
    //                 <= f64::EPSILON
    //         );
    //         assert!(
    //             f64::abs(*vpoints[idx].1 * (MPS2KPH / MPS2MPH) - *new_vpoints[idx].1)
    //                 <= f64::EPSILON
    //         );
    //     }
    // }
}
