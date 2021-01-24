use std::thread;
use std::sync::mpsc;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions;

use capnp::message::Builder;
use capnp::serialize;

use crate::console_backend_capnp as m;

/// The backend server
#[pyclass]
struct Server {
    client_recv: Option<mpsc::Receiver<Vec<u8>>>,
    server_send: Option<mpsc::Sender<Vec<u8>>>,
}

#[pymethods]
impl Server {
    #[new]
    pub fn __new__() -> Self {
        Server {
            client_recv: None,
            server_send: None,
        }
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
                    println!("no message, or error: {:?}", buf);
                    None
                }
            } else {
                println!("no client recv");
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
    pub fn start(&mut self) -> PyResult<()> {
        let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
        let (server_send, server_recv) = mpsc::channel::<Vec<u8>>();
        self.client_recv = Some(client_recv);
        self.server_send = Some(server_send);
        thread::spawn(move || {
            let mut builder = Builder::new_default();
            loop {
                let msg = builder.init_root::<m::message::Builder>();
                let mut status = msg.init_status();
                status.set_text("hello");
                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();
                client_send.send(msg_bytes).unwrap();
                /*
                let buf = server_recv.recv();
                if let Ok(buf) = buf {
                    println!("{:?}", buf);
                } else {
                    println!("error: {:?}", buf);
                    break;
                }
                */
                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        Ok(())
    }

    #[text_signature = "($self, bytes, /)"]
    pub fn send_message(&mut self, bytes: &PyBytes) -> PyResult<()> {
        let byte_vec: Vec<u8> = bytes.extract().unwrap();
        self.server_send.as_ref().unwrap().send(byte_vec).map_err(|e| exceptions::PyRuntimeError::new_err(format!("{}", e)))
    }
}

#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    Ok(())
}
