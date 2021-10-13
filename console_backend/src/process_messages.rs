use std::{io::ErrorKind, thread::sleep, time::Duration};

use crossbeam::sync::Parker;
use log::{debug, error};
use sbp::{
    link::LinkSource,
    messages::{
        imu::{MsgImuAux, MsgImuRaw},
        logging::MsgLog,
        mag::MsgMagRaw,
        navigation::{MsgAgeCorrections, MsgPosLlhCov, MsgUtcTime, MsgVelNed},
        observation::{MsgObsDepA, MsgSvAzEl},
        orientation::{MsgAngularRate, MsgBaselineHeading, MsgOrientEuler},
        piksi::{MsgCommandResp, MsgDeviceMonitor, MsgNetworkStateResp, MsgThreadState},
        system::{
            MsgCsacTelemetry, MsgCsacTelemetryLabels, MsgHeartbeat, MsgInsStatus, MsgInsUpdates,
        },
        tracking::{MsgMeasurementState, MsgTrackingState},
    },
    sbp_iter_ext::{ControlFlow, SbpIterExt},
    SbpMessage,
};

use crate::constants::PAUSE_LOOP_SLEEP_DURATION_MS;
use crate::errors::UNABLE_TO_CLONE_UPDATE_SHARED;
use crate::log_panel::handle_log_msg;
use crate::shared_state::SharedState;
use crate::types::{
    BaselineNED, CapnProtoSender, Dops, GpsTime, MsgSender, ObservationMsg, PosLLH, RealtimeDelay,
    Result, Specan, VelNED,
};
use crate::update_tab;
use crate::utils::{close_frontend, refresh_connection_frontend};
use crate::Tabs;
use crate::{connection::Connection, types::UartState};

pub fn process_messages<S>(
    conn: Connection,
    shared_state: SharedState,
    mut client_send: S,
) -> Result<()>
where
    S: CapnProtoSender,
{
    shared_state.set_running(true, client_send.clone());
    shared_state.set_settings_refresh(conn.settings_enabled());
    let realtime_delay = conn.realtime_delay();
    let (rdr, writer) = conn.try_connect(Some(shared_state.clone()))?;
    let msg_sender = MsgSender::new(writer);
    shared_state.set_current_connection(conn.name());
    refresh_connection_frontend(&mut client_send.clone(), shared_state.clone());
    let messages = {
        let state = shared_state.clone();
        let client = client_send.clone();
        sbp::iter_messages(rdr)
            .handle_errors(move |e| {
                debug!("{}", e);
                match e {
                    sbp::DeserializeError::IoError(err) => {
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
    let source: LinkSource<Tabs<S>> = LinkSource::new();
    let tabs = Tabs::new(
        shared_state.clone(),
        client_send.clone(),
        msg_sender.clone(),
        source.stateless_link(),
    );

    let link = source.link();

    link.register(|tabs: &Tabs<S>, msg: MsgAgeCorrections| {
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

    link.register(|tabs: &Tabs<S>, msg: MsgAngularRate| {
        tabs.solution.lock().unwrap().handle_angular_rate(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgBaselineHeading| {
        tabs.baseline.lock().unwrap().handle_baseline_heading(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgCommandResp| {
        tabs.update.lock().unwrap().handle_command_resp(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgCsacTelemetry| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_csac_telemetry(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgCsacTelemetryLabels| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_csac_telemetry_labels(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgDeviceMonitor| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_device_monitor(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: BaselineNED| {
        tabs.baseline
            .lock()
            .unwrap()
            .handle_baseline_ned(msg.clone());
        tabs.status_bar.lock().unwrap().handle_baseline_ned(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: Dops| {
        tabs.solution.lock().unwrap().handle_dops(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: GpsTime| {
        tabs.baseline.lock().unwrap().handle_gps_time(msg.clone());
        tabs.solution.lock().unwrap().handle_gps_time(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgHeartbeat| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_heartbeat();
        tabs.status_bar.lock().unwrap().handle_heartbeat(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgImuAux| {
        tabs.advanced_ins.lock().unwrap().handle_imu_aux(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgImuRaw| {
        tabs.advanced_ins.lock().unwrap().handle_imu_raw(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgInsStatus| {
        tabs.solution.lock().unwrap().handle_ins_status(msg.clone());
        tabs.status_bar.lock().unwrap().handle_ins_status(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgInsUpdates| {
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

    link.register(|tabs: &Tabs<S>, msg: MsgMagRaw| {
        tabs.advanced_magnetometer
            .lock()
            .unwrap()
            .handle_mag_raw(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgMeasurementState| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_msg_measurement_state(msg.states);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgNetworkStateResp| {
        tabs.advanced_networking
            .lock()
            .unwrap()
            .handle_network_state_resp(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: ObservationMsg| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_obs(msg.clone());
        tabs.observation.lock().unwrap().handle_obs(msg);
    });

    link.register(|_: MsgObsDepA| {
        println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
    });

    link.register(|tabs: &Tabs<S>, msg: MsgOrientEuler| {
        tabs.solution.lock().unwrap().handle_orientation_euler(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: PosLLH| {
        tabs.solution.lock().unwrap().handle_pos_llh(msg.clone());
        tabs.status_bar.lock().unwrap().handle_pos_llh(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgPosLlhCov| {
        tabs.solution.lock().unwrap().handle_pos_llh_cov(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: Specan| {
        tabs.advanced_spectrum_analyzer
            .lock()
            .unwrap()
            .handle_specan(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgSvAzEl| {
        tabs.tracking_sky_plot.lock().unwrap().handle_sv_az_el(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgThreadState| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_thread_state(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgTrackingState| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_msg_tracking_state(msg.states);
    });

    link.register(|tabs: &Tabs<S>, msg: VelNED| {
        tabs.solution.lock().unwrap().handle_vel_ned(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgVelNed| {
        // why does this tab not take both VelNED messages?
        tabs.solution_velocity.lock().unwrap().handle_vel_ned(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: UartState| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_uart_state(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgUtcTime| {
        tabs.baseline.lock().unwrap().handle_utc_time(msg.clone());
        tabs.solution.lock().unwrap().handle_utc_time(msg);
    });

    link.register(|tabs: &Tabs<S>, msg: MsgLog| {
        tabs.update.lock().unwrap().handle_log_msg(msg.clone());
        handle_log_msg(msg);
    });

    let update_tab_context = tabs
        .update
        .lock()
        .expect(UNABLE_TO_CLONE_UPDATE_SHARED)
        .clone_update_tab_context();
    update_tab_context.set_serial_prompt(conn.is_serial());
    let client_send_clone = client_send.clone();
    let (update_tab_tx, update_tab_rx) = tabs.update.lock().unwrap().clone_channel();
    let settings_parker = Parker::new();
    let settings_unparker = settings_parker.unparker().clone();
    crossbeam::scope(|scope| {
        let settings_tab = &tabs.settings_tab;
        let settings_shared_state = shared_state.clone();
        scope.spawn(move |_| {
            while settings_shared_state.is_running() {
                settings_tab.lock().unwrap().tick();
                settings_parker.park();
            }
        });
        scope.spawn(|_| {
            update_tab::update_tab_thread(
                update_tab_tx.clone(),
                update_tab_rx,
                update_tab_context,
                shared_state.clone(),
                client_send_clone,
                source.stateless_link(),
                msg_sender.clone(),
            );
        });
        for (message, gps_time) in messages {
            if shared_state.settings_needs_update() {
                settings_unparker.unpark();
            }
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
            let sent = source.send_with_state(&tabs, &message);
            tabs.main.lock().unwrap().serialize_sbp(&message);
            tabs.status_bar
                .lock()
                .unwrap()
                .add_bytes(message.encoded_len());
            tabs.advanced_networking
                .lock()
                .unwrap()
                .handle_sbp(&message);
            if let RealtimeDelay::On = realtime_delay {
                if sent {
                    tabs.main.lock().unwrap().realtime_delay(gps_time);
                } else {
                    debug!(
                        "Message, {}, ignored for realtime delay.",
                        message.message_name()
                    );
                }
            }
            log::logger().flush();
        }
        if let Err(err) = update_tab_tx.send(None) {
            error!("Issue stopping update tab: {}", err);
        }
        if conn.close_when_done() {
            shared_state.set_running(false, client_send.clone());
            close_frontend(&mut client_send);
        }
        settings_unparker.unpark();
    })
    .unwrap();

    Ok(())
}
