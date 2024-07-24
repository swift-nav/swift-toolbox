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
use log::error;
use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgPosLlhCov, MsgUtcTime},
    orientation::{MsgAngularRate, MsgOrientEuler},
    system::{MsgInsStatus, MsgInsUpdates},
};
use std::{collections::HashMap, time::Instant};

use crate::client_sender::BoxedClientSender;
use crate::constants::*;
use crate::output::{PosLLHLog, VelLog};
use crate::piksi_tools_constants::EMPTY_STR;
use crate::shared_state::SharedState;
use crate::tabs::solution_tab::LatLonUnits;
use crate::types::{
    Dops, GnssModes, GpsTime, PosLLH, ProtectionLevel, RingBuffer, UtcDateTime, VelNED,
};
use crate::utils::{date_conv::*, *};

#[derive(Debug)]
pub struct SolutionPositionTab {
    /// Stored age corrections to be displayed in the table.
    age_corrections: Option<f64>,
    ///  The available units of measure to send to frontend for selection.
    available_units: [&'static str; 2],
    client_sender: BoxedClientSender,
    /// The stored ins status flags expected by other tabs.
    ins_status_flags: u32,
    /// Indicates whether or not ins is currently used.
    ins_used: bool,
    /// The stored latitude values for quickly extracting aggregate data.
    lats: RingBuffer<f64>,
    /// The stored longitude values for quickly extracting aggregate data.
    lons: RingBuffer<f64>,
    alts: RingBuffer<f64>,
    /// The last ins status receipt monotonic time stored.
    last_ins_status_receipt_time: Instant,
    /// The most recent gnss mode stored.
    last_pos_mode: u8,
    /// The last odo update monotonic time stored.
    last_odo_update_time: Instant,
    lat_sf: f64,
    lat_offset: f64,
    lat_max: f64,
    lat_min: f64,
    lon_sf: f64,
    lon_offset: f64,
    lon_max: f64,
    lon_min: f64,
    /// The last horizontal accuracy
    h_acc: f64,
    /// The available modes in string formm to store updates for.
    mode_strings: Vec<String>,
    /// The stored mode values used for quickly extracting aggregate data.
    modes: RingBuffer<u8>,
    /// The stored nanosecond value from GPS Time messages.
    nsec: Option<i32>,
    /// A list of draw modes waiting to be updated.
    pending_draw_modes: Vec<String>,
    /// The shared state for communicating between frontend/backend/other backend tabs.
    shared_state: SharedState,
    /// The current most recent lat/lon point for each mode.
    sln_cur_data: Vec<Vec<(f64, f64)>>,
    /// The preprocessed solution data to be sent to the frontend.
    sln_data: Vec<Vec<(f64, f64)>>,
    sln_line: RingBuffer<(f64, f64)>,
    pending_sln_line: RingBuffer<(f64, f64)>,
    /// All solution data is stored before preparing for frontend.
    slns: HashMap<&'static str, RingBuffer<f64>>,
    /// This stores all the key/value pairs to be displayed in the Solution Table.
    table: HashMap<&'static str, String>,
    /// The current unit of measure for the solution position plot.
    unit: LatLonUnits,
    /// The string equivalent for the source of the UTC updates.
    utc_source: Option<String>,
    /// The stored monotonic Utc time.
    utc_time: Option<UtcDateTime>,
    /// The stored week value from GPS Time messages.
    week: Option<u16>,
}

impl SolutionPositionTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> SolutionPositionTab {
        let unit = LatLonUnits::Degrees;
        let (lat_sf, lon_sf) = unit.get_sig_figs(0.0);
        let mut mode_strings = vec![
            GnssModes::Spp.to_string(),
            GnssModes::Sbas.to_string(),
            GnssModes::Dgnss.to_string(),
            GnssModes::Float.to_string(),
            GnssModes::Fixed.to_string(),
            GnssModes::Dr.to_string(),
        ];
        mode_strings.reserve_exact(mode_strings.len());
        SolutionPositionTab {
            age_corrections: None,
            available_units: [DEGREES, METERS],
            client_sender,
            ins_status_flags: 0,
            ins_used: false,
            lats: RingBuffer::new(PLOT_HISTORY_MAX),
            lons: RingBuffer::new(PLOT_HISTORY_MAX),
            alts: RingBuffer::new(PLOT_HISTORY_MAX),
            last_ins_status_receipt_time: Instant::now(),
            last_pos_mode: 0,
            last_odo_update_time: Instant::now(),
            lat_sf,
            lat_offset: 0.0,
            lat_max: f64::MAX,
            lat_min: f64::MIN,
            lon_sf,
            lon_offset: 0.0,
            lon_max: f64::MAX,
            lon_min: f64::MIN,
            modes: RingBuffer::new(PLOT_HISTORY_MAX),
            pending_draw_modes: Vec::with_capacity(mode_strings.len()),
            mode_strings,
            nsec: Some(0),
            shared_state,
            sln_cur_data: {
                let mut data = (0..NUM_GNSS_MODES)
                    .map(|_| Vec::with_capacity(1))
                    .collect::<Vec<_>>();
                data.reserve_exact(NUM_GNSS_MODES);
                data
            },
            sln_data: {
                let mut data = (0..NUM_GNSS_MODES)
                    .map(|_| Vec::with_capacity(PLOT_HISTORY_MAX))
                    .collect::<Vec<_>>();
                data.reserve_exact(NUM_GNSS_MODES);
                data
            },
            slns: {
                SOLUTION_DATA_KEYS
                    .iter()
                    .map(|key| (*key, RingBuffer::new(PLOT_HISTORY_MAX)))
                    .collect()
            },
            sln_line: RingBuffer::new(PLOT_HISTORY_MAX),
            pending_sln_line: RingBuffer::new(PLOT_HISTORY_MAX),
            table: {
                SOLUTION_TABLE_KEYS
                    .iter()
                    .map(|key| (*key, String::from(EMPTY_STR)))
                    .collect()
            },
            unit,
            utc_source: None,
            utc_time: None,
            week: None,
            h_acc: 0.0,
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
            self.utc_time = Some(utc_time_from_msg(&msg));
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

    /// Handler for Angular Rate messages.
    ///
    /// # Parameters
    /// - `msg`: MsgOrientEuler to extract data from.
    pub fn handle_angular_rate(&mut self, msg: MsgAngularRate) {
        if (msg.flags & 0x03) != 0 {
            self.table.insert(
                ANG_RATE_X_DEG_P_S,
                format!("{: >6.2} deg", ((msg.x as f64) * UDEG2DEG)),
            );
            self.table.insert(
                ANG_RATE_Y_DEG_P_S,
                format!("{: >6.2} deg", ((msg.y as f64) * UDEG2DEG)),
            );
            self.table.insert(
                ANG_RATE_Z_DEG_P_S,
                format!("{: >6.2} deg", ((msg.z as f64) * UDEG2DEG)),
            );
        } else {
            self.table
                .insert(ANG_RATE_X_DEG_P_S, String::from(EMPTY_STR));
            self.table
                .insert(ANG_RATE_Y_DEG_P_S, String::from(EMPTY_STR));
            self.table
                .insert(ANG_RATE_Z_DEG_P_S, String::from(EMPTY_STR));
        }
    }

    /// Handler for Orientation / Attitude messages.
    ///
    /// # Parameters
    /// - `msg`: MsgOrientEuler to extract data from.
    pub fn handle_orientation_euler(&mut self, msg: MsgOrientEuler) {
        if (msg.flags & 0x07) != 0 {
            self.table.insert(
                ROLL,
                format!(
                    "{:.2} ({:.1})",
                    (msg.roll as f64) * UDEG2DEG,
                    msg.roll_accuracy
                ),
            );
            self.table.insert(
                PITCH,
                format!(
                    "{:.2} ({:.1})",
                    (msg.pitch as f64) * UDEG2DEG,
                    msg.pitch_accuracy
                ),
            );
            self.table.insert(
                YAW,
                format!(
                    "{:.2} ({:.1})",
                    (msg.yaw as f64) * UDEG2DEG,
                    msg.yaw_accuracy
                ),
            );
        } else {
            self.table.insert(ROLL, String::from(EMPTY_STR));
            self.table.insert(PITCH, String::from(EMPTY_STR));
            self.table.insert(YAW, String::from(EMPTY_STR));
        }
    }

    /// Handler for POS LLH COV covariance messages.
    ///
    /// # Parameters
    /// - `msg`: MsgPosLlhCov to extract data from.
    pub fn handle_pos_llh_cov(&mut self, msg: MsgPosLlhCov) {
        let (cov_n_n, cov_n_e, cov_n_d, cov_e_e, cov_e_d, cov_d_d) = if msg.flags != 0 {
            (
                format_fixed_decimal_and_sign(msg.cov_n_n, 5, 3),
                format_fixed_decimal_and_sign(msg.cov_n_e, 5, 3),
                format_fixed_decimal_and_sign(msg.cov_n_d, 5, 3),
                format_fixed_decimal_and_sign(msg.cov_e_e, 5, 3),
                format_fixed_decimal_and_sign(msg.cov_e_d, 5, 3),
                format_fixed_decimal_and_sign(msg.cov_d_d, 5, 3),
            )
        } else {
            (
                EMPTY_STR.into(),
                EMPTY_STR.into(),
                EMPTY_STR.into(),
                EMPTY_STR.into(),
                EMPTY_STR.into(),
                EMPTY_STR.into(),
            )
        };
        self.table.insert(COV_N_N, cov_n_n);
        self.table.insert(COV_N_E, cov_n_e);
        self.table.insert(COV_N_D, cov_n_d);
        self.table.insert(COV_E_E, cov_e_e);
        self.table.insert(COV_E_D, cov_e_d);
        self.table.insert(COV_D_D, cov_d_d);
    }

    /// Handle Vel NED / NEDDepA messages.
    ///
    /// # Parameters
    /// - `msg`: VelNED wrapper around a MsgVelNed or MsgVELNEDDepA.
    pub fn handle_vel_ned(&mut self, msg: VelNED) {
        let vel_ned_fields = msg.fields();
        let n = vel_ned_fields.n as f64;
        let e = vel_ned_fields.e as f64;
        let d = vel_ned_fields.d as f64;

        let speed: f64 = mm_to_m(euclidean_distance([n, e].iter()));
        let n = mm_to_m(n);
        let e = mm_to_m(e);
        let d = mm_to_m(d);
        let t = speed;
        let mut tow = ms_to_sec(vel_ned_fields.tow);
        if let Some(nsec) = self.nsec {
            tow += ns_to_sec(nsec as f64);
        }

        let (tloc, secloc) = convert_local_time_to_logging_format();
        let (tgps_, secgps_) = convert_gps_time_to_logging_format(self.week, tow);

        // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
        // Validate logging.
        {
            let mut shared_data = self.shared_state.lock();
            if let Some(ref mut vel_file) = shared_data.solution_tab.velocity_tab.log_file {
                let mut gps_time = None;
                if let Some(tgps) = tgps_ {
                    if let Some(secgps) = secgps_ {
                        gps_time = Some(format!("{tgps}:{secgps:0>6.06}"));
                    }
                }
                let pc_time = format!("{tloc}:{secloc:0>6.06}");
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
                    error!("Unable to to write to vel log, error {err}.");
                }
            }
        }
        self.table
            .insert(VEL_FLAGS, format!("0x{:<03x}", vel_ned_fields.flags));
        if (vel_ned_fields.flags & 0x7) != 0 {
            self.table.insert(VEL_N, format!("{n: >8.4}"));
            self.table.insert(VEL_E, format!("{e: >8.4}"));
            self.table.insert(VEL_D, format!("{d: >8.4}"));
            self.table.insert(VEL_TOTAL, format!("{t: >8.4}"));
        } else {
            self.table.insert(VEL_N, String::from(EMPTY_STR));
            self.table.insert(VEL_E, String::from(EMPTY_STR));
            self.table.insert(VEL_D, String::from(EMPTY_STR));
            self.table.insert(VEL_TOTAL, String::from(EMPTY_STR));
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
            self.shared_state
                .lock()
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
        self.table
            .insert(INS_STATUS, format!("0x{:<01x}", self.ins_status_flags));
        self.last_ins_status_receipt_time = Instant::now();
        let mut shared_data = self.shared_state.lock();
        shared_data.solution_tab.position_tab.ins_status_flags = msg.flags;
        shared_data
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
        if dops_fields.flags != 0 {
            self.table.insert(PDOP, dops_into_string(dops_fields.pdop));
            self.table.insert(GDOP, dops_into_string(dops_fields.gdop));
            self.table.insert(TDOP, dops_into_string(dops_fields.tdop));
            self.table.insert(HDOP, dops_into_string(dops_fields.hdop));
            self.table.insert(VDOP, dops_into_string(dops_fields.vdop));
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
        self.age_corrections = if msg.age != 0xFFFF {
            Some(decisec_to_sec(msg.age as f64))
        } else {
            None
        };
    }

    pub fn handle_prot_lvl(&mut self, msg: ProtectionLevel) {
        let fields = msg.fields();
        // Send protection level
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut solution_protection_level = msg.init_solution_protection_level();
        solution_protection_level.set_lat(fields.lat);
        solution_protection_level.set_lon(fields.lon);
        solution_protection_level.set_hpl(fields.hpl);
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    /// Handle PosLLH / PosLLHDepA messages.
    ///
    /// TODO(johnmichael.burke@) <https://swift-nav.atlassian.net/browse/CPP-95>
    /// Need to validate logging.
    pub fn handle_pos_llh(&mut self, msg: PosLLH) {
        self.last_pos_mode = msg.mode();
        let pos_llh_fields = msg.fields();
        self.h_acc = pos_llh_fields.h_accuracy;
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
            utc_time_str = Some(format!("{tutc}:{secutc:0>6.03}"));
        }
        let mut gps_time = None;
        let mut gps_time_short = None;
        if let Some(tgps) = tgps_ {
            if let Some(secgps) = secgps_ {
                gps_time = Some(format!("{tgps}:{secgps:0>6.06}"));
                gps_time_short = Some(format!("{tgps}:{secgps:0>6.03}"));
            }
        }

        // TODO(johnmichael.burke@) https://swift-nav.atlassian.net/browse/CPP-95
        // Validate logging.
        {
            let mut shared_data = self.shared_state.lock();
            if let Some(ref mut pos_file) = shared_data.solution_tab.position_tab.log_file {
                let pc_time = format!("{tloc}:{secloc:>6.06}");
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
                    error!("Unable to to write to pos llh log, error {}.", err);
                }
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
            self.table.insert(LON, String::from(EMPTY_STR));
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
            self.table.insert(GPS_TOW, format!("{tow:.3}"));
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
            self.table.insert(LAT, format!("{:.9}", pos_llh_fields.lat));
            self.table.insert(LON, format!("{:.9}", pos_llh_fields.lon));
            self.table
                .insert(HEIGHT, format!("{:.3}", pos_llh_fields.height));
            self.table
                .insert(HORIZ_ACC, format!("{:.3}", pos_llh_fields.h_accuracy));
            self.table
                .insert(VERT_ACC, format!("{:.3}", pos_llh_fields.v_accuracy));
            self.lats.push(pos_llh_fields.lat);
            self.lons.push(pos_llh_fields.lon);
            self.alts.push(pos_llh_fields.height);
            self.modes.push(self.last_pos_mode);

            if self.shared_state.auto_survey_requested() {
                let lat = self.lats.iter().sum::<f64>() / self.lats.len() as f64;
                let lon = self.lons.iter().sum::<f64>() / self.lons.len() as f64;
                let alt = self.alts.iter().sum::<f64>() / self.alts.len() as f64;

                self.shared_state.set_auto_survey_result(lat, lon, alt);
                self.shared_state.set_settings_auto_survey_request(true);
            }
        }
        self.table
            .insert(POS_FLAGS, format!("0x{:<03x}", pos_llh_fields.flags));
        self.table.insert(INS_USED, format_bool(self.ins_used));
        self.table.insert(
            POS_FIX_MODE,
            GnssModes::from(self.last_pos_mode).to_string(),
        );
        if let Some(corr_age) = self.age_corrections {
            self.table.insert(CORR_AGE_S, format!("{corr_age:.1}"));
        }
        let (clear, pause) = self.check_state();
        self.solution_draw(clear, pause);
        self.send_solution_data();
        self.send_table_data();
    }

    pub fn check_state(&mut self) -> (bool, bool) {
        let (clear, pause, new_unit) = {
            let mut shared_data = self.shared_state.lock();
            let clear = shared_data.solution_tab.position_tab.clear;
            shared_data.solution_tab.position_tab.clear = false;
            let pause = shared_data.solution_tab.position_tab.pause;
            let new_unit = shared_data.solution_tab.position_tab.unit.take();
            (clear, pause, new_unit)
        };
        if let Some(unit) = new_unit {
            if unit != self.unit {
                self.convert_points(unit);
            }
        }
        (clear, pause)
    }

    pub fn clear_sln(&mut self) {
        self.pending_sln_line.clear();
        for (_, deque) in &mut self.slns.iter_mut() {
            deque.clear();
        }
    }

    /// Initiates preprocessing of solution data and handles frontend input.
    ///
    /// # Parameters
    /// - `clear`: Indicates whether to initiate a clearing of all solution data stored.
    /// - `pause`: Indicates whther or not to pause the plot updates.
    pub fn solution_draw(&mut self, clear: bool, pause: bool) {
        if clear {
            self.clear_sln();
        } else if pause {
            return;
        }
        let current_mode: Option<String> = if !self.pending_draw_modes.is_empty() {
            self.lat_min = f64::MAX;
            self.lat_max = f64::MIN;
            self.lon_min = f64::MAX;
            self.lon_max = f64::MIN;
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

    /// Calculate mean lat/lon sigfigs and offsets used for roughly converting
    /// lat/lon degrees to x/y meters. Use lat/lon points from the mode currently
    /// used for as the current solution mode. Drops NAN values.
    fn calc_deg_to_meters_lat_lon_sf_and_offset(
        &mut self,
        unit: &LatLonUnits,
    ) -> (f64, f64, f64, f64) {
        let gnss_mode = GnssModes::from(self.last_pos_mode);
        let mode_string = gnss_mode.to_string();
        let lat_str = format!("lat_{mode_string}");
        let lon_str = format!("lon_{mode_string}");
        let (lat_offset, lat_sf, lon_sf) = {
            if let Some(lats) = self.slns.get_mut(lat_str.as_str()) {
                let lats_counts = lats.iter().filter(|&x| !x.is_nan()).count();
                let lat_offset =
                    lats.iter().filter(|&x| !x.is_nan()).sum::<f64>() / lats_counts as f64;
                let (lat_sf, lon_sf) = unit.get_sig_figs(lat_offset);
                (lat_offset, lat_sf, lon_sf)
            } else {
                (0.0, 1.0, 1.0)
            }
        };

        let lon_offset = if let Some(lons) = self.slns.get_mut(lon_str.as_str()) {
            let lons_counts = lons.iter().filter(|&x| !x.is_nan()).count();
            lons.iter().filter(|&x| !x.is_nan()).sum::<f64>() / lons_counts as f64
        } else {
            0.0
        };
        (lat_offset, lat_sf, lon_offset, lon_sf)
    }

    fn convert_points(&mut self, unit: LatLonUnits) {
        let (
            (lat_offset, lat_sf, lon_offset, lon_sf),
            (old_lat_offset, old_lat_sf, old_lon_offset, old_lon_sf),
            convert,
        ) = match &unit {
            LatLonUnits::Degrees => (
                (0.0, 1.0, 0.0, 1.0),
                (self.lat_offset, self.lat_sf, self.lon_offset, self.lon_sf),
                &ll_meters_to_deg as &dyn Fn(f64, f64, f64) -> f64,
            ),
            LatLonUnits::Meters => {
                let (lat_offset, lat_sf, lon_offset, lon_sf) =
                    self.calc_deg_to_meters_lat_lon_sf_and_offset(&unit);
                (
                    (lat_offset, lat_sf, lon_offset, lon_sf),
                    (lat_offset, lat_sf, lon_offset, lon_sf),
                    &ll_deg_to_meters as &dyn Fn(f64, f64, f64) -> f64,
                )
            }
        };
        for mode in self.mode_strings.iter() {
            let lat_str = format!("lat_{mode}");
            let lon_str = format!("lon_{mode}");
            if let Some(lats) = self.slns.get_mut(lat_str.as_str()) {
                for lat in lats.iter_mut() {
                    *lat = convert(*lat, old_lat_sf, old_lat_offset);
                }
            }
            if let Some(lons) = self.slns.get_mut(lon_str.as_str()) {
                for lon in lons.iter_mut() {
                    *lon = convert(*lon, old_lon_sf, old_lon_offset);
                }
            }
        }
        for (lon, lat) in self.pending_sln_line.iter_mut() {
            *lon = convert(*lon, old_lon_sf, old_lon_offset);
            *lat = convert(*lat, old_lat_sf, old_lat_offset);
        }
        self.lat_sf = lat_sf;
        self.lon_sf = lon_sf;
        self.lat_offset = lat_offset;
        self.lon_offset = lon_offset;
        self.unit = unit;
    }

    /// Update a single mode's solution data for with lat and lon values.
    ///
    /// # Parameters
    /// - `last_lat`: The latitude value to update solution data with.
    /// - `last_lon`: The longitude value to update solution data with.
    /// - `mode_string`: The mode associated with the update in string form.
    fn _update_sln_data_by_mode(&mut self, last_lat: f64, last_lon: f64, mode_string: String) {
        let lat = (last_lat - self.lat_offset) * self.lat_sf;
        let lon = (last_lon - self.lon_offset) * self.lon_sf;

        let lat_str = format!("lat_{mode_string}");
        let lon_str = format!("lon_{mode_string}");
        let lat_str = lat_str.as_str();
        let lon_str = lon_str.as_str();
        self.slns.get_mut(lat_str).unwrap().push(lat);
        self.slns.get_mut(lon_str).unwrap().push(lon);
        self.pending_sln_line.push((lon, lat));
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
            let lat_str = format!("lat_{each_mode}");
            let lon_str = format!("lon_{each_mode}");
            let lat_str = lat_str.as_str();
            let lon_str = lon_str.as_str();
            self.slns.get_mut(lat_str).unwrap().push(f64::NAN);
            self.slns.get_mut(lon_str).unwrap().push(f64::NAN);
        }
    }

    /// Prepare data to be sent to frontend for a single mode.
    ///
    /// # Parameters
    /// - `mode_string`: The mode string to attempt to prepare data for frontend.
    /// - `update_current`: Indicating whether the current solution should be updated by
    /// this modes last lat/lon entry.
    fn _synchronize_plot_data_by_mode(&mut self, mode_string: &str, update_current: bool) {
        let idx = match self.mode_strings.iter().position(|x| x == mode_string) {
            Some(idx) => idx,
            _ => return,
        };

        let lat_string = format!("lat_{mode_string}");
        let lat_string = lat_string.as_str();
        let lon_string = format!("lon_{mode_string}");
        let lon_string = lon_string.as_str();

        let lat_values = &self.slns[&lat_string];
        let lon_values = &self.slns[&lon_string];

        self.sln_data[idx].clear();
        for jdx in 0..lat_values.len() {
            if lat_values[jdx].is_nan() || lon_values[jdx].is_nan() {
                continue;
            }
            self.lat_min = f64::min(self.lat_min, lat_values[jdx]);
            self.lat_max = f64::max(self.lat_max, lat_values[jdx]);
            self.lon_min = f64::min(self.lon_min, lon_values[jdx]);
            self.lon_max = f64::max(self.lon_max, lon_values[jdx]);
            self.sln_data[idx].push((lon_values[jdx], lat_values[jdx]));
        }
        self.sln_cur_data[idx].clear();
        if update_current && !self.sln_data[idx].is_empty() {
            self.sln_cur_data[idx].push(self.sln_data[idx][self.sln_data[idx].len() - 1]);
        }
        self.sln_line = self.pending_sln_line.clone();
    }

    /// Package solution data into a message buffer and send to frontend.
    pub fn send_solution_data(&mut self) {
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
            available_units.set(i as u32, unit);
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
        let mut solution_line = solution_status
            .reborrow()
            .init_line_data(self.sln_line.len() as u32);
        for (i, (x, y)) in self.sln_line.iter().enumerate() {
            let mut point_idx = solution_line.reborrow().get(i as u32);
            point_idx.set_x(*x);
            point_idx.set_y(*y);
        }

        solution_status.set_h_acc(self.h_acc);
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    /// Package solution table data into a message buffer and send to frontend.
    pub fn send_table_data(&mut self) {
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

fn ll_deg_to_meters(l: f64, sf: f64, offset: f64) -> f64 {
    (l - offset) * sf
}
fn ll_meters_to_deg(l: f64, sf: f64, offset: f64) -> f64 {
    l / sf + offset
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use sbp::messages::navigation::{
        MsgAgeCorrections, MsgDops, MsgDopsDepA, MsgGpsTime, MsgPosLlh, MsgPosLlhDepA, MsgVelNed,
        MsgVelNedDepA,
    };
    use std::{thread::sleep, time::Duration};

    #[test]
    fn handle_utc_time_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut solution_table = SolutionPositionTab::new(shared_state, client_send);
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
        let datetime = utc_time(
            year as i32,
            month as u32,
            day as u32,
            hours as u32,
            minutes as u32,
            seconds as u32,
            ns,
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
        let client_send = TestSender::boxed();
        let mut solution_table = SolutionPositionTab::new(shared_state, client_send);
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
        solution_table.week = Some(old_wn);
        solution_table.nsec = Some(old_nsec);
        solution_table.handle_gps_time(GpsTime::MsgGpsTime(msg));
        assert_eq!(solution_table.week, Some(old_wn));
        assert_eq!(solution_table.nsec, Some(old_nsec));

        let good_flags = 1_u8;
        let msg = MsgGpsTime {
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
        let good_flags = 0x07;
        let bad_flags = 0xF0;
        let n = 1;
        let e = 2;
        let d = 3;
        let n_sats = 13;
        let msg = VelNED::MsgVelNed(MsgVelNed {
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
        assert_eq!(solution_tab.table[VEL_FLAGS], format!("0x{bad_flags:<03x}"));
        assert_eq!(solution_tab.table[VEL_N], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_E], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VEL_D], String::from(EMPTY_STR));
        let msg = VelNED::MsgVelNed(MsgVelNed {
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
            format!("0x{good_flags:<03x}")
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
        let msg = VelNED::MsgVelNedDepA(MsgVelNedDepA {
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
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
        let client_send = TestSender::boxed();
        let mut solution_tab = SolutionPositionTab::new(shared_state, client_send);
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
        let msg = PosLLH::MsgPosLlh(MsgPosLlh {
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
        assert_eq!(solution_tab.table[LON], String::from(EMPTY_STR));
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
        assert_eq!(solution_tab.table[LON], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.last_pos_mode, 0);

        let good_flags = 0x01;
        let msg = PosLLH::MsgPosLlh(MsgPosLlh {
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
        assert_eq!(solution_tab.table[LON], String::from(EMPTY_STR));
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
        assert_ne!(solution_tab.table[LON], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[HEIGHT], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[HORIZ_ACC], String::from(EMPTY_STR));
        assert_ne!(solution_tab.table[VERT_ACC], String::from(EMPTY_STR));
        assert_eq!(solution_tab.last_pos_mode, good_flags);

        assert_eq!(solution_tab.last_pos_mode, 1);

        let msg = PosLLH::MsgPosLlhDepA(MsgPosLlhDepA {
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
