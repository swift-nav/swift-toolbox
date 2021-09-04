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

use crate::broadcaster::with_link;
use crate::connection::Connection;
use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::log_panel::handle_log_msg;
use crate::types::*;
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

    use std::sync::Arc;

    with_link(|source| {
        let tabs = Arc::new(Box::new(Tabs::new(
            shared_state.clone(),
            client_send.clone(),
            msg_sender,
            source.link(),
        )));

        let link = source.link();

        let tabs1 = tabs.clone();

        link.register_cb(move |msg: MsgAgeCorrections| {
            tabs1.baseline
                .lock()
                .unwrap()
                .handle_age_corrections(msg.clone());
            tabs1.solution
                .lock()
                .unwrap()
                .handle_age_corrections(msg.clone());
            tabs1.status_bar.lock().unwrap().handle_age_corrections(msg);
        });

        let tabs2 = tabs.clone();

        link.register_cb(move |msg: MsgAngularRate| {
            tabs2.solution.lock().unwrap().handle_angular_rate(msg);
        });

        let tabs3 = tabs.clone();

        link.register_cb(move |msg: MsgBaselineHeading| {
            tabs3.baseline.lock().unwrap().handle_baseline_heading(msg);
        });

        let tabs4 = tabs.clone();

        link.register_cb(move |msg: BaselineNED| {
            tabs4.clone().baseline
                .lock()
                .unwrap()
                .handle_baseline_ned(msg.clone());
            tabs4.status_bar.lock().unwrap().handle_baseline_ned(msg);
        });

        let tabs5 = tabs.clone();

        link.register_cb(move |msg: Dops| {
            tabs5.solution.lock().unwrap().handle_dops(msg);
        });

        let tabs6 = tabs.clone();

        link.register_cb(move |msg: GpsTime| {
            tabs6.baseline.lock().unwrap().handle_gps_time(msg.clone());
            tabs6.solution.lock().unwrap().handle_gps_time(msg);
        });

        let tabs7 = tabs.clone();

        link.register_cb(move |_: MsgHeartbeat| {
            tabs7.status_bar.lock().unwrap().handle_heartbeat();
        });

        let tabs8 = tabs.clone();

        link.register_cb(move |msg: MsgImuAux| {
            tabs8.advanced_ins.lock().unwrap().handle_imu_aux(msg);
        });

        let tabs9 = tabs.clone();

        link.register_cb(move |msg: MsgImuRaw| {
            tabs9.advanced_ins.lock().unwrap().handle_imu_raw(msg);
        });

        let tabs10 = tabs.clone();

        link.register_cb(move |msg: MsgInsStatus| {
            tabs10.solution.lock().unwrap().handle_ins_status(msg.clone());
            tabs10.status_bar.lock().unwrap().handle_ins_status(msg);
        });

        let tabs11 = tabs.clone();

        link.register_cb(move |msg: MsgInsUpdates| {
            tabs11.advanced_ins
                .lock()
                .unwrap()
                .fusion_engine_status_bar
                .handle_ins_updates(msg.clone());
            tabs11.solution
                .lock()
                .unwrap()
                .handle_ins_updates(msg.clone());
            tabs11.status_bar.lock().unwrap().handle_ins_updates(msg);
        });

        let tabs12 = tabs.clone();
        link.register_cb(move |msg: MsgMagRaw| {
            tabs12.advanced_magnetometer
                .lock()
                .unwrap()
                .handle_mag_raw(msg);
        });

        let tabs13 = tabs.clone();
        link.register_cb(move |msg: MsgMeasurementState| {
            tabs13.tracking_signals
                .lock()
                .unwrap()
                .handle_msg_measurement_state(msg.states);
        });

        let tabs14 = tabs.clone();
        link.register_cb(move |msg: ObservationMsg| {
            tabs14.tracking_signals
                .lock()
                .unwrap()
                .handle_obs(msg.clone());
            tabs14.observation.lock().unwrap().handle_obs(msg);
        });

        link.register_cb(|_: MsgObsDepA| {
            println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
        });

        let tabs15 = tabs.clone();
        link.register_cb(move |msg: MsgOrientEuler| {
            tabs15.solution.lock().unwrap().handle_orientation_euler(msg);
        });

        let tabs16 = tabs.clone();
        link.register_cb(move |msg: PosLLH| {
            tabs16.solution.lock().unwrap().handle_pos_llh(msg.clone());
            tabs16.status_bar.lock().unwrap().handle_pos_llh(msg);
        });

        let tabs17 = tabs.clone();
        link.register_cb(move |msg: MsgPosLLHCov| {
            tabs17.solution.lock().unwrap().handle_pos_llh_cov(msg);
        });

        let tabs18 = tabs.clone();
        link.register_cb(move |msg: Specan| {
            tabs18.advanced_spectrum_analyzer
                .lock()
                .unwrap()
                .handle_specan(msg);
        });

        let tabs19 = tabs.clone();
        link.register_cb(move |msg: MsgTrackingState| {
            tabs19.tracking_signals
                .lock()
                .unwrap()
                .handle_msg_tracking_state(msg.states);
        });

        let tabs20 = tabs.clone();
        link.register_cb(move |msg: VelNED| {
            tabs20.solution.lock().unwrap().handle_vel_ned(msg);
        });

        let tabs21 = tabs.clone();
        link.register_cb(move |msg: MsgVelNED| {
            // why does this tab not take both VelNED messages?
            tabs21.solution_velocity.lock().unwrap().handle_vel_ned(msg);
        });

        let tabs22 = tabs.clone();
        link.register_cb(move |msg: MsgUtcTime| {
            tabs22.baseline.lock().unwrap().handle_utc_time(msg.clone());
            tabs22.solution.lock().unwrap().handle_utc_time(msg);
        });

        link.register_cb(handle_log_msg);

        let shared_state = shared_state.clone();
        let mut client_send = client_send.clone();

        crossbeam::scope(|scope| {
            scope.spawn(|_| loop {
                tabs.settings_tab.lock().unwrap().tick();
                sleep(Duration::from_millis(100));
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
            if conn.close_when_done() {
                shared_state.set_running(false, client_send.clone());
                close_frontend(&mut client_send);
            }
        })
        .unwrap();
    });

    Ok(())
}
