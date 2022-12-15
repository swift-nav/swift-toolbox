use std::collections::HashMap;

use capnp::message::Builder;
use log::error;
use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgUtcTime},
    orientation::MsgBaselineHeading,
    piksi::MsgResetFilters,
};

use crate::output::BaselineLog;
use crate::piksi_tools_constants::EMPTY_STR;
use crate::shared_state::{SharedState, TabName};
use crate::types::{BaselineNED, GnssModes, GpsTime, MsgSender, Result, RingBuffer, UtcDateTime};
use crate::utils::{date_conv::*, *};
use crate::zip;
use crate::{client_sender::BoxedClientSender, constants::*};

/// Baseline Tab Button Struct.
///
/// # Parameters
/// - `clear`: Indicates whether to initiate a clearing of all solution data stored.
/// - `pause`: Indicates whther or not to pause the plot updates.
/// - `reset`: Indicates whether or not to reset filters.
pub(crate) struct BaselineTabButtons {
    clear: bool,
    pause: bool,
    reset: bool,
}

/// BaselineTab struct.
///
/// # Fields
/// - `age_corrections`: Stored age corrections to be displayed in the table.
/// - `client_sender`: Client Sender channel for communication from backend to frontend.
/// - `heading`: The stored heading value.
/// - `last_mode`: The most recent gnss mode stored.
/// - `n_max`: The baseline north direction maximimum value storage for plot bounds.
/// - `n_min`: The baseline north direction minimum value storage for plot bounds.
/// - `n_max`: The baseline east direction maximimum value storage for plot bounds.
/// - `n_min`: The baseline east direction minimum value storage for plot bounds.
/// - `mode_strings`: The available modes in string form to store updates for.
/// - `nsec`: The stored nanosecond value from GPS Time messages.
/// - `pending_draw_modes`: A list of draw modes waiting to be updated.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `baseline_log_file`: The CsvSerializer corresponding to an open position log if any.
/// - `sln_cur_data`: The current most recent n/e point for each mode.
/// - `sln_data`: The preprocessed solution data to be sent to the frontend.
/// - `slns`: All solution data is stored before preparing for frontend.
/// - `table`: This stores all the key/value pairs to be displayed in the Baseline Table.
/// - `utc_source`: The string equivalent for the source of the UTC updates.
/// - `utc_time`: The stored monotonic Utc time.
/// - `week`: The stored week value from GPS Time messages.
pub struct BaselineTab {
    age_corrections: Option<f64>,
    client_sender: BoxedClientSender,
    heading: Option<f64>,
    last_mode: u8,
    n_max: f64,
    n_min: f64,
    e_max: f64,
    e_min: f64,
    mode_strings: Vec<String>,
    nsec: Option<i32>,
    pending_draw_modes: Vec<String>,
    shared_state: SharedState,
    sln_cur_data: Vec<Vec<(f64, f64)>>,
    sln_data: Vec<Vec<(f64, f64)>>,
    slns: HashMap<&'static str, RingBuffer<f64>>,
    table: HashMap<&'static str, String>,
    utc_source: Option<String>,
    utc_time: Option<UtcDateTime>,
    week: Option<u16>,
    writer: MsgSender,
}

impl BaselineTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
        writer: MsgSender,
    ) -> BaselineTab {
        let mut mode_strings = vec![
            GnssModes::Dgnss.to_string(),
            GnssModes::Float.to_string(),
            GnssModes::Fixed.to_string(),
        ];
        mode_strings.reserve_exact(mode_strings.len());
        BaselineTab {
            age_corrections: None,
            client_sender,
            heading: None,
            last_mode: 0,
            n_max: BASELINE_DIRECTION_MAX,
            n_min: BASELINE_DIRECTION_MIN,
            e_max: BASELINE_DIRECTION_MAX,
            e_min: BASELINE_DIRECTION_MIN,
            pending_draw_modes: Vec::with_capacity(mode_strings.len()),
            mode_strings,
            nsec: Some(0),
            shared_state,
            sln_cur_data: {
                let mut data = vec![Vec::with_capacity(1); NUM_GNSS_MODES];
                data.reserve_exact(NUM_GNSS_MODES);
                data
            },
            sln_data: {
                let mut data = vec![Vec::with_capacity(PLOT_HISTORY_MAX); NUM_GNSS_MODES];
                data.reserve_exact(NUM_GNSS_MODES);
                data
            },
            slns: {
                BASELINE_DATA_KEYS
                    .iter()
                    .map(|key| (*key, RingBuffer::new(PLOT_HISTORY_MAX)))
                    .collect()
            },
            table: {
                BASELINE_TABLE_KEYS
                    .iter()
                    .map(|key| (*key, EMPTY_STR.to_string()))
                    .collect()
            },
            utc_source: None,
            utc_time: None,
            week: None,
            writer,
        }
    }

    /// Prepare data to be sent to frontend for a single mode.
    ///
    /// # Parameters
    /// - `mode_string`: The mode string to attempt to prepare data for frontend.
    /// - `update_current`: Indicating whether the current solution should be updated by
    /// this modes last n/e entry.
    fn _synchronize_plot_data_by_mode(&mut self, mode_string: &str, update_current: bool) {
        let mode_idx = match self.mode_strings.iter().position(|x| *x == *mode_string) {
            Some(idx) => idx,
            _ => return,
        };

        let n_string = format!("n_{mode_string}");
        let e_string = format!("e_{mode_string}");
        let n_values = &self.slns[&*n_string];
        let e_values = &self.slns[&*e_string];

        self.sln_data[mode_idx].clear();
        for (n, e) in zip!(n_values, e_values) {
            if n.is_nan() || e.is_nan() {
                continue;
            }
            self.n_min = n.min(self.n_min);
            self.n_max = n.max(self.n_max);
            self.e_min = e.min(self.e_min);
            self.e_max = e.max(self.e_max);
            self.sln_data[mode_idx].push((*e, *n));
        }
        self.sln_cur_data[mode_idx].clear();
        if update_current && !self.sln_data[mode_idx].is_empty() {
            self.sln_cur_data[mode_idx]
                .push(self.sln_data[mode_idx][self.sln_data[mode_idx].len() - 1]);
        }
    }

    /// Append NANs to all modes unless explicitly excluded.
    ///
    /// # Parameters:
    /// - `exclude_mode`: The mode as a string not to update. Otherwise, None.
    fn _append_empty_sln_data(&mut self, exclude_mode: Option<String>) {
        for each_mode in self.mode_strings.iter() {
            if exclude_mode == Some(each_mode.clone()) {
                continue;
            }
            let n_str = format!("n_{each_mode}",);
            let e_str = format!("e_{each_mode}");
            self.slns.get_mut(&*n_str).unwrap().push(f64::NAN);
            self.slns.get_mut(&*e_str).unwrap().push(f64::NAN);
        }
    }

    /// Update a single mode's solution data for with lat and lon values.
    ///
    ///
    /// # Parameters
    /// - `last_n`: The baseline north coordinate in meters.
    /// - `last_e`: The baseline east coordinate in meters.
    /// - `mode_string`: The mode associated with the update in string form.
    fn _update_sln_data_by_mode(&mut self, last_n: f64, last_e: f64, mode_string: String) {
        let n_str = format!("n_{mode_string}");
        let e_str = format!("e_{mode_string}");
        self.slns.get_mut(&*n_str).unwrap().push(last_n);
        self.slns.get_mut(&*e_str).unwrap().push(last_e);
        self._append_empty_sln_data(Some(mode_string));
    }

    pub fn clear_sln(&mut self) {
        for (_, deque) in &mut self.slns.iter_mut() {
            deque.clear();
        }
    }

    /// Handle Age Corrections messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgAgeCorrections to extract data from.
    pub fn handle_age_corrections(&mut self, msg: MsgAgeCorrections) {
        self.age_corrections = if msg.age != 0xFFFF {
            Some(decisec_to_sec(msg.age as f64))
        } else {
            None
        };
    }

    /// Handler for GPS time messages.
    ///
    /// # Parameters
    /// - `msg`: GpsTime to extract data from.
    pub fn handle_gps_time(&mut self, msg: GpsTime) {
        let gps_time_fields = msg.fields();
        if gps_time_fields.flags != 0 {
            self.week = Some(gps_time_fields.wn);
            self.nsec = Some(gps_time_fields.ns_residual);
        }
    }

    /// Handler for UTC time messages.
    ///
    /// # Parameters
    /// - `msg`: MsgUtcTime to extract data from.
    pub fn handle_utc_time(&mut self, msg: MsgUtcTime) {
        if msg.flags & 0x1 == 1 {
            self.utc_time = Some(utc_time_from_msg(&msg));
            self.utc_source = Some(utc_source(msg.flags));
        } else {
            self.utc_time = None;
            self.utc_source = None;
        }
    }

    /// Handler for Baseline Heading messages.
    ///
    /// # Parameters
    /// - `msg`: MsgBaselineHeading to extract data from.
    pub fn handle_baseline_heading(&mut self, msg: MsgBaselineHeading) {
        if msg.flags & 0x7 != 0 {
            self.heading = Some(mdeg_to_deg(msg.heading as f64));
        } else {
            self.heading = None;
        }
    }

    /// Handle MsgBaselineNed / MsgBaselineNedDepA messages.
    ///
    /// # Parameters
    /// - `msg`: MsgBaselineNed / MsgBaselineNedDepA to extract data from.
    pub fn handle_baseline_ned(&mut self, msg: BaselineNED) {
        let baseline_ned_fields = msg.fields();
        let n = mm_to_m(baseline_ned_fields.n as f64);
        let e = mm_to_m(baseline_ned_fields.e as f64);
        let d = mm_to_m(baseline_ned_fields.d as f64);
        let h_accuracy = mm_to_m(baseline_ned_fields.h_accuracy as f64);
        let v_accuracy = mm_to_m(baseline_ned_fields.v_accuracy as f64);

        let dist = euclidean_distance([n, e, d].iter());

        let mut tow = ms_to_sec(baseline_ned_fields.tow);
        if let Some(nsec) = self.nsec {
            tow += ns_to_sec(nsec as f64);
        }

        let (tloc, secloc) = convert_local_time_to_logging_format();
        let (tgps_, secgps_) = convert_gps_time_to_logging_format(self.week, tow);

        let mut utc_time_str = None;
        if let Some(utc_time_) = self.utc_time {
            let (tutc, secutc) = datetime_to_string_and_seconds(utc_time_);
            utc_time_str = Some(format!("{}:{:0>6.03}", tutc, secutc));
        }
        let mut gps_time = None;
        let mut gps_time_short = None;
        if let (Some(tgps), Some(secgps)) = (tgps_, secgps_) {
            gps_time = Some(format!("{tgps}:{secgps:0>6.06}"));
            gps_time_short = Some(format!("{tgps}:{secgps:0>6.03}"));
        }

        {
            let mut shared_data = self.shared_state.lock();
            if let Some(ref mut baseline_file) = shared_data.baseline_tab.log_file {
                let pc_time = format!("{tloc}:{secloc:0>6.06}");
                if let Err(err) = baseline_file.serialize(&BaselineLog {
                    pc_time,
                    gps_time,
                    tow_s: Some(tow),
                    north_m: Some(n),
                    east_m: Some(e),
                    down_m: Some(d),
                    h_accuracy_m: Some(h_accuracy),
                    v_accuracy_m: Some(v_accuracy),
                    distance_m: Some(dist),
                    flags: baseline_ned_fields.flags,
                    num_sats: baseline_ned_fields.n_sats,
                }) {
                    error!("Unable to to write to baseline log, error {}.", err);
                }
            }
        }

        self.last_mode = msg.mode();

        let gnss_mode = GnssModes::from(self.last_mode);
        let mode_string = gnss_mode.to_string();

        if self.last_mode == 0 {
            self.table.insert(GPS_WEEK, String::from(EMPTY_STR));
            self.table.insert(GPS_TOW, String::from(EMPTY_STR));
            self.table.insert(GPS_TIME, String::from(EMPTY_STR));
            self.table.insert(UTC_TIME, String::from(EMPTY_STR));
            self.table.insert(UTC_SRC, String::from(EMPTY_STR));
            self.table.insert(NORTH, String::from(EMPTY_STR));
            self.table.insert(EAST, String::from(EMPTY_STR));
            self.table.insert(DOWN, String::from(EMPTY_STR));
            self.table.insert(HORIZ_ACC, String::from(EMPTY_STR));
            self.table.insert(VERT_ACC, String::from(EMPTY_STR));
            self.table.insert(DIST, String::from(EMPTY_STR));
            self.table.insert(SATS_USED, String::from(EMPTY_STR));
            self.table.insert(FLAGS, String::from(EMPTY_STR));
            self.table.insert(MODE, String::from(EMPTY_STR));
            self.table.insert(HEADING, String::from(EMPTY_STR));
            self.table.insert(CORR_AGE_S, String::from(EMPTY_STR));
        } else {
            if let Some(week) = self.week {
                self.table.insert(GPS_WEEK, week.to_string());
                if let Some(gps_time_) = gps_time_short {
                    self.table.insert(GPS_TIME, gps_time_);
                }
            }
            self.table.insert(GPS_TOW, format!("{:.3}", tow));
            if let Some(utc_time_) = utc_time_str {
                self.table.insert(UTC_TIME, utc_time_);
                if let Some(utc_src_) = self.utc_source.clone() {
                    self.table.insert(UTC_SRC, utc_src_);
                }
            } else {
                self.table.insert(UTC_TIME, String::from(EMPTY_STR));
                self.table.insert(UTC_SRC, String::from(EMPTY_STR));
            }
            self.table.insert(NORTH, n.to_string());
            self.table.insert(EAST, e.to_string());
            self.table.insert(DOWN, d.to_string());
            self.table.insert(HORIZ_ACC, h_accuracy.to_string());
            self.table.insert(VERT_ACC, v_accuracy.to_string());
            self.table.insert(DIST, format!("{dist:.3}"));
            self.table
                .insert(SATS_USED, baseline_ned_fields.n_sats.to_string());
            self.table
                .insert(FLAGS, format!("0x{:<02x}", baseline_ned_fields.flags));
            self.table.insert(MODE, mode_string.clone());
            if let Some(heading_) = self.heading {
                self.table.insert(HEADING, heading_.to_string());
            }
            if let Some(age_corrections_) = self.age_corrections {
                self.table
                    .insert(CORR_AGE_S, format!("{age_corrections_:.1}"));
            }
        }

        if self.last_mode != 0 {
            if !self.pending_draw_modes.contains(&mode_string) {
                self.pending_draw_modes.push(mode_string.clone());
            }
            self._update_sln_data_by_mode(n, e, mode_string);
        } else {
            self._append_empty_sln_data(None);
        }

        let buttons = self.check_state();
        self.solution_draw(buttons);
        if self.shared_state.current_tab() != TabName::Baseline {
            return;
        }
        self.send_solution_data();
        self.send_table_data();
    }

    fn check_state(&self) -> BaselineTabButtons {
        let mut shared_data = self.shared_state.lock();
        let clear = shared_data.baseline_tab.clear;
        shared_data.baseline_tab.clear = false;
        let pause = shared_data.baseline_tab.pause;
        let reset = shared_data.baseline_tab.reset;
        shared_data.baseline_tab.reset = false;
        BaselineTabButtons {
            clear,
            pause,
            reset,
        }
    }

    /// Reset Solution Filters
    fn reset_button_fired(&mut self) -> Result<()> {
        self.writer.send(MsgResetFilters {
            sender_id: Some(WRITE_TO_DEVICE_SENDER_ID),
            filter: 0,
        })
    }

    /// Initiates preprocessing of solution data and handles frontend input.
    ///
    /// TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-245
    /// Need to complete missing functionalities:
    /// - Center on solution
    /// - Handle zoom feature.
    /// - Reset Filters button.
    ///
    /// # Parameters
    /// - `buttons`: Instance of BaselineTabButtons which trigger different behaviors.
    fn solution_draw(&mut self, buttons: BaselineTabButtons) {
        if buttons.clear {
            self.clear_sln();
        } else if buttons.pause {
            return;
        }
        if buttons.reset {
            if let Err(err) = self.reset_button_fired() {
                error!("unable to reset solution filters, {}", err);
            }
        }
        let current_mode: Option<String> = if !self.pending_draw_modes.is_empty() {
            self.n_min = BASELINE_DIRECTION_MAX;
            self.n_max = BASELINE_DIRECTION_MIN;
            self.e_min = BASELINE_DIRECTION_MAX;
            self.e_max = BASELINE_DIRECTION_MIN;
            Some(self.pending_draw_modes[self.pending_draw_modes.len() - 1].clone())
        } else {
            None
        };
        let mut update_current = true;
        for mode_string in self.mode_strings.clone() {
            if let Some(cur_mode) = &current_mode {
                update_current = mode_string == *cur_mode;
            }

            self._synchronize_plot_data_by_mode(&mode_string, update_current);
            if self.pending_draw_modes.contains(&mode_string) {
                self.pending_draw_modes.retain(|x| x != &mode_string);
            }
        }
    }

    /// Package solution data into a message buffer and send to frontend.
    pub fn send_solution_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_baseline_plot_status();
        tab_status.set_n_min(self.n_min);
        tab_status.set_n_max(self.n_max);
        tab_status.set_e_min(self.e_min);
        tab_status.set_e_max(self.e_max);

        let mut solution_points = tab_status.reborrow().init_data(self.sln_data.len() as u32);
        for idx in 0..self.sln_data.len() {
            let points = self.sln_data.get_mut(idx).unwrap();
            let mut point_idx = solution_points
                .reborrow()
                .init(idx as u32, points.len() as u32);
            for (i, (x, y)) in points.iter().enumerate() {
                let mut point_val = point_idx.reborrow().get(i as u32);
                point_val.set_x(*x);
                point_val.set_y(*y);
            }
        }
        let mut solution_points = tab_status
            .reborrow()
            .init_cur_data(self.sln_cur_data.len() as u32);
        for idx in 0..self.sln_cur_data.len() {
            let points = self.sln_cur_data.get_mut(idx).unwrap();
            let mut point_idx = solution_points
                .reborrow()
                .init(idx as u32, points.len() as u32);
            for (i, (x, y)) in points.iter().enumerate() {
                let mut point_val = point_idx.reborrow().get(i as u32);
                point_val.set_x(*x);
                point_val.set_y(*y);
            }
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    /// Package solution table data into a message buffer and send to frontend.
    pub fn send_table_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut baseline_table_status = msg.init_baseline_table_status();
        let mut table_entries = baseline_table_status
            .reborrow()
            .init_data(self.table.len() as u32);
        {
            for (i, key) in BASELINE_TABLE_KEYS.iter().enumerate() {
                let mut entry = table_entries.reborrow().get(i as u32);
                let val = self.table[*key].clone();
                entry.set_key(key);
                entry.set_val(&val);
            }
        }
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use sbp::messages::navigation::{MsgBaselineNed, MsgBaselineNedDepA, MsgGpsTime};
    use std::io::sink;
    #[test]
    fn handle_age_corrections_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut baseline_tab = BaselineTab::new(shared_state, client_send, writer);
        assert!(baseline_tab.age_corrections.is_none());
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: 0xFFFF,
            tow: 0,
        };
        baseline_tab.handle_age_corrections(msg);
        assert!(baseline_tab.age_corrections.is_none());
        let good_age = 0x4DC6;
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: good_age,
            tow: 0,
        };
        baseline_tab.handle_age_corrections(msg);
        assert!(baseline_tab.age_corrections.is_some());
        if let Some(age) = baseline_tab.age_corrections {
            assert!(f64::abs(age - 1991_f64) <= f64::EPSILON);
        }
    }

    #[test]
    fn handle_gps_time_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut baseline_table = BaselineTab::new(shared_state, client_send, writer);
        let wn = 0_u16;
        let ns_residual = 1337_i32;
        let bad_flags = 0_u8;
        let msg = MsgGpsTime {
            sender_id: Some(1337),
            wn,
            tow: 0,
            ns_residual,
            flags: bad_flags,
        };
        let old_wn = 5_u16;
        let old_nsec = 678_i32;
        baseline_table.week = Some(old_wn);
        baseline_table.nsec = Some(old_nsec);
        baseline_table.handle_gps_time(GpsTime::MsgGpsTime(msg));
        assert_eq!(baseline_table.week, Some(old_wn));
        assert_eq!(baseline_table.nsec, Some(old_nsec));

        let good_flags = 1_u8;
        let msg = MsgGpsTime {
            sender_id: Some(1337),
            wn,
            tow: 0,
            ns_residual,
            flags: good_flags,
        };
        baseline_table.handle_gps_time(GpsTime::MsgGpsTime(msg));
        assert_eq!(baseline_table.week, Some(wn));
        assert_eq!(baseline_table.nsec, Some(ns_residual));
    }

    #[test]
    fn handle_utc_time_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut baseline_table = BaselineTab::new(shared_state, client_send, writer);
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
        baseline_table.utc_time = None;
        baseline_table.utc_source = None;
        baseline_table.handle_utc_time(msg);
        assert_eq!(baseline_table.utc_time, None);
        assert_eq!(baseline_table.utc_source, None);
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
        baseline_table.utc_time = None;
        baseline_table.utc_source = None;
        baseline_table.handle_utc_time(msg);
        let datetime = utc_time(
            year as i32,
            month as u32,
            day as u32,
            hours as u32,
            minutes as u32,
            seconds as u32,
            ns,
        );
        assert_eq!(baseline_table.utc_time, Some(datetime));
        assert_eq!(
            baseline_table.utc_source,
            Some(String::from(NON_VOLATILE_MEMORY))
        );
    }

    #[test]
    fn handle_baseline_heading_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut baseline_tab = BaselineTab::new(shared_state, client_send, writer);
        assert!(baseline_tab.heading.is_none());
        let heading = 1337;
        let msg = MsgBaselineHeading {
            sender_id: Some(1337),
            heading,
            n_sats: 1,
            flags: 0x0,
            tow: 0,
        };
        baseline_tab.handle_baseline_heading(msg);
        assert!(baseline_tab.heading.is_none());
        let good_flags = 0xF;
        let msg = MsgBaselineHeading {
            sender_id: Some(1337),
            heading,
            n_sats: 1,
            flags: good_flags,
            tow: 0,
        };
        baseline_tab.handle_baseline_heading(msg);
        assert!(baseline_tab.heading.is_some());
        if let Some(heading_) = baseline_tab.heading {
            assert!(f64::abs(heading_ - heading as f64 * 1.0e-3_f64) <= f64::EPSILON);
        }
    }

    #[test]
    fn handle_baseline_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut baseline_tab = BaselineTab::new(shared_state, client_send, writer);
        baseline_tab.utc_time = Some(utc_time(1_i32, 3_u32, 3_u32, 7_u32, 6_u32, 6_u32, 6_u32));
        baseline_tab.utc_source = Some(utc_source(0x02));
        baseline_tab.nsec = Some(1337);
        baseline_tab.week = Some(13);
        let bad_flags = 0;
        let n = 45_i32;
        let e = -45_i32;
        let d = 1337_i32;
        let n_sats = 13;
        let h_accuracy = 0;
        let v_accuracy = 0;
        let tow = 1337;
        let msg = BaselineNED::MsgBaselineNed(MsgBaselineNed {
            sender_id: Some(1337),
            flags: bad_flags,
            n,
            e,
            d,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });

        assert_eq!(baseline_tab.last_mode, 0);
        assert_eq!(baseline_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[NORTH], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[EAST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DOWN], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DIST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[FLAGS], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[MODE], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HEADING], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[CORR_AGE_S], String::from(EMPTY_STR));
        baseline_tab.handle_baseline_ned(msg);
        assert_eq!(baseline_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[NORTH], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[EAST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DOWN], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DIST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[FLAGS], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[MODE], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HEADING], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[CORR_AGE_S], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.last_mode, 0);

        let good_flags = 0xF;
        let msg = MsgBaselineHeading {
            sender_id: Some(1337),
            heading: 1337,
            n_sats: 1,
            flags: good_flags,
            tow: 0,
        };
        baseline_tab.handle_baseline_heading(msg);

        let good_age = 0x4DC6;
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: good_age,
            tow: 0,
        };
        baseline_tab.handle_age_corrections(msg);

        let good_flags = 0x02;
        let msg = BaselineNED::MsgBaselineNed(MsgBaselineNed {
            sender_id: Some(1337),
            flags: good_flags,
            n,
            e,
            d,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });
        assert_eq!(baseline_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[NORTH], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[EAST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DOWN], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[DIST], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[FLAGS], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[MODE], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[HEADING], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.table[CORR_AGE_S], String::from(EMPTY_STR));
        baseline_tab.handle_baseline_ned(msg);
        assert_ne!(baseline_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[NORTH], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[EAST], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[DOWN], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[DIST], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[FLAGS], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[MODE], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[HEADING], String::from(EMPTY_STR));
        assert_ne!(baseline_tab.table[CORR_AGE_S], String::from(EMPTY_STR));
        assert_eq!(baseline_tab.last_mode, good_flags);

        assert_eq!(baseline_tab.last_mode, 2);

        let msg = BaselineNED::MsgBaselineNedDepA(MsgBaselineNedDepA {
            sender_id: Some(1337),
            flags: good_flags,
            n,
            e,
            d,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });
        baseline_tab.handle_baseline_ned(msg);
        assert_eq!(baseline_tab.last_mode, 2);
    }

    #[test]
    fn check_state_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let baseline_tab = BaselineTab::new(shared_state, client_send, writer);
        let buttons = baseline_tab.check_state();
        assert!(!buttons.clear);
        assert!(!buttons.pause);
        assert!(!buttons.reset);
        {
            let mut shared_data = baseline_tab.shared_state.lock();
            shared_data.baseline_tab.clear = true;
            shared_data.baseline_tab.pause = true;
            shared_data.baseline_tab.reset = true;
        }
        let buttons = baseline_tab.check_state();
        assert!(buttons.clear);
        assert!(buttons.pause);
        assert!(buttons.reset);
        let buttons = baseline_tab.check_state();
        assert!(!buttons.clear);
        assert!(buttons.pause);
        assert!(!buttons.reset);
    }

    // #[test]
    // fn solution_draw_test() {
    // }
    // #[test]
    // fn update_sln_data_by_mode_test() {
    // }
    // #[test]
    // fn append_empty_sln_data_test() {
    // }
    // #[test]
    // fn synchronize_plot_data_by_mode_test() {
    // }
}
