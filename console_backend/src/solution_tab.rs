use capnp::message::Builder;
use capnp::serialize;

use chrono::Local;

use sbp::messages::{
    navigation::{MsgAgeCorrections, MsgGPSTime, MsgUtcTime},
    system::{MsgInsStatus, MsgInsUpdates},
};
use std::{collections::HashMap, time::Instant};

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::output::CsvSerializer;
use crate::types::{
    Deque, Dops, MessageSender, PosLLH, PosLLHLog, SharedState, UtcDateTime, VelLog, VelNED,
};

const VEL_TIME_STR_FILEPATH: &str = "velocity_log_%Y%m%d-%H%M%S.csv";
const POS_LLH_TIME_STR_FILEPATH: &str = "position_log_%Y%m%d-%H%M%S.csv";

// }
/// SolutionTab struct.
///
/// # Fields
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
pub struct SolutionTab {
    pub age_corrections: Option<f64>,
    pub alts: Deque<f64>,
    pub directory_name: Option<String>,
    pub ins_status_flags: u32,
    pub ins_used: bool,
    pub lats: Deque<f64>,
    pub lngs: Deque<f64>,
    pub last_ins_status_receipt_time: Instant,
    pub last_pos_mode: u8,
    pub last_odo_update_time: Instant,
    pub logging: bool,
    pub max: f64,
    pub min: f64,
    pub mode_strings: Vec<String>,
    pub modes: Deque<u8>,
    pub nsec: Option<i32>,
    pub pending_draw_modes: Vec<String>,
    pub pos_log_file: Option<CsvSerializer>,
    pub shared_state: SharedState,
    pub sln_cur_data: Vec<(f64, f64)>,
    pub sln_data: Vec<Vec<(f64, f64)>>,
    pub slns: HashMap<String, Deque<f64>>,
    pub table: HashMap<String, String>,
    pub tows: Deque<f64>,
    pub utc_source: Option<String>,
    pub utc_time: Option<UtcDateTime>,
    pub vel_log_file: Option<CsvSerializer>,
    pub week: Option<u16>,
}

impl SolutionTab {
    pub fn new(shared_state: SharedState) -> SolutionTab {
        SolutionTab {
            age_corrections: None,
            alts: Deque::with_size_limit(PLOT_HISTORY_MAX),
            directory_name: None,
            ins_status_flags: 0,
            ins_used: false,
            lats: Deque::with_size_limit(PLOT_HISTORY_MAX),
            lngs: Deque::with_size_limit(PLOT_HISTORY_MAX),
            last_ins_status_receipt_time: Instant::now(),
            last_pos_mode: 0,
            last_odo_update_time: Instant::now(),
            logging: false,
            max: 0_f64,
            min: 0_f64,
            modes: Deque::with_size_limit(PLOT_HISTORY_MAX),
            mode_strings: vec![
                GnssModes::Spp.to_string(),
                GnssModes::Dgnss.to_string(),
                GnssModes::Float.to_string(),
                GnssModes::Fixed.to_string(),
                GnssModes::Dr.to_string(),
                GnssModes::Sbas.to_string(),
            ],
            nsec: Some(0),
            pending_draw_modes: Vec::new(),
            pos_log_file: None,
            shared_state,
            sln_cur_data: {
                let mut sln_data = Vec::new();
                for _ in 0..NUM_GNSS_MODES {
                    sln_data.push((f64::NAN, f64::NAN));
                }
                sln_data
            },
            sln_data: {
                let mut sln_data = Vec::new();
                for _ in 0..NUM_GNSS_MODES {
                    sln_data.push(Vec::new());
                }
                sln_data
            },
            slns: {
                let mut slns_map = HashMap::new();
                for key in SOLUTIONS_KEYS {
                    slns_map.insert(String::from(*key), Deque::with_size_limit(PLOT_HISTORY_MAX));
                }
                slns_map
            },
            table: {
                let mut table = HashMap::new();
                for key in SOLUTION_TABLE_KEYS {
                    table.insert(String::from(*key), String::from(EMPTY_STR));
                }
                table
            },
            tows: Deque::with_size_limit(PLOT_HISTORY_MAX),
            utc_source: None,
            utc_time: None,
            vel_log_file: None,
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

    pub fn init_logging(
        &mut self,
        directory_name: Option<String>,
        filepath_: String,
    ) -> Option<CsvSerializer> {
        let mut filepath = filepath_.clone();
        if let Some(dir_name) = directory_name.clone() {
            filepath = format!("{}/{}", dir_name, filepath);
        }
        match CsvSerializer::new(filepath.clone()) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                if directory_name.is_some() {
                    eprintln!(
                        "Issue creating file in directory, {}, error, {}.",
                        filepath, e
                    );
                    self.init_logging(None, filepath_);
                } else {
                    eprintln!("issue creating file, {}, error, {}", filepath, e);
                }
                None
            }
        }
    }

    pub fn handle_vel_ned(&mut self, msg: VelNED) {
        let (flags, tow, n, e, d, n_sats) = match msg {
            VelNED::MsgVelNED(msg) => (
                msg.flags,
                msg.tow as f64,
                msg.n as f64,
                msg.e as f64,
                msg.d as f64,
                msg.n_sats,
            ),
            VelNED::MsgVelNEDDepA(msg) => (
                1,
                msg.tow as f64,
                msg.n as f64,
                msg.e as f64,
                msg.d as f64,
                msg.n_sats,
            ),
        };
        let speed: f64 = mm_to_m(f64::sqrt(n * n + e * e));
        let n = mm_to_m(n);
        let e = mm_to_m(e);
        let d = mm_to_m(d);
        let mut tow = tow * 1.0e-3_f64;
        if let Some(nsec) = self.nsec {
            tow += nsec as f64 * 1.0e-9_f64;
        }

        let ((tloc, secloc), (tgps_, secgps_)) = log_time_strings(self.week, tow);

        if self.logging {
            if let None = self.vel_log_file {
                let local_t = Local::now();
                let filepath = local_t.format(VEL_TIME_STR_FILEPATH).to_string();

                self.vel_log_file = self.init_logging(self.directory_name.clone(), filepath);
            }
            if let Some(vel_file) = &mut self.vel_log_file {
                let mut gps_time = None;
                if let Some(tgps) = tgps_ {
                    if let Some(secgps) = secgps_ {
                        gps_time = Some(format!("{}:{:0>6.06}", tgps, secgps));
                    }
                }
                let pc_time = format!("{}:{:0>6.06}", tloc, secloc);
                if let Err(err) = vel_file.serialize_vel_log(&VelLog {
                    pc_time,
                    gps_time,
                    tow_s: Some(tow),
                    north_mps: Some(n),
                    east_mps: Some(e),
                    down_mps: Some(d),
                    speed_mps: Some(speed),
                    flags,
                    num_signals: n_sats,
                }) {
                    eprintln!("Unable to to write to vel log, error {}.", err);
                }
            } else {
                eprintln!("Unable to write to vel log file.");
            }
        } else {
            self.vel_log_file = None;
        }
        self.table
            .insert(String::from(VEL_FLAGS), format!("0x{:<03x}", flags));
        if (flags & 0x7) != 0 {
            self.table
                .insert(String::from(VEL_N), format!("{: >8.4}", n));
            self.table
                .insert(String::from(VEL_E), format!("{: >8.4}", e));
            self.table
                .insert(String::from(VEL_D), format!("{: >8.4}", d));
        } else {
            self.table
                .insert(String::from(VEL_N), String::from(EMPTY_STR));
            self.table
                .insert(String::from(VEL_E), String::from(EMPTY_STR));
            self.table
                .insert(String::from(VEL_D), String::from(EMPTY_STR));
        }
    }

    pub fn handle_ins_updates(&mut self, msg: MsgInsUpdates) {
        let tic = msg.wheelticks;
        if ((tic & 0xF0) >> 4) > (tic & 0x0F) {
            self.last_odo_update_time = Instant::now();
            let mut shared_data = self.shared_state.lock().unwrap();
            (*shared_data)
                .solution_tab
                .position_tab
                .last_odo_update_time = self.last_odo_update_time.clone();
        }
    }

    pub fn handle_ins_status(&mut self, msg: MsgInsStatus) {
        self.ins_status_flags = msg.flags;
        self.last_ins_status_receipt_time = Instant::now();
        let mut shared_data = self.shared_state.lock().unwrap();
        (*shared_data).solution_tab.position_tab.ins_status_flags = msg.flags;
        (*shared_data)
            .solution_tab
            .position_tab
            .last_ins_status_receipt_time = self.last_ins_status_receipt_time.clone();
    }

    pub fn handle_dops(&mut self, msg: Dops) {
        let (pdop, gdop, tdop, hdop, vdop, flags) = match msg {
            Dops::MsgDops(msg_) => (
                msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, msg_.flags,
            ),
            Dops::MsgDopsDepA(msg_) => {
                (msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, 1_u8)
            }
        };
        self.table
            .insert(String::from(DOPS_FLAGS), format!("0x{:<03x}", flags));
        self.table.insert(
            String::from(INS_STATUS),
            format!("0x{:<08x}", self.ins_status_flags),
        );
        if flags != 0 {
            self.table.insert(
                String::from(PDOP),
                format!("{:.1}", pdop as f64 * DILUTION_OF_PRECISION_UNITS),
            );
            self.table.insert(
                String::from(GDOP),
                format!("{:.1}", gdop as f64 * DILUTION_OF_PRECISION_UNITS),
            );
            self.table.insert(
                String::from(TDOP),
                format!("{:.1}", tdop as f64 * DILUTION_OF_PRECISION_UNITS),
            );
            self.table.insert(
                String::from(HDOP),
                format!("{:.1}", hdop as f64 * DILUTION_OF_PRECISION_UNITS),
            );
            self.table.insert(
                String::from(VDOP),
                format!("{:.1}", vdop as f64 * DILUTION_OF_PRECISION_UNITS),
            );
        } else {
            self.table
                .insert(String::from(PDOP), String::from(EMPTY_STR));
            self.table
                .insert(String::from(GDOP), String::from(EMPTY_STR));
            self.table
                .insert(String::from(TDOP), String::from(EMPTY_STR));
            self.table
                .insert(String::from(HDOP), String::from(EMPTY_STR));
            self.table
                .insert(String::from(VDOP), String::from(EMPTY_STR));
        }
    }

    pub fn handle_age_corrections(&mut self, msg: MsgAgeCorrections) {
        if msg.age != 0xFFFF {
            self.age_corrections = Some(decisec_to_sec(msg.age as f64));
        } else {
            self.age_corrections = None;
        }
    }

    pub fn handle_pos_llh<P: MessageSender>(&mut self, msg: PosLLH, client_send: &mut P) {
        self.last_pos_mode = msg.mode();
        let (flags, h_accuracy, v_accuracy, tow, lat, lon, height, n_sats) = match msg {
            PosLLH::MsgPosLLH(msg_) => (
                msg_.flags,
                mm_to_m(msg_.h_accuracy as f64),
                mm_to_m(msg_.v_accuracy as f64),
                msg_.tow as f64,
                msg_.lat,
                msg_.lon,
                msg_.height,
                msg_.n_sats,
            ),
            PosLLH::MsgPosLLHDepA(msg_) => (
                msg_.flags,
                mm_to_m(msg_.h_accuracy as f64),
                mm_to_m(msg_.v_accuracy as f64),
                msg_.tow as f64,
                msg_.lat,
                msg_.lon,
                msg_.height,
                msg_.n_sats,
            ),
        };
        let gnss_mode = GnssModes::from(self.last_pos_mode);
        let mode_string = gnss_mode.to_string();
        if self.last_pos_mode != 0 {
            if !self.pending_draw_modes.contains(&mode_string) {
                self.pending_draw_modes.push(mode_string.clone());
            }
            self._update_sln_data_by_mode(lat, lon, mode_string);
        } else {
            self._append_empty_sln_data(None);
        }
        self.ins_used = ((flags & 0x8) >> 3) == 1;
        let mut tow = tow * 1.0e-3_f64;
        if let Some(nsec) = self.nsec {
            tow += nsec as f64 * 1.0e-9_f64;
        }

        let ((tloc, secloc), (tgps_, secgps_)) = log_time_strings(self.week, tow);
        let mut utc_time_str = None;
        if let Some(utc_time_) = self.utc_time {
            let (tutc, secutc) = datetime_2_str_utc(utc_time_);
            utc_time_str = Some(format!("{}:{:0>6.03}", tutc, secutc));
        }
        let mut gps_time = None;
        let mut gps_time_short = None;

        if self.logging {
            if let None = self.pos_log_file {
                let local_t = Local::now();
                let filepath = local_t.format(POS_LLH_TIME_STR_FILEPATH).to_string();

                self.pos_log_file = self.init_logging(self.directory_name.clone(), filepath);
            }
            if let Some(pos_file) = &mut self.pos_log_file {
                if let Some(tgps) = tgps_ {
                    if let Some(secgps) = secgps_ {
                        gps_time = Some(format!("{}:{:0>6.06}", tgps, secgps));
                        gps_time_short = Some(format!("{}:{:0>6.03}", tgps, secgps));
                    }
                }
                let pc_time = format!("{}:{:0>6.06}", tloc, secloc);
                if let Err(err) = pos_file.serialize_pos_llh_log(&PosLLHLog {
                    pc_time,
                    gps_time,
                    tow_s: Some(tow),
                    latitude_d: Some(lat),
                    longitude_d: Some(lon),
                    altitude_m: Some(height),
                    h_accuracy_m: Some(h_accuracy),
                    v_accuracy_m: Some(v_accuracy),
                    n_sats,
                    flags,
                }) {
                    eprintln!("Unable to to write to pos llh log, error {}.", err);
                }
            } else {
                eprintln!("Unable to write to vel pos llh file.");
            }
        } else {
            self.pos_log_file = None;
        }

        if self.last_pos_mode == 0 {
            self.table
                .insert(String::from(GPS_WEEK), String::from(EMPTY_STR));
            self.table
                .insert(String::from(GPS_TOW), String::from(EMPTY_STR));
            self.table
                .insert(String::from(GPS_TIME), String::from(EMPTY_STR));
            self.table
                .insert(String::from(UTC_TIME), String::from(EMPTY_STR));
            self.table
                .insert(String::from(UTC_SRC), String::from(EMPTY_STR));
            self.table
                .insert(String::from(SATS_USED), String::from(EMPTY_STR));
            self.table
                .insert(String::from(LAT), String::from(EMPTY_STR));
            self.table
                .insert(String::from(LNG), String::from(EMPTY_STR));
            self.table
                .insert(String::from(HEIGHT), String::from(EMPTY_STR));
            self.table
                .insert(String::from(HORIZ_ACC), String::from(EMPTY_STR));
            self.table
                .insert(String::from(VERT_ACC), String::from(EMPTY_STR));
        } else {
            if let Some(week) = self.week {
                self.table.insert(String::from(GPS_WEEK), week.to_string());
                if let Some(gps_time_) = gps_time_short {
                    self.table.insert(String::from(GPS_TIME), gps_time_);
                }
            }
            self.table
                .insert(String::from(GPS_TOW), format!("{:.3}", tow));
            if let Some(utc_time_) = utc_time_str {
                self.table.insert(String::from(UTC_TIME), utc_time_);
                if let Some(utc_src_) = self.utc_source.clone() {
                    self.table.insert(String::from(UTC_SRC), utc_src_);
                }
            } else {
                self.table
                    .insert(String::from(UTC_TIME), String::from(EMPTY_STR));
                self.table
                    .insert(String::from(UTC_SRC), String::from(EMPTY_STR));
            }
            self.table
                .insert(String::from(SATS_USED), n_sats.to_string());
            self.table.insert(String::from(LAT), format!("{:.12}", lat));
            self.table.insert(String::from(LNG), format!("{:.12}", lon));
            self.table
                .insert(String::from(HEIGHT), format!("{:.3}", height));
            self.table
                .insert(String::from(HORIZ_ACC), format!("{:.12}", h_accuracy));
            self.table
                .insert(String::from(VERT_ACC), format!("{:.12}", v_accuracy));

            self.lats.add(lat);
            self.lngs.add(lon);
            self.alts.add(height);
            self.tows.add(tow);
            self.modes.add(self.last_pos_mode);
        }
        self.table
            .insert(String::from(POS_FLAGS), format!("0x{:<03x}", flags));
        self.table
            .insert(String::from(INS_USED), self.ins_used.to_string());
        self.table
            .insert(String::from(POS_FIX_MODE), gnss_mode.to_string());
        if let Some(age_corrections_) = self.age_corrections {
            self.table
                .insert(String::from(CORR_AGE_S), age_corrections_.to_string());
        }
        self.solution_draw();
        self.send_solution_data(client_send);
        self.send_table_data(client_send);
    }

    pub fn solution_draw(&mut self) {
        let current_mode: Option<String> = if self.pending_draw_modes.len()>0 {
            Some(self.pending_draw_modes[self.pending_draw_modes.len()-1].clone())
        } else {
            None
        };
        for mode_string in self.mode_strings.clone() {
            let mut update_current = true;
            if let Some( cur_mode) = current_mode.clone() {
                update_current = mode_string == cur_mode;
            }
            self._synchronize_plot_data_by_mode(&mode_string, update_current);
            if self.pending_draw_modes.contains(&mode_string) {
                self.pending_draw_modes.retain(|x| *x != mode_string);
            }
        }
    }

    pub fn rescale_for_units_change() {}

    pub fn _display_units_changed() {}

    fn _update_sln_data_by_mode(&mut self, last_lat: f64, last_lng: f64, mode_string: String) {
        let lat = last_lat; // - self.offset) * self.sf
        let lng = last_lng; // - self.offset) * self.sf

        let lat_str = format!("lat_{}", mode_string);
        let lng_str = format!("lng_{}", mode_string);
        self.slns.get_mut(&lat_str).unwrap().add(lat);
        self.slns.get_mut(&lng_str).unwrap().add(lng);
        self._append_empty_sln_data(Some(mode_string));
    }

    fn _append_empty_sln_data(&mut self, exclude_mode: Option<String>) {
        for each_mode in self.mode_strings.clone() {
            if exclude_mode.is_some() {
                continue;
            }
            let lat_str = format!("lat_{}", each_mode);
            let lng_str = format!("lng_{}", each_mode);
            self.slns.get_mut(&lat_str).unwrap().add(f64::NAN);
            self.slns.get_mut(&lng_str).unwrap().add(f64::NAN);
        }
    }

    fn _synchronize_plot_data_by_mode(&mut self, mode_string: &String, update_current: bool) {
        let lat_string = format!("lat_{}", mode_string);
        let lng_string = format!("lng_{}", mode_string);

        if let Some(idx) = self.mode_strings.iter().position(|x| *x == *mode_string) {
            self.sln_data[idx] = self.slns[&lat_string].get()
                .iter()
                .zip(self.slns[&lng_string].get().iter())
                .filter(|(x, y)| !x.is_nan() || !y.is_nan())
                .map(|(x, y)| (*x, *y))
                .collect();
            if update_current {
                if self.sln_data[idx].len() > 0 {
                    self.sln_cur_data[idx] = self.sln_data[idx][self.sln_data[idx].len()-1];
                } else {
                    self.sln_cur_data[idx] = (f64::NAN, f64::NAN);
                }
            }
        }
    }

    /// Package data into a message buffer and send to frontend.
    ///
    /// # Parameters:
    ///
    /// - `client_send`: The MessageSender channel to be used to send data to frontend.
    fn send_solution_data<P: MessageSender>(&mut self, client_send: &mut P) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let mut solution_status = msg.init_solution_position_status();
        // velocity_status.set_min(self.min);
        // velocity_status.set_max(self.max);

        let mut solution_points = solution_status
            .reborrow()
            .init_data(self.sln_data.len() as u32);
        for idx in 0..self.sln_data.len() {
            let points = self.sln_data.get_mut(idx).unwrap().get();
            let mut point_idx = solution_points
                .reborrow()
                .init(idx as u32, points.len() as u32);
            for (i, (x, OrderedFloat(y))) in points.iter().enumerate() {
                let mut point_val = point_idx.reborrow().get(i as u32);
                point_val.set_x(*x);
                point_val.set_y(*y);
            }
        }
        // let mut colors = velocity_status
        //     .reborrow()
        //     .init_colors(self.colors.len() as u32);

        // for (i, color) in self.colors.iter().enumerate() {
        //     colors.set(i as u32, color);
        // }

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        client_send.send_data(msg_bytes);
    }

    /// Package solution table data into a message buffer and send to frontend.
    ///
    /// # Parameters:
    ///
    /// - `client_send`: The Sender channel to be used to send data to frontend.
    fn send_table_data<P: MessageSender>(&mut self, client_send: &mut P) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();
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
        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();
        client_send.send_data(msg_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::types::TestSender;
    use chrono::{TimeZone, Utc};

    #[test]
    fn handle_utc_time_test() {
        let shared_state = SharedState::new();
        let mut solution_table = SolutionTab::new(shared_state);
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
        let mut solution_table = SolutionTab::new(shared_state);
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
        solution_table.handle_gps_time(msg);
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
        solution_table.handle_gps_time(msg);
        assert_eq!(solution_table.week, Some(wn));
        assert_eq!(solution_table.nsec, Some(ns_residual));
    }

    #[test]
    fn handle_vel_ned_test() {}

    #[test]
    fn handle_ins_status_test() {}

    #[test]
    fn handle_ins_updates_test() {}

    #[test]
    fn handle_dops_test() {}

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
    //     let mut solution_velocity_tab = SolutionTab::new(shared_state);

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
