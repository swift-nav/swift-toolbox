use std::path::PathBuf;

use chrono::Local;
use log::error;
use sbp::Frame;

use crate::client_sender::BoxedClientSender;
use crate::common_constants::SbpLogging;
use crate::constants::{
    BASELINE_TIME_STR_FILEPATH, POS_LLH_TIME_STR_FILEPATH, SBP_FILEPATH, SBP_JSON_FILEPATH,
    VEL_TIME_STR_FILEPATH,
};
use crate::output::{CsvLogging, SbpLogger};
use crate::shared_state::{create_directory, SharedState};
use crate::utils::{
    refresh_log_recording_name, refresh_log_recording_size, refresh_loggingbar, OkOrLog,
};

pub struct MainTab {
    logging_directory: PathBuf,
    last_csv_logging: CsvLogging,
    last_sbp_logging: bool,
    last_sbp_logging_format: SbpLogging,
    sbp_logger: Option<SbpLogger>,
    client_sender: BoxedClientSender,
    shared_state: SharedState,
}

impl MainTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> MainTab {
        let sbp_logging_format = shared_state.sbp_logging_format();
        // reopen an existing log if we disconnected
        let sbp_logger =
            shared_state
                .sbp_logging_filepath()
                .and_then(|path| match sbp_logging_format {
                    SbpLogging::SBP_JSON => SbpLogger::open_sbp_json(path).ok(),
                    SbpLogging::SBP => SbpLogger::open_sbp(path).ok(),
                });
        let last_sbp_logging = if sbp_logger.is_none() && shared_state.sbp_logging() {
            false
        } else {
            shared_state.sbp_logging()
        };
        let csv_logging_live = shared_state
            .lock()
            .solution_tab
            .velocity_tab
            .log_file
            .is_some();
        let last_csv_logging =
            if !csv_logging_live && matches!(shared_state.csv_logging(), CsvLogging::ON) {
                CsvLogging::OFF
            } else {
                shared_state.csv_logging()
            };
        MainTab {
            logging_directory: shared_state.logging_directory(),
            last_csv_logging,
            last_sbp_logging,
            last_sbp_logging_format: sbp_logging_format,
            sbp_logger,
            client_sender,
            shared_state,
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

        if let Err(e) = create_directory(self.logging_directory.clone()) {
            error!("Issue creating directory {}.", e);
        }
        let vel_log_file = local_t.format(VEL_TIME_STR_FILEPATH).to_string();
        let vel_log_file = self.logging_directory.join(vel_log_file);
        self.shared_state.start_vel_log(&vel_log_file);

        let pos_log_file = local_t.format(POS_LLH_TIME_STR_FILEPATH).to_string();
        let pos_log_file = self.logging_directory.join(pos_log_file);
        self.shared_state.start_pos_log(&pos_log_file);

        let baseline_log_file = local_t.format(BASELINE_TIME_STR_FILEPATH).to_string();
        let baseline_log_file = self.logging_directory.join(baseline_log_file);
        self.shared_state.start_baseline_log(&baseline_log_file);

        self.shared_state.set_csv_logging(CsvLogging::ON);
    }

    pub fn end_csv_logging(&mut self) -> crate::types::Result<()> {
        self.shared_state.set_csv_logging(CsvLogging::OFF);
        self.shared_state.end_vel_log()?;
        self.shared_state.end_pos_log()?;
        self.shared_state.end_baseline_log()?;
        Ok(())
    }

    /// Initialize SBP Logger.
    ///
    /// # Parameters:
    /// - `logging`: The type of sbp logging to use; otherwise, None.
    pub fn init_sbp_logging(&mut self, logging: SbpLogging) {
        let filepath = self.sbp_logging_filepath(logging.clone());
        if let Some(parent) = filepath.parent() {
            if let Err(e) = create_directory(parent.to_path_buf()) {
                error!("Issue creating directory {}.", e);
            }
        }

        self.sbp_logger = match logging {
            SbpLogging::SBP => SbpLogger::new_sbp(&filepath),
            SbpLogging::SBP_JSON => SbpLogger::new_sbp_json(&filepath),
        }
        .ok_or_log(|e| error!("issue creating file, {}, error, {e}", filepath.display()));

        if self.sbp_logger.is_some() {
            self.shared_state.set_sbp_logging(true);
            self.shared_state
                .set_sbp_logging_filepath(Some(filepath.clone()));
            self.shared_state.set_settings_refresh(true);
        }
        self.shared_state.set_sbp_logging_format(logging);
        refresh_log_recording_name(&self.client_sender, filepath.display().to_string());
    }

    pub fn serialize_frame(&mut self, frame: &Frame) {
        let csv_logging;
        let sbp_logging;
        let sbp_logging_format;
        let directory;
        {
            let shared_data = self.shared_state.lock();
            csv_logging = shared_data.logging_bar.csv_logging.clone();
            sbp_logging = shared_data.logging_bar.sbp_logging;
            sbp_logging_format = shared_data.logging_bar.sbp_logging_format.clone();
            directory = shared_data.logging_bar.logging_directory.clone();
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
            refresh_loggingbar(&self.client_sender, &self.shared_state);
        }

        if self.last_csv_logging != csv_logging {
            if let Err(e) = self.end_csv_logging() {
                error!("Issue closing csv file, {}", e);
            }
            if let CsvLogging::ON = &csv_logging {
                self.init_csv_logging();
            }
            self.last_csv_logging = csv_logging;
            refresh_loggingbar(&self.client_sender, &self.shared_state);
        }
        if self.last_sbp_logging != sbp_logging
            || self.last_sbp_logging_format != sbp_logging_format
        {
            self.close_sbp();
            if sbp_logging {
                self.init_sbp_logging(sbp_logging_format.clone());
            }
            self.last_sbp_logging = sbp_logging;
            self.last_sbp_logging_format = sbp_logging_format;
            refresh_loggingbar(&self.client_sender, &self.shared_state);
        }

        if let Some(ref mut sbp_logger) = self.sbp_logger {
            if let Err(e) = sbp_logger.serialize(frame) {
                error!("error, {e}, unable to log sbp frame, {frame:?}");
            } else {
                let bytes_size = frame.as_bytes().len() as u16;
                refresh_log_recording_size(&self.client_sender, bytes_size);
            }
        }
    }

    pub fn close_sbp(&mut self) {
        self.sbp_logger = None;
        self.shared_state.set_sbp_logging(false);
        self.shared_state.set_sbp_logging_filepath(None);
        refresh_loggingbar(&self.client_sender, &self.shared_state);
    }

    fn sbp_logging_filepath(&self, logging: SbpLogging) -> PathBuf {
        let name = self.shared_state.sbp_logging_filename().unwrap_or_else(|| {
            let fmt = match logging {
                SbpLogging::SBP => SBP_FILEPATH,
                SbpLogging::SBP_JSON => SBP_JSON_FILEPATH,
            };
            Local::now().format(fmt).to_string().into()
        });
        self.logging_directory.join(&name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use crate::tabs::baseline_tab::BaselineTab;
    use crate::tabs::solution_tab::solution_position_tab::SolutionPositionTab;
    use crate::test_common::msg_to_frame;
    use crate::types::{BaselineNED, MsgSender, PosLLH, VelNED};
    use crate::utils::{mm_to_m, ms_to_sec};
    use glob::glob;
    use sbp::messages::navigation::{MsgBaselineNed, MsgPosLlh, MsgVelNed};
    use sbp::Sbp;
    use std::{
        fs::File,
        io::{sink, BufRead, BufReader},
    };
    use tempfile::TempDir;

    #[test]
    fn csv_logging_test() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = sink();
        let msg_sender = MsgSender::new(writer);
        let mut main = MainTab::new(shared_state.clone(), client_send.clone());
        let mut solution_tab = SolutionPositionTab::new(shared_state.clone(), client_send.clone());
        let mut baseline_tab = BaselineTab::new(shared_state, client_send, msg_sender);
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

        let msg = MsgPosLlh {
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
        let msg_two = MsgVelNed {
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

        let n_m3 = 4;
        let e_m3 = 5;
        let d_m3 = 6;
        let flags = 0x2;
        let msg_three = MsgBaselineNed {
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
            main.serialize_frame(&msg_to_frame(msg.clone()));
            solution_tab.handle_pos_llh(PosLLH::MsgPosLlh(msg));
            main.serialize_frame(&msg_to_frame(msg_two.clone()));
            solution_tab.handle_vel_ned(VelNED::MsgVelNed(msg_two.clone()));
            main.serialize_frame(&msg_to_frame(msg_three.clone()));
            baseline_tab.handle_baseline_ned(BaselineNED::MsgBaselineNed(msg_three));
            assert_eq!(main.last_csv_logging, CsvLogging::ON);
            main.end_csv_logging().unwrap();
            main.serialize_frame(&msg_to_frame(msg_two));
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

        let pattern = tmp_dir.join("baseline_log_*");
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
        assert!(n_ - mm_to_m(n_m3 as f64) <= f64::EPSILON);
        assert!(e_ - mm_to_m(e_m3 as f64) <= f64::EPSILON);
        assert!(d_ - mm_to_m(d_m3 as f64) <= f64::EPSILON);
    }

    #[test]
    fn sbp_logging_test() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut main = MainTab::new(shared_state, client_send.clone());
        assert!(!main.last_sbp_logging);
        main.shared_state.set_sbp_logging_format(SbpLogging::SBP);
        main.shared_state.set_sbp_logging(true);
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

        let msg_one = MsgPosLlh {
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
        let msg_two = MsgVelNed {
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
            main.serialize_frame(&msg_to_frame(msg_one.clone()));
            main.serialize_frame(&msg_to_frame(msg_two.clone()));
            assert_eq!(main.last_sbp_logging_format, SbpLogging::SBP);
            main.close_sbp();
            main.serialize_frame(&msg_to_frame(msg_two.clone()));
            assert!(!main.last_sbp_logging);
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
            Sbp::MsgPosLlh(msg) => {
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
            Sbp::MsgVelNed(msg) => {
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
        let client_send = TestSender::boxed();
        let mut main = MainTab::new(shared_state, client_send.clone());
        assert!(!main.last_sbp_logging);
        main.shared_state
            .set_sbp_logging_format(SbpLogging::SBP_JSON);
        main.shared_state.set_sbp_logging(true);
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

        let msg_one = MsgPosLlh {
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
        let msg_two = MsgVelNed {
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
            main.serialize_frame(&msg_to_frame(msg_one));
            main.serialize_frame(&msg_to_frame(msg_two.clone()));
            assert_eq!(main.last_sbp_logging_format, SbpLogging::SBP_JSON);
            main.close_sbp();
            main.serialize_frame(&msg_to_frame(msg_two));
            assert!(!main.last_sbp_logging);
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
