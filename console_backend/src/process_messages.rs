use sbp::messages::{SBPMessage, SBP};
use std::{thread::sleep, time::Duration};

use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::log_panel::handle_log_msg;
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
fn strip_errors_iter(
    log_errors: bool,
    messages: impl Iterator<Item = sbp::Result<SBP>>,
) -> impl Iterator<Item = SBP> {
    messages
        .inspect(move |msg| {
            if let Err(e) = msg {
                if log_errors {
                    eprintln!("error reading message: {}", e);
                }
            }
        })
        .filter_map(sbp::Result::ok)
}

pub fn process_messages<S: MessageSender>(
    messages: impl Iterator<Item = sbp::Result<SBP>>,
    shared_state: SharedState,
    client_send: S,
    realtime_delay: bool,
) {
    let mut main = MainTab::new(shared_state.clone(), client_send);
    let messages = strip_errors_iter(true, messages);

    for message in messages {
        if !shared_state.is_running() {
            break;
        }
        if shared_state.is_paused() {
            loop {
                if !shared_state.is_paused() {
                    break;
                }
                sleep(Duration::from_millis(PAUSE_LOOP_SLEEP_DURATION_MS));
            }
        }
        let gps_time = message.gps_time();
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
                    .handle_msg_measurement_state(msg.states);
            }
            SBP::MsgObs(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObs(msg));
                    continue;
            }
            SBP::MsgObsDepA(_msg) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot tab.");
            }
            SBP::MsgObsDepB(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepB(msg));
                continue;
            }
            SBP::MsgObsDepC(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepC(msg));
                continue;
            }
            SBP::MsgPosLLH(msg) => {
                main.solution_tab.handle_pos_llh(PosLLH::MsgPosLLH(msg));
            }
            SBP::MsgPosLLHDepA(msg) => {
                main.solution_tab.handle_pos_llh(PosLLH::MsgPosLLHDepA(msg));
            }
            SBP::MsgTrackingState(msg) => {
                main.tracking_signals_tab
                    .handle_msg_tracking_state(msg.states);
            }
            SBP::MsgVelNED(msg) => {
                main.solution_tab
                    .handle_vel_ned(VelNED::MsgVelNED(msg.clone()));
                main.solution_velocity_tab.handle_vel_ned(msg);
            }
            SBP::MsgVelNEDDepA(msg) => {
                main.solution_tab.handle_vel_ned(VelNED::MsgVelNEDDepA(msg));
            }
            SBP::MsgUtcTime(msg) => {
                main.solution_tab.handle_utc_time(msg);
            }
            SBP::MsgLog(msg) => handle_log_msg(msg),

            _ => {
                continue;
            }
        }
        if realtime_delay {
            main.realtime_delay(gps_time);
        }
        log::logger().flush();
    }
}
