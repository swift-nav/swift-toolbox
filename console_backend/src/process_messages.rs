use std::io;

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
        piksi::{MsgDeviceMonitor, MsgNetworkStateResp, MsgThreadState},
        system::{MsgHeartbeat, MsgInsStatus, MsgInsUpdates, MsgStartup},
        tracking::{MsgMeasurementState, MsgTrackingState},
    },
    Sbp, SbpMessage,
};

use crate::client_sender::BoxedClientSender;
use crate::connection::Connection;
use crate::errors::{PROCESS_MESSAGES_FAILURE, UNABLE_TO_CLONE_UPDATE_SHARED};
use crate::log_panel;
use crate::shared_state::SharedState;
use crate::tabs::{settings_tab, update_tab};
use crate::types::{
    BaselineNED, Dops, GpsTime, MsgSender, ObservationMsg, PosLLH, Specan, UartState, VelNED,
};
use crate::Tabs;

pub use messages::{Messages, StopToken};

pub fn process_messages(
    mut messages: Messages,
    msg_sender: MsgSender,
    conn: Connection,
    shared_state: SharedState,
    client_sender: BoxedClientSender,
) -> Result<(), io::Error> {
    let source: LinkSource<Tabs> = LinkSource::new();
    let tabs = if conn.settings_enabled() {
        Tabs::with_settings(
            shared_state.clone(),
            client_sender.clone(),
            msg_sender.clone(),
        )
    } else {
        Tabs::new(
            shared_state.clone(),
            client_sender.clone(),
            msg_sender.clone(),
        )
    };
    register_events(source.link());
    let update_tab_context = tabs
        .update
        .lock()
        .expect(UNABLE_TO_CLONE_UPDATE_SHARED)
        .clone_update_tab_context();
    update_tab_context.set_serial_prompt(conn.is_serial());
    let (update_tab_tx, update_tab_rx) = tabs.update.lock().unwrap().clone_channel();
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
        if conn.settings_enabled() {
            scope.spawn(|_| {
                let tab = tabs.settings.as_ref().unwrap();
                shared_state.set_settings_refresh(true);
                settings_tab::start_thd(tab);
            });
        }
        for (message, _) in &mut messages {
            source.send_with_state(&tabs, &message);
            if let Some(ref tab) = tabs.settings {
                tab.handle_msg(message);
            }
            log::logger().flush();
        }
        if let Some(ref tab) = tabs.settings {
            tab.stop()
        }
        if let Err(err) = update_tab_tx.send(None) {
            error!("Issue stopping update tab: {err}");
        }
    })
    .expect(PROCESS_MESSAGES_FAILURE);

    let err = messages.take_err();
    let handle = messages.into_handle();
    handle.join().unwrap();
    err
}

fn register_events(link: sbp::link::Link<Tabs>) {
    link.register(|tabs: &Tabs, msg: MsgAgeCorrections| {
        tabs.baseline
            .lock()
            .unwrap()
            .handle_age_corrections(msg.clone());
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_age_corrections(msg.clone());
        tabs.status_bar.lock().unwrap().handle_age_corrections(msg);
    });
    link.register(|tabs: &Tabs, msg: MsgAngularRate| {
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_angular_rate(msg);
    });
    link.register(|tabs: &Tabs, msg: MsgBaselineHeading| {
        tabs.baseline.lock().unwrap().handle_baseline_heading(msg);
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
        tabs.solution_position.lock().unwrap().handle_dops(msg);
    });
    link.register(|tabs: &Tabs, msg: GpsTime| {
        tabs.baseline.lock().unwrap().handle_gps_time(msg.clone());
        tabs.solution_position.lock().unwrap().handle_gps_time(msg);
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
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_ins_status(msg.clone());
        tabs.status_bar.lock().unwrap().handle_ins_status(msg);
    });
    link.register(|tabs: &Tabs, msg: MsgInsUpdates| {
        tabs.advanced_imu
            .lock()
            .unwrap()
            .fusion_engine_status_bar
            .handle_ins_updates(msg.clone());
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_ins_updates(msg.clone());
        tabs.status_bar.lock().unwrap().handle_ins_updates(msg);
    });
    link.register(|_tabs: &Tabs, msg: MsgLog| {
        log_panel::handle_log_msg(msg);
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
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_orientation_euler(msg);
    });
    link.register(|tabs: &Tabs, msg: PosLLH| {
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_pos_llh(msg.clone());
        tabs.status_bar.lock().unwrap().handle_pos_llh(msg);
    });
    link.register(|tabs: &Tabs, msg: MsgPosLlhCov| {
        tabs.solution_position
            .lock()
            .unwrap()
            .handle_pos_llh_cov(msg);
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
    link.register(|tabs: &Tabs, _msg: MsgStartup| {
        if let Some(settings) = &tabs.settings {
            settings.shared_state.set_settings_refresh(true);
        }
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
        tabs.solution_position.lock().unwrap().handle_vel_ned(msg);
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
        tabs.solution_position.lock().unwrap().handle_utc_time(msg);
    });
    link.register(|tabs: &Tabs, msg: Sbp| {
        tabs.main.lock().unwrap().serialize_sbp(&msg);
        tabs.status_bar.lock().unwrap().add_bytes(msg.encoded_len());
        tabs.advanced_networking.lock().unwrap().handle_sbp(&msg);
    });
}

mod messages {
    use std::{
        fmt, io,
        sync::Arc,
        thread::{self, JoinHandle},
        time::{Duration, Instant},
    };

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
        handle: JoinHandle<()>,
    }

    impl Messages {
        const TIMEOUT: Duration = Duration::from_secs(2);

        pub fn new<R>(reader: R) -> (Self, StopToken)
        where
            R: io::Read + Send + 'static,
        {
            let messages = sbp::iter_messages_with_timeout(reader, Self::TIMEOUT).with_rover_time();
            Self::from_boxed(Box::new(messages))
        }

        pub fn with_realtime_delay<R>(reader: R) -> (Self, StopToken)
        where
            R: io::Read + Send + 'static,
        {
            let messages = sbp::iter_messages_with_timeout(reader, Self::TIMEOUT).with_rover_time();
            let messages = Box::new(RealtimeIter::new(messages));
            Self::from_boxed(messages)
        }

        pub fn take_err(&mut self) -> Result<(), io::Error> {
            std::mem::replace(&mut self.err, Ok(()))
        }

        pub fn into_handle(self) -> JoinHandle<()> {
            self.handle
        }

        fn from_boxed(inner: MessageWithTimeIter) -> (Self, StopToken) {
            let (stop_token, stop_recv) = StopToken::new();
            let (messages, handle) = start_read_thd(inner);
            (
                Self {
                    messages,
                    stop_recv,
                    err: Ok(()),
                    handle,
                },
                stop_token,
            )
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
            }
        }
    }

    fn start_read_thd(
        messages: MessageWithTimeIter,
    ) -> (Receiver<MessageWithTime>, JoinHandle<()>) {
        let (tx, rx) = channel::bounded(1000);
        let h = thread::spawn(move || {
            for message in messages {
                if tx.send(message).is_err() {
                    break;
                }
            }
        });
        (rx, h)
    }

    struct RealtimeIter<M> {
        messages: M,
        last_time: Option<GpsTime>,
        updated_at: Instant,
    }

    impl<M> RealtimeIter<M> {
        fn new(messages: M) -> Self {
            Self {
                messages,
                last_time: None,
                updated_at: Instant::now(),
            }
        }
    }

    impl<M> Iterator for RealtimeIter<M>
    where
        M: Iterator<Item = MessageWithTime>,
    {
        type Item = M::Item;

        fn next(&mut self) -> Option<Self::Item> {
            let msg = self.messages.next()?;
            match (self.last_time, &msg.1) {
                (Some(last_time), Some(Ok(time))) if &last_time < time => {
                    let diff = *time - last_time;
                    let elapsed = self.updated_at.elapsed();
                    if diff > elapsed {
                        let sleep_dur = diff - elapsed;
                        debug!("Realtime delay sleeping for {:?}", sleep_dur);
                        thread::sleep(sleep_dur);
                    }
                    self.last_time = Some(*time);
                    self.updated_at = Instant::now();
                }
                (None, Some(Ok(time))) => {
                    self.last_time = Some(*time);
                    self.updated_at = Instant::now();
                }
                _ => (),
            };
            Some(msg)
        }
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

    #[cfg(test)]
    mod tests {
        use std::io::Cursor;
        use std::time::Duration;

        use sbp::messages::logging::MsgLog;
        use sbp::messages::navigation::MsgGpsTime;

        use super::*;

        // wiggle room for timing the delay
        const JIFFY: Duration = Duration::from_millis(10);

        fn msg_gps_time(tow: u32) -> Sbp {
            MsgGpsTime {
                sender_id: Some(0),
                wn: 1,
                tow,
                ns_residual: 1,
                flags: 1,
            }
            .into()
        }

        // any message without time would do
        fn msg_log() -> Sbp {
            MsgLog {
                sender_id: Some(0),
                level: 1,
                text: String::from("hello").into(),
            }
            .into()
        }

        #[test]
        fn realtime_delay() {
            let mut data = Vec::new();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            sbp::to_writer(&mut data, &msg_gps_time(1000)).unwrap();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            sbp::to_writer(&mut data, &msg_gps_time(2000)).unwrap(); // one second from the last MsgGpsTime
            let (messages, _token) = Messages::with_realtime_delay(Cursor::new(data));
            let start = Instant::now();
            assert_eq!(messages.count(), 4);
            assert!(start.elapsed() - Duration::from_secs(1) < JIFFY);
        }

        #[test]
        fn no_realtime_delay() {
            let mut data = Vec::new();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            sbp::to_writer(&mut data, &msg_gps_time(1000)).unwrap();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            sbp::to_writer(&mut data, &msg_gps_time(2000)).unwrap();
            let (messages, _token) = Messages::new(Cursor::new(data));
            let start = Instant::now();
            assert_eq!(messages.count(), 4);
            assert!(start.elapsed() < JIFFY);
        }

        #[test]
        fn realtime_delay_no_last_time() {
            let mut data = Vec::new();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            sbp::to_writer(&mut data, &msg_gps_time(1000)).unwrap();
            sbp::to_writer(&mut data, &msg_log()).unwrap();
            let (messages, _token) = Messages::with_realtime_delay(Cursor::new(data));
            let start = Instant::now();
            assert_eq!(messages.count(), 3);
            // only one message with time so no delay should have been added
            assert!(start.elapsed() < JIFFY);
        }
    }
}
