use capnp::message::Builder;

use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgGPSTime, MsgUtcTime},
    orientation::MsgBaselineHeading,
};
use std::collections::HashMap;

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::date_conv::*;
use crate::errors::*;
use crate::output::{BaselineLog, CsvSerializer};
use crate::piksi_tools_constants::{mode_dict, EMPTY_STR};
use crate::types::{BaselineNED, Deque, GnssModes, MessageSender, SharedState, UtcDateTime};
use crate::utils::*;

/// BaselineTab struct.
///
/// # Fields
/// - `age_corrections`: Stored age corrections to be displayed in the table.
/// - `available_units` - The available units of measure to send to frontend for selection.
/// - `colors`: Stored rgb codes for frontend correspond to index of sv_labels.
/// - `directory_name`: The directory path to use when creating vel/pos logs.
/// - `ins_status_flags`: The stored ins status flags expected by other tabs.
/// - `ins_used`: Indicates whether or not ins is currently used.
/// - `labels`: The labels to show for the position tab modes.
/// - `lats`: The stored latitude values for quickly extracting aggregate data.
/// - `lngs`: The stored longitude values for quickly extracting aggregate data.
/// - `last_ins_status_receipt_time`: The last ins status receipt monotonic time stored.
/// - `last_mode`: The most recent gnss mode stored.
/// - `last_odo_update_time`: The last odo update monotonic time stored.
/// - `logging`: Indicates whether or not to log PosLLH/VelNED data to csvs.
/// - `mode_strings`: The available modes in string formm to store updates for.
/// - `modes`: The stored mode values used for quickly extracting aggregate data.
/// - `nsec`: The stored nanosecond value from GPS Time messages.
/// - `pending_draw_modes`: A list of draw modes waiting to be updated.
/// - `pos_log_file`: The CsvSerializer corresponding to an open position log if any.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `sln_cur_data`: The current most recent lat/lon point for each mode.
/// - `sln_data`: The preprocessed solution data to be sent to the frontend.
/// - `slns`: All solution data is stored before preparing for frontend.
/// - `table`: This stores all the key/value pairs to be displayed in the Solution Table.
/// - `unit`: The current unit of measure for the solution position plot.
/// - `utc_source`: The string equivalent for the source of the UTC updates.
/// - `utc_time`: The stored monotonic Utc time.
/// - `baseline_log_file`: The CsvSerializer corresponding to an open velocity log if any.
/// - `week`: The stored week value from GPS Time messages.
#[derive(Debug)]
pub struct BaselineTab<'a, S: MessageSender> {
    age_corrections: Option<f64>,
    available_units: [&'a str; 2],
    client_sender: S,
    colors: Vec<String>,
    directory_name: Option<String>,
    heading: Option<f64>,
    ins_status_flags: u32,
    ins_used: bool,
    labels: Vec<String>,
    last_mode: u8,
    logging: bool,
    n_max: f64,
    n_min: f64,
    e_max: f64,
    e_min: f64,
    mode_strings: Vec<String>,
    modes: Deque<u8>,
    nsec: Option<i32>,
    pending_draw_modes: Vec<String>,
    shared_state: SharedState,
    sln_cur_data: Vec<Vec<(f64, f64)>>,
    sln_data: Vec<Vec<(f64, f64)>>,
    slns: HashMap<&'a str, Deque<f64>>,
    table: HashMap<&'a str, String>,
    utc_source: Option<String>,
    utc_time: Option<UtcDateTime>,
    baseline_log_file: Option<CsvSerializer>,
    week: Option<u16>,
}

impl<'a, S: MessageSender> BaselineTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> BaselineTab<'a, S> {
        BaselineTab {
            age_corrections: None,
            available_units: [DEGREES, METERS],
            client_sender,
            colors: {
                vec![
                    GnssModes::Dgnss.color(),
                    GnssModes::Float.color(),
                    GnssModes::Fixed.color(),
                ]
            },
            directory_name: None,
            heading: None,
            ins_status_flags: 0,
            ins_used: false,
            labels: {
                vec![
                    GnssModes::Dgnss.label(),
                    GnssModes::Float.label(),
                    GnssModes::Fixed.label(),
                ]
            },
            last_mode: 0,
            logging: false,
            n_max: BASELINE_DIRECTION_MAX,
            n_min: BASELINE_DIRECTION_MIN,
            e_max: BASELINE_DIRECTION_MAX,
            e_min: BASELINE_DIRECTION_MIN,
            modes: Deque::with_size_limit(PLOT_HISTORY_MAX, /*fill_value=*/ None),
            mode_strings: vec![
                GnssModes::Dgnss.to_string(),
                GnssModes::Float.to_string(),
                GnssModes::Fixed.to_string(),
            ],
            nsec: Some(0),
            pending_draw_modes: vec![],
            shared_state,
            sln_cur_data: { vec![vec![]; NUM_GNSS_MODES as usize] },
            sln_data: { vec![vec![]; NUM_GNSS_MODES as usize] },
            slns: {
                BASELINE_DATA_KEYS
                    .iter()
                    .map(|key| {
                        (
                            *key,
                            Deque::with_size_limit(PLOT_HISTORY_MAX, /*fill_value=*/ None),
                        )
                    })
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
            baseline_log_file: None,
            week: None,
        }
    }

    /*
    def _zoomall_button_fired(self):
        self.zoomall = not self.zoomall

    def _center_button_fired(self):
        self.position_centered = not self.position_centered

    def _paused_button_fired(self):
        self.running = not self.running

    def _reset_button_fired(self):
        self.link(MsgResetFilters(filter=0))

    def _get_update_current(self, current_dict={}):
        out_dict = {'cur_n_fixed': [],
                    'cur_e_fixed': [],
                    'cur_d_fixed': [],
                    'cur_n_float': [],
                    'cur_e_float': [],
                    'cur_d_float': [],
                    'cur_n_dgnss': [],
                    'cur_e_dgnss': [],
                    'cur_d_dgnss': []
                    }
        out_dict.update(current_dict)
        return out_dict


    */

    /// Prepare data to be sent to frontend for a single mode.
    ///
    /// # Parameters
    /// - `mode_string`: The mode string to attempt to prepare data for frontend.
    /// - `update_current`: Indicating whether the current solution should be updated by
    /// this modes last n/e entry.
    fn _synchronize_plot_data_by_mode(&mut self, mode_string: &str, update_current: bool) {
        let n_string = format!("n_{}", mode_string);
        let e_string = format!("e_{}", mode_string);

        if let Some(idx) = self.mode_strings.iter().position(|x| *x == *mode_string) {
            let n_values = self.slns[&*n_string].get();
            let e_values = self.slns[&*e_string].get();
            let mut new_sln: Vec<(f64, f64)> = vec![];
            for jdx in 0..n_values.len() {
                if n_values[jdx].is_nan() || e_values[jdx].is_nan() {
                    continue;
                }
                self.n_min = f64::min(self.n_min, n_values[jdx]);
                self.n_max = f64::max(self.n_max, n_values[jdx]);
                self.e_min = f64::min(self.e_min, e_values[jdx]);
                self.e_max = f64::max(self.e_max, e_values[jdx]);
                new_sln.push((e_values[jdx], n_values[jdx]));
            }
            self.sln_data[idx] = new_sln;

            if update_current {
                if !self.sln_data[idx].is_empty() {
                    self.sln_cur_data[idx] = vec![self.sln_data[idx][self.sln_data[idx].len() - 1]];
                } else {
                    self.sln_cur_data[idx].clear();
                }
            }
        }
    }

    /// Append NANs to all modes unless explicitly excluded.
    ///
    /// # Parameters:
    /// - `exclude_mode`: The mode as a string not to update. Otherwise, None.
    fn _append_empty_sln_data(&mut self, exclude_mode: Option<String>) {
        for each_mode in self.mode_strings.iter() {
            if exclude_mode.is_some() {
                continue;
            }
            let n_str = format!("n_{}", each_mode);
            let e_str = format!("e_{}", each_mode);
            self.slns.get_mut(&*n_str).unwrap().add(f64::NAN);
            self.slns.get_mut(&*e_str).unwrap().add(f64::NAN);
        }
    }

    /// Update a single mode's solution data for with lat and lon values.
    ///
    /// TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    /// Need to implement offset and multiplier logic.
    ///
    /// # Parameters
    /// - `last_n`: The baseline north coordinate in meters.
    /// - `last_e`: The baseline east coordinate in meters.
    /// - `mode_string`: The mode associated with the update in string form.
    fn _update_sln_data_by_mode(&mut self, last_n: f64, last_e: f64, mode_string: String) {
        let n_str = format!("n_{}", mode_string);
        let e_str = format!("e_{}", mode_string);
        self.slns.get_mut(&*n_str).unwrap().add(last_n);
        self.slns.get_mut(&*e_str).unwrap().add(last_e);
        self._append_empty_sln_data(Some(mode_string));
    }

    pub fn clear_sln(&mut self) {
        for (_, val) in &mut self.slns.iter_mut() {
            let deque = val;
            deque.clear();
        }
    }

    /// Handle Age Corrections messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgAgeCorrections to extract data from.
    pub fn handle_age_corrections(&mut self, msg: MsgAgeCorrections) {
        if msg.age != 0xFFFF {
            self.age_corrections = Some(decisec_to_sec(msg.age as f64));
        } else {
            self.age_corrections = None;
        }
    }

    /// Handler for GPS time messages.
    ///
    /// # Parameters
    /// - `msg`: MsgGPSTime to extract data from.
    pub fn handle_gps_time(&mut self, msg: MsgGPSTime) {
        if msg.flags != 0 {
            self.week = Some(msg.wn);
            self.nsec = Some(msg.ns_residual);
        }
    }

    /// Handler for UTC time messages.
    ///
    /// # Parameters
    /// - `msg`: MsgUtcTime to extract data from.
    pub fn handle_utc_time(&mut self, msg: MsgUtcTime) {
        if msg.flags & 0x1 == 1 {
            self.utc_time = Some(utc_time(
                msg.year as i32,
                msg.month as u32,
                msg.day as u32,
                msg.hours as u32,
                msg.minutes as u32,
                msg.seconds as u32,
                msg.ns as u32,
            ));
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

    /// Handle MsgBaselineNED / MsgBaselineNEDDepA messages.
    ///
    /// # Parameters
    /// - `msg`: MsgBaselineNED / MsgBaselineNEDDepA to extract data from.
    pub fn handle_baseline_ned(&mut self, msg: BaselineNED) {
        let baseline_ned_fields = msg.fields();
        let n = mm_to_m(baseline_ned_fields.n as f64);
        let e = mm_to_m(baseline_ned_fields.e as f64);
        let d = mm_to_m(baseline_ned_fields.d as f64);
        let h_accuracy = mm_to_m(baseline_ned_fields.h_accuracy as f64);
        let v_accuracy = mm_to_m(baseline_ned_fields.v_accuracy as f64);

        let dist: f64 = f64::sqrt(f64::powi(n, 2) + f64::powi(e, 2) + f64::powi(d, 2));

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
        if let Some(tgps) = tgps_ {
            if let Some(secgps) = secgps_ {
                gps_time = Some(format!("{}:{:0>6.06}", tgps, secgps));
                gps_time_short = Some(format!("{}:{:0>6.03}", tgps, secgps));
            }
        }

        if let Some(baseline_file) = &mut self.baseline_log_file {
            let pc_time = format!("{}:{:0>6.06}", tloc, secloc);
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
                eprintln!("Unable to to write to baseline log, error {}.", err);
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
            self.table.insert(N, String::from(EMPTY_STR));
            self.table.insert(E, String::from(EMPTY_STR));
            self.table.insert(D, String::from(EMPTY_STR));
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
            self.table.insert(N, format!("{:.12}", n));
            self.table.insert(E, format!("{:.12}", e));
            self.table.insert(D, format!("{:.12}", d));
            self.table
                .insert(HORIZ_ACC, format!("{:.12}", baseline_ned_fields.h_accuracy));
            self.table
                .insert(VERT_ACC, format!("{:.12}", baseline_ned_fields.v_accuracy));
            self.table.insert(DIST, format!("{:.3}", dist));
            self.table
                .insert(SATS_USED, baseline_ned_fields.n_sats.to_string());
            self.table
                .insert(FLAGS, format!("0x{:<02x}", baseline_ned_fields.flags));
            self.table.insert(MODE, mode_string.clone());
            if let Some(heading_) = self.heading {
                self.table.insert(HEADING, heading_.to_string());
            }
            if let Some(age_corrections_) = self.age_corrections {
                self.table.insert(CORR_AGE_S, age_corrections_.to_string());
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

        let (center, clear, pause, zoom) = self.check_state();
        self.solution_draw(center, clear, pause, zoom);
        // self.send_solution_data();
        // self.send_table_data();
    }

    pub fn check_state(&self) -> (bool, bool, bool, bool) {
        let mut shared_data = self.shared_state.lock().unwrap();
        let center = (*shared_data).solution_tab.position_tab.center;
        let clear = (*shared_data).solution_tab.position_tab.clear;
        (*shared_data).solution_tab.position_tab.clear = false;
        let pause = (*shared_data).solution_tab.position_tab.pause;
        let zoom = (*shared_data).solution_tab.position_tab.zoom;
        (center, clear, pause, zoom)
    }

    /// Initiates preprocessing of solution data and handles frontend input.
    ///
    /// TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    /// Need to complete missing functionalities:
    /// - Center on solution
    /// - Change unit of measure
    /// - Handle zoom feature.
    ///
    /// # Parameters
    /// - `center`: Indicates to whether or not to center on the current solution on the frontend.
    /// - `clear`: Indicates whether to initiate a clearing of all solution data stored.
    /// - `pause`: Indicates whther or not to pause the plot updates.
    /// - `unit`: The current unit of measure to cast the data to.
    /// - `zoom`: Indicates whether or not to zoom into the solution.
    pub fn solution_draw(&mut self, _center: bool, clear: bool, pause: bool, _zoom: bool) {
        if clear {
            self.clear_sln();
        }
        if pause {
            return;
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

    // /// Package solution data into a message buffer and send to frontend.
    // fn send_solution_data(&mut self) {
    //     let mut builder = Builder::new_default();
    //     let msg = builder.init_root::<m::message::Builder>();

    //     let mut solution_status = msg.init_solution_position_status();
    //     solution_status.set_n_min(self.n_min);
    //     solution_status.set_n_max(self.n_max);
    //     solution_status.set_e_min(self.e_min);
    //     solution_status.set_e_max(self.e_max);

    //     let mut solution_points = solution_status
    //         .reborrow()
    //         .init_data(self.sln_data.len() as u32);
    //     for idx in 0..self.sln_data.len() {
    //         let points = self.sln_data.get_mut(idx).unwrap();
    //         let mut point_idx = solution_points
    //             .reborrow()
    //             .init(idx as u32, points.len() as u32);
    //         for (i, (x, y)) in points.iter().enumerate() {
    //             let mut point_val = point_idx.reborrow().get(i as u32);
    //             point_val.set_x(*x);
    //             point_val.set_y(*y);
    //         }
    //     }
    //     let mut available_units = solution_status
    //         .reborrow()
    //         .init_available_units(self.available_units.len() as u32);
    //     for (i, unit) in self.available_units.iter().enumerate() {
    //         available_units.set(i as u32, *unit);
    //     }
    //     let mut solution_points = solution_status
    //         .reborrow()
    //         .init_cur_data(self.sln_cur_data.len() as u32);
    //     for idx in 0..self.sln_cur_data.len() {
    //         let points = self.sln_cur_data.get_mut(idx).unwrap();
    //         let mut point_idx = solution_points
    //             .reborrow()
    //             .init(idx as u32, points.len() as u32);
    //         for (i, (x, y)) in points.iter().enumerate() {
    //             let mut point_val = point_idx.reborrow().get(i as u32);
    //             point_val.set_x(*x);
    //             point_val.set_y(*y);
    //         }
    //     }
    //     let mut colors = solution_status
    //         .reborrow()
    //         .init_colors(self.colors.len() as u32);

    //     for (i, color) in self.colors.iter().enumerate() {
    //         colors.set(i as u32, color);
    //     }

    //     let mut labels = solution_status
    //         .reborrow()
    //         .init_labels(self.labels.len() as u32);

    //     for (i, label) in self.labels.iter().enumerate() {
    //         labels.set(i as u32, label);
    //     }

    //     self.client_sender
    //         .send_data(serialize_capnproto_builder(builder));
    // }

    // /// Package solution table data into a message buffer and send to frontend.
    // fn send_table_data(&mut self) {
    //     let mut builder = Builder::new_default();
    //     let msg = builder.init_root::<m::message::Builder>();
    //     let mut solution_table_status = msg.init_solution_table_status();
    //     let mut table_entries = solution_table_status
    //         .reborrow()
    //         .init_data(self.table.len() as u32);
    //     {
    //         for (i, key) in SOLUTION_TABLE_KEYS.iter().enumerate() {
    //             let mut entry = table_entries.reborrow().get(i as u32);
    //             let val = self.table[*key].clone();
    //             entry.set_key(key);
    //             entry.set_val(&val);
    //         }
    //     }
    //     self.client_sender
    //         .send_data(serialize_capnproto_builder(builder));
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::types::TestSender;
//     use chrono::{TimeZone, Utc};
//     use sbp::messages::navigation::{
//         MsgAgeCorrections, MsgDops, MsgDopsDepA, MsgPosLLH, MsgPosLLHDepA, MsgVelNED, MsgVelNEDDepA,
//     };
//     use std::{thread::sleep, time::Duration};

//     #[test]
//     fn handle_utc_time_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_table = BaselineTab::new(shared_state, client_send);
//         let year = 2020_u16;
//         let month = 3_u8;
//         let day = 19_u8;
//         let hours = 13_u8;
//         let minutes = 3_u8;
//         let seconds = 7_u8;
//         let ns = 666_u32;
//         let bad_flags = 0x00_u8;
//         let tow = 1337_u32;
//         let msg: MsgUtcTime = MsgUtcTime {
//             sender_id: Some(1337),
//             year,
//             month,
//             day,
//             hours,
//             minutes,
//             seconds,
//             ns,
//             flags: bad_flags,
//             tow,
//         };
//         solution_table.utc_time = None;
//         solution_table.utc_source = None;
//         solution_table.handle_utc_time(msg);
//         assert_eq!(solution_table.utc_time, None);
//         assert_eq!(solution_table.utc_source, None);
//         let good_flags = 0x0f_u8;
//         let msg: MsgUtcTime = MsgUtcTime {
//             sender_id: Some(1337),
//             year,
//             month,
//             day,
//             hours,
//             minutes,
//             seconds,
//             ns,
//             flags: good_flags,
//             tow,
//         };
//         solution_table.utc_time = None;
//         solution_table.utc_source = None;
//         solution_table.handle_utc_time(msg);
//         let datetime = Utc.ymd(year as i32, month as u32, day as u32).and_hms_nano(
//             hours as u32,
//             minutes as u32,
//             seconds as u32,
//             ns as u32,
//         );
//         assert_eq!(solution_table.utc_time, Some(datetime));
//         assert_eq!(
//             solution_table.utc_source,
//             Some(String::from(NON_VOLATILE_MEMORY))
//         );
//     }

//     #[test]
//     fn handle_gps_time_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_table = BaselineTab::new(shared_state, client_send);
//         let wn = 0_u16;
//         let ns_residual = 1337_i32;
//         let bad_flags = 0_u8;
//         let msg = MsgGPSTime {
//             sender_id: Some(1337),
//             wn,
//             tow: 0,
//             ns_residual,
//             flags: bad_flags,
//         };
//         let old_wn = 5_u16;
//         let old_nsec = 678_i32;
//         solution_table.week = Some(old_wn);
//         solution_table.nsec = Some(old_nsec);
//         solution_table.handle_gps_time(msg);
//         assert_eq!(solution_table.week, Some(old_wn));
//         assert_eq!(solution_table.nsec, Some(old_nsec));

//         let good_flags = 1_u8;
//         let msg = MsgGPSTime {
//             sender_id: Some(1337),
//             wn,
//             tow: 0,
//             ns_residual,
//             flags: good_flags,
//         };
//         solution_table.handle_gps_time(msg);
//         assert_eq!(solution_table.week, Some(wn));
//         assert_eq!(solution_table.nsec, Some(ns_residual));
//     }

//     #[test]
//     fn handle_vel_ned_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         let good_flags = 0x07;
//         let bad_flags = 0xF0;
//         let n = 1;
//         let e = 2;
//         let d = 3;
//         let n_sats = 13;
//         let msg = VelNED::MsgVelNED(MsgVelNED {
//             sender_id: Some(1337),
//             flags: bad_flags,
//             n,
//             e,
//             d,
//             n_sats,
//             tow: 0,
//             h_accuracy: 0,
//             v_accuracy: 0,
//         });
//         assert_eq!(solution_tab.table[VEL_FLAGS], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VEL_N], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VEL_E], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VEL_D], String::from(EMPTY_STR));
//         solution_tab.handle_vel_ned(msg);
//         assert_eq!(
//             solution_tab.table[VEL_FLAGS],
//             format!("0x{:<03x}", bad_flags)
//         );
//         assert_eq!(solution_tab.table[VEL_N], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VEL_E], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VEL_D], String::from(EMPTY_STR));
//         let msg = VelNED::MsgVelNED(MsgVelNED {
//             sender_id: Some(1337),
//             flags: good_flags,
//             n,
//             e,
//             d,
//             n_sats,
//             tow: 0,
//             h_accuracy: 0,
//             v_accuracy: 0,
//         });
//         solution_tab.handle_vel_ned(msg);
//         assert_eq!(
//             solution_tab.table[VEL_FLAGS],
//             format!("0x{:<03x}", good_flags)
//         );
//         assert_eq!(
//             solution_tab.table[VEL_N],
//             format!("{: >8.4}", n as f64 / 1000_f64)
//         );
//         assert_eq!(
//             solution_tab.table[VEL_E],
//             format!("{: >8.4}", e as f64 / 1000_f64)
//         );
//         assert_eq!(
//             solution_tab.table[VEL_D],
//             format!("{: >8.4}", d as f64 / 1000_f64)
//         );
//         let n = 3;
//         let e = 2;
//         let d = 1;
//         let msg = VelNED::MsgVelNEDDepA(MsgVelNEDDepA {
//             sender_id: Some(1337),
//             flags: good_flags,
//             n,
//             e,
//             d,
//             n_sats,
//             tow: 0,
//             h_accuracy: 0,
//             v_accuracy: 0,
//         });
//         solution_tab.handle_vel_ned(msg);
//         assert_eq!(solution_tab.table[VEL_FLAGS], format!("0x{:<03x}", 1));
//         assert_eq!(
//             solution_tab.table[VEL_N],
//             format!("{: >8.4}", n as f64 / 1000_f64)
//         );
//         assert_eq!(
//             solution_tab.table[VEL_E],
//             format!("{: >8.4}", e as f64 / 1000_f64)
//         );
//         assert_eq!(
//             solution_tab.table[VEL_D],
//             format!("{: >8.4}", d as f64 / 1000_f64)
//         );
//     }

//     #[test]
//     fn handle_ins_status_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         let flags = 0xf0_u32;
//         let msg = MsgInsStatus {
//             sender_id: Some(1337),
//             flags,
//         };
//         let update_time = Instant::now();
//         solution_tab.handle_ins_status(msg);
//         assert!(solution_tab.last_ins_status_receipt_time > update_time);
//         assert_eq!(solution_tab.ins_status_flags, flags);
//     }

//     #[test]
//     fn handle_ins_updates_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         let msg = MsgInsUpdates {
//             sender_id: Some(1337),
//             gnsspos: 0,
//             gnssvel: 0,
//             wheelticks: 0xf0_u8,
//             speed: 0,
//             nhc: 0,
//             zerovel: 0,
//             tow: 0,
//         };

//         let odo_update_time = Instant::now();
//         sleep(Duration::from_secs(1));
//         solution_tab.handle_ins_updates(msg);

//         assert!(solution_tab.last_odo_update_time > odo_update_time);

//         let msg = MsgInsUpdates {
//             sender_id: Some(1337),
//             gnsspos: 4,
//             gnssvel: 4,
//             wheelticks: 0xff_u8,
//             speed: 0,
//             nhc: 0,
//             zerovel: 0,
//             tow: 0,
//         };

//         let odo_update_time = Instant::now();
//         solution_tab.handle_ins_updates(msg);

//         assert!(solution_tab.last_odo_update_time < odo_update_time);
//     }

//     #[test]
//     fn handle_dops_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         let pdop = 1;
//         let gdop = 2;
//         let tdop = 3;
//         let hdop = 4;
//         let vdop = 5;
//         let good_flags = 1;
//         let bad_flags = 0;
//         let msg = Dops::MsgDops(MsgDops {
//             sender_id: Some(1337),
//             tow: 0,
//             pdop,
//             gdop,
//             tdop,
//             hdop,
//             vdop,
//             flags: bad_flags,
//         });
//         assert_eq!(solution_tab.table[PDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[TDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VDOP], String::from(EMPTY_STR));
//         solution_tab.handle_dops(msg);
//         assert_eq!(solution_tab.table[PDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[TDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HDOP], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VDOP], String::from(EMPTY_STR));
//         let msg = Dops::MsgDops(MsgDops {
//             sender_id: Some(1337),
//             tow: 0,
//             pdop,
//             gdop,
//             tdop,
//             hdop,
//             vdop,
//             flags: good_flags,
//         });
//         solution_tab.handle_dops(msg);
//         assert_eq!(
//             solution_tab.table[PDOP],
//             format!("{:.1}", pdop as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[GDOP],
//             format!("{:.1}", gdop as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[TDOP],
//             format!("{:.1}", tdop as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[HDOP],
//             format!("{:.1}", hdop as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[VDOP],
//             format!("{:.1}", vdop as f64 * DILUTION_OF_PRECISION_UNITS)
//         );

//         let msg = Dops::MsgDopsDepA(MsgDopsDepA {
//             sender_id: Some(1337),
//             tow: 0,
//             pdop: pdop + 1,
//             gdop: gdop + 1,
//             tdop: tdop + 1,
//             hdop: hdop + 1,
//             vdop: vdop + 1,
//         });
//         solution_tab.handle_dops(msg);
//         assert_eq!(
//             solution_tab.table[PDOP],
//             format!("{:.1}", (pdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[GDOP],
//             format!("{:.1}", (gdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[TDOP],
//             format!("{:.1}", (tdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[HDOP],
//             format!("{:.1}", (hdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//         assert_eq!(
//             solution_tab.table[VDOP],
//             format!("{:.1}", (vdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
//         );
//     }

//     #[test]
//     fn handle_age_corrections_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         assert!(solution_tab.age_corrections.is_none());
//         let msg = MsgAgeCorrections {
//             sender_id: Some(1337),
//             age: 0xFFFF,
//             tow: 0,
//         };
//         solution_tab.handle_age_corrections(msg);
//         assert!(solution_tab.age_corrections.is_none());
//         let good_age = 0x4DC6;
//         let msg = MsgAgeCorrections {
//             sender_id: Some(1337),
//             age: good_age,
//             tow: 0,
//         };
//         solution_tab.handle_age_corrections(msg);
//         assert!(solution_tab.age_corrections.is_some());
//         if let Some(age) = solution_tab.age_corrections {
//             assert!(f64::abs(age - 1991_f64) <= f64::EPSILON);
//         }
//     }

//     #[test]
//     fn handle_pos_llh_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let mut solution_tab = BaselineTab::new(shared_state, client_send);
//         solution_tab.utc_time = Some(utc_time(1_i32, 3_u32, 3_u32, 7_u32, 6_u32, 6_u32, 6_u32));
//         solution_tab.utc_source = Some(utc_source(0x02));
//         solution_tab.nsec = Some(1337);
//         solution_tab.week = Some(13);
//         let bad_flags = 0;
//         let lat = 45_f64;
//         let lon = -45_f64;
//         let height = 1337_f64;
//         let n_sats = 13;
//         let h_accuracy = 0;
//         let v_accuracy = 0;
//         let tow = 1337;
//         let msg = PosLLH::MsgPosLLH(MsgPosLLH {
//             sender_id: Some(1337),
//             flags: bad_flags,
//             lat,
//             lon,
//             height,
//             n_sats,
//             h_accuracy,
//             v_accuracy,
//             tow,
//         });

//         assert_eq!(solution_tab.last_mode, 0);
//         assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
//         solution_tab.handle_pos_llh(msg);
//         assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.last_mode, 0);

//         let good_flags = 0x01;
//         let msg = PosLLH::MsgPosLLH(MsgPosLLH {
//             sender_id: Some(1337),
//             flags: good_flags,
//             lat,
//             lon,
//             height,
//             n_sats,
//             h_accuracy,
//             v_accuracy,
//             tow,
//         });
//         assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
//         solution_tab.handle_pos_llh(msg);
//         assert_ne!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[LAT], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[LNG], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
//         assert_ne!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
//         assert_eq!(solution_tab.last_mode, good_flags);

//         assert_eq!(solution_tab.last_mode, 1);

//         let msg = PosLLH::MsgPosLLHDepA(MsgPosLLHDepA {
//             sender_id: Some(1337),
//             flags: good_flags,
//             lat,
//             lon,
//             height,
//             n_sats,
//             h_accuracy,
//             v_accuracy,
//             tow,
//         });
//         solution_tab.handle_pos_llh(msg);
//         assert_eq!(solution_tab.last_mode, 4);
//     }

//     // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
//     // Add missing unittests.
//     // Also add more breadth to handle_pos_llh tests.
//     // #[test]
//     // fn handle_init_logging_test() {
//     // }
//     // #[test]
//     // fn check_state_test() {
//     // }
//     // #[test]
//     // fn solution_draw_test() {
//     // }
//     // #[test]
//     // fn update_sln_data_by_mode_test() {
//     // }
//     // #[test]
//     // fn append_empty_sln_data_test() {
//     // }
//     // #[test]
//     // fn synchronize_plot_data_by_mode_test() {
//     // }
// }
