use capnp::serialize;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use async_logger_log::Logger;

use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
    str::FromStr,
    sync::mpsc,
    thread,
};

use crate::cli_options::*;
use crate::console_backend_capnp as m;
// use crate::common_constants::;
use crate::constants::LOG_WRITER_BUFFER_MESSAGE_COUNT;
use crate::errors::*;
use crate::log_panel::{splitable_log_formatter, LogPanelWriter};
use crate::output::{CsvLogging, SbpLogging};
use crate::types::{ClientSender, FlowControl, ServerState, SharedState};
use crate::utils::{refresh_loggingbar, refresh_navbar};

/// The backend server
#[pyclass]
struct Server {
    client_recv: Option<mpsc::Receiver<Vec<u8>>>,
}

#[pyclass]
struct ServerEndpoint {
    server_send: Option<mpsc::Sender<Vec<u8>>>,
}

#[pymethods]
impl ServerEndpoint {
    #[new]
    pub fn __new__() -> Self {
        ServerEndpoint { server_send: None }
    }

    #[text_signature = "($self, bytes, /)"]
    pub fn send_message(&mut self, bytes: &PyBytes) -> PyResult<()> {
        let byte_vec: Vec<u8> = bytes.extract().unwrap();
        if let Some(server_send) = &self.server_send {
            server_send
                .send(byte_vec)
                .map_err(|e| exceptions::PyRuntimeError::new_err(format!("{}", e)))
        } else {
            Err(exceptions::PyRuntimeError::new_err(
                "no server send endpoint",
            ))
        }
    }
}

/// Start connections based on CLI options.
///
/// # Parameters
/// - `opt`: CLI Options to start specific connection type.
/// - `server_state`: The Server state to start a specific connection.
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for validating another connection is not already running.
fn handle_cli(
    opt: CliOptions,
    server_state: ServerState,
    client_send: ClientSender,
    shared_state: SharedState,
) {
    if let Some(opt_input) = opt.input {
        match opt_input {
            Input::Tcp { host, port } => {
                server_state.connect_to_host(client_send, shared_state.clone(), host, port);
            }
            Input::File { file_in } => {
                let filename = file_in.display().to_string();
                server_state.connect_to_file(
                    client_send,
                    shared_state.clone(),
                    filename,
                    opt.exit_after,
                );
            }
            Input::Serial {
                serialport,
                baudrate,
                flow_control,
            } => {
                let serialport = serialport.display().to_string();
                server_state.connect_to_serial(
                    client_send,
                    shared_state.clone(),
                    serialport,
                    baudrate,
                    flow_control,
                );
            }
        }
    }
    if let Some(folder) = opt.dirname {
        shared_state.set_logging_directory(PathBuf::from(folder));
    }
    // let shared_state_clone = shared_state.clone();
    let mut shared_data = shared_state.lock().expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
    (*shared_data).logging_bar.csv_logging = CsvLogging::from(opt.csv_log);
    if let Some(sbp_log) = opt.sbp_log {
        (*shared_data).logging_bar.sbp_logging =
            SbpLogging::from_str(&sbp_log.to_string()).expect(CONVERT_TO_STR_FAILURE);
    }
}

#[pymethods]
impl Server {
    #[new]
    pub fn __new__() -> Self {
        Server { client_recv: None }
    }

    #[text_signature = "($self, /)"]
    pub fn fetch_message(&mut self, py: Python) -> Option<PyObject> {
        let result = py.allow_threads(move || {
            let client_recv = self.client_recv.as_ref();
            if let Some(client_recv) = client_recv {
                let buf = client_recv.recv();
                if let Ok(buf) = buf {
                    Some(buf)
                } else {
                    println!("error receiving message: {:?}", buf);
                    None
                }
            } else {
                println!("no client receive endpoint");
                None
            }
        });
        result.map(|result| PyBytes::new(py, &result).into())
    }

    #[text_signature = "($self, /)"]
    pub fn start(&mut self) -> PyResult<ServerEndpoint> {
        let opt = CliOptions::from_filtered_cli();
        let (client_send_, client_recv) = mpsc::channel::<Vec<u8>>();
        let (server_send, server_recv) = mpsc::channel::<Vec<u8>>();
        self.client_recv = Some(client_recv);
        let server_endpoint = ServerEndpoint {
            server_send: Some(server_send),
        };
        let client_send = ClientSender {
            inner: client_send_,
        };
        let shared_state = SharedState::new();
        let server_state = ServerState::new();
        refresh_navbar(&mut client_send.clone(), shared_state.clone());
        refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
        let logger = Logger::builder()
            .buf_size(LOG_WRITER_BUFFER_MESSAGE_COUNT)
            .formatter(splitable_log_formatter)
            .writer(Box::new(LogPanelWriter::new(client_send.clone())))
            .build()
            .unwrap();

        log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
        log::set_max_level(log::LevelFilter::Info);

        // Handle CLI Opts.
        handle_cli(
            opt,
            server_state.clone(),
            client_send.clone(),
            shared_state.clone(),
        );
        thread::spawn(move || loop {
            let buf = server_recv.recv();
            if let Ok(buf) = buf {
                let mut buf_reader = BufReader::new(Cursor::new(buf));
                let message_reader = serialize::read_message(
                    &mut buf_reader,
                    ::capnp::message::ReaderOptions::new(),
                )
                .unwrap();
                let message = message_reader
                    .get_root::<m::message::Reader>()
                    .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                let message = match message.which() {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("error reading message: {}", e);
                        continue;
                    }
                };
                match message {
                    m::message::ConnectRequest(Ok(conn_req)) => {
                        let request = conn_req
                            .get_request()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        let request = request
                            .get_as::<m::message::Reader>()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        let request = match request.which() {
                            Ok(msg) => msg,
                            Err(e) => {
                                eprintln!("error reading message: {}", e);
                                continue;
                            }
                        };
                        let server_state_clone = server_state.clone();
                        let shared_state_clone = shared_state.clone();
                        let client_send_clone = client_send.clone();
                        match request {
                            m::message::SerialRefreshRequest(Ok(_)) => {
                                refresh_navbar(&mut client_send_clone.clone(), shared_state_clone);
                            }
                            m::message::DisconnectRequest(Ok(_)) => {
                                shared_state_clone.set_running(false, client_send_clone.clone());
                                server_state_clone.connection_join();
                                println!("Disconnected successfully.");
                            }
                            m::message::FileRequest(Ok(req)) => {
                                let filename = req
                                    .get_filename()
                                    .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                                let filename = filename.to_string();
                                server_state_clone.connect_to_file(
                                    client_send_clone,
                                    shared_state_clone,
                                    filename,
                                    /*close_when_done = */ false,
                                );
                            }
                            m::message::PauseRequest(Ok(_)) => {
                                if shared_state_clone.is_paused() {
                                    shared_state_clone.set_paused(false);
                                } else {
                                    shared_state_clone.set_paused(true);
                                }
                            }
                            m::message::TcpRequest(Ok(req)) => {
                                let host =
                                    req.get_host().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                                let port = req.get_port();
                                server_state_clone.connect_to_host(
                                    client_send_clone,
                                    shared_state_clone,
                                    host.to_string(),
                                    port,
                                );
                            }
                            m::message::SerialRequest(Ok(req)) => {
                                let device =
                                    req.get_device().expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                                let device = device.to_string();
                                let baudrate = req.get_baudrate();
                                let flow = req.get_flow_control().unwrap();
                                let flow = FlowControl::from_str(flow).unwrap();
                                server_state_clone.connect_to_serial(
                                    client_send_clone,
                                    shared_state_clone,
                                    device,
                                    baudrate,
                                    flow,
                                );
                            }
                            _ => println!("err"),
                        }
                    }
                    m::message::TrackingSignalsStatusFront(Ok(cv_in)) => {
                        let check_visibility = cv_in
                            .get_tracking_signals_check_visibility()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        let check_visibility: Vec<String> = check_visibility
                            .iter()
                            .map(|x| String::from(x.unwrap()))
                            .collect();
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone
                                .lock()
                                .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                            (*shared_data).tracking_tab.signals_tab.check_visibility =
                                check_visibility;
                        }
                    }
                    m::message::LoggingBarFront(Ok(cv_in)) => {
                        let directory = cv_in
                            .get_directory()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        shared_state.set_logging_directory(PathBuf::from(directory));
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).logging_bar.csv_logging =
                            CsvLogging::from(cv_in.get_csv_logging());
                        let sbp_logging = cv_in
                            .get_sbp_logging()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        (*shared_data).logging_bar.sbp_logging =
                            SbpLogging::from_str(sbp_logging).expect(CONVERT_TO_STR_FAILURE);
                    }
                    m::message::SolutionVelocityStatusFront(Ok(cv_in)) => {
                        let unit = cv_in
                            .get_solution_velocity_unit()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone
                                .lock()
                                .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                            (*shared_data).solution_tab.velocity_tab.unit = unit.to_string();
                        }
                    }
                    m::message::SolutionPositionStatusUnitFront(Ok(cv_in)) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        let unit = cv_in
                            .get_solution_position_unit()
                            .expect(CAP_N_PROTO_DESERIALIZATION_FAILURE);
                        (*shared_data).solution_tab.position_tab.unit = unit.to_string();
                    }
                    m::message::SolutionPositionStatusButtonFront(Ok(cv_in)) => {
                        let shared_state_clone = shared_state.clone();
                        let mut shared_data = shared_state_clone
                            .lock()
                            .expect(SHARED_STATE_LOCK_MUTEX_FAILURE);
                        (*shared_data).solution_tab.position_tab.zoom =
                            cv_in.get_solution_position_zoom();
                        (*shared_data).solution_tab.position_tab.center =
                            cv_in.get_solution_position_center();
                        (*shared_data).solution_tab.position_tab.clear =
                            cv_in.get_solution_position_clear();
                        (*shared_data).solution_tab.position_tab.pause =
                            cv_in.get_solution_position_pause();
                    }
                    _ => {
                        eprintln!("unknown message from front-end");
                    }
                }
            } else {
                println!("error: {:?}", buf);
                break;
            }
        });
        Ok(server_endpoint)
    }
}

#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<ServerEndpoint>()?;
    Ok(())
}
