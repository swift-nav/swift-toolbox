use sbp::messages::SBP;

use crate::main_tab::*;
use crate::types::*;

pub fn process_messages(
    messages: impl Iterator<Item = sbp::Result<SBP>>,
    shared_state: SharedState,
    client_send_clone: ClientSender,
) {
    let mut main = MainTab::new(shared_state);
    for message in messages {
        match message {
            Ok(SBP::MsgDops(msg)) => {
                main.solution_tab.handle_dops(Dops::MsgDops(msg.clone()));
            }
            Ok(SBP::MsgDopsDepA(msg)) => {
                main.solution_tab
                    .handle_dops(Dops::MsgDopsDepA(msg.clone()));
            }
            Ok(SBP::MsgGPSTime(msg)) => {
                main.solution_tab.handle_gps_time(msg.clone());
            }
            Ok(SBP::MsgInsStatus(msg)) => {
                main.solution_tab.handle_ins_status(msg.clone());
            }
            Ok(SBP::MsgInsUpdates(msg)) => {
                main.solution_tab.handle_ins_updates(msg.clone());
            }
            Ok(SBP::MsgMeasurementState(msg)) => {
                main.tracking_signals_tab.handle_msg_measurement_state(
                    msg.states.clone(),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgObs(msg)) => {
                main.tracking_signals_tab.handle_obs(
                    ObservationMsg::MsgObs(msg.clone()),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgObsDepA(_msg)) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot tab.");
            }
            Ok(SBP::MsgObsDepB(msg)) => {
                main.tracking_signals_tab.handle_obs(
                    ObservationMsg::MsgObsDepB(msg.clone()),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgObsDepC(msg)) => {
                main.tracking_signals_tab.handle_obs(
                    ObservationMsg::MsgObsDepC(msg.clone()),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgPosLLH(msg)) => {
                main.solution_tab.handle_pos_llh(
                    PosLLH::MsgPosLLH(msg.clone()),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgPosLLHDepA(msg)) => {
                main.solution_tab.handle_pos_llh(
                    PosLLH::MsgPosLLHDepA(msg.clone()),
                    &mut client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgTrackingState(msg)) => {
                main.tracking_signals_tab
                    .handle_msg_tracking_state(msg.states.clone(), &mut client_send_clone.clone());
            }
            Ok(SBP::MsgVelNED(msg)) => {
                main.solution_velocity_tab
                    .handle_vel_ned(msg.clone(), &mut client_send_clone.clone());
            }
            Ok(SBP::MsgUtcTime(msg)) => {
                main.solution_tab.handle_utc_time(msg.clone());
            }

            _ => {
                // no-op
            }
        }
    }
}
