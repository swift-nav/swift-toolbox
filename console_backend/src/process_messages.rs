use log::{debug, error};
use sbp::sbp_tools::{ControlFlow, SBPTools};
use sbp::{
    messages::{SBPMessage, SBP},
    serialize::SbpSerialize,
};
use std::{io::ErrorKind, thread::sleep, time::Duration};

use crate::connection::Connection;
use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::log_panel::handle_log_msg;
use crate::main_tab::*;
use crate::types::*;
use crate::utils::{close_frontend, refresh_navbar};

pub fn process_messages<S>(
    conn: Connection,
    shared_state: SharedState,
    mut client_send: S,
) -> Result<()>
where
    S: CapnProtoSender,
{
    shared_state.set_running(true, client_send.clone());
    let realtime_delay = conn.realtime_delay();
    let (rdr, wtr) = conn.try_connect(Some(shared_state.clone()))?;
    let msg_sender = MsgSender::new(wtr);
    shared_state.set_current_connection(conn.name());
    refresh_navbar(&mut client_send.clone(), shared_state.clone());
    let mut main = MainTab::new(shared_state.clone(), client_send.clone(), msg_sender);
    let messages = sbp::iter_messages(rdr)
        .handle_errors(|e| {
            debug!("{}", e);
            match e {
                sbp::Error::IoError(err) => {
                    if (*err).kind() == ErrorKind::TimedOut {
                        shared_state.set_running(false, client_send.clone());
                    }
                    ControlFlow::Break
                }
                _ => ControlFlow::Continue,
            }
        })
        .with_rover_time();
    let mut attempt_delay;
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
        attempt_delay = true;
        match message {
            SBP::MsgAgeCorrections(msg) => {
                main.baseline_tab.handle_age_corrections(msg.clone());
                main.solution_tab.handle_age_corrections(msg.clone());
                main.status_bar.handle_age_corrections(msg);
            }
            SBP::MsgBaselineHeading(msg) => {
                main.baseline_tab.handle_baseline_heading(msg);
            }
            SBP::MsgBaselineNED(msg) => {
                main.baseline_tab
                    .handle_baseline_ned(BaselineNED::MsgBaselineNED(msg.clone()));
                main.status_bar
                    .handle_baseline_ned(BaselineNED::MsgBaselineNED(msg));
            }
            SBP::MsgBaselineNEDDepA(msg) => {
                main.baseline_tab
                    .handle_baseline_ned(BaselineNED::MsgBaselineNEDDepA(msg.clone()));
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
                main.baseline_tab
                    .handle_gps_time(GpsTime::MsgGpsTime(msg.clone()));
                main.solution_tab.handle_gps_time(GpsTime::MsgGpsTime(msg));
            }
            SBP::MsgGPSTimeDepA(msg) => {
                main.baseline_tab
                    .handle_gps_time(GpsTime::MsgGpsTimeDepA(msg.clone()));
                main.solution_tab
                    .handle_gps_time(GpsTime::MsgGpsTimeDepA(msg));
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
                    .fusion_engine_status_bar
                    .handle_ins_updates(msg.clone());
                main.solution_tab.handle_ins_updates(msg.clone());
                main.status_bar.handle_ins_updates(msg);
            }
            SBP::MsgMagRaw(msg) => {
                main.advanced_magnetometer_tab.handle_mag_raw(msg);
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
            SBP::MsgOrientEuler(msg) => {
                main.solution_tab.handle_orientation_euler(msg);
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
            SBP::MsgPosLLHCov(msg) => {
                main.solution_tab.handle_pos_llh_cov(msg);
            }
            SBP::MsgSpecan(msg) => {
                main.advanced_spectrum_analyzer_tab.handle_specan(msg);
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
                main.baseline_tab.handle_utc_time(msg.clone());
                main.solution_tab.handle_utc_time(msg);
            }
            SBP::MsgLog(msg) => handle_log_msg(msg),

            _ => {
                attempt_delay = false;
            }
        }
        if let RealtimeDelay::On = realtime_delay {
            println!("{:?}", gps_time);
            if attempt_delay {
                main.realtime_delay(gps_time);
            } else {
                debug!("Message, {}, ignored for realtime delay.", msg_name);
            }
        }
        log::logger().flush();
    }
    if conn.close_when_done() {
        shared_state.set_running(false, client_send.clone());
        close_frontend(&mut client_send);
    }
    Ok(())
}
