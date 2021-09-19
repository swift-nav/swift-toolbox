use std::collections::HashMap;

use capnp::message::Builder;
use capnp::message::HeapAllocator;
use capnp::serialize;
use indexmap::IndexSet;
use log::warn;
use semver::{Version, VersionReq}; //BuildMetadata, Prerelease,
use serialport::available_ports;

use crate::constants::*;
use crate::errors::*;
use crate::types::{CapnProtoSender, SignalCodes};
use crate::{common_constants as cc, shared_state::SharedState};

/// Compare to semvar strings and return true if the later_version is greater than the early version.
///
/// Assumes that the versions are in the form of `MAJOR.MINOR.PATCH<DEV>`.
/// Where `DEV` is optional and begins with any non numeric value.
/// If the semver matches up to the DEV value, then it will return true if DEV does not match.
///
/// Arguments:
/// - `early_version`: The early version string.
/// - `later_version`: The later version string.
/// - `less_than_or_equal`: If true, then check whether early version is less than or equal to later version.
///   Otherwise, check whether early version is less than later version.
///
/// Returns:
/// - `true` if the later_version is greater than the early version.
/// - `false` if the later_version is not greater than the early version.
pub fn compare_semvers(
    early_version: String,
    later_version: String,
    less_than_or_equal: bool,
) -> anyhow::Result<bool> {
    let early_version_num_split: Vec<&str> = early_version.splitn(2, char::is_numeric).collect();
    let later_version_num_split: Vec<&str> = later_version.splitn(2, char::is_numeric).collect();
    let early_version = early_version[early_version_num_split[0].len()..].to_string();
    let later_version = later_version[later_version_num_split[0].len()..].to_string();
    let operation = if less_than_or_equal { "<=" } else { "<" };
    if let Ok(req) = VersionReq::parse(&format!("{}{}", operation, early_version)) {
        if let Ok(version) = Version::parse(&later_version) {
            if req.matches(&version) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Send a CLOSE, or kill, signal to the frontend.
pub fn close_frontend<P: CapnProtoSender>(client_send: &mut P) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let mut status = msg.init_status();
    let app_state = cc::ApplicationStates::CLOSE;
    status.set_text(&app_state.to_string());
    client_send.send_data(serialize_capnproto_builder(builder));
}

/// Send a CONNECTED or DISCONNECTED, signal to the frontend.
pub fn set_connected_frontend<P: CapnProtoSender>(
    app_state: cc::ApplicationStates,
    client_send: &mut P,
) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let mut status = msg.init_status();
    status.set_text(&app_state.to_string());
    client_send.send_data(serialize_capnproto_builder(builder));
}

pub fn refresh_navbar<P: CapnProtoSender>(client_send: &mut P, shared_state: SharedState) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

    let mut nav_bar_status = msg.init_nav_bar_status();
    let mut ports: Vec<String> = vec![];
    if let Ok(ports_) = &mut available_ports() {
        // TODO(johnmichael.burke@) [CPP-114]Find solution to this hack for Linux serialport.
        ports = ports_
            .iter_mut()
            .map(|x| x.port_name.replace("/sys/class/tty/", "/dev/"))
            .collect();
    }

    let mut available_ports = nav_bar_status
        .reborrow()
        .init_available_ports(ports.len() as u32);

    for (i, serialportinfo) in ports.iter().enumerate() {
        available_ports.set(i as u32, &(*serialportinfo));
    }

    let mut available_baudrates = nav_bar_status
        .reborrow()
        .init_available_baudrates(AVAILABLE_BAUDRATES.len() as u32);

    for (i, baudrate) in AVAILABLE_BAUDRATES.iter().enumerate() {
        available_baudrates.set(i as u32, *baudrate);
    }

    let mut available_flows = nav_bar_status
        .reborrow()
        .init_available_flows(AVAILABLE_FLOWS.len() as u32);

    for (i, flow) in AVAILABLE_FLOWS.iter().enumerate() {
        available_flows.set(i as u32, &flow.to_string());
    }

    let mut available_refresh_rates = nav_bar_status
        .reborrow()
        .init_available_refresh_rates(AVAILABLE_REFRESH_RATES.len() as u32);

    for (i, rr) in AVAILABLE_REFRESH_RATES.iter().enumerate() {
        available_refresh_rates.set(i as u32, *rr);
    }

    let addresses = shared_state.address_history();
    let hosts: IndexSet<String> = addresses
        .clone()
        .into_iter()
        .map(|addy| addy.host)
        .rev()
        .collect();
    let ports: IndexSet<u16> = addresses.into_iter().map(|addy| addy.port).rev().collect();
    let mut prevous_hosts = nav_bar_status
        .reborrow()
        .init_previous_hosts(hosts.len() as u32);

    for (i, hosts) in hosts.iter().enumerate() {
        prevous_hosts.set(i as u32, hosts);
    }

    let mut prevous_ports = nav_bar_status
        .reborrow()
        .init_previous_ports(ports.len() as u32);

    for (i, ports) in ports.iter().enumerate() {
        prevous_ports.set(i as u32, *ports);
    }
    let mut files = shared_state.file_history();
    files.reverse();
    let mut prevous_files = nav_bar_status
        .reborrow()
        .init_previous_files(files.len() as u32);

    for (i, filename) in files.iter().enumerate() {
        prevous_files.set(i as u32, filename);
    }

    nav_bar_status.set_log_level(&shared_state.log_level().to_string());

    client_send.send_data(serialize_capnproto_builder(builder));
}

pub fn serialize_capnproto_builder(builder: Builder<HeapAllocator>) -> Vec<u8> {
    let mut msg_bytes: Vec<u8> = vec![];
    serialize::write_message(&mut msg_bytes, &builder).expect(CAP_N_PROTO_SERIALIZATION_FAILURE);
    msg_bytes
}

pub fn refresh_loggingbar<P: CapnProtoSender>(client_send: &mut P, shared_state: SharedState) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

    let mut logging_bar_status = msg.init_logging_bar_status();
    let csv_logging = shared_state.csv_logging();
    logging_bar_status.set_csv_logging(csv_logging.to_bool());
    let sbp_logging = shared_state.sbp_logging();
    logging_bar_status.set_sbp_logging(&sbp_logging.to_string());

    let mut folders = shared_state.folder_history();
    folders.reverse();
    let mut prevous_folders = logging_bar_status
        .reborrow()
        .init_previous_folders(folders.len() as u32);

    for (i, folder) in folders.iter().enumerate() {
        prevous_folders.set(i as u32, folder);
    }

    client_send.send_data(serialize_capnproto_builder(builder));
}

pub fn signal_key_label(
    key: (SignalCodes, i16),
    extra: &HashMap<i16, i16>,
) -> (Option<String>, Option<String>, Option<String>) {
    let (code, sat) = key;
    let code_lbl = Some(code.to_string());
    let mut freq_lbl = None;
    let id_lbl;

    if code.code_is_glo() {
        let freq_lbl_ = format!("F+{:02}", sat);
        freq_lbl = Some(freq_lbl_);
        if extra.contains_key(&sat) {
            id_lbl = Some(format!("R{:<02}", extra[&sat]));
        } else {
            id_lbl = Some(format!("R{:<02}", sat));
        }
    } else if code.code_is_sbas() {
        id_lbl = Some(format!("S{: >3}", sat));
    } else if code.code_is_bds() {
        id_lbl = Some(format!("C{:0>2}", sat));
    } else if code.code_is_qzss() {
        id_lbl = Some(format!("J{: >3}", sat));
    } else if code.code_is_galileo() {
        id_lbl = Some(format!("E{:0>2}", sat));
    } else {
        id_lbl = Some(format!("G{:0>2}", sat));
    }
    (code_lbl, freq_lbl, id_lbl)
}

/// These colors are distinguishable from one another based on expected codes.
///
/// # Parameters
///
/// - `code`: The signal code.
pub fn color_map(code: i16) -> &'static str {
    match code {
        1 => "#e58a8a",
        2 => "#664949",
        3 => "#590c00",
        4 => "#cc4631",
        5 => "#e56c1c",
        6 => "#4c2a12",
        7 => "#996325",
        8 => "#f2b774",
        9 => "#ffaa00",
        10 => "#ccb993",
        11 => "#997a00",
        12 => "#4c4700",
        13 => "#d0d94e",
        14 => "#aaff00",
        15 => "#4ea614",
        16 => "#123306",
        17 => "#18660c",
        18 => "#6e9974",
        19 => "#8ae6a2",
        20 => "#00ff66",
        21 => "#57f2e8",
        22 => "#1f7980",
        23 => "#263e40",
        24 => "#004d73",
        25 => "#37abe6",
        26 => "#7790a6",
        27 => "#144ea6",
        28 => "#263040",
        29 => "#152859",
        30 => "#1d39f2",
        31 => "#828ed9",
        32 => "#000073",
        33 => "#000066",
        34 => "#8c7aff",
        35 => "#1b0033",
        36 => "#d900ca",
        37 => "#730e6c",
        _ => "#ff0000",
    }
}

/// Retreive the associated color based on provided key.
///
/// # Parameters
///
/// - `key`: The code, which is signal code and satellite constellation-specific satellite identifier.
pub fn signal_key_color(key: (SignalCodes, i16)) -> &'static str {
    let (code, mut sat) = key;

    if code.code_is_glo() {
        sat += GLO_FCN_OFFSET;
    } else if code.code_is_sbas() {
        sat -= SBAS_NEG_OFFSET;
    } else if code.code_is_qzss() {
        sat -= QZSS_NEG_OFFSET;
    }
    if sat > NUM_COLORS as i16 {
        sat %= NUM_COLORS as i16;
    }
    color_map(sat)
}

/// Calculate the length of a degree of latitude and longitude in meters.
///
/// # Parameters
///
/// - `lat_deg`: The latitude degree value to convert to lat/lon meters.
pub fn meters_per_deg(lat_deg: f64) -> (f64, f64) {
    let lat_term_1: f64 = 111132.92;
    let lat_term_2: f64 = -559.82;
    let lat_term_3: f64 = 1.175;
    let lat_term_4: f64 = -0.0023;
    let lon_term_1: f64 = 111412.84;
    let lon_term_2: f64 = -93.5;
    let lon_term_3: f64 = 0.118;

    let latlen = lat_term_1
        + (lat_term_2 * f64::cos(2_f64 * f64::to_radians(lat_deg)))
        + (lat_term_3 * f64::cos(4_f64 * f64::to_radians(lat_deg)))
        + (lat_term_4 * f64::cos(6_f64 * f64::to_radians(lat_deg)));
    let lonlen = (lon_term_1 * f64::cos(f64::to_radians(lat_deg)))
        + (lon_term_2 * f64::cos(3_f64 * f64::to_radians(lat_deg)))
        + (lon_term_3 * f64::cos(5_f64 * f64::to_radians(lat_deg)));
    (latlen, lonlen)
}

/// Nanoseconds to Microseconds
///
/// # Parameters
/// - `ns`: The nanoseconds value to be converted.
///
/// # Returns
/// - Newly converted microseconds value.
pub fn nano_to_micro_sec(ns: f64) -> f64 {
    ns / 1000_f64
}

/// Convert millimeters to meters.
/// Taken from ICBINS/src/msg_utils.rs.
///
/// # Parameters
/// - `mm`: Value in millimeters.
///
/// # Returns
/// - Value in meters.
pub fn mm_to_m(mm: f64) -> f64 {
    mm / 1.0e+3_f64
}

/// Convert deciseconds to seconds.
///
/// # Parameters
/// - `ds`: Value in deciseconds.
///
/// # Returns
/// - Value in seconds.
pub fn decisec_to_sec(ds: f64) -> f64 {
    ds / 10_f64
}

/// Convert milliseconds to seconds.
///
/// # Parameters
/// - `ms`: Value in milliseconds.
///
/// # Returns
/// - Value in seconds.
pub fn ms_to_sec(ms: f64) -> f64 {
    ms / 1.0e+3_f64
}

/// Convert nanoseconds to seconds.
///
/// # Parameters
/// - `ns`: Value in nanoseconds.
///
/// # Returns
/// - Value in seconds.
pub fn ns_to_sec(ns: f64) -> f64 {
    ns / NANOSECONDS_PER_SECOND
}

/// Convert seconds to nanoseconds.
///
/// # Parameters
/// - `ns`: Value in econds.
///
/// # Returns
/// - Value in nanoseconds.
pub fn sec_to_ns(ns: f64) -> f64 {
    ns * SECONDS_PER_NANOSECOND
}

/// Convert millisdegree to degree.
///
/// # Parameters
/// - `ms`: Value in millidegrees.
///
/// # Returns
/// - Value in degrees.
pub fn mdeg_to_deg(mdeg: f64) -> f64 {
    mdeg / 1.0e+3_f64
}

pub fn compute_doppler(
    new_carrier_phase: f64,
    old_carrier_phase: f64,
    current_gps_tow: f64,
    previous_tow: f64,
    is_deprecated_message: bool,
) -> f64 {
    if (current_gps_tow - previous_tow).abs() <= f64::EPSILON {
        warn!("Received two complete observation sets with identical TOW");
        return 0 as f64;
    }
    let mut computed_doppler =
        (old_carrier_phase - new_carrier_phase) as f64 / (current_gps_tow - previous_tow) as f64;
    if is_deprecated_message {
        computed_doppler = -computed_doppler;
    }
    computed_doppler
}

/// Convert bytes to kilobytes.
///
/// # Parameters:
/// - `bytes`: The bytes to convert.
///
/// # Returns
/// - The converted bytes in kilobytes.
pub fn bytes_to_kb(bytes: f64) -> f64 {
    bytes / 1024_f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piksi_tools_constants::*;

    fn float_eq(f1: f64, f2: f64) -> bool {
        f64::abs(f1 - f2) <= f64::EPSILON
    }

    #[test]
    fn get_signal_key_label_test() {
        let mut extra: HashMap<i16, i16> = HashMap::new();
        extra.insert(
            SignalCodes::CodeGloL2P as i16,
            SignalCodes::CodeGloL2P as i16,
        );

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGloL2P, SignalCodes::CodeGloL2P as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2P_STR);
        assert_eq!(freq_lbl.unwrap(), "F+30");
        assert_eq!(id_lbl.unwrap(), "R30");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGloL2Of, SignalCodes::CodeGloL2Of as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2OF_STR);
        assert_eq!(freq_lbl.unwrap(), "F+04");
        assert_eq!(id_lbl.unwrap(), "R04");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeSbasL5Q, SignalCodes::CodeSbasL5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), SBAS_L5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "S 42");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeBds3B5Q, SignalCodes::CodeBds3B5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), BDS3_B5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "C48");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeQzsL2Cx, SignalCodes::CodeQzsL2Cx as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), QZS_L2CX_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "J 37");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGalE8X, SignalCodes::CodeGalE8X as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GAL_E8X_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "E25");
    }

    #[test]
    fn meters_per_deg_test() {
        // Latitude range: [-90, 90]
        assert_eq!(
            meters_per_deg(-90_f64),
            (111693.9173, 6.839280692934427e-12)
        );
        assert_eq!(meters_per_deg(-45_f64), (111131.745, 78846.80572069259));
        assert_eq!(meters_per_deg(0_f64), (110574.2727, 111319.458));
        assert_eq!(meters_per_deg(45_f64), (111131.745, 78846.80572069259));
        assert_eq!(meters_per_deg(90_f64), (111693.9173, 6.839280692934427e-12));
    }

    #[test]
    fn nano_to_micro_sec_test() {
        assert!(float_eq(nano_to_micro_sec(1000_f64), 1_f64));
        assert!(float_eq(nano_to_micro_sec(1000000_f64), 1000_f64));
        assert!(float_eq(nano_to_micro_sec(0_f64), 0_f64));
        assert!(float_eq(nano_to_micro_sec(1337_f64), 1.337_f64));
    }

    #[test]
    fn compare_semvers_test() {
        assert!(!compare_semvers(String::from("2.0.0"), String::from("1.0.0"), false).unwrap());
        assert!(compare_semvers(String::from("v2.0.0"), String::from("v2.0.0"), false).unwrap());
        assert!(compare_semvers(String::from("v2.0.0"), String::from("v2.2.0"), false).unwrap());
        assert!(!compare_semvers(String::from("1.0.0"), String::from("1.0.0"), true).unwrap());
        assert!(compare_semvers(String::from("1.0.0"), String::from("1.0.1"), true).unwrap());
        assert!(compare_semvers(String::from("1.0.0"), String::from("1.1.0"), true).unwrap());
        assert!(compare_semvers(String::from("1.0.0"), String::from("2.0.0"), true).unwrap());
        assert!(!compare_semvers(String::from("1.0.0"), String::from("0.0.0"), true).unwrap());
        assert!(!compare_semvers(String::from("1.0.0"), String::from("0.0.1"), true).unwrap());
        assert!(!compare_semvers(String::from("1.0.0"), String::from("0.1.0"), true).unwrap());
        assert!(
            compare_semvers(String::from("2.5.6"), String::from("2.5.6-dev5432"), true).unwrap()
        );
        assert!(
            compare_semvers(String::from("2.5.6-dev5432"), String::from("2.5.6"), true).unwrap()
        );
        assert!(compare_semvers(
            String::from("2.5.6-dev5432"),
            String::from("2.5.6-dev54321"),
            true
        )
        .unwrap());
        assert!(
            compare_semvers(String::from("2.5.6"), String::from("2.5.6.dev5432"), true).unwrap()
        );
        assert!(
            compare_semvers(String::from("2.5.6.dev5432"), String::from("2.5.6"), true).unwrap()
        );
        assert!(compare_semvers(
            String::from("2.5.6.dev5432"),
            String::from("2.5.6.dev12345"),
            true
        )
        .unwrap());
    }

    #[test]
    fn compute_doppler_test() {
        assert!(float_eq(
            compute_doppler(
                123438650.3359375,
                123438590.203125,
                251746.8,
                251746.8,
                false
            ),
            0.0
        ));
        assert!(float_eq(
            compute_doppler(
                123438650.3359375,
                123438590.203125,
                251746.9,
                251746.8,
                false
            ),
            -601.3281249649981
        ));
        assert!(float_eq(
            compute_doppler(89473356.9453125, 89473456.921875, 251746.9, 251746.8, false),
            999.765624941806
        ));
        assert!(float_eq(
            compute_doppler(
                96692940.6015625,
                96692834.87890625,
                251746.9,
                251746.8,
                false
            ),
            -1057.2265624384613
        ));
        assert!(float_eq(
            compute_doppler(
                108296328.85546875,
                108296130.609375,
                251746.9,
                251746.8,
                false
            ),
            -1982.4609373846056
        ));
        assert!(float_eq(
            compute_doppler(99816633.2109375, 99816774.25, 251746.9, 251746.8, false),
            1410.3906249179045
        ));
        assert!(float_eq(
            compute_doppler(
                109036269.546875,
                109036058.60546875,
                251746.9,
                251746.8,
                false
            ),
            -2109.414062377216
        ));
        assert!(float_eq(
            compute_doppler(
                94582860.46484375,
                94582814.38671875,
                251746.9,
                251746.8,
                false
            ),
            -460.781249973179
        ));
    }
}
