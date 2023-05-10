use chrono::{DateTime, Utc};
use curl::easy::{Easy, HttpVersion, List};

use std::time::{Duration, SystemTime};

struct NtripOptions {
    url: String,
    lat: f64,
    lon: f64,
    alt: f64,
    client_id: String,
    epoch: Option<u32>,
    username: Option<String>,
    password: Option<String>,
    nmea_period: u64,
    nmea_header: bool,
    request_counter: Option<u8>,
    area_id: Option<u32>,
    corr_mask: Option<u16>,
    soln_id: Option<u8>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Message {
    Gga {
        lat: f64,
        lon: f64,
        height: f64,
    },
    Cra {
        request_counter: Option<u8>,
        area_id: Option<u32>,
        corrections_mask: Option<u16>,
        solution_id: Option<u8>,
    },
}

fn build_cra(opt: &NtripOptions) -> Command {
    Command {
        epoch: opt.epoch,
        after: 0,
        crc: None,
        message: Message::Cra {
            request_counter: opt.request_counter,
            area_id: opt.area_id,
            corrections_mask: opt.corr_mask,
            solution_id: opt.soln_id,
        },
    }
}

fn build_gga(opt: &NtripOptions) -> Command {
    Command {
        epoch: opt.epoch,
        after: 0,
        crc: None,
        message: Message::Gga {
            lat: opt.lat,
            lon: opt.lon,
            height: opt.alt,
        },
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
            }
            Message::Cra {
                request_counter,
                area_id,
                corrections_mask,
                solution_id,
            } => {
                let mut s = String::from("$PSWTCRA,");
                if let Some(request_counter) = request_counter {
                    s.push_str(&format!("{request_counter},"));
                }
                s.push(',');
                if let Some(area_id) = area_id {
                    s.push_str(&format!("{area_id},"));
                }
                s.push(',');
                if let Some(corrections_mask) = corrections_mask {
                    s.push_str(&format!("{corrections_mask},"));
                }
                s.push(',');
                if let Some(solution_id) = solution_id {
                    s.push_str(&format!("{solution_id},"));
                }
                s
            }
        }
    }
}

fn fetch_ntrip(opt: NtripOptions) -> anyhow::Result<()> {
    let mut curl = Easy::new();
    let mut headers = List::new();
    headers.append("Transfer-Encoding:")?;
    headers.append("Ntrip-Version: Ntrip/2.0")?;
    headers.append(&format!("X-SwiftNav-Client-Id: {}", opt.client_id))?;
    if opt.nmea_header {
        if opt.area_id.is_some() {
            headers.append(&format!("Ntrip-CRA: {}", build_cra(&opt)))?;
        } else {
            headers.append(&format!("Ntrip-GGA: {}", build_gga(&opt)))?;
        }
    }
    curl.http_headers(headers)?;
    curl.useragent("NTRIP ntrip-client/1.0")?;
    curl.url(&opt.url)?;
    curl.progress(true)?;
    curl.put(true)?;
    curl.custom_request("GET")?;
    curl.http_version(HttpVersion::Any)?;

    if let Some(username) = &opt.username {
        curl.username(username)?;
    }

    if let Some(password) = &opt.password {
        curl.password(password)?;
    }
    Ok(())
}