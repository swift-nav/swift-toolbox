use chrono::Local;
use log::{debug, error};
use sbp::{messages::SBP, time::GpsTime};
use std::{result::Result, thread::sleep, time::Instant};

use crate::constants::*;
use crate::observation_tab::ObservationTab;
use crate::output::*;
use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::status_bar::StatusBar;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a, S: MessageSender> {
    sbp_logger: Option<SbpLogger>,
    last_gps_update: Instant,
    last_gps_time: Option<GpsTime>,
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
    pub observation_tab: ObservationTab<S>,
    pub solution_velocity_tab: SolutionVelocityTab<'a, S>,
    pub status_bar: StatusBar<S>,
}

impl<'a, S: MessageSender> MainTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> MainTab<'a, S> {
        MainTab {
            sbp_logger: None,
            last_gps_time: None,
            last_gps_update: Instant::now(),
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
        let vel_log_file = DATA_DIRECTORY.path().join(vel_log_file);
        self.solution_tab.vel_log_file = match CsvSerializer::new(&vel_log_file) {
            Ok(vel_csv) => Some(vel_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", vel_log_file, e);
                None
            }
        };
        let pos_log_file = local_t.format(POS_LLH_TIME_STR_FILEPATH).to_string();
        let pos_log_file = DATA_DIRECTORY.path().join(pos_log_file);
        self.solution_tab.pos_log_file = match CsvSerializer::new(&pos_log_file) {
            Ok(pos_csv) => Some(pos_csv),
            Err(e) => {
                error!("issue creating file, {:?}, error, {}", pos_log_file, e);
                None
            }
        };
    }
    pub fn end_csv_logging(&mut self) -> crate::types::Result<()> {
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
            SbpLogging::Sbp => {
                let sbp_log_file = local_t.format(SBP_FILEPATH).to_string();
                let sbp_log_file = DATA_DIRECTORY.path().join(sbp_log_file);
                match SbpLogger::new_sbp(&sbp_log_file) {
                    Ok(logger) => Some(logger),
                    Err(e) => {
                        error!("issue creating file, {:?}, error, {}", sbp_log_file, e);
                        None
                    }
                }
            }
            SbpLogging::Json => {
                let sbp_json_log_file = local_t.format(SBP_JSON_FILEPATH).to_string();
                let sbp_json_log_file = DATA_DIRECTORY.path().join(sbp_json_log_file);
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
    }
    pub fn serialize_sbp(&mut self, msg: &SBP) {
        if let Some(sbp_logger) = &mut self.sbp_logger {
            if let Err(e) = sbp_logger.serialize(msg) {
                error!("error, {}, unable to log sbp msg, {:?}", e, msg);
            }
        }
    }
    pub fn close_sbp(&mut self, msg: &SBP) {
        if let Some(sbp_logger) = &mut self.sbp_logger {
            if let Err(e) = sbp_logger.serialize(msg) {
                error!("error, {}, unable to log sbp msg, {:?}", e, msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc, time::Duration};
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
}
