use chrono::{prelude::*, DateTime, Duration, Local, TimeZone, Utc};
use std::fmt::Display;

use crate::constants::{DECODED_THIS_SESSION, FACTORY_DEFAULT, NON_VOLATILE_MEMORY, UNKNOWN};
use crate::types::UtcDateTime;

/// Get string corresponding to UTC source.
///
/// # Parameters
/// - `utc_flags`: The utc flags to decipher and extract source.
///
/// # Returns
/// - The string corresponding to the utc source.
pub fn utc_source(utc_flags: u8) -> String {
    let source_str = match (utc_flags >> 3) & 0x3 {
        0 => FACTORY_DEFAULT,
        1 => NON_VOLATILE_MEMORY,
        2 => DECODED_THIS_SESSION,
        _ => UNKNOWN,
    };
    String::from(source_str)
}

/// Get UTC date time.
/// Code taken from ICBINS/src/runner.rs.
///
/// # Parameters
/// - `year`: The datetime year.
/// - `month`: The datetime month.
/// - `day`: The datetime day.
/// - `hours`: The datetime hours.
/// - `minutes`: The datetime minutes.
/// - `seconds`: The datetime seconds.
/// - `nanoseconds`: The datetime nanoseconds.
pub fn utc_time(
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    nanoseconds: u32,
) -> UtcDateTime {
    Utc.ymd(year, month, day)
        .and_hms_nano(hours, minutes, seconds, nanoseconds)
}

/// Return generic datetime as date and seconds.
///
/// # Parameters
/// - `datetm`: The datetime to be converted into partial date and seconds strings.
///
/// # Returns:
/// - Partial datetime string and seconds/microseconds string.
pub fn datetime_to_string_and_seconds<T>(datetm: DateTime<T>) -> (String, f64)
where
    T: TimeZone,
    T::Offset: Display,
{
    let seconds = datetm.second() as f64 + datetm.nanosecond() as f64 / 1e9_f64;
    (datetm.format("%Y-%m-%d %H:%M").to_string(), seconds)
}

/// Returns gps time as a string date and precise seconds string.
///
/// # Parameters
/// - `week`: The week number.
/// - `gnss_tow`: The GPS time of week in seconds.
///
/// # Returns
/// - GPS Date string and Seconds float.
pub fn convert_gps_time_to_logging_format(
    week: Option<u16>,
    gnss_tow: f64,
) -> (Option<String>, Option<f64>) {
    let mut t_gps_date = None;
    let mut t_gps_secs = None;

    if let Some(wn) = week {
        if gnss_tow > 0_f64 {
            let t_gps = Utc.ymd(1980, 1, 6).and_hms(0, 0, 0)
                + Duration::weeks(wn as i64)
                + Duration::seconds(gnss_tow as i64);
            let (t_gps_date_, t_gps_secs_) = datetime_to_string_and_seconds(t_gps);
            t_gps_date = Some(t_gps_date_);
            t_gps_secs = Some(t_gps_secs_);
        }
    }
    (t_gps_date, t_gps_secs)
}

/// Returns local time as a string date and precise seconds string.
///
/// # Returns
/// - Local Date string and Seconds float.
pub fn convert_local_time_to_logging_format() -> (String, f64) {
    let local_t = Local::now();
    let (t_local_date, t_local_secs) = datetime_to_string_and_seconds(local_t);
    (t_local_date, t_local_secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utc_source_test() {
        assert_eq!(utc_source(5_u8), String::from(FACTORY_DEFAULT));
        assert_eq!(utc_source(8_u8), String::from(NON_VOLATILE_MEMORY));
        assert_eq!(utc_source(16_u8), String::from(DECODED_THIS_SESSION));
        assert_eq!(utc_source(255_u8), String::from(UNKNOWN));
    }
}
