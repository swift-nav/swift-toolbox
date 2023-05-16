use chrono::{DateTime, Utc};
use curl::easy::{Easy, HttpVersion, List, ReadError};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{iter, thread};

use crate::types::{MsgSender, PosLLH};

use crossbeam::channel;

use anyhow::Context;
use std::time::{Duration, SystemTime};

#[derive(Debug, Default)]
pub struct NtripState {
    pub(crate) connected_thd: Option<JoinHandle<()>>,
    pub(crate) options: NtripOptions,
    pub(crate) is_running: Arc<Mutex<bool>>,
    last_data: Arc<Mutex<LastData>>,
}

#[derive(Debug, Default, Copy, Clone)]
struct LastData {
    lat: f64,
    lon: f64,
    alt: f64,
    epoch: f64,
}

#[derive(Debug, Default, Clone)]
pub enum PositionMode {
    #[default]
    Dynamic,
    Static {
        lat: f64,
        lon: f64,
        alt: f64,
    },
}

#[derive(Debug, Default, Clone)]
pub struct NtripOptions {
    pub(crate) url: String,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) nmea_period: u64,
    pub(crate) pos_mode: PositionMode,
    pub(crate) client_id: String,
    pub(crate) epoch: Option<u32>,
    // nmea_header: bool,
    // request_counter: Option<u8>,
    // area_id: Option<u32>,
    // corr_mask: Option<u16>,
    // soln_id: Option<u8>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Message {
    Gga { lat: f64, lon: f64, height: f64 },
    // Cra {
    //     request_counter: Option<u8>,
    //     area_id: Option<u32>,
    //     corrections_mask: Option<u16>,
    //     solution_id: Option<u8>,
    // },
}

// fn build_cra(opt: &NtripOptions) -> Command {
//     Command {
//         epoch: opt.epoch,
//         after: 0,
//         crc: None,
//         message: Message::Cra {
//             request_counter: opt.request_counter,
//             area_id: opt.area_id,
//             corrections_mask: opt.corr_mask,
//             solution_id: opt.soln_id,
//         },
//     }
// }

fn build_gga(opts: &NtripOptions, last_data: &Arc<Mutex<LastData>>) -> Command {
    let (lat, lon, height) = match opts.pos_mode {
        PositionMode::Dynamic => {
            let guard = last_data.lock().unwrap();
            (guard.lat, guard.lon, guard.alt)
        }
        PositionMode::Static { lat, lon, alt } => (lat, lon, alt),
    };
    Command {
        epoch: opts.epoch,
        after: 0,
        crc: None,
        message: Message::Gga { lat, lon, height },
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
struct Command {
    #[serde(default = "default_after")]
    after: u64,
    epoch: Option<u32>,
    crc: Option<u8>,
    #[serde(flatten)]
    message: Message,
}

fn default_after() -> u64 {
    10
}

impl Command {
    fn to_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = self.epoch.map_or_else(SystemTime::now, |e| {
            SystemTime::UNIX_EPOCH + Duration::from_secs(e.into())
        });
        let message = self.message.format(now.into());
        let checksum = self.crc.unwrap_or_else(|| checksum(message.as_bytes()));
        write!(f, "{message}*{checksum:X}")
    }
}

fn checksum(buf: &[u8]) -> u8 {
    let mut sum = 0;
    for c in &buf[1..] {
        sum ^= c;
    }
    sum
}

impl Message {
    fn format(&self, time: DateTime<Utc>) -> String {
        match *self {
            Message::Gga { lat, lon, height } => {
                let time = time.format("%H%M%S.00");

                let latn = ((lat * 1e8).round() / 1e8).abs();
                let lonn = ((lon * 1e8).round() / 1e8).abs();

                let lat_deg = latn as u16;
                let lon_deg = lonn as u16;

                let lat_min = (latn - (lat_deg as f64)) * 60.0;
                let lon_min = (lonn - (lon_deg as f64)) * 60.0;

                let lat_dir = if lat < 0.0 { 'S' } else { 'N' };
                let lon_dir = if lon < 0.0 { 'W' } else { 'E' };

                format!(
                    "$GPGGA,{},{:02}{:010.7},{},{:03}{:010.7},{},4,12,1.3,{:.2},M,0.0,M,1.7,0078",
                    time, lat_deg, lat_min, lat_dir, lon_deg, lon_min, lon_dir, height
                )
            } // Message::Cra {
              //     request_counter,
              //     area_id,
              //     corrections_mask,
              //     solution_id,
              // } => {
              //     let mut s = String::from("$PSWTCRA,");
              //     if let Some(request_counter) = request_counter {
              //         s.push_str(&format!("{request_counter},"));
              //     }
              //     s.push(',');
              //     if let Some(area_id) = area_id {
              //         s.push_str(&format!("{area_id},"));
              //     }
              //     s.push(',');
              //     if let Some(corrections_mask) = corrections_mask {
              //         s.push_str(&format!("{corrections_mask},"));
              //     }
              //     s.push(',');
              //     if let Some(solution_id) = solution_id {
              //         s.push_str(&format!("{solution_id},"));
              //     }
              //     s
              // }
        }
    }
}

fn get_commands(
    opt: NtripOptions,
    last_data: Arc<Mutex<LastData>>,
) -> anyhow::Result<Box<dyn Iterator<Item = Command> + Send>> {
    // if let Some(path) = opt.input {
    //     let file = std::fs::File::open(path)?;
    //     let cmds: Vec<_> = serde_yaml::from_reader(file)?;
    //     return Ok(Box::new(cmds.into_iter()));
    // }

    if opt.nmea_period == 0 {
        return Ok(Box::new(iter::empty()));
    }

    // if opt.area_id.is_some() {
    // let first = build_cra(&opt);
    // let it = iter::successors(Some(first), move |prev| {
    //     let mut next = *prev;
    //     if let Message::Cra {
    //         request_counter: Some(ref mut counter),
    //         ..
    //     } = &mut next.message
    //     {
    //         *counter = counter.wrapping_add(1);
    //     }
    //     next.after = opt.nmea_period;
    //     Some(next)
    // });
    // Ok(Box::new(it))
    // Err(anyhow!("cra not implemented"))
    // } else {
    let first = build_gga(&opt, &last_data);
    let rest = iter::repeat(Command {
        after: opt.nmea_period,
        ..first
    });
    Ok(Box::new(iter::once(first).chain(rest)))
    // }
}

fn main(
    mut msg_sender: MsgSender,
    opt: NtripOptions,
    last_data: Arc<Mutex<LastData>>,
    is_running: Arc<Mutex<bool>>,
) -> anyhow::Result<()> {
    let mut curl = Easy::new();
    let mut headers = List::new();
    headers.append("Transfer-Encoding:")?;
    headers.append("Ntrip-Version: Ntrip/2.0")?;
    headers.append(&format!("X-SwiftNav-Client-Id: {}", opt.client_id))?;
    // if opt.nmea_header {
    // if opt.area_id.is_some() {
    //     // headers.append(&format!("Ntrip-CRA: {}", build_cra(&opt)))?;
    //     return Err(anyhow!("cra not implemented"));
    // } else {

    let gga = build_gga(&opt, &last_data);
    headers.append(&format!("Ntrip-GGA: {gga}"))?;
    // }
    // }
    curl.http_headers(headers)?;
    curl.useragent("NTRIP ntrip-client/1.0")?;
    curl.url(&opt.url)?;
    curl.progress(true)?;
    curl.put(true)?;
    curl.custom_request("GET")?;
    curl.http_version(HttpVersion::Any)?;
    curl.http_09_allowed(true)?;

    if let Some(username) = &opt.username {
        curl.username(username)?;
    }

    if let Some(password) = &opt.password {
        curl.password(password)?;
    }
    let (tx, rx) = channel::bounded::<Vec<u8>>(1);
    let transfer = Rc::new(RefCell::new(curl.transfer()));

    transfer.borrow_mut().progress_function({
        let rx = &rx;
        let transfer = Rc::clone(&transfer);
        move |_dltot, _dlnow, _ultot, _ulnow| {
            {
                let running = *is_running.lock().unwrap();
                if !running {
                    return false;
                }
            }
            if !rx.is_empty() {
                if let Err(e) = transfer.borrow().unpause_read() {
                    println!("unpause error: {e}");
                    return false;
                }
            }
            true
        }
    })?;

    transfer.borrow_mut().write_function(|data| {
        std::io::stdout().write_all(data).unwrap(); // should be infallible
        if let Err(e) = msg_sender.write_all(data) {
            println!("write error: {e}");
            return Ok(0);
        }
        Ok(data.len())
    })?;
    transfer.borrow_mut().read_function(|mut data: &mut [u8]| {
        let mut bytes = match rx.try_recv() {
            Ok(bytes) => bytes,
            Err(_) => return Err(ReadError::Pause),
        };
        bytes.extend_from_slice(b"\r\n");
        if let Err(e) = data.write_all(&bytes) {
            println!("read error: {e}");
            return Err(ReadError::Abort);
        }
        Ok(bytes.len())
    })?;

    let commands = get_commands(opt.clone(), last_data)?;
    let handle = thread::spawn(move || {
        for cmd in commands {
            if cmd.after > 0 {
                // need to unpark thread (?)
                thread::park_timeout(Duration::from_secs(cmd.after));
            }
            if tx.send(cmd.to_bytes()).is_err() {
                break;
            }
        }
        Ok(())
    });

    transfer
        .borrow()
        .perform()
        .context("ntrip curl perform errored")?;
    if !handle.is_finished() {
        Ok(())
    } else {
        // an error stopped the thread early
        handle.join().unwrap()
    }
}

impl NtripState {
    pub fn connect(
        &mut self,
        msg_sender: MsgSender,
        url: String,
        username: String, // empty username is None
        password: String,
        gga_period: u64,
        pos: Option<(f64, f64, f64)>,
    ) {
        if self.connected_thd.is_some() {
            // is already connected
            return;
        }

        let pos_mode = pos
            .map(|(lat, lon, alt)| PositionMode::Static { lat, lon, alt })
            .unwrap_or(PositionMode::Dynamic);

        let options = NtripOptions {
            url,
            pos_mode,
            username: Some(username).filter(|s| !s.is_empty()),
            password: Some(password).filter(|s| !s.is_empty()),
            nmea_period: gga_period,
            client_id: "00000000-0000-0000-0000-000000000000".to_string(),
            epoch: None,
        };

        self.options = options.clone();
        let last_data = self.last_data.clone();
        self.set_running(true);
        let running = self.is_running.clone();
        let thd = thread::spawn(move || {
            let r = main(msg_sender, options, last_data, running);
            println!("{:?}", r);
        });

        self.connected_thd = Some(thd);
    }

    pub fn disconnect(&mut self) {
        if let Some(thd) = self.connected_thd.take() {
            self.set_running(false);
            let _ = thd.join();
        }
        println!("disconnected");
    }

    pub fn set_running(&mut self, val: bool) {
        let mut lock = self.is_running.lock().unwrap();
        *lock = val;
    }

    pub fn set_lastdata(&mut self, val: PosLLH) {
        let fields = val.fields();
        let mut guard = self.last_data.lock().unwrap();
        guard.lat = fields.lat;
        guard.lon = fields.lon;
        guard.alt = fields.height;
        guard.epoch = fields.tow;
    }
}
