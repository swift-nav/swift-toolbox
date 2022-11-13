use std::{io::Cursor, path::PathBuf, str::FromStr, thread};

use capnp::serialize;
use chrono::{DateTime, Utc};
use crossbeam::channel;
use log::{debug, error, warn};

use crate::client_sender::BoxedClientSender;
use crate::common_constants::SbpLogging;
use crate::connection::ConnectionManager;
use crate::console_backend_capnp as m;
use crate::errors::{
    CAP_N_PROTO_DESERIALIZATION_FAILURE, CONVERT_TO_STR_FAILURE,
    SOLUTION_POSITION_UNIT_SELECTION_NOT_AVAILABLE,
};
use crate::log_panel::LogLevel;
use crate::output::CsvLogging;
use crate::shared_state::{AdvancedNetworkingState, SharedState};
use crate::tabs::{
    settings_tab::SaveRequest, solution_tab::LatLonUnits, update_tab::UpdateTabUpdate,
};
use crate::types::{FlowControl, RealtimeDelay};
use crate::utils::refresh_connection_frontend;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;
pub type UtcDateTime = DateTime<Utc>;

/// Handles all capnproto messages, links from front-end dispatched to backend
pub fn server_recv_thread(
    conn_manager: ConnectionManager,
    client_sender: BoxedClientSender,
    server_recv: channel::Receiver<Vec<u8>>,
    shared_state: SharedState,
) {
    thread::spawn(move || {
        // We use warning here because it is the minimum log level displayed
        // in the log panel until the user selects a different level.
        warn!("Console started...");
        loop {
            log::logger().flush();
            let mut reader = match server_recv.recv() {
                Ok(buf) => Cursor::new(buf),
                Err(_) => break,
            };
            let message_reader =
                serialize::read_message(&mut reader, capnp::message::ReaderOptions::new()).unwrap();
            let message = message_reader
                .get_root::<m::message::Reader>()
                .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
            let message = match message.which() {
                Ok(msg) => msg,
                Err(e) => {
                    error!("error reading message: {}", e);
                    continue;
                }
            };
            match message {
                m::message::SerialRefreshRequest(Ok(_)) => {
                    refresh_connection_frontend(&client_sender, &shared_state);
                }
                m::message::DisconnectRequest(Ok(_)) => {
                    conn_manager.disconnect();
                }
                m::message::FileRequest(Ok(req)) => {
                    let filename = req
                        .get_filename()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let filename = filename.to_string();
                    conn_manager.connect_to_file(filename, RealtimeDelay::On, false);
                }
                m::message::TcpRequest(Ok(req)) => {
                    let host = req.get_host().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let port = req.get_port();
                    if let Err(e) = conn_manager.connect_to_host(host.to_string(), port) {
                        error!("Failed to establish tcp connection: {}", e);
                    };
                }
                m::message::SerialRequest(Ok(req)) => {
                    let device = req.get_device().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let device = device.to_string();
                    let baudrate = req.get_baudrate();
                    let flow = req.get_flow_control().unwrap();
                    let flow = FlowControl::from_str(flow).unwrap();
                    conn_manager.connect_to_serial(device, baudrate, flow);
                }
                m::message::TrackingSignalsStatusFront(Ok(cv_in)) => {
                    let check_visibility = cv_in
                        .get_tracking_signals_check_visibility()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let check_visibility: Vec<String> = check_visibility
                        .iter()
                        .map(|x| String::from(x.unwrap()))
                        .collect();
                    shared_state.set_check_visibility(check_visibility);
                }
                m::message::LoggingBarFront(Ok(cv_in)) => {
                    let directory = cv_in
                        .get_directory()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.set_logging_directory(PathBuf::from(directory));
                    shared_state.lock().logging_bar.csv_logging =
                        CsvLogging::from(cv_in.get_csv_logging());
                    shared_state.set_sbp_logging(cv_in.get_sbp_logging(), client_sender.clone());
                    let sbp_logging_format = cv_in
                        .get_sbp_logging_format()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.set_sbp_logging_format(
                        SbpLogging::from_str(sbp_logging_format).expect(CONVERT_TO_STR_FAILURE),
                    );
                }
                m::message::LogLevelFront(Ok(cv_in)) => {
                    let shared_state = shared_state.clone();
                    let log_level = cv_in
                        .get_log_level()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let log_level = LogLevel::from_str(log_level).expect(CONVERT_TO_STR_FAILURE);
                    shared_state.set_log_level(log_level);
                }
                m::message::ConnectionDialogStatus(Ok(cv_in)) => {
                    let visible = cv_in.get_visible();
                    shared_state.lock().connection_dialog_visible = visible;
                }
                m::message::SolutionVelocityStatusFront(Ok(cv_in)) => {
                    let unit = cv_in
                        .get_solution_velocity_unit()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.lock().solution_tab.velocity_tab.unit = unit.to_string();
                }
                m::message::SolutionPositionStatusUnitFront(Ok(cv_in)) => {
                    let unit = cv_in
                        .get_solution_position_unit()
                        .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.lock().solution_tab.position_tab.unit = Some(
                        LatLonUnits::from_str(unit)
                            .expect(SOLUTION_POSITION_UNIT_SELECTION_NOT_AVAILABLE),
                    );
                }
                m::message::SolutionPositionStatusButtonFront(Ok(cv_in)) => {
                    shared_state.lock().solution_tab.position_tab.clear =
                        cv_in.get_solution_position_clear();
                    shared_state.lock().solution_tab.position_tab.pause =
                        cv_in.get_solution_position_pause();
                }
                m::message::BaselinePlotStatusButtonFront(Ok(cv_in)) => {
                    let mut guard = shared_state.lock();
                    guard.baseline_tab.clear = cv_in.get_clear();
                    guard.baseline_tab.pause = cv_in.get_pause();
                    guard.baseline_tab.reset = cv_in.get_reset_filters();
                }
                m::message::AdvancedSpectrumAnalyzerStatusFront(Ok(cv_in)) => {
                    shared_state
                        .lock()
                        .advanced_spectrum_analyzer_tab
                        .channel_idx = cv_in.get_channel();
                }
                m::message::UpdateTabStatusFront(Ok(cv_in)) => {
                    if let Some(update_tab_sender) = shared_state.update_tab_sender() {
                        let download_latest_firmware = cv_in.get_download_latest_firmware();
                        let update_firmware = cv_in.get_update_firmware();
                        let send_file_to_device = cv_in.get_send_file_to_device();
                        let serial_prompt_confirm = cv_in.get_serial_prompt_confirm();
                        let firmware_directory = match cv_in.get_download_directory().which() {
                            Ok(m::update_tab_status_front::download_directory::Directory(Ok(
                                directory,
                            ))) => Some(PathBuf::from(directory)),
                            Err(e) => {
                                error!("{}", e);
                                None
                            }
                            _ => None,
                        };
                        let firmware_local_filepath = match cv_in
                            .get_update_local_filepath()
                            .which()
                        {
                            Ok(m::update_tab_status_front::update_local_filepath::Filepath(
                                Ok(filepath),
                            )) => Some(PathBuf::from(filepath)),
                            Err(e) => {
                                error!("{}", e);
                                None
                            }
                            _ => None,
                        };
                        let firmware_local_filename = match cv_in
                            .get_update_local_filename()
                            .which()
                        {
                            Ok(m::update_tab_status_front::update_local_filename::Filepath(
                                Ok(filepath),
                            )) => Some(PathBuf::from(filepath)),
                            Err(e) => {
                                error!("{}", e);
                                None
                            }
                            _ => None,
                        };
                        let fileio_local_filepath = match cv_in.get_fileio_local_filepath().which()
                        {
                            Ok(m::update_tab_status_front::fileio_local_filepath::Filepath(
                                Ok(filepath),
                            )) => Some(PathBuf::from(filepath)),
                            Err(e) => {
                                error!("{}", e);
                                None
                            }
                            _ => None,
                        };
                        let fileio_destination_filepath = match cv_in
                            .get_fileio_destination_filepath()
                            .which()
                        {
                            Ok(
                                m::update_tab_status_front::fileio_destination_filepath::Filepath(
                                    Ok(filepath),
                                ),
                            ) => Some(PathBuf::from(filepath)),
                            Err(e) => {
                                error!("{}", e);
                                None
                            }
                            _ => None,
                        };

                        let check_for_updates = cv_in.get_check_for_updates();
                        if let Err(err) = update_tab_sender.send(Some(UpdateTabUpdate {
                            download_latest_firmware,
                            update_firmware,
                            send_file_to_device,
                            firmware_directory,
                            firmware_local_filepath,
                            firmware_local_filename,
                            fileio_local_filepath,
                            fileio_destination_filepath,
                            serial_prompt_confirm,
                            check_for_updates,
                        })) {
                            error!("{}", err);
                        }
                    }
                }
                m::message::AutoSurveyRequest(Ok(_)) => {
                    shared_state.lock().auto_survey_data.requested = true;
                }
                m::message::SettingsRefreshRequest(Ok(_)) => {
                    shared_state.set_settings_refresh(true);
                }
                m::message::SettingsResetRequest(Ok(_)) => {
                    shared_state.set_settings_reset(true);
                }
                m::message::SettingsSaveRequest(Ok(_)) => {
                    shared_state.set_settings_save(true);
                }
                m::message::SettingsExportRequest(Ok(path)) => {
                    let path = path.get_path().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.set_export_settings(Some(PathBuf::from(path)));
                }
                m::message::SettingsImportRequest(Ok(path)) => {
                    let path = path.get_path().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    shared_state.set_import_settings(Some(PathBuf::from(path)));
                }
                m::message::SettingsWriteRequest(Ok(req)) => {
                    let group = req.get_group().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let name = req.get_name().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let value = req.get_value().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                    let req = SaveRequest {
                        group: group.to_string(),
                        name: name.to_string(),
                        value: value.to_string(),
                    };
                    shared_state.set_write_setting(Some(req));
                }
                m::message::AdvancedSystemMonitorStatusFront(Ok(_)) => {
                    shared_state.set_device_reboot(true);
                }
                m::message::AdvancedNetworkingStatusFront(Ok(cv_in)) => {
                    let refresh = cv_in.get_refresh();
                    let start = cv_in.get_start();
                    let stop = cv_in.get_stop();
                    let ip_address = match cv_in.get_ipv4_address().which() {
                        Ok(m::advanced_networking_status_front::ipv4_address::Address(Ok(
                            address,
                        ))) => Some(String::from(address)),
                        Err(e) => {
                            error!("{}", e);
                            None
                        }
                        _ => None,
                    };
                    let port: Option<u16> = match cv_in.get_port().which() {
                        Ok(m::advanced_networking_status_front::port::Port(port)) => Some(port),
                        Err(e) => {
                            error!("{}", e);
                            None
                        }
                        _ => None,
                    };
                    let all_messages: Option<bool> = match cv_in.get_all_messages().which() {
                        Ok(m::advanced_networking_status_front::all_messages::Toggle(toggle)) => {
                            Some(toggle)
                        }
                        Err(e) => {
                            error!("{}", e);
                            None
                        }
                        _ => None,
                    };
                    shared_state.set_advanced_networking_update(AdvancedNetworkingState {
                        refresh,
                        start,
                        stop,
                        all_messages,
                        ip_address,
                        port,
                    })
                }
                m::message::ConfirmInsChange(Ok(_)) => {
                    shared_state.set_settings_confirm_ins_change(true);
                }
                _ => {
                    error!("unknown message from front-end");
                }
            }
        }
        debug!("client recv loop shutdown");
        client_sender.set_connected(false);
    });
}
