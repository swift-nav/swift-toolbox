use std::collections::HashMap;
use std::ops::Index;

use capnp::message::Builder;
use capnp::message::HeapAllocator;
use capnp::serialize;

use indexmap::IndexSet;
use log::warn;
use sbp::SbpString;
use serialport::available_ports;

use crate::client_sender::BoxedClientSender;
use crate::constants::*;
use crate::errors::*;
use crate::shared_state::{ConnectionState, SerialConfig, SharedState};
use crate::types::SignalCodes;

/// Create a new SbpString of L size with T termination.
pub fn fixed_sbp_string<T, const L: usize>(data: &str) -> SbpString<[u8; L], T> {
    let mut arr = [0u8; L];
    arr[0..data.len()].copy_from_slice(data.as_bytes());
    SbpString::new(arr)
}

/// Notify the frontend of a Connection notification.
pub fn send_conn_notification(client_sender: &BoxedClientSender, message: String) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let mut status = msg.init_connection_notification();
    status.set_message(&message);
    client_sender.send_data(serialize_capnproto_builder(builder));
}

/// Notify the frontend of an [ConnectionState] change.
pub fn send_conn_state(app_state: ConnectionState, client_sender: &BoxedClientSender) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
    let mut status = msg.init_status();
    status.set_text(&app_state.to_string());
    client_sender.send_data(serialize_capnproto_builder(builder));
}

pub fn refresh_connection_frontend(client_sender: &BoxedClientSender, shared_state: &SharedState) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

    let mut connection_status = msg.init_connection_status();
    connection_status.set_console_version(&shared_state.console_version());
    let mut ports: Vec<String> = vec![];
    if let Ok(ports_) = &mut available_ports() {
        // TODO(johnmichael.burke@) [CPP-114]Find solution to this hack for Linux serialport.
        ports = ports_
            .iter_mut()
            .map(|x| x.port_name.replace("/sys/class/tty/", "/dev/"))
            .collect();
    }

    let previous_configs = shared_state.serial_history();

    // Filter out previous devices that aren't currently connected
    let filtered_previous: Vec<(&String, &SerialConfig)> = previous_configs
        .iter()
        .filter(|(device, _)| ports.iter().any(|curr_serial| &curr_serial == device))
        .collect();

    match filtered_previous.len() {
        0 => {
            connection_status
                .reborrow()
                .get_last_serial_device()
                .set_none(());
        }
        n => {
            let last_device = filtered_previous.index(n - 1).0;
            connection_status
                .reborrow()
                .get_last_serial_device()
                .set_port(last_device);
        }
    };

    let mut previous_serial_configs = connection_status
        .reborrow()
        .init_previous_serial_configs(filtered_previous.len() as u32);

    for (i, (device, config)) in filtered_previous.iter().enumerate() {
        let mut entry = previous_serial_configs.reborrow().get(i as u32);

        entry.set_device(device);
        entry.set_baudrate(config.baud);
        entry.set_flow_control(
            AVAILABLE_FLOWS
                .get(config.flow as usize)
                .expect("Unknown flow value"),
        );
    }

    let mut available_ports = connection_status
        .reborrow()
        .init_available_ports(ports.len() as u32);

    for (i, serialportinfo) in ports.iter().enumerate() {
        available_ports.set(i as u32, serialportinfo);
    }

    let mut available_baudrates = connection_status
        .reborrow()
        .init_available_baudrates(AVAILABLE_BAUDRATES.len() as u32);

    for (i, baudrate) in AVAILABLE_BAUDRATES.iter().enumerate() {
        available_baudrates.set(i as u32, *baudrate);
    }

    let mut available_flows = connection_status
        .reborrow()
        .init_available_flows(AVAILABLE_FLOWS.len() as u32);

    for (i, flow) in AVAILABLE_FLOWS.iter().enumerate() {
        available_flows.set(i as u32, flow.as_ref());
    }

    let addresses = shared_state.address_history();
    let hosts: IndexSet<String> = addresses
        .clone()
        .into_iter()
        .map(|addy| addy.host)
        .rev()
        .collect();
    let ports: IndexSet<u16> = addresses.into_iter().map(|addy| addy.port).rev().collect();
    let mut prevous_hosts = connection_status
        .reborrow()
        .init_previous_hosts(hosts.len() as u32);

    for (i, hosts) in hosts.iter().enumerate() {
        prevous_hosts.set(i as u32, hosts);
    }

    let mut prevous_ports = connection_status
        .reborrow()
        .init_previous_ports(ports.len() as u32);

    for (i, ports) in ports.iter().enumerate() {
        prevous_ports.set(i as u32, *ports);
    }
    let mut files = shared_state.file_history();
    files.reverse();
    let mut prevous_files = connection_status
        .reborrow()
        .init_previous_files(files.len() as u32);

    for (i, filename) in files.iter().enumerate() {
        prevous_files.set(i as u32, filename);
    }

    let prev_conn_type = shared_state.connection_type_history().to_string();
    connection_status.set_previous_connection_type(&prev_conn_type);

    client_sender.send_data(serialize_capnproto_builder(builder));
}

pub fn serialize_capnproto_builder(builder: Builder<HeapAllocator>) -> Vec<u8> {
    let mut msg_bytes: Vec<u8> = vec![];
    serialize::write_message(&mut msg_bytes, &builder).expect(CAP_N_PROTO_SERIALIZATION_FAILURE);
    msg_bytes
}

pub fn refresh_loggingbar(client_sender: &BoxedClientSender, shared_state: &SharedState) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

    let mut logging_bar_status = msg.init_logging_bar_status();
    let csv_logging = shared_state.csv_logging();
    logging_bar_status.set_csv_logging(csv_logging.to_bool());
    let sbp_logging = shared_state.sbp_logging();
    logging_bar_status.set_sbp_logging(sbp_logging);
    let sbp_logging_format = shared_state.sbp_logging_format();
    logging_bar_status.set_sbp_logging_format(&sbp_logging_format.to_string());

    let mut folders = shared_state.folder_history();
    folders.reverse();
    let mut prevous_folders = logging_bar_status
        .reborrow()
        .init_previous_folders(folders.len() as u32);

    for (i, folder) in folders.iter().enumerate() {
        prevous_folders.set(i as u32, folder);
    }

    client_sender.send_data(serialize_capnproto_builder(builder));
}

pub fn refresh_loggingbar_recording(
    client_sender: &BoxedClientSender,
    size: u64,
    duration: u64,
    filename: Option<String>,
) {
    let mut builder = Builder::new_default();
    let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

    let mut logging_bar_status = msg.init_logging_bar_recording_status();
    logging_bar_status.set_recording_duration_sec(duration);
    logging_bar_status.set_recording_size(size);
    if let Some(filename_) = filename {
        logging_bar_status
            .reborrow()
            .get_recording_filename()
            .set_filename(&filename_);
    } else {
        logging_bar_status
            .reborrow()
            .get_recording_filename()
            .set_none(());
    }
    client_sender.send_data(serialize_capnproto_builder(builder));
}

pub fn signal_key_label(
    key: (SignalCodes, i16),
    extra: Option<&HashMap<i16, i16>>,
) -> (Option<String>, Option<String>, Option<String>) {
    let (code, sat) = key;
    let code_lbl = Some(code.to_string());
    let mut freq_lbl = None;
    let id_lbl;
    let default_extra = HashMap::new();
    let extra = extra.unwrap_or(&default_extra);

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

/// Convert bytes to the equivalent human readable format as a string.
///
/// - Modified based on https://stackoverflow.com/a/1094933
///
/// # Parameters
///
/// - `bytes`: The number of bytes to convert.
/// # Result
///
/// - The number of bytes converted to a human readable string.
pub fn bytes_to_human_readable(bytes: u128) -> String {
    let mut bytes = bytes as f64;
    for unit in ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"].iter() {
        if bytes < 1024.0 {
            return format!("{:3.1}{}", bytes as f64, unit);
        } else {
            bytes /= 1024.0;
        }
    }
    format!("{:.1}YB", bytes)
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

/// Normalize CPU usage from [0,1000] to [0,100].
///
/// # Parameters
/// - `cpu`: The CPU usage value to be normalized.
///
/// # Returns
/// - The normalized CPU usage value.
pub fn normalize_cpu_usage(cpu: u16) -> f64 {
    cpu as f64 / 10_f64
}

/// Convert centiCelsius to Celsius.
///
/// # Parameters
/// - `cc`: Value in centiCelsius.
///
/// # Returns
/// - Value in Celsius.
pub fn cc_to_c(cc: i16) -> f64 {
    cc as f64 / 1.0e+2_f64
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

/// Attempts to format a float into a string such that the sign and decimal
/// point are consistently aligned.
///
/// # Parameters:
/// - `num`: The float to format
/// - `width`: How wide the resulting string should be padded to (if it is
///   shorter than it should be)
/// - `precision`: The maximum number of digits expected before the decimal
///   place. This informs how many digits of precision are permitted.
///
/// # Returns
/// - The formatted string
///
/// # Examples
/// - `format_fixed_decimal_and_sign(0.1, 8, 3)`:    `"   0.100"`
/// - `format_fixed_decimal_and_sign(-320.6, 8, 3)`: `"-320.600"`
pub fn format_fixed_decimal_and_sign(num: f32, width: usize, precision: usize) -> String {
    let sign = if num < 0. { "-" } else { " " };
    format!(
        "{}{: >width$.prec$}",
        sign,
        num.abs(),
        width = width - 1,
        prec = precision
    )
}

/// Formats bools with uppercase T's and F's
pub fn format_bool(b: bool) -> String {
    if b { "True" } else { "False" }.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piksi_tools_constants::*;

    fn float_eq(f1: f64, f2: f64) -> bool {
        f64::abs(f1 - f2) <= f64::EPSILON
    }

    #[test]
    fn bytes_to_human_readable_test() {
        ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"]
            .iter()
            .enumerate()
            .for_each(|(idx, &unit)| {
                assert_eq!(
                    bytes_to_human_readable(u128::pow(1024, idx as u32)),
                    format!("{:3.1}{}", 1.0, unit)
                );
            });
        assert_eq!(
            bytes_to_human_readable(u128::pow(1024, 8)),
            format!("{:.1}YB", 1.0)
        );
        assert_eq!(
            bytes_to_human_readable(u128::pow(1024, 9)),
            format!("{:.1}YB", 1024.0)
        );
        assert_eq!(bytes_to_human_readable(230123123), "219.5MB");
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
            Some(&extra),
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2P_STR);
        assert_eq!(freq_lbl.unwrap(), "F+30");
        assert_eq!(id_lbl.unwrap(), "R30");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGloL2Of, SignalCodes::CodeGloL2Of as i16),
            Some(&extra),
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2OF_STR);
        assert_eq!(freq_lbl.unwrap(), "F+04");
        assert_eq!(id_lbl.unwrap(), "R04");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeSbasL5Q, SignalCodes::CodeSbasL5Q as i16),
            Some(&extra),
        );
        assert_eq!(code_lbl.unwrap(), SBAS_L5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "S 42");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeBds3B5Q, SignalCodes::CodeBds3B5Q as i16),
            Some(&extra),
        );
        assert_eq!(code_lbl.unwrap(), BDS3_B5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "C48");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeQzsL2Cx, SignalCodes::CodeQzsL2Cx as i16),
            Some(&extra),
        );
        assert_eq!(code_lbl.unwrap(), QZS_L2CX_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "J 37");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGalE8X, SignalCodes::CodeGalE8X as i16),
            Some(&extra),
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

    #[test]
    #[rustfmt::skip]
    fn format_fixed_sign_test() {
        #[rustfmt::skip]
        assert_eq!(format_fixed_decimal_and_sign(0.1, 8, 3),    "   0.100");
        assert_eq!(format_fixed_decimal_and_sign(20.0, 8, 3),   "  20.000");
        assert_eq!(format_fixed_decimal_and_sign(100.0, 8, 3),  " 100.000");
        assert_eq!(format_fixed_decimal_and_sign(-1.0, 8, 3),   "-  1.000");
        assert_eq!(format_fixed_decimal_and_sign(-30.4, 8, 3),  "- 30.400");
        assert_eq!(format_fixed_decimal_and_sign(-320.6, 8, 3), "-320.600");

        assert_eq!(format_fixed_decimal_and_sign(0.1953421, 6, 1), "   0.2");
        assert_eq!(format_fixed_decimal_and_sign(-200.1, 6, 1),    "-200.1");
    }
}
