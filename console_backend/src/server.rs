#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg(not(feature = "benches"))]
use capnp::message::Builder;
use capnp::serialize;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use std::fs;
use std::io::{BufReader, Cursor};
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::console_backend_capnp as m;
use crate::process_messages::process_messages;
use crate::types::{ClientSender, MessageSender, SharedState, VelocityUnits};

const CLOSE: &str = "CLOSE";

/// The backend server
#[pyclass]
struct Server {
    client_recv: Option<mpsc::Receiver<Vec<u8>>>,
}

#[pyclass]
struct ServerEndpoint {
    server_send: Option<mpsc::Sender<Vec<u8>>>,
}

#[cfg(not(test))]
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

/// Helper function for attempting to open a file and process SBP messages from it.
///
/// # Parameters
/// - `client_send`: Client Sender channel for bidirectional communication between front/backend.
/// - `shared_state`: The shared state for validating another connection is not already running.
/// - `filename`: The path to the filename to be read for SBP messages.
pub fn connect_to_file(
    client_send: &mut ClientSender,
    shared_state: SharedState,
    filename: String,
) {
    if let Ok(stream) = fs::File::open(filename) {
        println!("Opened file successfully!");
        let shared_state_clone_ = shared_state.clone();
        let messages = sbp::iter_messages(stream);
        process_messages(messages, shared_state_clone_, client_send.clone());
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();
        let mut status = msg.init_status();
        status.set_text(CLOSE);
        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        client_send.send_data(msg_bytes);
    } else {
        println!("Couldn't open file...");
    }
    let shared_state_clone_ = shared_state;
    shared_state_clone_.server_set_connected(false);
}

/// Helper function for attempting to open a tcp connection and process SBP messages from it.
///
/// # Parameters
/// - `client_send`: Client Sender channel for bidirectional communication between front/backend.
/// - `shared_state`: The shared state for validating another connection is not already running.
/// - `host_port`: The host and port combined as a string to be opend as a TCP stream.
pub fn connect_to_host(client_send: ClientSender, shared_state: SharedState, host_port: String) {
    let shared_state_clone = shared_state.clone();
    if let Ok(stream) = TcpStream::connect(host_port) {
        println!("Connected to the server!");
        let messages = sbp::iter_messages(stream);
        process_messages(messages, shared_state_clone, client_send);
    } else {
        println!("Couldn't connect to server...");
    }
    let shared_state_clone = shared_state;
    shared_state_clone.server_set_connected(false);
}

#[cfg(not(test))]
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
        if let Some(result) = result {
            Some(PyBytes::new(py, &result).into())
        } else {
            None
        }
    }

    #[text_signature = "($self, /)"]
    pub fn start(&mut self) -> PyResult<ServerEndpoint> {
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
        thread::spawn(move || loop {
            let buf = server_recv.recv();
            if let Ok(buf) = buf {
                let mut buf_reader = BufReader::new(Cursor::new(buf));
                let message_reader = serialize::read_message(
                    &mut buf_reader,
                    ::capnp::message::ReaderOptions::new(),
                )
                .unwrap();
                let message = message_reader.get_root::<m::message::Reader>().unwrap();
                match message.which() {
                    Ok(m::message::Which::ConnectRequest(Ok(conn_req))) => {
                        let shared_state_clone = shared_state.clone();
                        if shared_state_clone.server_is_connected() {
                            println!("Already connected.");
                            continue;
                        } else {
                            shared_state_clone.server_set_connected(true);
                        }
                        let host = conn_req.get_host().unwrap();
                        let port = conn_req.get_port();
                        println!("connect request, host: {}, port: {}", host, port);
                        let host_port = format!("{}:{}", host, port);
                        let client_send_clone = client_send.clone();
                        let shared_state_clone = shared_state.clone();
                        thread::spawn(move || {
                            let shared_state_clone_ = shared_state_clone.clone();
                            connect_to_host(client_send_clone, shared_state_clone_, host_port);
                        });
                    }
                    Ok(m::message::Which::FileinRequest(Ok(file_in))) => {
                        let shared_state_clone = shared_state.clone();
                        if shared_state_clone.server_is_connected() {
                            println!("Already connected.");
                            continue;
                        } else {
                            shared_state_clone.server_set_connected(true);
                        }
                        let filename = file_in.get_filename().unwrap();
                        let filename = filename.to_string();
                        println!("{}", filename);
                        let shared_state_clone = shared_state.clone();
                        let mut client_send_clone = client_send.clone();
                        thread::spawn(move || {
                            let shared_state_clone_ = shared_state_clone.clone();
                            connect_to_file(&mut client_send_clone, shared_state_clone_, filename);
                        });
                    }
                    Ok(m::message::Which::TrackingSignalsStatusFront(Ok(cv_in))) => {
                        let check_visibility =
                            cv_in.get_tracking_signals_check_visibility().unwrap();
                        let check_visibility: Vec<String> = check_visibility
                            .iter()
                            .map(|x| String::from(x.unwrap()))
                            .collect();
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone.lock().unwrap();
                            (*shared_data).tracking_tab.signals_tab.check_visibility =
                                check_visibility;
                        }
                    }
                    Ok(m::message::Which::SolutionVelocityStatusFront(Ok(cv_in))) => {
                        let unit = cv_in.get_solution_velocity_unit().unwrap();
                        let shared_state_clone = shared_state.clone();
                        {
                            let mut shared_data = shared_state_clone.lock().unwrap();
                            (*shared_data).solution_tab.velocity_tab.unit = unit.to_string();
                        }
                    }
                    Ok(_) => {
                        println!("something else");
                    }
                    Err(err) => {
                        println!("error: {}", err);
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
