use capnp::message::Builder;

use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgPosLLHCov, MsgUtcTime},
    orientation::MsgOrientEuler,
    system::{MsgInsStatus, MsgInsUpdates},
};
use std::{collections::HashMap, time::Instant};

use crate::constants::*;
use crate::date_conv::*;
use crate::output::{CsvSerializer, PosLLHLog, VelLog};
use crate::piksi_tools_constants::EMPTY_STR;
use crate::types::{
    CapnProtoSender, Deque, Dops, GnssModes, GpsTime, PosLLH, SharedState, UtcDateTime, VelNED,
};
use crate::utils::*;

/// SolutionTab struct.
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
/// - `last_pos_mode`: The most recent gnss mode stored.
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
/// - `vel_log_file`: The CsvSerializer corresponding to an open velocity log if any.
/// - `week`: The stored week value from GPS Time messages.
#[derive(Debug)]
pub struct SolutionTab<S: CapnProtoSender> {
    pub age_corrections: Option<f64>,
    pub available_units: [&'static str; 2],
    pub client_sender: S,
    pub colors: Vec<String>,
    pub directory_name: Option<String>,
    pub ins_status_flags: u32,
    pub ins_used: bool,
    pub labels: Vec<String>,
    pub lats: Deque<f64>,
    pub lngs: Deque<f64>,
    pub last_ins_status_receipt_time: Instant,
    pub last_pos_mode: u8,
    pub last_odo_update_time: Instant,
    pub logging: bool,
    pub lat_max: f64,
    pub lat_min: f64,
    pub lon_max: f64,
    pub lon_min: f64,
    pub mode_strings: Vec<String>,
    pub modes: Deque<u8>,
    pub nsec: Option<i32>,
    pub pending_draw_modes: Vec<String>,
    pub pos_log_file: Option<CsvSerializer>,
    pub shared_state: SharedState,
    pub sln_cur_data: Vec<Vec<(f64, f64)>>,
    pub sln_data: Vec<Vec<(f64, f64)>>,
    pub slns: HashMap<&'static str, Deque<f64>>,
    pub table: HashMap<&'static str, String>,
    pub unit: &'static str,
    pub utc_source: Option<String>,
    pub utc_time: Option<UtcDateTime>,
    pub vel_log_file: Option<CsvSerializer>,
    pub week: Option<u16>,
}

impl<S: CapnProtoSender> SolutionTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> SolutionTab<S> {
        SolutionTab {
            age_corrections: None,
            available_units: [DEGREES, METERS],
            client_sender,
            colors: {
                vec![
                    GnssModes::Spp.color(),
                    GnssModes::Dgnss.color(),
                    GnssModes::Float.color(),
                    GnssModes::Fixed.color(),
                    GnssModes::Dr.color(),
                    GnssModes::Sbas.color(),
                ]
            },
            directory_name: None,
            ins_status_flags: 0,
            ins_used: false,
            labels: {
                vec![
                    GnssModes::Spp.label(),
                    GnssModes::Dgnss.label(),
                    GnssModes::Float.label(),
                    GnssModes::Fixed.label(),
                    GnssModes::Dr.label(),
                    GnssModes::Sbas.label(),
                ]
            },
            lats: Deque::with_size_limit(PLOT_HISTORY_MAX, /*fill_value=*/ None),
            lngs: Deque::with_size_limit(PLOT_HISTORY_MAX, /*fill_value=*/ None),
            last_ins_status_receipt_time: Instant::now(),
            last_pos_mode: 0,
            last_odo_update_time: Instant::now(),
            logging: false,
            lat_max: LAT_MAX,
            lat_min: LAT_MIN,
            lon_max: LON_MAX,
            lon_min: LON_MIN,
            modes: Deque::with_size_limit(PLOT_HISTORY_MAX, /*fill_value=*/ None),
            mode_strings: vec![
                GnssModes::Spp.to_string(),
                GnssModes::Dgnss.to_string(),
                GnssModes::Float.to_string(),
                GnssModes::Fixed.to_string(),
                GnssModes::Dr.to_string(),
                GnssModes::Sbas.to_string(),
            ],
            nsec: Some(0),
            pending_draw_modes: vec![],
            pos_log_file: None,
            shared_state,
            sln_cur_data: { vec![vec![]; NUM_GNSS_MODES as usize] },
            sln_data: { vec![vec![]; NUM_GNSS_MODES as usize] },
            slns: {
                SOLUTION_DATA_KEYS
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
                SOLUTION_TABLE_KEYS
                    .iter()
                    .map(|key| (*key, String::from(EMPTY_STR)))
                    .collect()
            },
            unit: DEGREES,
            utc_source: None,
            utc_time: None,
            vel_log_file: None,
            week: None,
        }
    }

    /// Handler for UTC time messages.
    ///
    /// # Parameters
    /// - `msg`: MsgUtcTime to extract data from.
    pub fn handle_utc_time(&mut self, msg: MsgUtcTime) {
        if msg.flags & 0x7 == 0 {
            self.utc_time = None;
            self.utc_source = None;
        } else {
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
        }
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

    /// Handler for Orientation / Attitude messages.
    ///
    /// # Parameters
    /// - `msg`: MsgOrientEuler to extract data from.
    pub fn handle_orientation_euler(&mut self, msg: MsgOrientEuler) {
        if msg.flags != 0 {
            self.table.insert(
                ROLL,
                format!("{: >6.2} deg", ((msg.roll as f64) * UDEG2DEG)),
            );
            self.table.insert(
                PITCH,
                format!("{: >6.2} deg", ((msg.pitch as f64) * UDEG2DEG)),
            );
            self.table
                .insert(YAW, format!("{: >6.2} deg", ((msg.yaw as f64) * UDEG2DEG)));
            self.table
                .insert(ROLL_ACC, format!("{: >6.2} deg", msg.roll_accuracy));
            self.table
                .insert(PITCH_ACC, format!("{: >6.2} deg", msg.pitch_accuracy));
            self.table
                .insert(YAW_ACC, format!("{: >6.2} deg", msg.yaw_accuracy));
        } else {
            self.table.insert(ROLL, String::from(EMPTY_STR));
            self.table.insert(PITCH, String::from(EMPTY_STR));
            self.table.insert(YAW, String::from(EMPTY_STR));
            self.table.insert(ROLL_ACC, String::from(EMPTY_STR));
            self.table.insert(PITCH_ACC, String::from(EMPTY_STR));
            self.table.insert(YAW_ACC, String::from(EMPTY_STR));
        }
    }

    /// Handler for POS LLH COV covariance messages.
    ///
    /// # Parameters
    /// - `msg`: MsgPosLLHCov to extract data from.
    pub fn handle_pos_llh_cov(&mut self, msg: MsgPosLLHCov) {
        if msg.flags != 0 {
            self.table.insert(COV_N_N, format!("{}", msg.cov_n_n));
            self.table.insert(COV_N_E, format!("{}", msg.cov_n_e));
            self.table.insert(COV_N_D, format!("{}", msg.cov_n_d));
            self.table.insert(COV_E_E, format!("{}", msg.cov_e_e));
            self.table.insert(COV_E_D, format!("{}", msg.cov_e_d));
            self.table.insert(COV_D_D, format!("{}", msg.cov_d_d));
        } else {
            self.table.insert(COV_N_N, String::from(EMPTY_STR));
            self.table.insert(COV_N_E, String::from(EMPTY_STR));
            self.table.insert(COV_N_D, String::from(EMPTY_STR));
            self.table.insert(COV_E_E, String::from(EMPTY_STR));
            self.table.insert(COV_E_D, String::from(EMPTY_STR));
            self.table.insert(COV_D_D, String::from(EMPTY_STR));
        }
    }

    /// Handle Vel NED / NEDDepA messages.
    ///
    /// # Parameters
    /// - `msg`: VelNED wrapper around a MsgVelNED or MsgVELNEDDepA.
    pub fn handle_vel_ned(&mut self, msg: VelNED) {
        let vel_ned_fields = msg.fields();
        let speed: f64 = mm_to_m(f64::sqrt(
            (i32::pow(vel_ned_fields.n, 2) + i32::pow(vel_ned_fields.e, 2)) as f64,
        ));
        let n = mm_to_m(vel_ned_fields.n as f64);
        let e = mm_to_m(vel_ned_fields.e as f64);
        let d = mm_to_m(vel_ned_fields.d as f64);
        let mut tow = ms_to_sec(vel_ned_fields.tow);
        if let Some(nsec) = self.nsec {
            tow += ns_to_sec(nsec as f64);
        }

        let (tloc, secloc) = convert_local_time_to_logging_format();
        let (tgps_, secgps_) = convert_gps_time_to_logging_format(self.week, tow);

        // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
        // Validate logging.
        if let Some(vel_file) = &mut self.vel_log_file {
            let mut gps_time = None;
            if let Some(tgps) = tgps_ {
                if let Some(secgps) = secgps_ {
                    gps_time = Some(format!("{}:{:0>6.06}", tgps, secgps));
                }
            }
            let pc_time = format!("{}:{:0>6.06}", tloc, secloc);
            if let Err(err) = vel_file.serialize(&VelLog {
                pc_time,
                gps_time,
                tow_s: Some(tow),
                north_mps: Some(n),
                east_mps: Some(e),
                down_mps: Some(d),
                speed_mps: Some(speed),
                flags: vel_ned_fields.flags,
                num_signals: vel_ned_fields.n_sats,
            }) {
                eprintln!("Unable to to write to vel log, error {}.", err);
            }
        }
        self.table
            .insert(VEL_FLAGS, format!("0x{:<03x}", vel_ned_fields.flags));
        if (vel_ned_fields.flags & 0x7) != 0 {
            self.table.insert(VEL_N, format!("{: >8.4}", n));
            self.table.insert(VEL_E, format!("{: >8.4}", e));
            self.table.insert(VEL_D, format!("{: >8.4}", d));
        } else {
            self.table.insert(VEL_N, String::from(EMPTY_STR));
            self.table.insert(VEL_E, String::from(EMPTY_STR));
            self.table.insert(VEL_D, String::from(EMPTY_STR));
        }
    }

    /// Handle INS Updates messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsUpdates to extract data from.
    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        let tic = msg.wheelticks;
        if ((tic & 0xF0) >> 4) > (tic & 0x0F) {
            self.last_odo_update_time = Instant::now();
            let mut shared_data = self.shared_state.lock().unwrap();
            (*shared_data)
                .solution_tab
                .position_tab
                .last_odo_update_time = self.last_odo_update_time;
        }
    }

    /// Handle INS Status messages.
    ///
    /// # Parameters
    /// - `msg`: The MsgInsStatus to extract data from.
    pub fn handle_ins_status(&mut self, msg: MsgInsStatus) {
        self.ins_status_flags = msg.flags;
        self.last_ins_status_receipt_time = Instant::now();
        let mut shared_data = self.shared_state.lock().unwrap();
        (*shared_data).solution_tab.position_tab.ins_status_flags = msg.flags;
        (*shared_data)
            .solution_tab
            .position_tab
            .last_ins_status_receipt_time = self.last_ins_status_receipt_time;
    }

    /// Handle Dops / DopsDepA messages.
    ///
    /// # Parameters
    /// - `msg`: Dops wrapper around a MsgDops or MsgDopsDepA.
    pub fn handle_dops(&mut self, msg: Dops) {
        let dops_fields = msg.fields();
        self.table
            .insert(DOPS_FLAGS, format!("0x{:<03x}", dops_fields.flags));
        self.table
            .insert(INS_STATUS, format!("0x{:<01x}", self.ins_status_flags));
        if dops_fields.flags != 0 {
            self.table.insert(
                PDOP,
                format!(
                    "{:.1}",
                    dops_fields.pdop as f64 * DILUTION_OF_PRECISION_UNITS
                ),
            );
            self.table.insert(
                GDOP,
                format!(
                    "{:.1}",
                    dops_fields.gdop as f64 * DILUTION_OF_PRECISION_UNITS
                ),
            );
            self.table.insert(
                TDOP,
                format!(
                    "{:.1}",
                    dops_fields.tdop as f64 * DILUTION_OF_PRECISION_UNITS
                ),
            );
            self.table.insert(
                HDOP,
                format!(
                    "{:.1}",
                    dops_fields.hdop as f64 * DILUTION_OF_PRECISION_UNITS
                ),
            );
            self.table.insert(
                VDOP,
                format!(
                    "{:.1}",
                    dops_fields.vdop as f64 * DILUTION_OF_PRECISION_UNITS
                ),
            );
        } else {
            self.table.insert(PDOP, String::from(EMPTY_STR));
            self.table.insert(GDOP, String::from(EMPTY_STR));
            self.table.insert(TDOP, String::from(EMPTY_STR));
            self.table.insert(HDOP, String::from(EMPTY_STR));
            self.table.insert(VDOP, String::from(EMPTY_STR));
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

    /// Handle PosLLH / PosLLHDepA messages.
    ///
    /// TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    /// Need to validate logging.
    pub fn handle_pos_llh(&mut self, msg: PosLLH) {
        self.last_pos_mode = msg.mode();
        let pos_llh_fields = msg.fields();
        let gnss_mode = GnssModes::from(self.last_pos_mode);
        let mode_string = gnss_mode.to_string();
        if self.last_pos_mode != 0 {
            if !self.pending_draw_modes.contains(&mode_string) {
                self.pending_draw_modes.push(mode_string.clone());
            }
            self._update_sln_data_by_mode(pos_llh_fields.lat, pos_llh_fields.lon, mode_string);
        } else {
            self._append_empty_sln_data(None);
        }
        self.ins_used = ((pos_llh_fields.flags & 0x8) >> 3) == 1;
        let mut tow = pos_llh_fields.tow * 1.0e-3_f64;
        if let Some(nsec) = self.nsec {
            tow += nsec as f64 * 1.0e-9_f64;
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

        // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
        // Validate logging.
        if let Some(pos_file) = &mut self.pos_log_file {
            let pc_time = format!("{}:{:>6.06}", tloc, secloc);
            if let Err(err) = pos_file.serialize(&PosLLHLog {
                pc_time,
                gps_time,
                tow_s: Some(tow),
                latitude_d: Some(pos_llh_fields.lat),
                longitude_d: Some(pos_llh_fields.lon),
                altitude_m: Some(pos_llh_fields.height),
                h_accuracy_m: Some(pos_llh_fields.h_accuracy),
                v_accuracy_m: Some(pos_llh_fields.v_accuracy),
                n_sats: pos_llh_fields.n_sats,
                flags: pos_llh_fields.flags,
            }) {
                eprintln!("Unable to to write to pos llh log, error {}.", err);
            }
        }

        if self.last_pos_mode == 0 {
            self.table.insert(GPS_WEEK, String::from(EMPTY_STR));
            self.table.insert(GPS_TOW, String::from(EMPTY_STR));
            self.table.insert(GPS_TIME, String::from(EMPTY_STR));
            self.table.insert(UTC_TIME, String::from(EMPTY_STR));
            self.table.insert(UTC_SRC, String::from(EMPTY_STR));
            self.table.insert(SATS_USED, String::from(EMPTY_STR));
            self.table.insert(LAT, String::from(EMPTY_STR));
            self.table.insert(LNG, String::from(EMPTY_STR));
            self.table.insert(HEIGHT, String::from(EMPTY_STR));
            self.table.insert(HORIZ_ACC, String::from(EMPTY_STR));
            self.table.insert(VERT_ACC, String::from(EMPTY_STR));
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
            self.table
                .insert(SATS_USED, pos_llh_fields.n_sats.to_string());
            self.table
                .insert(LAT, format!("{:.12}", pos_llh_fields.lat));
            self.table
                .insert(LNG, format!("{:.12}", pos_llh_fields.lon));
            self.table
                .insert(HEIGHT, format!("{:.3}", pos_llh_fields.height));
            self.table
                .insert(HORIZ_ACC, format!("{:.3}", pos_llh_fields.h_accuracy));
            self.table
                .insert(VERT_ACC, format!("{:.3}", pos_llh_fields.v_accuracy));
            self.lats.add(pos_llh_fields.lat);
            self.lngs.add(pos_llh_fields.lon);
            self.modes.add(self.last_pos_mode);
        }
        self.table
            .insert(POS_FLAGS, format!("0x{:<03x}", pos_llh_fields.flags));
        self.table.insert(INS_USED, self.ins_used.to_string());
        self.table.insert(POS_FIX_MODE, self.ins_used.to_string());
        if let Some(age_corrections_) = self.age_corrections {
            self.table
                .insert(CORR_AGE_S, format!("{}", age_corrections_));
        }
        let (center, clear, pause, unit, zoom) = self.check_state();
        self.solution_draw(center, clear, pause, unit, zoom);
        self.send_solution_data();
        self.send_table_data();
    }

    pub fn check_state(&self) -> (bool, bool, bool, String, bool) {
        let mut shared_data = self.shared_state.lock().unwrap();
        let center = (*shared_data).solution_tab.position_tab.center;
        let clear = (*shared_data).solution_tab.position_tab.clear;
        (*shared_data).solution_tab.position_tab.clear = false;
        let pause = (*shared_data).solution_tab.position_tab.pause;
        let unit = (*shared_data).solution_tab.position_tab.unit.clone();
        let zoom = (*shared_data).solution_tab.position_tab.zoom;
        (center, clear, pause, unit, zoom)
    }

    pub fn clear_sln(&mut self) {
        for (_, deque) in &mut self.slns.iter_mut() {
            deque.clear();
        }
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
    pub fn solution_draw(
        &mut self,
        _center: bool,
        clear: bool,
        pause: bool,
        _unit: String,
        _zoom: bool,
    ) {
        if clear {
            self.clear_sln();
        }
        if pause {
            return;
        }
        let current_mode: Option<String> = if !self.pending_draw_modes.is_empty() {
            self.lat_min = LAT_MAX;
            self.lat_max = LAT_MIN;
            self.lon_min = LON_MAX;
            self.lon_max = LON_MIN;
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

    // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    // pub fn rescale_for_units_change() {
    //
    // }

    // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    // pub fn _display_units_changed(&mut self) {
    //
    //     let lats = self.lats.get();
    //     let lons = self.lngs.get();
    //     let modes = self.modes.get();
    //     let mut lats_sum: f64 = 0.0;
    //     let mut lons_sum: f64 = 0.0;
    //     let mut num_eles: usize = 0;
    //     for (idx, lat) in lats.iter().enumerate() {
    //         if modes[idx] == 0 {
    //             continue;
    //         }
    //         lats_sum += lat;
    //         lons_sum += lons[idx];
    //         num_eles += 1;
    //     }
    //     let lats_mean = lats_sum/num_eles as f64;
    //     let lons_mean = lons_sum/num_eles as f64;
    // }

    /// Update a single mode's solution data for with lat and lon values.
    ///
    /// TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    /// Need to implement offset and multiplier logic.
    ///
    /// # Parameters
    /// - `last_lat`: The latitude value to update solution data with.
    /// - `last_lon`: The longitude value to update solution data with.
    /// - `mode_string`: The mode associated with the update in string form.
    fn _update_sln_data_by_mode(&mut self, last_lat: f64, last_lng: f64, mode_string: String) {
        let lat = last_lat; // - self.offset) * self.sf
        let lng = last_lng; // - self.offset) * self.sf

        let lat_str = format!("lat_{}", mode_string);
        let lon_str = format!("lng_{}", mode_string);
        let lat_str = lat_str.as_str();
        let lon_str = lon_str.as_str();
        self.slns.get_mut(lat_str).unwrap().add(lat);
        self.slns.get_mut(lon_str).unwrap().add(lng);
        self._append_empty_sln_data(Some(mode_string));
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
            let lat_str = format!("lat_{}", each_mode);
            let lon_str = format!("lng_{}", each_mode);
            let lat_str = lat_str.as_str();
            let lon_str = lon_str.as_str();
            self.slns.get_mut(lat_str).unwrap().add(f64::NAN);
            self.slns.get_mut(lon_str).unwrap().add(f64::NAN);
        }
    }

    /// Prepare data to be sent to frontend for a single mode.
    ///
    /// # Parameters
    /// - `mode_string`: The mode string to attempt to prepare data for frontend.
    /// - `update_current`: Indicating whether the current solution should be updated by
    /// this modes last lat/lon entry.
    fn _synchronize_plot_data_by_mode(&mut self, mode_string: &str, update_current: bool) {
        let lat_string = format!("lat_{}", mode_string);
        let lat_string = lat_string.as_str();
        let lon_string = format!("lng_{}", mode_string);
        let lon_string = lon_string.as_str();

        if let Some(idx) = self.mode_strings.iter().position(|x| *x == *mode_string) {
            let lat_values = self.slns[&lat_string].get();
            let lon_values = self.slns[&lon_string].get();

            let mut new_sln: Vec<(f64, f64)> = vec![];
            for jdx in 0..lat_values.len() {
                if lat_values[jdx].is_nan() || lon_values[jdx].is_nan() {
                    continue;
                }
                self.lat_min = f64::min(self.lat_min, lat_values[jdx]);
                self.lat_max = f64::max(self.lat_max, lat_values[jdx]);
                self.lon_min = f64::min(self.lon_min, lon_values[jdx]);
                self.lon_max = f64::max(self.lon_max, lon_values[jdx]);
                new_sln.push((lon_values[jdx], lat_values[jdx]));
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

    /// Package solution data into a message buffer and send to frontend.
    fn send_solution_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut solution_status = msg.init_solution_position_status();
        solution_status.set_lat_min(self.lat_min);
        solution_status.set_lat_max(self.lat_max);
        solution_status.set_lon_min(self.lon_min);
        solution_status.set_lon_max(self.lon_max);

        let mut solution_points = solution_status
            .reborrow()
            .init_data(self.sln_data.len() as u32);
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
        let mut available_units = solution_status
            .reborrow()
            .init_available_units(self.available_units.len() as u32);
        for (i, unit) in self.available_units.iter().enumerate() {
            available_units.set(i as u32, *unit);
        }
        let mut solution_points = solution_status
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
        let mut colors = solution_status
            .reborrow()
            .init_colors(self.colors.len() as u32);

        for (i, color) in self.colors.iter().enumerate() {
            colors.set(i as u32, color);
        }

        let mut labels = solution_status
            .reborrow()
            .init_labels(self.labels.len() as u32);

        for (i, label) in self.labels.iter().enumerate() {
            labels.set(i as u32, label);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    /// Package solution table data into a message buffer and send to frontend.
    fn send_table_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut solution_table_status = msg.init_solution_table_status();
        let mut table_entries = solution_table_status
            .reborrow()
            .init_data(self.table.len() as u32);
        {
            for (i, key) in SOLUTION_TABLE_KEYS.iter().enumerate() {
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
    use crate::types::TestSender;
    use chrono::{TimeZone, Utc};
    use sbp::messages::navigation::{
        MsgAgeCorrections, MsgDops, MsgDopsDepA, MsgGPSTime, MsgPosLLH, MsgPosLLHDepA, MsgVelNED,
        MsgVelNEDDepA,
    };
    use std::{thread::sleep, time::Duration};

    #[test]
    fn handle_utc_time_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_table = SolutionTab::new(shared_state, client_send);
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
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_table = SolutionTab::new(shared_state, client_send);
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
        solution_table.week = Some(old_wn);
        solution_table.nsec = Some(old_nsec);
        solution_table.handle_gps_time(GpsTime::MsgGpsTime(msg));
        assert_eq!(solution_table.week, Some(old_wn));
        assert_eq!(solution_table.nsec, Some(old_nsec));

        let good_flags = 1_u8;
        let msg = MsgGPSTime {
            sender_id: Some(1337),
            wn,
            tow: 0,
            ns_residual,
            flags: good_flags,
        };
        solution_table.handle_gps_time(GpsTime::MsgGpsTime(msg));
        assert_eq!(solution_table.week, Some(wn));
        assert_eq!(solution_table.nsec, Some(ns_residual));
    }

    #[test]
    fn handle_vel_ned_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
        let good_flags = 0x07;
        let bad_flags = 0xF0;
        let n = 1;
        let e = 2;
        let d = 3;
        let n_sats = 13;
        let msg = VelNED::MsgVelNED(MsgVelNED {
            sender_id: Some(1337),
            flags: bad_flags,
            n,
            e,
            d,
            n_sats,
            tow: 0,
            h_accuracy: 0,
            v_accuracy: 0,
        });
        assert_eq!(solution_tab.table[VEL_FLAGS], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_N], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_E], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_D], String::from(EMPTY_STR));
        solution_tab.handle_vel_ned(msg);
        assert_eq!(
            solution_tab.table[VEL_FLAGS],
            format!("0x{:<03x}", bad_flags)
        );
        assert_eq!(solution_tab.table[VEL_N], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_E], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_D], String::from(EMPTY_STR));
        let msg = VelNED::MsgVelNED(MsgVelNED {
            sender_id: Some(1337),
            flags: good_flags,
            n,
            e,
            d,
            n_sats,
            tow: 0,
            h_accuracy: 0,
            v_accuracy: 0,
        });
        solution_tab.handle_vel_ned(msg);
        assert_eq!(
            solution_tab.table[VEL_FLAGS],
            format!("0x{:<03x}", good_flags)
        );
        assert_eq!(
            solution_tab.table[VEL_N],
            format!("{: >8.4}", n as f64 / 1000_f64)
        );
        assert_eq!(
            solution_tab.table[VEL_E],
            format!("{: >8.4}", e as f64 / 1000_f64)
        );
        assert_eq!(
            solution_tab.table[VEL_D],
            format!("{: >8.4}", d as f64 / 1000_f64)
        );
        let n = 3;
        let e = 2;
        let d = 1;
        let msg = VelNED::MsgVelNEDDepA(MsgVelNEDDepA {
            sender_id: Some(1337),
            flags: good_flags,
            n,
            e,
            d,
            n_sats,
            tow: 0,
            h_accuracy: 0,
            v_accuracy: 0,
        });
        solution_tab.handle_vel_ned(msg);
        assert_eq!(solution_tab.table[VEL_FLAGS], format!("0x{:<03x}", 1));
        assert_eq!(
            solution_tab.table[VEL_N],
            format!("{: >8.4}", n as f64 / 1000_f64)
        );
        assert_eq!(
            solution_tab.table[VEL_E],
            format!("{: >8.4}", e as f64 / 1000_f64)
        );
        assert_eq!(
            solution_tab.table[VEL_D],
            format!("{: >8.4}", d as f64 / 1000_f64)
        );
    }

    #[test]
    fn handle_ins_status_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
        let flags = 0xf0_u32;
        let msg = MsgInsStatus {
            sender_id: Some(1337),
            flags,
        };
        let update_time = Instant::now();
        solution_tab.handle_ins_status(msg);
        assert!(solution_tab.last_ins_status_receipt_time > update_time);
        assert_eq!(solution_tab.ins_status_flags, flags);
    }

    #[test]
    fn handle_ins_updates_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
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
        solution_tab.handle_ins_updates(msg);

        assert!(solution_tab.last_odo_update_time > odo_update_time);

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
        solution_tab.handle_ins_updates(msg);

        assert!(solution_tab.last_odo_update_time < odo_update_time);
    }

    #[test]
    fn handle_dops_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
        let pdop = 1;
        let gdop = 2;
        let tdop = 3;
        let hdop = 4;
        let vdop = 5;
        let good_flags = 1;
        let bad_flags = 0;
        let msg = Dops::MsgDops(MsgDops {
            sender_id: Some(1337),
            tow: 0,
            pdop,
            gdop,
            tdop,
            hdop,
            vdop,
            flags: bad_flags,
        });
        assert_eq!(solution_tab.table[PDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[TDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VDOP], String::from(EMPTY_STR));
        solution_tab.handle_dops(msg);
        assert_eq!(solution_tab.table[PDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[TDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HDOP], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VDOP], String::from(EMPTY_STR));
        let msg = Dops::MsgDops(MsgDops {
            sender_id: Some(1337),
            tow: 0,
            pdop,
            gdop,
            tdop,
            hdop,
            vdop,
            flags: good_flags,
        });
        solution_tab.handle_dops(msg);
        assert_eq!(
            solution_tab.table[PDOP],
            format!("{:.1}", pdop as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[GDOP],
            format!("{:.1}", gdop as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[TDOP],
            format!("{:.1}", tdop as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[HDOP],
            format!("{:.1}", hdop as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[VDOP],
            format!("{:.1}", vdop as f64 * DILUTION_OF_PRECISION_UNITS)
        );

        let msg = Dops::MsgDopsDepA(MsgDopsDepA {
            sender_id: Some(1337),
            tow: 0,
            pdop: pdop + 1,
            gdop: gdop + 1,
            tdop: tdop + 1,
            hdop: hdop + 1,
            vdop: vdop + 1,
        });
        solution_tab.handle_dops(msg);
        assert_eq!(
            solution_tab.table[PDOP],
            format!("{:.1}", (pdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[GDOP],
            format!("{:.1}", (gdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[TDOP],
            format!("{:.1}", (tdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[HDOP],
            format!("{:.1}", (hdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
        );
        assert_eq!(
            solution_tab.table[VDOP],
            format!("{:.1}", (vdop + 1_u16) as f64 * DILUTION_OF_PRECISION_UNITS)
        );
    }

    #[test]
    fn handle_age_corrections_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
        assert!(solution_tab.age_corrections.is_none());
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: 0xFFFF,
            tow: 0,
        };
        solution_tab.handle_age_corrections(msg);
        assert!(solution_tab.age_corrections.is_none());
        let good_age = 0x4DC6;
        let msg = MsgAgeCorrections {
            sender_id: Some(1337),
            age: good_age,
            tow: 0,
        };
        solution_tab.handle_age_corrections(msg);
        assert!(solution_tab.age_corrections.is_some());
        if let Some(age) = solution_tab.age_corrections {
            assert!(f64::abs(age - 1991_f64) <= f64::EPSILON);
        }
    }

    #[test]
    fn handle_pos_llh_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut solution_tab = SolutionTab::new(shared_state, client_send);
        solution_tab.utc_time = Some(utc_time(1_i32, 3_u32, 3_u32, 7_u32, 6_u32, 6_u32, 6_u32));
        solution_tab.utc_source = Some(utc_source(0x02));
        solution_tab.nsec = Some(1337);
        solution_tab.week = Some(13);
        let bad_flags = 0;
        let lat = 45_f64;
        let lon = -45_f64;
        let height = 1337_f64;
        let n_sats = 13;
        let h_accuracy = 0;
        let v_accuracy = 0;
        let tow = 1337;
        let msg = PosLLH::MsgPosLLH(MsgPosLLH {
            sender_id: Some(1337),
            flags: bad_flags,
            lat,
            lon,
            height,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });

        assert_eq!(solution_tab.last_pos_mode, 0);
        assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        solution_tab.handle_pos_llh(msg);
        assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.last_pos_mode, 0);

        let good_flags = 0x01;
        let msg = PosLLH::MsgPosLLH(MsgPosLLH {
            sender_id: Some(1337),
            flags: good_flags,
            lat,
            lon,
            height,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });
        assert_eq!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LAT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[LNG], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        solution_tab.handle_pos_llh(msg);
        assert_ne!(solution_tab.table[GPS_WEEK], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[GPS_TOW], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[GPS_TIME], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[UTC_TIME], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[UTC_SRC], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[SATS_USED], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[LAT], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[LNG], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.last_pos_mode, good_flags);

        assert_eq!(solution_tab.last_pos_mode, 1);

        let msg = PosLLH::MsgPosLLHDepA(MsgPosLLHDepA {
            sender_id: Some(1337),
            flags: good_flags,
            lat,
            lon,
            height,
            n_sats,
            h_accuracy,
            v_accuracy,
            tow,
        });
        solution_tab.handle_pos_llh(msg);
        assert_eq!(solution_tab.last_pos_mode, 4);
    }

    // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
    // Add missing unittests.
    // Also add more breadth to handle_pos_llh tests.
    // #[test]
    // fn handle_init_logging_test() {
    // }
    // #[test]
    // fn check_state_test() {
    // }
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
