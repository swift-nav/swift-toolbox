use crate::observation_tab::ObservationTab;
use crate::solution_tab::SolutionTab;
use crate::solution_velocity_tab::SolutionVelocityTab;
use crate::tracking_signals_tab::TrackingSignalsTab;
use crate::types::*;

pub struct MainTab<'a, S: MessageSender> {
    pub tracking_signals_tab: TrackingSignalsTab<S>,
    pub solution_tab: SolutionTab<S>,
    pub observation_tab: ObservationTab<S>,
    pub solution_velocity_tab: SolutionVelocityTab<'a, S>,
}

impl<'a, S: MessageSender> MainTab<'a, S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> MainTab<'a, S> {
        MainTab {
            tracking_signals_tab: TrackingSignalsTab::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            observation_tab: ObservationTab::new(shared_state.clone(), client_sender.clone()),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()),
            solution_velocity_tab: SolutionVelocityTab::new(shared_state, client_sender),
        }
    }
}
