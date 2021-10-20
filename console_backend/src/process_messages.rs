use std::{io, sync::Arc, thread};

use crossbeam::channel::{bounded, Receiver, Sender};
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
    SbpIterExt, SbpMessage,
};

use crate::types::{
    BaselineNED, CapnProtoSender, Dops, GpsTime, MsgSender, ObservationMsg, PosLLH, RealtimeDelay,
    Specan, UartState, VelNED,
};
use crate::utils::refresh_connection_frontend;
use crate::Tabs;
use crate::{connection::Connection, shared_state::SharedState};
use crate::{errors::UNABLE_TO_CLONE_UPDATE_SHARED, settings_tab};
use crate::{log_panel::handle_log_msg, settings_tab::SettingsTab};
use crate::{main_tab, update_tab};

pub fn process_messages<S>(
    messages: Messages,
    msg_sender: MsgSender,
    conn: Connection,
    shared_state: SharedState,
    client_send: S,
) where
    S: CapnProtoSender,
{
    refresh_connection_frontend(&mut client_send.clone(), shared_state.clone());

    let source: LinkSource<Tabs<S>> = LinkSource::new();
    let tabs = Tabs::new(
        shared_state.clone(),
        client_send.clone(),
        msg_sender.clone(),
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
    let (update_tab_tx, update_tab_rx) = tabs.update.lock().unwrap().clone_channel();
    let (logging_stats_tx, logging_stats_rx): (Sender<bool>, Receiver<bool>) = bounded(1);
    let settings_tab = conn.settings_enabled().then(|| {
        SettingsTab::new(
            shared_state.clone(),
            client_send.clone(),
            msg_sender.clone(),
            source.stateless_link(),
        )
    });
    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            update_tab::update_tab_thread(
                update_tab_tx.clone(),
                update_tab_rx,
                update_tab_context,
                shared_state.clone(),
                client_send.clone(),
                source.stateless_link(),
                msg_sender.clone(),
            );
        });
        scope.spawn(|_| {
            main_tab::logging_stats_thread(
                logging_stats_rx,
                shared_state.clone(),
                client_send.clone(),
            )
        });

        if conn.settings_enabled() {
            scope.spawn(|_| {
                let tab = settings_tab.as_ref().unwrap();
                shared_state.set_settings_refresh(true);
                settings_tab::start_thd(tab);
            });
        }

        for (message, gps_time) in messages {
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
            if let RealtimeDelay::On = conn.realtime_delay() {
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
        if let Some(ref tab) = settings_tab {
            tab.stop()
        }
        if let Err(err) = update_tab_tx.send(None) {
            error!("Issue stopping update tab: {}", err);
        }
        if let Err(err) = logging_stats_tx.send(false) {
            error!("Issue stopping logging stats thread: {}", err);
        }
        if let Err(e) = tabs.main.lock().unwrap().end_csv_logging() {
            error!("Issue closing csv file, {}", e);
        }
        tabs.main.lock().unwrap().close_sbp();
    })
    .unwrap();
}

type Message = (
    sbp::Sbp,
    Option<std::result::Result<sbp::time::GpsTime, sbp::time::GpsTimeError>>,
);

/// Wrapper around the iterator of messages that enables other threads
/// to stop the iterator.
pub struct Messages {
    messages: Receiver<Message>,
    canceled: Receiver<()>,
}

impl Messages {
    pub fn new<R>(reader: R) -> (StopToken, Self)
    where
        R: io::Read + Send + 'static,
    {
        let (sink, messages) = crossbeam::channel::bounded(100);
        let (cancel, canceled) = crossbeam::channel::bounded(1);
        thread::spawn(move || Messages::start_thd(reader, sink));
        (StopToken(Arc::new(cancel)), Self { messages, canceled })
    }

    fn start_thd<R: io::Read>(reader: R, sink: Sender<Message>) {
        for message in sbp::iter_messages(reader)
            .log_errors(log::Level::Debug)
            .with_rover_time()
        {
            // this will error after `Messages` is dropped which will
            // stop this thread
            if sink.send(message).is_err() {
                break;
            }
        }
    }
}

impl Iterator for Messages {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        crossbeam::select! {
            recv(self.canceled) -> _ => None,
            recv(self.messages) -> msg => msg.ok(),
        }
    }
}

/// Used to break the `process_messages` loop. Can be stopped manually
/// or will automatically stop after all copies of this have been dropped.
#[derive(Debug)]
pub struct StopToken(Arc<Sender<()>>);

impl StopToken {
    pub fn stop(&self) {
        let _ = self.0.try_send(());
    }
}

impl Clone for StopToken {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Drop for StopToken {
    fn drop(&mut self) {
        // if this is the last one make sure we stop the thread
        if Arc::strong_count(&self.0) == 1 {
            self.stop();
        }
    }
}
