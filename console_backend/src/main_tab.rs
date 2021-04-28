use sbp::messages::SBP;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a, S: MessageSender> {
    last_gps_update: Instant,
    last_gps_time: Option<f64>,
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
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
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state, client_sender),
        }
    }

    pub fn realtime_delay(&mut self, msg: SBP) {
        let time_aware = sbp_chopper::maybe_time_aware(&msg);
        if let Some(time_aware) = time_aware {
            if let Some(gps_time) = time_aware.gps_time() {
                let gps_time = gps_time.tow();
                if let Some(l_time) = self.last_gps_time {
                    if l_time < gps_time {
                        let diff = gps_time - l_time;

                        let elapsed = self.last_gps_update.elapsed().as_secs_f64();
                        if diff > elapsed {
                            let sleep_duration = diff - elapsed;

                            sleep(Duration::from_secs_f64(sleep_duration));
                        }
                        self.last_gps_update = Instant::now();
                    }
                }
                self.last_gps_time = Some(gps_time);
            }
        }
    }
}
