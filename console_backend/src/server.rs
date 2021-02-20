#![allow(dead_code)]
#![allow(unused_imports)]
use capnp::message::Builder;
use capnp::serialize;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use std::fs;
use std::io::{BufReader, Cursor};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

use crate::console_backend_capnp as m;
use crate::process_messages::process_messages;

/// The backend server
#[pyclass]
struct Server {
    client_recv: Option<mpsc::Receiver<Vec<u8>>>,
}

#[pyclass]
struct ServerEndpoint {
    server_send: Option<mpsc::Sender<Vec<u8>>>,
}


#[cfg(not(any(test, feature = "criterion_bench")))]
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

#[cfg(not(any(test, feature = "criterion_bench")))]
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
        let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
        let (server_send, server_recv) = mpsc::channel::<Vec<u8>>();
        self.client_recv = Some(client_recv);
        let server_endpoint = ServerEndpoint {
            server_send: Some(server_send),
        };
        let client_send_clone = client_send;
        thread::spawn(move || loop {
            let buf = server_recv.recv();
            let client_send_clone = client_send_clone.clone();
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
                        let host = conn_req.get_host().unwrap();
                        let port = conn_req.get_port();
                        println!("connect request, host: {}, port: {}", host, port);
                        let host_port = format!("{}:{}", host, port);
                        thread::spawn(move || {
                            if let Ok(stream) = TcpStream::connect(host_port) {
                                println!("Connected to the server!");
                                let messages = sbp::iter_messages(stream);
                                process_messages(messages, client_send_clone);
                            } else {
                                println!("Couldn't connect to server...");
                            }
                        });
                    }
                    Ok(m::message::Which::FileinRequest(Ok(file_in))) => {
                        let filename = file_in.get_filename().unwrap();
                        let filename = filename.to_string();
                        println!("{}", filename);
                        thread::spawn(move || {
                            if let Ok(stream) = fs::File::open(filename) {
                                println!("Opened file successfully!");

                                let messages = sbp::iter_messages(stream);
                                process_messages(messages, client_send_clone);
                            } else {
                                println!("Couldn't open file...");
                            }
                        });
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

#[cfg(not(any(test, feature = "criterion_bench")))]
#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<ServerEndpoint>()?;
    Ok(())
}
