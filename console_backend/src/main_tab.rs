use sbp::{messages::GpsTime, GpsTimeError};
use std::{
    result::Result,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a, S: MessageSender> {
    last_gps_update: Instant,
    last_gps_time: Option<GpsTime>,
    last_gps_week: i16,
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
    pub solution_velocity_tab: SolutionVelocityTab<'a, S>,
}

impl<'a, S: MessageSender> MainTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> MainTab<'a, S> {
        MainTab {
            last_gps_time: None,
            last_gps_update: Instant::now(),
            last_gps_week: 0,
            tracking_signals_tab: TrackingSignalsTab::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state, client_sender),
        }
    }

    /// Calculate time since last epoch began and sleep for previous epoch time difference.
    ///
    /// # Parameters
    /// - `gps_time`: The GpsTime corresponding to a message.
    pub fn realtime_delay(&mut self, gps_time: Option<Result<GpsTime, GpsTimeError>>) {
        if let Some(Ok(g_time)) = gps_time {
            let mut g_time = g_time;
            let gps_time_tow = g_time.tow();
            let gps_time_week = g_time.wn();
            
            if let Some(l_time) = self.last_gps_time {
                if gps_time_week != 0 {
                    self.last_gps_week = gps_time_week;
                } else {
                    g_time.
                }
                let last_gps_time_tow = l_time.tow();
                if last_gps_time_tow < gps_time_tow {
                    let diff = gps_time_tow - last_gps_time_tow;
                    let elapsed = self.last_gps_update.elapsed().as_secs_f64();
                    if diff > elapsed {
                        let sleep_duration = diff - elapsed;

                        sleep(Duration::from_secs_f64(sleep_duration));
                    }
                    self.last_gps_update = Instant::now();
                }
            }
            self.last_gps_time = Some(g_time);
            if gps_time_week != 0 {
                self.last_gps_week = gps_time_week;
            }
            
        }
    }
}
