use sbp::messages::SBP;

use crate::main_tab::*;
use crate::types::*;

/// SBP messages preprocessor to filter Result Ok and log Err.
///
/// Taken from ICBINS/src/lib.rs.
///
/// # Parameters:
/// - `messages`: The iterator of messages to process.
///
/// # Returns:
/// - The filtered out Ok messages iterator.
fn log_errors(messages: impl Iterator<Item = sbp::Result<SBP>>) -> impl Iterator<Item = SBP> {
    messages
        .inspect(|msg| {
            if let Err(e) = msg {
                eprintln!("error reading message: {}", e);
            }
        })
        .filter_map(sbp::Result::ok)
}

pub fn process_messages(
    messages: impl Iterator<Item = sbp::Result<SBP>>,
    shared_state: SharedState,
    client_send_clone: ClientSender,
) {
    let mut main = MainTab::new(shared_state);
    let messages = log_errors(messages);
    for message in messages {
        match message {
            SBP::MsgAgeCorrections(msg) => {
                main.solution_tab.handle_age_corrections(msg);
            }
            SBP::MsgDops(msg) => {
                main.solution_tab.handle_dops(Dops::MsgDops(msg));
            }
            SBP::MsgDopsDepA(msg) => {
                main.solution_tab.handle_dops(Dops::MsgDopsDepA(msg));
            }
            SBP::MsgGPSTime(msg) => {
                main.solution_tab.handle_gps_time(msg);
            }
            SBP::MsgInsStatus(msg) => {
                main.solution_tab.handle_ins_status(msg);
            }
            SBP::MsgInsUpdates(msg) => {
                main.solution_tab.handle_ins_updates(msg);
            }
            SBP::MsgMeasurementState(msg) => {
                main.tracking_signals_tab
                    .handle_msg_measurement_state(msg.states, &mut client_send_clone.clone());
            }
            SBP::MsgObs(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObs(msg), &mut client_send_clone.clone());
            }
            SBP::MsgObsDepA(_msg) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot tab.");
            }
            SBP::MsgObsDepB(msg) => {
                main.tracking_signals_tab.handle_obs(
                    ObservationMsg::MsgObsDepB(msg),
                    &mut client_send_clone.clone(),
                );
            }
            SBP::MsgObsDepC(msg) => {
                main.tracking_signals_tab.handle_obs(
                    ObservationMsg::MsgObsDepC(msg),
                    &mut client_send_clone.clone(),
                );
            }
            SBP::MsgPosLLH(msg) => {
                main.solution_tab
                    .handle_pos_llh(PosLLH::MsgPosLLH(msg), &mut client_send_clone.clone());
            }
            SBP::MsgPosLLHDepA(msg) => {
                main.solution_tab
                    .handle_pos_llh(PosLLH::MsgPosLLHDepA(msg), &mut client_send_clone.clone());
            }
            SBP::MsgTrackingState(msg) => {
                main.tracking_signals_tab
                    .handle_msg_tracking_state(msg.states, &mut client_send_clone.clone());
            }
            SBP::MsgVelNED(msg) => {
                main.solution_tab
                    .handle_vel_ned(VelNED::MsgVelNED(msg.clone()));
                main.solution_velocity_tab
                    .handle_vel_ned(msg, &mut client_send_clone.clone());
            }
            SBP::MsgVelNEDDepA(msg) => {
                main.solution_tab.handle_vel_ned(VelNED::MsgVelNEDDepA(msg));
            }
            SBP::MsgUtcTime(msg) => {
                main.solution_tab.handle_utc_time(msg);
            }

            _ => {
                // no-op
            }
        }
    }
}
