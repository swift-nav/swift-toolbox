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
    SbpMessage,
};
use std::io;

use crate::client_sender::BoxedClientSender;
use crate::types::{
    BaselineNED, Dops, GpsTime, MsgSender, ObservationMsg, PosLLH, RealtimeDelay, Specan,
    UartState, VelNED,
};
use crate::utils::refresh_connection_frontend;
use crate::Tabs;
use crate::{connection::Connection, shared_state::SharedState};
use crate::{errors::UNABLE_TO_CLONE_UPDATE_SHARED, settings_tab};
use crate::{log_panel::handle_log_msg, settings_tab::SettingsTab};
use crate::{main_tab, update_tab};

pub use messages::{Messages, StopToken};

pub fn process_messages(
    mut messages: Messages,
    msg_sender: MsgSender,
    conn: Connection,
    shared_state: SharedState,
    mut client_sender: BoxedClientSender,
) -> Result<(), io::Error> {
    shared_state.set_current_connection(conn.name());
    refresh_connection_frontend(&mut client_sender, shared_state.clone());

    let source: LinkSource<Tabs> = LinkSource::new();
    let tabs = Tabs::new(
        shared_state.clone(),
        client_sender.clone(),
        msg_sender.clone(),
    );

    let link = source.link();

    link.register(|tabs: &Tabs, msg: MsgAgeCorrections| {
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

    link.register(|tabs: &Tabs, msg: MsgAngularRate| {
        tabs.solution.lock().unwrap().handle_angular_rate(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgBaselineHeading| {
        tabs.baseline.lock().unwrap().handle_baseline_heading(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgCommandResp| {
        tabs.update.lock().unwrap().handle_command_resp(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgCsacTelemetry| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_csac_telemetry(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgCsacTelemetryLabels| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_csac_telemetry_labels(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgDeviceMonitor| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_device_monitor(msg);
    });

    link.register(|tabs: &Tabs, msg: BaselineNED| {
        tabs.baseline
            .lock()
            .unwrap()
            .handle_baseline_ned(msg.clone());
        tabs.status_bar.lock().unwrap().handle_baseline_ned(msg);
    });

    link.register(|tabs: &Tabs, msg: Dops| {
        tabs.solution.lock().unwrap().handle_dops(msg);
    });

    link.register(|tabs: &Tabs, msg: GpsTime| {
        tabs.baseline.lock().unwrap().handle_gps_time(msg.clone());
        tabs.solution.lock().unwrap().handle_gps_time(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgHeartbeat| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_heartbeat();
        tabs.status_bar.lock().unwrap().handle_heartbeat(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgImuAux| {
        tabs.advanced_imu.lock().unwrap().handle_imu_aux(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgImuRaw| {
        tabs.advanced_imu.lock().unwrap().handle_imu_raw(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgInsStatus| {
        tabs.solution.lock().unwrap().handle_ins_status(msg.clone());
        tabs.status_bar.lock().unwrap().handle_ins_status(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgInsUpdates| {
        tabs.advanced_imu
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

    link.register(|tabs: &Tabs, msg: MsgMagRaw| {
        tabs.advanced_magnetometer
            .lock()
            .unwrap()
            .handle_mag_raw(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgMeasurementState| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_msg_measurement_state(msg.states);
    });

    link.register(|tabs: &Tabs, msg: MsgNetworkStateResp| {
        tabs.advanced_networking
            .lock()
            .unwrap()
            .handle_network_state_resp(msg);
    });

    link.register(|tabs: &Tabs, msg: ObservationMsg| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_obs(msg.clone());
        tabs.observation.lock().unwrap().handle_obs(msg);
    });

    link.register(|_: MsgObsDepA| {
        debug!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot or Observation tab.");
    });

    link.register(|tabs: &Tabs, msg: MsgOrientEuler| {
        tabs.solution.lock().unwrap().handle_orientation_euler(msg);
    });

    link.register(|tabs: &Tabs, msg: PosLLH| {
        tabs.solution.lock().unwrap().handle_pos_llh(msg.clone());
        tabs.status_bar.lock().unwrap().handle_pos_llh(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgPosLlhCov| {
        tabs.solution.lock().unwrap().handle_pos_llh_cov(msg);
    });

    link.register(|tabs: &Tabs, msg: Specan| {
        tabs.advanced_spectrum_analyzer
            .lock()
            .unwrap()
            .handle_specan(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgSvAzEl| {
        tabs.tracking_sky_plot.lock().unwrap().handle_sv_az_el(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgThreadState| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_thread_state(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgTrackingState| {
        tabs.tracking_signals
            .lock()
            .unwrap()
            .handle_msg_tracking_state(msg.states);
    });

    link.register(|tabs: &Tabs, msg: VelNED| {
        tabs.solution.lock().unwrap().handle_vel_ned(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgVelNed| {
        // why does this tab not take both VelNED messages?
        tabs.solution_velocity.lock().unwrap().handle_vel_ned(msg);
    });

    link.register(|tabs: &Tabs, msg: UartState| {
        tabs.advanced_system_monitor
            .lock()
            .unwrap()
            .handle_uart_state(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgUtcTime| {
        tabs.baseline.lock().unwrap().handle_utc_time(msg.clone());
        tabs.solution.lock().unwrap().handle_utc_time(msg);
    });

    link.register(|tabs: &Tabs, msg: MsgLog| {
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
            client_sender.clone(),
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
                client_sender.clone(),
                source.stateless_link(),
                msg_sender.clone(),
            );
        });
        scope.spawn(|_| {
            main_tab::logging_stats_thread(
                logging_stats_rx,
                shared_state.clone(),
                client_sender.clone(),
            )
        });

        if conn.settings_enabled() {
            scope.spawn(|_| {
                let tab = settings_tab.as_ref().unwrap();
                shared_state.set_settings_refresh(true);
                settings_tab::start_thd(tab);
            });
        }

        for (message, gps_time) in &mut messages {
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

    messages.take_err()
}

mod messages {
    use std::{fmt, io, sync::Arc, thread, time::Duration};

    use crossbeam::channel::{self, Receiver, Sender};
    use log::debug;
    use sbp::{
        time::{GpsTime, GpsTimeError},
        DeserializeError, Sbp, SbpIterExt,
    };

    type MessageWithTimeIter = Box<dyn Iterator<Item = MessageWithTime> + Send>;

    type MessageWithTime = (
        Result<Sbp, DeserializeError>,
        Option<Result<GpsTime, GpsTimeError>>,
    );

    pub struct Messages {
        messages: Receiver<MessageWithTime>,
        stop_recv: Receiver<()>,
        err: Result<(), io::Error>,
    }

    impl Messages {
        const TIMEOUT: Duration = Duration::from_secs(30);

        pub fn new(inner: MessageWithTimeIter) -> (Self, StopToken) {
            let (stop_token, stop_recv) = StopToken::new();
            let messages = start_read_thd(inner);
            (
                Self {
                    messages,
                    stop_recv,
                    err: Ok(()),
                },
                stop_token,
            )
        }

        pub fn from_reader<R>(reader: R) -> (Self, StopToken)
        where
            R: io::Read + Send + 'static,
        {
            let messages = sbp::iter_messages(reader).with_rover_time();
            Self::new(Box::new(messages))
        }

        pub fn take_err(&mut self) -> Result<(), io::Error> {
            std::mem::replace(&mut self.err, Ok(()))
        }
    }

    impl Iterator for Messages {
        type Item = (Sbp, Option<Result<GpsTime, GpsTimeError>>);

        fn next(&mut self) -> Option<Self::Item> {
            crossbeam::select! {
                recv(self.messages) -> msg => {
                    match msg.ok()? {
                        (Ok(msg), time) => Some((msg, time)),
                        (Err(e), _) => match e {
                            DeserializeError::IoError(e) => {
                                self.err = Err(e);
                                None
                            }
                            _ => {
                                debug!("{}", e);
                                self.next()
                            }
                        },
                    }
                }
                recv(self.stop_recv) -> _ => None,
                default(Self::TIMEOUT) => {
                    self.err = Err(io::Error::new(io::ErrorKind::TimedOut, "timeout"));
                    None
                }
            }
        }
    }

    fn start_read_thd(messages: MessageWithTimeIter) -> Receiver<MessageWithTime> {
        let (tx, rx) = channel::bounded(1000);
        thread::spawn(move || {
            for message in messages {
                if tx.send(message).is_err() {
                    break;
                }
            }
        });
        rx
    }

    /// Used to stop the [Messages](super::Messages) iterator. This can be called manually,
    /// but will automatically be called after all copies of this token have been dropped.
    pub struct StopToken(Arc<Shared>);

    impl StopToken {
        fn new() -> (Self, Receiver<()>) {
            let (send, recv) = channel::bounded(1);
            (StopToken(Arc::new(Shared(send))), recv)
        }

        pub fn stop(&self) {
            self.0.stop();
        }
    }

    impl Clone for StopToken {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    impl fmt::Debug for StopToken {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("StopToken").finish()
        }
    }

    // Wrapper type so can hook into the drop call that happens when the last Arc<Shared> is dropped.
    // We could not use this and drop when the arc's strong count is 1, but the count might change
    // between calling `Arc::strong_count` and `self.stop`
    struct Shared(Sender<()>);

    impl Shared {
        fn stop(&self) {
            // try_send to avoid blocking. We don't care about the error because that means either:
            // 1. The reciever was dropped - meaning the message thread has already ended
            // 2. The channel is full - meaning someone already called stop
            let _ = self.0.try_send(());
        }
    }

    impl Drop for Shared {
        fn drop(&mut self) {
            self.stop();
        }
    }
}
