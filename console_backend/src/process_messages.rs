use std::{io::ErrorKind, thread::sleep, time::Duration};

use log::{debug, error};
use sbp::{
    messages::{
        imu::{MsgImuAux, MsgImuRaw},
        logging::MsgLog,
        mag::MsgMagRaw,
        navigation::{MsgAgeCorrections, MsgPosLLHCov, MsgUtcTime, MsgVelNED},
        observation::MsgObsDepA,
        orientation::{MsgAngularRate, MsgBaselineHeading, MsgOrientEuler},
        piksi::MsgCommandResp,
        system::{MsgHeartbeat, MsgInsStatus, MsgInsUpdates},
        tracking::{MsgMeasurementState, MsgTrackingState},
        SBPMessage,
    },
    sbp_tools::{ControlFlow, SBPTools},
    serialize::SbpSerialize,
};

use crate::broadcaster::with_link;
use crate::connection::Connection;
use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::errors::UNABLE_TO_CLONE_UPDATE_SHARED;
use crate::log_panel::handle_log_msg;
use crate::types::*;
use crate::update_tab;
use crate::utils::{close_frontend, refresh_navbar};
use crate::Tabs;

pub fn process_messages<S>(
    conn: Connection,
    shared_state: SharedState,
    client_send: S,
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
    let messages = {
        let state = shared_state.clone();
        let client = client_send.clone();
        sbp::iter_messages(rdr)
            .handle_errors(move |e| {
                debug!("{}", e);
                match e {
                    sbp::Error::IoError(err) => {
                        if (*err).kind() == ErrorKind::TimedOut {
                            state.set_running(false, client.clone());
                        }
                        ControlFlow::Break
                    }
                    _ => ControlFlow::Continue,
                }
            })
            .with_rover_time()
    };
    with_link(|source| {
        let tabs = Tabs::new(
            shared_state.clone(),
            client_send.clone(),
            msg_sender.clone(),
            source.link(),
        );

        let link = source.link();

        link.register_cb(|msg: MsgAgeCorrections| {
            tabs.baseline
                .lock()
                .unwrap()
                .handle_age_corrections(msg.clone());
            tabs.solution
                .lock()
                .unwrap()
                .handle_age_corrections(msg.clone());
            tabs.status_bar.lock().unwrap().handle_age_corrections(msg);
        });

        link.register_cb(|msg: MsgAngularRate| {
            tabs.solution.lock().unwrap().handle_angular_rate(msg);
        });

        link.register_cb(|msg: MsgBaselineHeading| {
            tabs.baseline.lock().unwrap().handle_baseline_heading(msg);
        });

        link.register_cb(|msg: MsgCommandResp| {
            tabs.update.lock().unwrap().handle_command_resp(msg);
        });

        link.register_cb(|msg: BaselineNED| {
            tabs.baseline
                .lock()
                .unwrap()
                .handle_baseline_ned(msg.clone());
            tabs.status_bar.lock().unwrap().handle_baseline_ned(msg);
        });

        link.register_cb(|msg: Dops| {
            tabs.solution.lock().unwrap().handle_dops(msg);
        });

        link.register_cb(|msg: GpsTime| {
            tabs.baseline.lock().unwrap().handle_gps_time(msg.clone());
            tabs.solution.lock().unwrap().handle_gps_time(msg);
        });

        link.register_cb(|_: MsgHeartbeat| {
            tabs.status_bar.lock().unwrap().handle_heartbeat();
        });

        link.register_cb(|msg: MsgImuAux| {
            tabs.advanced_ins.lock().unwrap().handle_imu_aux(msg);
        });

        link.register_cb(|msg: MsgImuRaw| {
            tabs.advanced_ins.lock().unwrap().handle_imu_raw(msg);
        });

        link.register_cb(|msg: MsgInsStatus| {
            tabs.solution.lock().unwrap().handle_ins_status(msg.clone());
            tabs.status_bar.lock().unwrap().handle_ins_status(msg);
        });

        link.register_cb(|msg: MsgInsUpdates| {
            tabs.advanced_ins
                .lock()
                .unwrap()
                .fusion_engine_status_bar
                .handle_ins_updates(msg.clone());
            tabs.solution
                .lock()
                .unwrap()
                .handle_ins_updates(msg.clone());
            tabs.status_bar.lock().unwrap().handle_ins_updates(msg);
        });

        link.register_cb(|msg: MsgMagRaw| {
            tabs.advanced_magnetometer
                .lock()
                .unwrap()
                .handle_mag_raw(msg);
        });

        link.register_cb(|msg: MsgMeasurementState| {
            tabs.tracking_signals
                .lock()
                .unwrap()
                .handle_msg_measurement_state(msg.states);
        });

        link.register_cb(|msg: ObservationMsg| {
            tabs.tracking_signals
                .lock()
                .unwrap()
                .handle_obs(msg.clone());
            tabs.observation.lock().unwrap().handle_obs(msg);
        });

        link.register_cb(|_: MsgObsDepA| {
            println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
        });

        link.register_cb(|msg: MsgOrientEuler| {
            tabs.solution.lock().unwrap().handle_orientation_euler(msg);
        });

        link.register_cb(|msg: PosLLH| {
            tabs.solution.lock().unwrap().handle_pos_llh(msg.clone());
            tabs.status_bar.lock().unwrap().handle_pos_llh(msg);
        });

        link.register_cb(|msg: MsgPosLLHCov| {
            tabs.solution.lock().unwrap().handle_pos_llh_cov(msg);
        });

        link.register_cb(|msg: Specan| {
            tabs.advanced_spectrum_analyzer
                .lock()
                .unwrap()
                .handle_specan(msg);
        });

        link.register_cb(|msg: MsgTrackingState| {
            tabs.tracking_signals
                .lock()
                .unwrap()
                .handle_msg_tracking_state(msg.states);
        });

        link.register_cb(|msg: VelNED| {
            tabs.solution.lock().unwrap().handle_vel_ned(msg);
        });

        link.register_cb(|msg: MsgVelNED| {
            // why does this tab not take both VelNED messages?
            tabs.solution_velocity.lock().unwrap().handle_vel_ned(msg);
        });

        link.register_cb(|msg: MsgUtcTime| {
            tabs.baseline.lock().unwrap().handle_utc_time(msg.clone());
            tabs.solution.lock().unwrap().handle_utc_time(msg);
        });

        link.register_cb(|msg: MsgLog| {
            tabs.update.lock().unwrap().handle_log_msg(msg.clone());
            handle_log_msg(msg);
        });

        let shared_state = shared_state.clone();
        let mut client_send = client_send.clone();
        let update_shared = tabs
            .update
            .lock()
            .expect(UNABLE_TO_CLONE_UPDATE_SHARED)
            .update_tab_context_clone();
        let link_clone = source.link();
        let client_send_clone = client_send.clone();
        let (update_tab_sender, update_tab_recv) = tabs.update.lock().unwrap().clone_channel();
        crossbeam::scope(|scope| {
            let handle = scope.spawn(|_| {
                update_tab::update_tab_thread(
                    update_tab_sender.clone(),
                    update_tab_recv,
                    update_shared,
                    client_send_clone,
                    link_clone,
                    msg_sender.clone(),
                );
            });
            for (message, gps_time) in messages {
                if !shared_state.is_running() {
                    if let Err(e) = tabs.main.lock().unwrap().end_csv_logging() {
                        error!("Issue closing csv file, {}", e);
                    }
                    tabs.main.lock().unwrap().close_sbp();
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
                let sent = source.send(&message, gps_time.clone());
                tabs.main.lock().unwrap().serialize_sbp(&message);
                tabs.status_bar
                    .lock()
                    .unwrap()
                    .add_bytes(message.sbp_size());
                if let RealtimeDelay::On = realtime_delay {
                    if sent {
                        tabs.main.lock().unwrap().realtime_delay(gps_time);
                    } else {
                        debug!(
                            "Message, {}, ignored for realtime delay.",
                            message.get_message_name()
                        );
                    }
                }
                log::logger().flush();
            }
            if update_tab_sender.send(None).is_ok() {
                if let Err(err) = handle.join() {
                    error!("Error joining update tab, {:?}", err);
                }
            }
            if conn.close_when_done() {
                shared_state.set_running(false, client_send.clone());
                close_frontend(&mut client_send);
            }
        })
        .unwrap();
    });

    Ok(())
}
