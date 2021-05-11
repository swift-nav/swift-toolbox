use log::debug;
use sbp::time::GpsTime;
use std::{result::Result, thread::sleep, time::Instant};

use crate::observation_tab::ObservationTab;
use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a, S: MessageSender> {
    last_gps_update: Instant,
    last_gps_time: Option<GpsTime>,
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
    pub observation_tab: ObservationTab<S>,
    pub solution_velocity_tab: SolutionVelocityTab<'a, S>,
}

impl<'a, S: MessageSender> MainTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> MainTab<'a, S> {
        MainTab {
            last_gps_time: None,
            last_gps_update: Instant::now(),
            tracking_signals_tab: TrackingSignalsTab::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            observation_tab: ObservationTab::new(shared_state.clone(), client_sender.clone()),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state, client_sender),
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
    fn realtime_delay_bad_last_test() {
        let shared_state = SharedState::new();
        let (client_send_, _) = mpsc::channel::<Vec<u8>>();
        let client_send = ClientSender {
            inner: client_send_,
        };
        let gps_s = GpsTimeTests::new();
        let mut main = MainTab::new(shared_state, client_send);
        let early_gps_time_good = GpsTime::new(gps_s.zero_week, gps_s.early_gps_tow_good).unwrap();
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
    fn realtime_delay_bad_current_test() {
        let shared_state = SharedState::new();
        let (client_send_, _) = mpsc::channel::<Vec<u8>>();
        let client_send = ClientSender {
            inner: client_send_,
        };
        let gps_s = GpsTimeTests::new();
        let mut main = MainTab::new(shared_state, client_send);
        let early_gps_time_good = GpsTime::new(gps_s.good_week, gps_s.early_gps_tow_good).unwrap();
        let later_gps_time_good = GpsTime::new(gps_s.zero_week, gps_s.later_gps_tow_good);
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
