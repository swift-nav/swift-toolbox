use crate::solution_table::SolutionTable;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a> {
    pub tracking_signals_tab: TrackingSignalsTab,
    pub solution_table: SolutionTable,
    pub solution_velocity_tab: SolutionVelocityTab<'a>,
}

impl<'a> MainTab<'a> {
    pub fn new(shared_state: SharedState) -> MainTab<'a> {
        MainTab {
            tracking_signals_tab: TrackingSignalsTab::new(shared_state.clone()),
            solution_table: SolutionTable::new(shared_state.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state),
        }
    }
}
