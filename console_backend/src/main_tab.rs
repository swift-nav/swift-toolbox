// use crate::bottom_nav_bar::BottomNavBar;
use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a> {
    // pub bottom_nav_bar: BottomNavBar,
    pub tracking_signals_tab: TrackingSignalsTab,
    pub solution_tab: SolutionTab,
    pub solution_velocity_tab: SolutionVelocityTab<'a>,
}

impl<'a> MainTab<'a> {
    pub fn new<P: 'static + MessageSender + Clone>(shared_state: SharedState, client_sender: P) -> MainTab<'a>{
        MainTab {
            // bottom_nav_bar: BottomNavBar::new(shared_state.clone()),
            tracking_signals_tab: TrackingSignalsTab::new(shared_state.clone()),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state),
        }
    }
}
