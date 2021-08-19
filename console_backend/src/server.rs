use cfg_if::cfg_if;
use log::{error, info};

#[cfg(feature = "python")]
use pyo3::{exceptions, prelude::*, types::{PyBytes, PyInt}, IntoPy};

use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
    str::FromStr,
    thread, time,
};

use crossbeam::channel;

use crate::cli_options::*;
use crate::connection::ConnectionState;
use crate::errors::*;
use crate::log_panel::{setup_logging, LogLevel};
use crate::ipc::Message;
use crate::output::{CsvLogging, SbpLogging};
use crate::types::{IPC_KIND_CAPNP, ClientSender, FlowControl, RealtimeDelay, SharedState};
use crate::utils::{refresh_loggingbar, refresh_navbar};

/// The backend server
#[cfg_attr(feature = "python", pyclass)]
pub struct Server {
    client_recv: Option<channel::Receiver<(u8, Vec<u8>)>>,
    client_sender: Option<ClientSender>,
}

#[cfg_attr(feature = "python", pyclass)]
pub struct ServerEndpoint {
    server_send: Option<channel::Sender<(u8, Vec<u8>)>>,
}

cfg_if! {
    if #[cfg(feature = "python")] {
        #[pymethods]
        impl ServerEndpoint {
            #[new]
            pub fn __new__() -> Self {
                ServerEndpoint { server_send: None }
            }

            #[text_signature = "($self, /)"]
            pub fn shutdown(&mut self) -> PyResult<()> {
                if let Some(server_send) = self.server_send.take() {
                    drop(server_send);
                    Ok(())
                } else {
                    Err(exceptions::PyRuntimeError::new_err(
                        "no server send endpoint",
                    ))
                }
            }

            #[text_signature = "($self, kind, bytes, /)"]
            pub fn send_message(&mut self, kind: &PyInt, bytes: &PyBytes) -> PyResult<()> {
                let kind: u8 = kind.extract().unwrap();
                let bytes: Vec<u8> = bytes.extract().unwrap();
                if let Some(server_send) = &self.server_send {
                    server_send
                        .send((kind, bytes))
                        .map_err(|e| exceptions::PyRuntimeError::new_err(format!("{}", e)))
                } else {
                    Err(exceptions::PyRuntimeError::new_err(
                        "no server send endpoint",
                    ))
                }
            }
        }
    } else {
        impl ServerEndpoint {
            pub fn new() -> Self {
                ServerEndpoint { server_send: None }
            }
            pub fn shutdown(&mut self) -> crate::types::Result<()> {
                if let Some(server_send) = self.server_send.take() {
                    drop(server_send);
                    Ok(())
                } else {
                    Err("no server send endpoint".into())
                }
                if let Some(server_send) = &self.server_send {
                    server_send
                        .send((kind, byte_vec))
                        .map_err(crate::types::Error::from)
                } else {
                    Err("no server send endpoint".into())
                }
            }
        }
    }
}

/// Start connections based on CLI options.
///
/// # Parameters
/// - `opt`: CLI Options to start specific connection type.
/// - `connection_state`: The Server state to start a specific connection.
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for validating another connection is not already running.
fn handle_cli(opt: CliOptions, connection_state: &ConnectionState, shared_state: SharedState) {
    if let Some(opt_input) = opt.input {
        match opt_input {
            Input::Tcp { host, port } => {
                connection_state.connect_to_host(host, port);
            }
            Input::File { file_in } => {
                let filename = file_in.display().to_string();
                connection_state.connect_to_file(filename, RealtimeDelay::On, opt.exit_after);
            }
            Input::Serial {
                serialport,
                baudrate,
                flow_control,
            } => {
                let serialport = serialport.display().to_string();
                connection_state.connect_to_serial(serialport, baudrate, flow_control);
            }
        }
    }
    if let Some(folder) = opt.dirname {
        shared_state.set_logging_directory(PathBuf::from(folder));
    }
    let log_level = if let Some(log_level_) = opt.log_level {
        (*log_level_).clone()
    } else {
        LogLevel::INFO
    };
    shared_state.set_log_level(log_level);
    let mut shared_data = shared_state.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
    (*shared_data).logging_bar.csv_logging = CsvLogging::from(opt.csv_log);
    if let Some(sbp_log) = opt.sbp_log {
        (*shared_data).logging_bar.sbp_logging =
            SbpLogging::from_str(&sbp_log.to_string()).expect(CONVERT_TO_STR_FAILURE);
    }
    log::logger().flush();
}

cfg_if! {
    if #[cfg(feature = "python")] {
        #[pymethods]
        impl Server {

            #[new]
            pub fn __new__() -> Self {
                Server {
                    client_recv: None,
                    client_sender: None,
                }
            }

            #[text_signature = "($self, /)"]
            pub fn fetch_message(&mut self, py: Python) -> Option<PyObject> {
                let result = py.allow_threads(move || loop {
                    if let Some(client_recv) = &self.client_recv {
                        match client_recv.recv_timeout(time::Duration::from_millis(1)) {
                            Ok((kind, buf)) => break Some((kind, buf)),
                            Err(err) => {
                                use crossbeam::channel::RecvTimeoutError;
                                if matches!(err, RecvTimeoutError::Timeout) {
                                    if self.client_sender.as_ref().unwrap().connected.get() {
                                        continue;
                                    } else {
                                        eprintln!("shutting down");
                                        break None;
                                    }
                                } else {
                                    eprintln!("client recv disconnected");
                                    break None;
                                }
                            }
                        }
                    } else {
                        eprintln!("no client receive endpoint");
                        break None;
                    }
                });
                result.map(|(kind, buf)| (kind, PyBytes::new(py, &buf)).into_py(py))
            }

            #[text_signature = "($self, /)"]
            pub fn start(&mut self) -> PyResult<ServerEndpoint> {
                let (client_send_, client_recv) = channel::unbounded::<(u8, Vec<u8>)>();
                let (server_send, server_recv) = channel::unbounded::<(u8, Vec<u8>)>();
                let client_send = ClientSender::new(client_send_);
                self.client_recv = Some(client_recv);
                self.client_sender = Some(client_send.clone());
                let server_endpoint = ServerEndpoint {
                    server_send: Some(server_send),
                };
                setup_logging(client_send.clone());
                let opt = CliOptions::from_filtered_cli();
                let shared_state = SharedState::new();
                let connection_state = ConnectionState::new(client_send.clone(), shared_state.clone());
                // Handle CLI Opts.
                handle_cli(opt, &connection_state, shared_state.clone());
                refresh_navbar(&mut client_send.clone(), shared_state.clone());
                refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
                backend_recv_thread(connection_state, client_send, server_recv, shared_state);
                Ok(server_endpoint)
            }
        }
    } else {
        impl Server {
            pub fn new() -> Self {
                Server {
                    client_recv: None,
                    client_sender: None,
                }
            }
            pub fn fetch_message(&self) -> crate::types::Result<Option<(u8, Vec<u8>)>> {
                if let Some(client_recv) = &self.client_recv {
                    match client_recv.recv_timeout(time::Duration::from_millis(1)) {
                        Ok(buf) => Ok(Some(buf)),
                        Err(err) => {
                            use crossbeam::channel::RecvTimeoutError;
                            if matches!(err, RecvTimeoutError::Timeout) {
                                if self.client_sender.as_ref().unwrap().connected.get() {
                                    Ok(None)
                                } else {
                                    Err("shutting down".into())
                                }
                            } else {
                                Err("client recv disconnected".into())
                            }
                        }
                    }
                } else {
                    Err("no client receive endpoint".into())
                }
            }
            pub fn start(&mut self) -> ServerEndpoint {
                let (client_send_, client_recv) = channel::unbounded::<(u8, Vec<u8>)>();
                let (server_send, server_recv) = channel::unbounded::<(u8, Vec<u8>)>();
                let client_send = ClientSender::new(client_send_);
                self.client_recv = Some(client_recv);
                self.client_sender = Some(client_send.clone());
                let server_endpoint = ServerEndpoint {
                    server_send: Some(server_send),
                };
                setup_logging(client_send.clone());
                let opt = CliOptions::from_filtered_cli();
                let shared_state = SharedState::new();
                let connection_state = ConnectionState::new(client_send.clone(), shared_state.clone());
                // Handle CLI Opts.
                handle_cli(opt, &connection_state, shared_state.clone());
                refresh_navbar(&mut client_send.clone(), shared_state.clone());
                refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
                backend_recv_thread(connection_state, client_send, server_recv, shared_state);
                server_endpoint
            }
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<ServerEndpoint>()?;
    Ok(())
}

fn backend_recv_thread(
    connection_state: ConnectionState,
    client_send: ClientSender,
    server_recv: channel::Receiver<(u8, Vec<u8>)>,
    shared_state: SharedState,
) {
    thread::spawn(move || {
        let client_send_clone = client_send.clone();
        loop {
            log::logger().flush();
            let result = server_recv.recv();
            if let Ok((kind, buf)) = result {
                eprintln!("kind: {}, buf length: {}", kind, buf.len());
                if kind == IPC_KIND_CAPNP {
                    eprintln!("skipping capnproto message");
                    continue;
                }
                let mut buf_reader = BufReader::new(Cursor::new(&buf));
                let value: serde_json::Value = match serde_cbor::from_reader(&mut buf_reader) {
                    Ok(value) => value,
                    Err(error) => {
                        error!("error reading message: {}", error);
                        continue;
                    }
                };
                let message: Message = match serde_json::from_value(value) {
                    Ok(value) => value,
                    Err(error) => {
                        error!("error parsing message: {}", error);
                        continue;
                    }
                };
                eprintln!("{:#?}", message);
                let shared_state_clone = shared_state.clone();
                match message {
                    Message::SerialRefreshRequest(_) => {
                        refresh_navbar(&mut client_send_clone.clone(), shared_state_clone);
                    }
                    Message::DisconnectRequest(_) => {
                        connection_state.disconnect(client_send_clone.clone());
                    }
                    Message::FileRequest(req) => {
                        let filename = req.filename;
                        connection_state.connect_to_file(
                            filename,
                            RealtimeDelay::On,
                            /*close_when_done*/ false,
                        );
                    }
                    Message::PauseRequest(_) => {
                        if shared_state_clone.is_paused() {
                            shared_state_clone.set_paused(false);
                        } else {
                            shared_state_clone.set_paused(true);
                        }
                    }
                    Message::TcpRequest(req) => {
                        connection_state.connect_to_host(req.host, req.port);
                    }
                    Message::SerialRequest(req) => {
                        let device = req.device;
                        let baudrate = req.baudrate;
                        let flow = req.flow_control;
                        let flow = FlowControl::from_str(&flow).unwrap();
                        connection_state.connect_to_serial(device, baudrate, flow);
                    }
                    Message::TrackingSignalsStatusFront(cv_in) => {
                        let check_visibility = cv_in.tracking_signals_check_visibility;
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone
                                .lock()
                                .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                            (*shared_data).tracking_tab.signals_tab.check_visibility =
                                check_visibility;
                        }
                    }
                    Message::LoggingBarFront(cv_in) => {
                        let directory = PathBuf::from(cv_in.directory);
                        shared_state.set_logging_directory(directory);
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).logging_bar.csv_logging = CsvLogging::from(cv_in.csv_logging);
                        let sbp_logging = cv_in.sbp_logging;
                        (*shared_data).logging_bar.sbp_logging =
                            SbpLogging::from_str(&sbp_logging).expect(CONVERT_TO_STR_FAILURE);
                    }
                    Message::LogLevelFront(cv_in) => {
                        let shared_state_clone = shared_state.clone();
                        let log_level = cv_in.log_level;
                        let log_level =
                            LogLevel::from_str(&log_level).expect(CONVERT_TO_STR_FAILURE);
                        info!("Log Level: {}", log_level);
                        shared_state_clone.set_log_level(log_level);
                        refresh_navbar(&mut client_send.clone(), shared_state.clone());
                    }
                    Message::SolutionVelocityStatusFront(cv_in) => {
                        let unit = cv_in.solution_velocity_units;
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone
                                .lock()
                                .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                            (*shared_data).solution_tab.velocity_tab.unit = unit;
                        }
                    }
                    Message::SolutionPositionStatusUnitFront(cv_in) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        let unit = cv_in.solution_position_unit;
                        (*shared_data).solution_tab.position_tab.unit = unit;
                    }
                    Message::SolutionPositionStatusButtonFront(cv_in) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).solution_tab.position_tab.clear =
                            cv_in.solution_position_clear;
                        (*shared_data).solution_tab.position_tab.pause =
                            cv_in.solution_position_pause;
                    }
                    Message::BaselinePlotStatusButtonFront(cv_in) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).baseline_tab.clear = cv_in.clear;
                        (*shared_data).baseline_tab.pause = cv_in.pause;
                        (*shared_data).baseline_tab.reset = cv_in.reset_filters;
                    }
                    Message::AdvancedSpectrumAnalyzerStatusFront(cv_in) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).advanced_spectrum_analyzer_tab.channel_idx =
                            cv_in.channel;
                    }
                    _ => {
                        error!("unknown message from front-end");
                    }
                }
            } else {
                break;
            }
        }
        eprintln!("client recv loop shutdown");
        client_send_clone.connected.set(false);
    });
}

