use std::sync::{Arc, Mutex};

use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a> {
    pub tracking_signals_tab: TrackingSignalsTab<'a>,
    pub solution_velocity_tab: SolutionVelocityTab<'a>,
}

impl<'a> MainTab<'a> {
    pub fn new(shared_state: &'a Arc<Mutex<SharedState>>) -> MainTab {
        MainTab {
            tracking_signals_tab: TrackingSignalsTab::new(shared_state),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state),
        }
    }
}
