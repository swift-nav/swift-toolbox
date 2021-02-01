use std::collections::HashMap;
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

///For tests.
use glob::glob;
use ndarray::{s, Array, Array2, Axis};
use std::{io::Write, path::Path};
const MAX_DISCEPANCY_THRESHOLD_PERCENT: f64 = 0.1;
const MAX_APPROXIMATION_ERROR_THRESHOLD: f64 = 0.001;
const TEST_DATA_DIRECTORY: &str = "src/test_data/";
const CSV_EXTENSION: &str = ".csv";
const ICBINS_POSTFIX: &str = "-icbins";
const GPS_TOW_SECS_M: &str = "GPS TOW [s]";
const PDOP: &str = "PDOP";
const HDOP: &str = "HDOP";
const DELTA_TOW_MS: &str = "Delta TOW [ms]";


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
    let mut hpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut vpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut sat_headers: Vec<u8> = vec![];
    let mut sats: Vec<Vec<(f64, OrderedFloat<f64>)>> = vec![];
    let mut tow: f64 = 0.0;
    for message in messages {
        match message {
            Ok(SBP::MsgMeasurementState(msg)) => {
                for state in msg.states {
                    if state.cn0 != 0{
                        let mut points = match sat_headers.iter().position(|&ele| ele == state.mesid.sat) {
                            Some(idx) => sats.get_mut(idx).unwrap(),
                            _ => {
                                sat_headers.push(state.mesid.sat);
                                sats.push(Vec::new());
                                sats.last_mut().unwrap()
                            },
                        };
                        if points.len() >= 200 {
                            points.remove(0);
                        }
                        points.push((tow, OrderedFloat(state.cn0 as f64/4.0)));
                    }
                }
                let mut builder = Builder::new_default();
                let msg = builder.init_root::<m::message::Builder>();

                let mut tracking_status = msg.init_tracking_status();
                tracking_status.set_min(0 as f64);
                tracking_status.set_max(60 as f64);
                let mut tracking_headers = tracking_status.reborrow().init_headers(sat_headers.len() as u32);
                
                for (i, header) in sat_headers.iter().enumerate() {
                    tracking_headers.set(i as u32, *header);
                }
                
                let mut tracking_points = tracking_status.reborrow().init_data(sat_headers.len() as u32);
                {
                    for idx in 0..sat_headers.len(){
                        let mut points = sats.get_mut(idx).unwrap();
                        let mut point_val_idx = tracking_points.reborrow().init(idx as u32, points.len() as u32);
                        for (i, (x, OrderedFloat(y))) in points.iter().enumerate() {
                            let mut point_val = point_val_idx.reborrow().get(i as u32);
                            point_val.set_x(*x);
                            point_val.set_y(*y);
                        }
                    }
                    
                }
                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();

                client_send_clone.send(msg_bytes).unwrap();
                
            }
            Ok(SBP::MsgTrackingState(msg)) => {
            }
            Ok(SBP::MsgObs(msg)) => {
            }

            Ok(SBP::MsgVelNED(velocity_ned)) => {

                let n = velocity_ned.n as f64;
                let e = velocity_ned.e as f64;
                let d = velocity_ned.d as f64;

                let h_vel = f64::sqrt(f64::powi(n, 2) + f64::powi(e, 2)) / 1000.0;
                let v_vel = (-1.0 * d) / 1000.0;

                tow = velocity_ned.tow as f64 / 1000.0;

                min_max = if let Some(_) = min_max {
                    let vmin = vpoints.iter().min_by_key(|i| i.1).unwrap();
                    let vmax = vpoints.iter().max_by_key(|i| i.1).unwrap();
                    let hmin = hpoints.iter().min_by_key(|i| i.1).unwrap();
                    let hmax = hpoints.iter().max_by_key(|i| i.1).unwrap();
                    let mut min = 0.0;
                    let mut max = 1.0;
                    if vmin.1.into_inner() < hmin.1.into_inner() {
                        min = vmin.1.into_inner();
                    } else {
                        min = hmin.1.into_inner();
                    }
                    if vmax.1.into_inner() > hmax.1.into_inner() {
                        max = vmax.1.into_inner();
                    } else {
                        max = hmax.1.into_inner();
                    }

                    Some((min, max))
                } else {
                    Some((-1.0 * f64::abs(h_vel) * 1.5, 1.0 * f64::abs(h_vel) * 1.5))
                };

                if hpoints.len() >= 200 {
                    hpoints.remove(0);
                }
                if vpoints.len() >= 200 {
                    vpoints.remove(0);
                }
                hpoints.push((tow, OrderedFloat(h_vel)));
                vpoints.push((tow, OrderedFloat(v_vel)));

                let mut builder = Builder::new_default();
                let msg = builder.init_root::<m::message::Builder>();

                let mut velocity_status = msg.init_velocity_status();

                let (min, max) = min_max.unwrap();
            
                velocity_status.set_min(min);
                velocity_status.set_max(max);

                {
                    let mut hvel_points = velocity_status.reborrow().init_hpoints(hpoints.len() as u32);
                    for (i, (x, OrderedFloat(y))) in hpoints.iter().enumerate() {
                        let mut point_val = hvel_points.reborrow().get(i as u32);
                        point_val.set_x(*x);
                        point_val.set_y(*y);
                    }
                }
                {
                    let mut vvel_points = velocity_status.reborrow().init_vpoints(vpoints.len() as u32);
                    for (i, (x, OrderedFloat(y))) in vpoints.iter().enumerate() {
                        let mut point_val = vvel_points.reborrow().get(i as u32);
                        point_val.set_x(*x);
                        point_val.set_y(*y);
                    }
                }
                

                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();

                client_send_clone.send(msg_bytes).unwrap();

            }
            _ => {
                // no-op
            }
        }
    }
    ()
}

#[cfg(not(test))]
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
                    // let message: &'static m::message::Reader = &message;
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
                            // let filename = String::from(filename).unwrap();
                            println!("{:?}", filename);
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
        // let client_send_clone2 = client_send.clone();
        thread::spawn(move || {
            // let mut builder = Builder::new_default();
            loop {
                // let msg = builder.init_root::<m::message::Builder>();
                // let mut status = msg.init_status();
                // status.set_text("hello");
                // let mut msg_bytes: Vec<u8> = vec![];
                // serialize::write_message(&mut msg_bytes, &builder).unwrap();
                // client_send_clone2.send(msg_bytes).unwrap();
                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        Ok(server_endpoint)
    }
}

#[cfg(not(test))]
#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<ServerEndpoint>()?;
    Ok(())
}


#[test]
fn test_readfile() {
    let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("**/*.sbp");
    let glob_pattern = match glob_pattern.to_str() {
        Some(i) => i,
        _ => "",
    };
    for ele in glob(glob_pattern).expect("failed to read glob") {
        match ele {
            Ok(filename) => {
                println!("{:?}", filename.display());
                let file_in_name = &filename;
                let file_out_name = file_in_name
                    .parent()
                    .unwrap()
                    .join(file_in_name.file_stem().unwrap());
                let file_out_name = file_out_name.to_str().unwrap();
                let file_out_orig_name = format!("{}{}", file_out_name, &CSV_EXTENSION);
                let file_out_name =
                    format!("{}{}{}", file_out_name, &ICBINS_POSTFIX, &CSV_EXTENSION);
                let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
                let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
                process_messages(messages, client_send);
                assert!(true);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
