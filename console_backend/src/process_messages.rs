use std::{io::ErrorKind, thread::sleep, time::Duration};

use log::{debug, error};
use sbp::{
    messages::{
        imu::{MsgImuAux, MsgImuRaw},
        mag::MsgMagRaw,
        navigation::{MsgAgeCorrections, MsgPosLLHCov, MsgUtcTime, MsgVelNED},
        observation::MsgObsDepA,
        orientation::{MsgAngularRate, MsgBaselineHeading, MsgOrientEuler},
        system::{MsgHeartbeat, MsgInsStatus, MsgInsUpdates},
        tracking::{MsgMeasurementState, MsgTrackingState},
        SBPMessage,
    },
    sbp_tools::{ControlFlow, SBPTools},
    serialize::SbpSerialize,
};

use crate::broadcaster::Link;
use crate::connection::Connection;
use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::log_panel::handle_log_msg;
use crate::types::*;
use crate::utils::{close_frontend, refresh_navbar};
use crate::Tabs;

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
    let tabs = Tabs::new(shared_state.clone(), client_send.clone(), msg_sender);
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

    let mut link = Link::new();

    link.register_cb(|msg: MsgAgeCorrections| {
        tabs.baseline
            .borrow_mut()
            .handle_age_corrections(msg.clone());
        tabs.solution
            .borrow_mut()
            .handle_age_corrections(msg.clone());
        tabs.status_bar.borrow_mut().handle_age_corrections(msg);
    });

    link.register_cb(|msg: MsgAngularRate| {
        tabs.solution.borrow_mut().handle_angular_rate(msg);
    });

    link.register_cb(|msg: MsgBaselineHeading| {
        tabs.baseline.borrow_mut().handle_baseline_heading(msg);
    });

    link.register_cb(|msg: BaselineNED| {
        tabs.baseline.borrow_mut().handle_baseline_ned(msg.clone());
        tabs.status_bar.borrow_mut().handle_baseline_ned(msg);
    });

    link.register_cb(|msg: Dops| {
        tabs.solution.borrow_mut().handle_dops(msg);
    });

    link.register_cb(|msg: GpsTime| {
        tabs.baseline.borrow_mut().handle_gps_time(msg.clone());
        tabs.solution.borrow_mut().handle_gps_time(msg);
    });

    link.register_cb(|_: MsgHeartbeat| {
        tabs.status_bar.borrow_mut().handle_heartbeat();
    });

    link.register_cb(|msg: MsgImuAux| {
        tabs.advanced_ins.borrow_mut().handle_imu_aux(msg);
    });

    link.register_cb(|msg: MsgImuRaw| {
        tabs.advanced_ins.borrow_mut().handle_imu_raw(msg);
    });

    link.register_cb(|msg: MsgInsStatus| {
        tabs.solution.borrow_mut().handle_ins_status(msg.clone());
        tabs.status_bar.borrow_mut().handle_ins_status(msg);
    });

    link.register_cb(|msg: MsgInsUpdates| {
        tabs.advanced_ins
            .borrow_mut()
            .fusion_engine_status_bar
            .handle_ins_updates(msg.clone());
        tabs.solution.borrow_mut().handle_ins_updates(msg.clone());
        tabs.status_bar.borrow_mut().handle_ins_updates(msg);
    });

    link.register_cb(|msg: MsgMagRaw| {
        tabs.advanced_magnetometer.borrow_mut().handle_mag_raw(msg);
    });

    link.register_cb(|msg: MsgMeasurementState| {
        tabs.tracking_signals
            .borrow_mut()
            .handle_msg_measurement_state(msg.states);
    });

    link.register_cb(|msg: ObservationMsg| {
        tabs.tracking_signals.borrow_mut().handle_obs(msg.clone());
        tabs.observation.borrow_mut().handle_obs(msg);
    });

    link.register_cb(|_: MsgObsDepA| {
        println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
    });

    link.register_cb(|msg: MsgOrientEuler| {
        tabs.solution.borrow_mut().handle_orientation_euler(msg);
    });

    link.register_cb(|msg: PosLLH| {
        tabs.solution.borrow_mut().handle_pos_llh(msg.clone());
        tabs.status_bar.borrow_mut().handle_pos_llh(msg);
    });

    link.register_cb(|msg: MsgPosLLHCov| {
        tabs.solution.borrow_mut().handle_pos_llh_cov(msg);
    });

    link.register_cb(|msg: Specan| {
        tabs.advanced_spectrum_analyzer
            .borrow_mut()
            .handle_specan(msg);
    });

    link.register_cb(|msg: MsgTrackingState| {
        tabs.tracking_signals
            .borrow_mut()
            .handle_msg_tracking_state(msg.states);
    });

    link.register_cb(|msg: VelNED| {
        tabs.solution.borrow_mut().handle_vel_ned(msg);
    });

    link.register_cb(|msg: MsgVelNED| {
        // why does this tab not take both VelNED messages?
        tabs.solution_velocity.borrow_mut().handle_vel_ned(msg);
    });

    link.register_cb(|msg: MsgUtcTime| {
        tabs.baseline.borrow_mut().handle_utc_time(msg.clone());
        tabs.solution.borrow_mut().handle_utc_time(msg);
    });

    link.register_cb(handle_log_msg);

    for (message, gps_time) in messages {
        if !shared_state.is_running() {
            if let Err(e) = tabs.main.borrow_mut().end_csv_logging() {
                error!("Issue closing csv file, {}", e);
            }
            tabs.main.borrow_mut().close_sbp();
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
        tabs.main.borrow_mut().serialize_sbp(&message);
        tabs.status_bar.borrow_mut().add_bytes(message.sbp_size());
        let sent = link.send(&message, gps_time.clone());
        if let RealtimeDelay::On = realtime_delay {
            if sent {
                tabs.main.borrow_mut().realtime_delay(gps_time);
            } else {
                debug!(
                    "Message, {}, ignored for realtime delay.",
                    message.get_message_name()
                );
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
