#![allow(clippy::borrow_deref_ref)] // Waiting on this to merge: https://github.com/rust-lang/rust-clippy/issues/8971
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

pub fn attach_console() {
    if std::env::var("SWIFTNAV_CONSOLE_DEBUG").is_ok() {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::System::Console::AttachConsole;
            unsafe {
                AttachConsole(u32::MAX).as_bool();
            }
        }
    }
}

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

    #[pyo3(text_signature = "($self, bytes, /)")]
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

    #[pyo3(text_signature = "($self, bytes, /)")]
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

    #[pyo3(text_signature = "($self, /)")]
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

    #[pyo3(text_signature = "($self, /)")]
    pub fn start(&mut self) -> PyResult<ServerEndpoint> {
        attach_console();
        let (client_send, client_recv) = channel::unbounded();
        let (server_send, server_recv) = channel::unbounded();
        let client_send = ChannelSender::boxed(client_send);
        self.client_recv = Some(client_recv);
        self.client_sender = Some(client_send.clone());
        let server_endpoint = ServerEndpoint {
            server_send: Some(server_send),
        };
        let shared_state = SharedState::new();
        let opt = CliOptions::from_filtered_cli();
        let conn_manager = ConnectionManager::new(client_send.clone(), shared_state.clone());
        // Handle CLI Opts.
        handle_cli(opt, &conn_manager, shared_state.clone());
        setup_logging(client_send.clone(), shared_state.clone());
        refresh_connection_frontend(&client_send, &shared_state);
        refresh_loggingbar(&client_send, &shared_state);
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
