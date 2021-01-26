use std::io::{BufReader, Cursor};
use std::fs;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions;

use capnp::message::Builder;
use capnp::serialize;

use ordered_float::*;

use sbp::{messages::SBP};

use crate::console_backend_capnp as m;

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
        ServerEndpoint {
            server_send: None,
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
            Err(exceptions::PyRuntimeError::new_err("no server send endpoint"))
        }
    }
}

pub fn process_messages(messages: impl Iterator<Item = sbp::Result<SBP>>, client_send_clone: mpsc::Sender<Vec<u8>>){
    let mut min_max: Option<(f64, f64)> = None;
    let mut points: Vec<(f64, OrderedFloat<f64>)> = vec![];
    for message in messages {
        match message {
            // Ok(SBP::MsgVelNED(msg)) if is_valid_vel_ned(msg.flags) => {
                
            //     let vel_ned_data = extract_vel_ned_data(msg.n, msg.e, msg.d);
            //     data_set.sog_mps = vel_ned_data.sog_mps;
            //     data_set.cog_deg = vel_ned_data.cog_deg;
            //     data_set.v_vel_mps = vel_ned_data.v_vel_mps;
            // }
            
            Ok(SBP::MsgVelNED(velocity_ned)) => {

                let n = velocity_ned.n as f64;
                let e = velocity_ned.e as f64;
                let d = velocity_ned.d as f64;

                let h_vel = f64::sqrt(f64::powi(n, 2) + f64::powi(e, 2)) / 1000.0;
                let _v_vel = (-1.0 * d) / 1000.0;

                let tow = velocity_ned.tow as f64 / 1000.0;

                min_max = if let Some(_) = min_max {
                    let min = points.iter().min_by_key(|i| i.1).unwrap();
                    let max = points.iter().max_by_key(|i| i.1).unwrap();
                    Some((min.1.into_inner(), max.1.into_inner()))
                } else {
                    Some((-1.0 * f64::abs(h_vel) * 1.5, 1.0 * f64::abs(h_vel) * 1.5))
                };

                if points.len() >= 200 {
                    points.remove(0);
                }
                points.push((tow, OrderedFloat(h_vel)));

                let mut builder = Builder::new_default();
                let msg = builder.init_root::<m::message::Builder>();

                let mut velocity_status = msg.init_velocity_status();

                let (min, max) = min_max.unwrap();

                velocity_status.set_min(min);
                velocity_status.set_max(max);

                let mut vel_points = velocity_status.init_points(points.len() as u32);
                for (i, (x, OrderedFloat(y))) in points.iter().enumerate() {
                    let mut point_val = vel_points.reborrow().get(i as u32);
                    point_val.set_x(*x);
                    point_val.set_y(*y);
                }

                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();

                client_send_clone.send(msg_bytes).unwrap();

                //println!("velocity_ned: {:?}", velocity_ned);
            }
            _ => {
                // no-op
            }
        }
    }
    ()
}


#[pymethods]
impl Server {
    #[new]
    pub fn __new__() -> Self {
        Server {
            client_recv: None,
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
        let server_endpoint = ServerEndpoint { server_send: Some(server_send) };
        let client_send_clone = client_send.clone();
        thread::spawn(move || {
            loop {
                let buf = server_recv.recv();
                let client_send_clone = client_send_clone.clone();
                if let Ok(buf) = buf {
                    let mut buf_reader = BufReader::new(Cursor::new(buf));
                    let message_reader = serialize::read_message(&mut buf_reader, ::capnp::message::ReaderOptions::new()).unwrap();
                    let message = message_reader.get_root::<m::message::Reader>().unwrap();
                    let message: &'static m::message::Reader = &message;
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
            }
        });
        let client_send_clone2 = client_send.clone();
        thread::spawn(move || {
            let mut builder = Builder::new_default();
            loop {
                let msg = builder.init_root::<m::message::Builder>();
                let mut status = msg.init_status();
                status.set_text("hello");
                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();
                client_send_clone2.send(msg_bytes).unwrap();
                thread::sleep(std::time::Duration::from_secs(1));
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
