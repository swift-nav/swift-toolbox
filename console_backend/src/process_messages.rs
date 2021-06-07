use log::{debug, error};
use sbp::sbp_tools::SBPTools;
use sbp::{
    messages::{SBPMessage, SBP},
    serialize::SbpSerialize,
};
use std::{thread::sleep, time::Duration};

use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::log_panel::handle_log_msg;
use crate::main_tab::*;
use crate::types::*;

pub fn process_messages<S: MessageSender, T: std::io::Read>(
    messages: T,
    shared_state: SharedState,
    client_send: S,
    realtime_delay: RealtimeDelay,
) {
    let mut main = MainTab::new(shared_state.clone(), client_send);

    let messages = sbp::iter_messages(messages)
        .log_errors(log::Level::Debug)
        .with_rover_time();
    for (message, gps_time) in messages {
        if !shared_state.is_running() {
            if let Err(e) = main.end_csv_logging() {
                error!("Issue closing csv file, {}", e);
            }
            main.close_sbp();
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
        main.serialize_sbp(&message);
        let msg_name = message.get_message_name();
        main.status_bar.add_bytes(message.sbp_size());
        let mut attempt_delay = true;
        match message {
            SBP::MsgAgeCorrections(msg) => {
                main.solution_tab.handle_age_corrections(msg.clone());
                main.status_bar.handle_age_corrections(msg);
            }
            SBP::MsgBaselineNED(msg) => {
                main.status_bar
                    .handle_baseline_ned(BaselineNED::MsgBaselineNED(msg));
            }
            SBP::MsgBaselineNEDDepA(msg) => {
                main.status_bar
                    .handle_baseline_ned(BaselineNED::MsgBaselineNEDDepA(msg));
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
            SBP::MsgHeartbeat(_) => {
                main.status_bar.handle_heartbeat();
            }
            SBP::MsgImuAux(msg) => {
                main.advanced_ins_tab.handle_imu_aux(msg);
            }
            SBP::MsgImuRaw(msg) => {
                main.advanced_ins_tab.handle_imu_raw(msg);
            }
            SBP::MsgInsStatus(msg) => {
                main.solution_tab.handle_ins_status(msg.clone());
                main.status_bar.handle_ins_status(msg);
            }
            SBP::MsgInsUpdates(msg) => {
                main.advanced_ins_tab
                    .fusion_engine_status
                    .handle_ins_updates(msg.clone());
                main.solution_tab.handle_ins_updates(msg.clone());
                main.status_bar.handle_ins_updates(msg);
            }
            SBP::MsgMeasurementState(msg) => {
                main.tracking_signals_tab
                    .handle_msg_measurement_state(msg.states);
            }
            SBP::MsgObs(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObs(msg.clone()));
                main.observation_tab.handle_obs(ObservationMsg::MsgObs(msg));
            }
            SBP::MsgObsDepA(_msg) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
            }
            SBP::MsgObsDepB(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepB(msg.clone()));

                main.observation_tab
                    .handle_obs(ObservationMsg::MsgObsDepB(msg));
            }
            SBP::MsgObsDepC(msg) => {
                main.tracking_signals_tab
                    .handle_obs(ObservationMsg::MsgObsDepC(msg.clone()));

                main.observation_tab
                    .handle_obs(ObservationMsg::MsgObsDepC(msg));
            }
            SBP::MsgOsr(msg) => {
                main.observation_tab.handle_obs(ObservationMsg::MsgOsr(msg));
            }
            SBP::MsgPosLLH(msg) => {
                main.solution_tab
                    .handle_pos_llh(PosLLH::MsgPosLLH(msg.clone()));
                main.status_bar.handle_pos_llh(PosLLH::MsgPosLLH(msg));
            }
            SBP::MsgPosLLHDepA(msg) => {
                main.solution_tab
                    .handle_pos_llh(PosLLH::MsgPosLLHDepA(msg.clone()));
                main.status_bar.handle_pos_llh(PosLLH::MsgPosLLHDepA(msg));
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
                attempt_delay = false;
            }
        }
        if let RealtimeDelay::On = realtime_delay {
            if attempt_delay {
                main.realtime_delay(gps_time);
            } else {
                debug!("Message, {}, ignored for realtime delay.", msg_name);
            }
        }
        log::logger().flush();
    }
}
