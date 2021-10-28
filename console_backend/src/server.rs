use crossbeam::channel;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use std::time;

use crate::cli_options::*;
use crate::connection::ConnectionManager;

use crate::client_sender::{BoxedClientSender, ChannelSender};
use crate::log_panel::setup_logging;
use crate::server_recv_thread::server_recv_thread;
use crate::shared_state::SharedState;
use crate::utils::{refresh_connection_frontend, refresh_loggingbar};

/// The backend server
#[pyclass]
struct Server {
    client_recv: Option<channel::Receiver<Vec<u8>>>,
    client_sender: Option<BoxedClientSender>,
}

#[pyclass]
struct ServerEndpoint {
    server_send: Option<channel::Sender<Vec<u8>>>,
}

#[pymethods]
impl ServerEndpoint {
    #[new]
    pub fn __new__() -> Self {
        ServerEndpoint { server_send: None }
    }

    #[text_signature = "($self, bytes, /)"]
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
                    Ok(buf) => break Some(buf),
                    Err(err) => {
                        use channel::RecvTimeoutError;
                        if matches!(err, RecvTimeoutError::Timeout) {
                            if self.client_sender.as_ref().unwrap().connected() {
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
        result.map(|result| PyBytes::new(py, &result).into())
    }

    #[text_signature = "($self, /)"]
    pub fn start(&mut self) -> PyResult<ServerEndpoint> {
        let (client_send, client_recv) = channel::unbounded();
        let (server_send, server_recv) = channel::unbounded();
        let client_send = ChannelSender::boxed(client_send);
        self.client_recv = Some(client_recv);
        self.client_sender = Some(client_send.clone());
        let server_endpoint = ServerEndpoint {
            server_send: Some(server_send),
        };
        let shared_state = SharedState::new();
        setup_logging(client_send.clone(), shared_state.clone());
        let opt = CliOptions::from_filtered_cli();
        if let Some(ref path) = opt.settings_yaml {
            sbp_settings::settings::load_from_path(path).expect("failed to load settings");
        }
        let conn_manager = ConnectionManager::new(client_send.clone(), shared_state.clone());
        // Handle CLI Opts.
        handle_cli(opt, &conn_manager, shared_state.clone());
        refresh_connection_frontend(&mut client_send.clone(), shared_state.clone());
        refresh_loggingbar(&mut client_send.clone(), shared_state.clone());
        server_recv_thread(conn_manager, client_send, server_recv, shared_state);
        Ok(server_endpoint)
    }
}

#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<ServerEndpoint>()?;
    Ok(())
}
