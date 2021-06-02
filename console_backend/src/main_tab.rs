use chrono::Local;
use log::{debug, error};
use sbp::{messages::SBP, time::GpsTime};
use std::{path::PathBuf, result::Result, thread::sleep, time::Instant};

use crate::advanced_ins_tab::AdvancedInsTab;
use crate::common_constants::SbpLogging;
use crate::constants::*;
use crate::observation_tab::ObservationTab;
use crate::output::*;
use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::status_bar::StatusBar;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;
use crate::utils::refresh_loggingbar;

pub struct MainTab<'a, S: MessageSender> {
    logging_directory: PathBuf,
    last_csv_logging: CsvLogging,
    last_sbp_logging: SbpLogging,
    sbp_logger: Option<SbpLogger>,
    last_gps_update: Instant,
    last_gps_time: Option<GpsTime>,
    client_sender: S,
    shared_state: SharedState,
    pub advanced_ins_tab: AdvancedInsTab<S>,
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
    pub observation_tab: ObservationTab<S>,
    pub solution_velocity_tab: SolutionVelocityTab<'a, S>,
    pub status_bar: StatusBar<S>,
}

impl<'a, S: MessageSender> MainTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> MainTab<'a, S> {
        MainTab {
            logging_directory: shared_state.logging_directory(),
            last_csv_logging: CsvLogging::OFF,
            last_sbp_logging: SbpLogging::OFF,
            sbp_logger: None,
            last_gps_time: None,
            last_gps_update: Instant::now(),
            client_sender: client_sender.clone(),
            shared_state: shared_state.clone(),
            advanced_ins_tab: AdvancedInsTab::new(shared_state.clone(), client_sender.clone()),
            tracking_signals_tab: TrackingSignalsTab::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            observation_tab: ObservationTab::new(shared_state.clone(), client_sender.clone()),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            status_bar: StatusBar::new(shared_state, client_sender),
        }
    }

    /// Calculate time since last epoch began and sleep for previous epoch time difference.
    ///
    /// # Parameters
    /// - `gps_time`: The GpsTime corresponding to a message.
    pub fn realtime_delay<T>(&mut self, gps_time: Option<Result<GpsTime, T>>) {
        if let Some(Ok(g_time)) = gps_time {
            if let Some(l_time) = self.last_gps_time {
                if l_time < g_time {
                    let diff = g_time - l_time;
                    let elapsed = self.last_gps_update.elapsed();
                    if diff > elapsed {
                        let sleep_duration = diff - elapsed;
                        debug!(
                            "Realtime delay encounterred. Sleeping for {:?}.",
                            sleep_duration
                        );
                        sleep(sleep_duration);
                    }
                    self.last_gps_update = Instant::now();
                    self.last_gps_time = Some(g_time);
                }
            } else {
                self.last_gps_time = Some(g_time);
            }
        }
    }

    /// Initialize Baseline and Solution Position and Velocity Loggers.
    ///
    /// # Generates:
    /// - `Solution Position Log`
    /// - `Solution Velocity Log`
    /// - `Baseline Log` // TODO(john-michaelburke@) [CPP-1337] Implement Baseline log.
    ///
    /// # Parameters:
    /// - `logging`: The type of sbp logging to use; otherwise, None.
    pub fn init_csv_logging(&mut self) {
        let local_t = Local::now();
        let vel_log_file = local_t.format(VEL_TIME_STR_FILEPATH).to_string();
        let vel_log_file = self.logging_directory.join(vel_log_file);
        self.solution_tab.vel_log_file = match CsvSerializer::new(&vel_log_file) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", vel_log_file, e);
                None
            }
        };
        let pos_log_file = local_t.format(POS_LLH_TIME_STR_FILEPATH).to_string();
        let pos_log_file = self.logging_directory.join(pos_log_file);
        self.solution_tab.pos_log_file = match CsvSerializer::new(&pos_log_file) {
            Ok(pos_csv) => Some(pos_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", pos_log_file, e);
                None
            }
        };
        self.shared_state.set_csv_logging(CsvLogging::ON);
    }
    pub fn end_csv_logging(&mut self) -> crate::types::Result<()> {
        self.shared_state.set_csv_logging(CsvLogging::OFF);
        if let Some(vel_log) = &mut self.solution_tab.vel_log_file {
            vel_log.flush()?;
            self.solution_tab.vel_log_file = None;
        }
        if let Some(pos_log) = &mut self.solution_tab.pos_log_file {
            pos_log.flush()?;
            self.solution_tab.pos_log_file = None;
        }
        Ok(())
    }

    /// Initialize SBP Logger.
    ///
    /// # Parameters:
    /// - `logging`: The type of sbp logging to use; otherwise, None.
    pub fn init_sbp_logging(&mut self, logging: SbpLogging) {
        let local_t = Local::now();
        self.sbp_logger = match logging {
            SbpLogging::SBP => {
                let sbp_log_file = local_t.format(SBP_FILEPATH).to_string();
                let sbp_log_file = self.logging_directory.join(sbp_log_file);
                match SbpLogger::new_sbp(&sbp_log_file) {
                    Ok(logger) => Some(logger),
                    Err(e) => {
                        error!("issue creating file, {:?}, error, {}", sbp_log_file, e);
                        None
                    }
                }
            }
            SbpLogging::SBP_JSON => {
                let sbp_json_log_file = local_t.format(SBP_JSON_FILEPATH).to_string();
                let sbp_json_log_file = self.logging_directory.join(sbp_json_log_file);
                match SbpLogger::new_sbp_json(&sbp_json_log_file) {
                    Ok(logger) => Some(logger),
                    Err(e) => {
                        error!("issue creating file, {:?}, error, {}", sbp_json_log_file, e);
                        None
                    }
                }
            }
            _ => None,
        };
        self.shared_state.set_sbp_logging(logging);
    }
    pub fn serialize_sbp(&mut self, msg: &SBP) {
        let csv_logging;
        let sbp_logging;
        let directory;
        {
            let shared_data = self.shared_state.lock().unwrap();
            csv_logging = (*shared_data).logging_bar.csv_logging.clone();
            sbp_logging = (*shared_data).logging_bar.sbp_logging.clone();
            directory = (*shared_data).logging_bar.logging_directory.clone();
        }
        self.logging_directory = self.shared_state.clone().logging_directory();

        if self.logging_directory != directory {
            if let Err(e) = create_directory(directory.clone()) {
                error!("Issue creating directory {}.", e);
                self.shared_state
                    .set_logging_directory(self.logging_directory.clone());
            } else {
                self.shared_state.update_folder_history(directory.clone());
                self.logging_directory = directory;
            }
            refresh_loggingbar(&mut self.client_sender, self.shared_state.clone());
        }

        if self.last_csv_logging != csv_logging {
            if let Err(e) = self.end_csv_logging() {
                error!("Issue closing csv file, {}", e);
            }
            if let CsvLogging::ON = &csv_logging {
                self.init_csv_logging();
            }
            self.last_csv_logging = csv_logging;
            refresh_loggingbar(&mut self.client_sender, self.shared_state.clone());
        }
        if self.last_sbp_logging != sbp_logging {
            self.close_sbp();
            if let SbpLogging::OFF = &sbp_logging {
            } else {
                self.init_sbp_logging(sbp_logging.clone());
            }
            self.last_sbp_logging = sbp_logging;
            refresh_loggingbar(&mut self.client_sender, self.shared_state.clone());
        }

        if let Some(sbp_logger) = &mut self.sbp_logger {
            if let Err(e) = sbp_logger.serialize(msg) {
                error!("error, {}, unable to log sbp msg, {:?}", e, msg);
            }
        }
    }
    pub fn close_sbp(&mut self) {
        self.shared_state.set_sbp_logging(SbpLogging::OFF);
        self.sbp_logger = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PosLLH, VelNED};
    use crate::utils::{mm_to_m, ms_to_sec};
    use glob::glob;
    use sbp::messages::navigation::{MsgPosLLH, MsgVelNED};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        sync::mpsc,
        time::Duration,
    };
    use tempfile::TempDir;

    struct GpsTimeTests {
        pub zero_week: i16,
        pub good_week: i16,
        pub early_gps_tow_good: f64,
        pub later_gps_tow_good: f64,
    }
    impl GpsTimeTests {
        fn new() -> GpsTimeTests {
            let zero_week: i16 = 0;
            let good_week: i16 = 2000;
            let early_gps_tow_good: f64 = 5432.0;
            let later_gps_tow_good: f64 = 5433.0;
            GpsTimeTests {
                zero_week,
                good_week,
                early_gps_tow_good,
                later_gps_tow_good,
            }
        }
    }

    #[test]
    fn realtime_delay_full_test() {
        let shared_state = SharedState::new();
        let (client_send_, _) = mpsc::channel::<Vec<u8>>();
        let client_send = ClientSender {
            inner: client_send_,
        };
        let gps_s = GpsTimeTests::new();
        let mut main = MainTab::new(shared_state, client_send);
        let early_gps_time_good = GpsTime::new(gps_s.good_week, gps_s.early_gps_tow_good).unwrap();
        let later_gps_time_good = GpsTime::new(gps_s.good_week, gps_s.later_gps_tow_good);
        main.last_gps_time = Some(early_gps_time_good);
        let now = Instant::now();
        main.last_gps_update = Instant::now();
        main.realtime_delay(Some(later_gps_time_good));
        assert!(
            now.elapsed()
                > Duration::from_secs_f64(gps_s.later_gps_tow_good - gps_s.early_gps_tow_good)
        );
    }

    #[test]
    fn realtime_delay_no_last_test() {
        let shared_state = SharedState::new();
        let (client_send_, _) = mpsc::channel::<Vec<u8>>();
        let client_send = ClientSender {
            inner: client_send_,
        };
        let gps_s = GpsTimeTests::new();
        let mut main = MainTab::new(shared_state, client_send);
        let later_gps_time_good = GpsTime::new(gps_s.good_week, gps_s.later_gps_tow_good);
        let now = Instant::now();
        main.last_gps_update = Instant::now();
        main.realtime_delay(Some(later_gps_time_good));
        assert!(now.elapsed() < Duration::from_millis(5));
    }

    #[test]
    fn csv_logging_test() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut main = MainTab::new(shared_state, client_send);
        assert_eq!(main.last_csv_logging, CsvLogging::OFF);
        main.shared_state.set_csv_logging(CsvLogging::ON);
        main.shared_state.set_logging_directory(tmp_dir.clone());

        let flags = 0x01;
        let lat = 45_f64;
        let lon = -45_f64;
        let height = 1337_f64;
        let n_sats = 13;
        let h_accuracy = 0;
        let v_accuracy = 0;
        let tow = 1337;
        let sender_id = Some(1337);

        let msg = MsgPosLLH {
            sender_id,
            tow,
            lat,
            lon,
            height,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        let n = 1;
        let e = 2;
        let d = 3;
        let msg_two = MsgVelNED {
            sender_id,
            tow,
            n,
            e,
            d,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        {
            main.serialize_sbp(&SBP::MsgPosLLH(msg.clone()));
            main.solution_tab.handle_pos_llh(PosLLH::MsgPosLLH(msg));
            main.serialize_sbp(&SBP::MsgVelNED(msg_two.clone()));
            main.solution_tab
                .handle_vel_ned(VelNED::MsgVelNED(msg_two.clone()));
            assert_eq!(main.last_csv_logging, CsvLogging::ON);
            main.end_csv_logging().unwrap();
            main.serialize_sbp(&SBP::MsgVelNED(msg_two));
            assert_eq!(main.last_csv_logging, CsvLogging::OFF);
        }

        let pattern = tmp_dir.join("position_log_*");
        let path = glob(&pattern.to_string_lossy())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let mut reader = csv::Reader::from_reader(File::open(path).unwrap());
        let mut records = reader.records();
        let record = records.next().unwrap().unwrap();
        let tow_: &f64 = &record[2].parse().unwrap();
        assert!(tow_ - ms_to_sec(tow as f64) <= f64::EPSILON);
        let lat_: &f64 = &record[3].parse().unwrap();
        assert!(lat_ - lat <= f64::EPSILON);
        let lon_: &f64 = &record[4].parse().unwrap();
        assert!(lon_ - lon <= f64::EPSILON);

        let pattern = tmp_dir.join("velocity_log_*");
        let path = glob(&pattern.to_string_lossy())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let mut reader = csv::Reader::from_reader(File::open(path).unwrap());
        let mut records = reader.records();
        let record = records.next().unwrap().unwrap();
        let tow_: &f64 = &record[2].parse().unwrap();
        assert!(tow_ - ms_to_sec(tow as f64) <= f64::EPSILON);
        let n_: &f64 = &record[3].parse().unwrap();
        let e_: &f64 = &record[4].parse().unwrap();
        let d_: &f64 = &record[5].parse().unwrap();
        assert!(n_ - mm_to_m(n as f64) <= f64::EPSILON);
        assert!(e_ - mm_to_m(e as f64) <= f64::EPSILON);
        assert!(d_ - mm_to_m(d as f64) <= f64::EPSILON);
    }

    #[test]
    fn sbp_logging_test() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut main = MainTab::new(shared_state, client_send);
        assert_eq!(main.last_sbp_logging, SbpLogging::OFF);
        main.shared_state.set_sbp_logging(SbpLogging::SBP);
        main.shared_state.set_logging_directory(tmp_dir.clone());

        let flags = 0x01;
        let lat = 45_f64;
        let lon = -45_f64;
        let height = 1337_f64;
        let n_sats = 13;
        let h_accuracy = 0;
        let v_accuracy = 0;
        let tow = 1337;
        let sender_id = Some(1337);

        let msg_one = MsgPosLLH {
            sender_id,
            tow,
            lat,
            lon,
            height,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        let n = 1;
        let e = 2;
        let d = 3;
        let msg_two = MsgVelNED {
            sender_id,
            tow,
            n,
            e,
            d,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        {
            main.serialize_sbp(&SBP::MsgPosLLH(msg_one.clone()));
            main.serialize_sbp(&SBP::MsgVelNED(msg_two.clone()));
            assert_eq!(main.last_sbp_logging, SbpLogging::SBP);
            main.close_sbp();
            main.serialize_sbp(&SBP::MsgVelNED(msg_two.clone()));
            assert_eq!(main.last_sbp_logging, SbpLogging::OFF);
        }

        let pattern = tmp_dir.join("swift-gnss-*.sbp");
        let path = glob(&pattern.to_string_lossy())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let file_read = File::open(path).unwrap();
        let mut messages = sbp::iter_messages(file_read);
        let msg = messages.next().unwrap().unwrap();
        match msg {
            SBP::MsgPosLLH(msg) => {
                assert_eq!(msg.sender_id, msg_one.sender_id);
                assert_eq!(msg.flags, msg_one.flags);
                assert_eq!(msg.tow, msg_one.tow);
                assert!(msg.lat - msg_one.lat <= f64::EPSILON);
                assert!(msg.lon - msg_one.lon <= f64::EPSILON);
                assert!(msg.height - msg_one.height <= f64::EPSILON);
            }
            _ => panic!("first message does not match"),
        }
        let msg = messages.next().unwrap().unwrap();
        match msg {
            SBP::MsgVelNED(msg) => {
                assert_eq!(msg.sender_id, msg_two.sender_id);
                assert_eq!(msg.flags, msg_two.flags);
                assert_eq!(msg.n, msg_two.n);
                assert_eq!(msg.e, msg_two.e);
                assert_eq!(msg.d, msg_two.d);
            }
            _ => panic!("second message does not match"),
        }
    }

    #[test]
    fn sbp_json_logging_test() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut main = MainTab::new(shared_state, client_send);
        assert_eq!(main.last_sbp_logging, SbpLogging::OFF);
        main.shared_state.set_sbp_logging(SbpLogging::SBP_JSON);
        main.shared_state.set_logging_directory(tmp_dir.clone());

        let flags = 0x01;
        let lat = 45_f64;
        let lon = -45_f64;
        let height = 1337_f64;
        let n_sats = 13;
        let h_accuracy = 0;
        let v_accuracy = 0;
        let tow = 1337;
        let sender_id = Some(1337);

        let msg_one = MsgPosLLH {
            sender_id,
            tow,
            lat,
            lon,
            height,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        let n = 1;
        let e = 2;
        let d = 3;
        let msg_two = MsgVelNED {
            sender_id,
            tow,
            n,
            e,
            d,
            h_accuracy,
            v_accuracy,
            n_sats,
            flags,
        };

        {
            main.serialize_sbp(&SBP::MsgPosLLH(msg_one));
            main.serialize_sbp(&SBP::MsgVelNED(msg_two.clone()));
            assert_eq!(main.last_sbp_logging, SbpLogging::SBP_JSON);
            main.close_sbp();
            main.serialize_sbp(&SBP::MsgVelNED(msg_two));
            assert_eq!(main.last_sbp_logging, SbpLogging::OFF);
        }

        let pattern = tmp_dir.join("swift-gnss-*.sbp.json");
        let path = glob(&pattern.to_string_lossy())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let file_read = File::open(path).unwrap();
        let output_file = BufReader::new(file_read);
        let mut lines = output_file.lines();
        let line = lines.next().unwrap();
        let value: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
        let value = value.as_object().unwrap();
        let lat_ = value.get("lat").unwrap();
        assert_eq!(*lat_, serde_json::json!(lat));

        let line = lines.next().unwrap();
        let value: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
        let value = value.as_object().unwrap();
        let n_ = value.get("n").unwrap();
        assert_eq!(*n_, serde_json::json!(n));
    }
}
