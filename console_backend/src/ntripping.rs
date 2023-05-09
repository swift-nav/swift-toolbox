use curl::easy::{Easy, HttpVersion, List};

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

fn fetch_ntrip(opt: NtripOptions) {
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
    curl.http_09_allowed(true)?;

    if let Some(username) = &opt.username {
        curl.username(username)?;
    }

    if let Some(password) = &opt.password {
        curl.password(password)?;
    }
}
