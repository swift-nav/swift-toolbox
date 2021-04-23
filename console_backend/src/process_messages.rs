use sbp::messages::SBP;
use std::{thread::sleep, time::Duration};

use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::main_tab::*;
use crate::types::*;
use crate::utils::sec_to_ms;

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
) {
    let mut main = MainTab::new(shared_state.clone(), client_send);
    let messages = strip_errors_iter(true, messages);
    let mut old_tow = u32::MAX;
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
        match message {
            SBP::MsgAgeCorrections(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgAgeCorrections(msg.clone()).tow(), old_tow, 0);
                main.solution_tab.handle_age_corrections(msg);
            }
            SBP::MsgDops(msg) => {
                main.solution_tab.handle_dops(Dops::MsgDops(msg));
            }
            SBP::MsgDopsDepA(msg) => {
                main.solution_tab.handle_dops(Dops::MsgDopsDepA(msg));
            }
            SBP::MsgGPSTime(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgGPSTime(msg.clone()).tow(), old_tow, 0);
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
                old_tow = handle_gps_time(0, GPSTime::MsgObs(msg.clone()).tow(), old_tow, 0);
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObs(msg));
            }
            SBP::MsgObsDepA(_msg) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot tab.");
            }
            SBP::MsgObsDepB(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgObsDepB(msg.clone()).tow(), old_tow, 0);
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepB(msg));
            }
            SBP::MsgObsDepC(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgObsDepC(msg.clone()).tow(), old_tow, 0);
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepC(msg));
            }
            SBP::MsgPosLLH(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgPosLLH(msg.clone()).tow(), old_tow, 0);
                main.solution_tab.handle_pos_llh(PosLLH::MsgPosLLH(msg));
            }
            SBP::MsgPosLLHDepA(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgPosLLHDepA(msg.clone()).tow(), old_tow, 0);
                main.solution_tab.handle_pos_llh(PosLLH::MsgPosLLHDepA(msg));
            }
            SBP::MsgTrackingState(msg) => {
                main.tracking_signals_tab
                    .handle_msg_tracking_state(msg.states);
            }
            SBP::MsgVelNED(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgVelNED(msg.clone()).tow(), old_tow, 0);
                main.solution_tab
                    .handle_vel_ned(VelNED::MsgVelNED(msg.clone()));
                main.solution_velocity_tab.handle_vel_ned(msg);
            }
            SBP::MsgVelNEDDepA(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgVelNEDDepA(msg.clone()).tow(), old_tow, 0);
                main.solution_tab.handle_vel_ned(VelNED::MsgVelNEDDepA(msg));
            }
            SBP::MsgUtcTime(msg) => {
                old_tow = handle_gps_time(0, GPSTime::MsgUtcTime(msg.clone()).tow(), old_tow, 0);
                main.solution_tab.handle_utc_time(msg);
            }

            _ => {
                // no-op
            }
        }
    }
}

fn handle_gps_time(
    wn: u16,   // GPS week number.
    tow: u32,  // GPS time of week rounded to the nearest millisecond.
    old_tow: u32,  // Old GPS time of week rounded to the nearest millisecond.
    flags: u8, // Status flags (reserved).
) -> u32 {
    
    if tow > old_tow {
        let gps_tow_delta_ms = (tow - old_tow) as u64;
        // let gps_tow_delta_ms = //sec_to_ms(gps_tow_delta as f64) as u64;
        sleep(Duration::from_millis(gps_tow_delta_ms));
    }
    tow
}