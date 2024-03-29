//! ```cargo
//! [package]
//! edition = "2018"
//! [dependencies]
//! strum = "*"
//! strum_macros = "*"
//! ```

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::double_parens)] // https://github.com/adsharma/py2many/issues/17
#![allow(clippy::map_identity)]
#![allow(clippy::needless_return)]
#![allow(clippy::print_literal)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_static_lifetimes)] // https://github.com/adsharma/py2many/issues/266
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_vec)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

extern crate strum;
extern crate strum_macros;
use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum Tabs {
    #[strum(serialize = "TRACKING_SIGNALS")]
    TRACKING_SIGNALS,
    #[strum(serialize = "TRACKING_SKYPLOT")]
    TRACKING_SKYPLOT,
    #[strum(serialize = "SOLUTION_POSITION")]
    SOLUTION_POSITION,
    #[strum(serialize = "SOLUTION_VELOCITY")]
    SOLUTION_VELOCITY,
    #[strum(serialize = "BASELINE")]
    BASELINE,
    #[strum(serialize = "OBSERVATIONS")]
    OBSERVATIONS,
    #[strum(serialize = "SETTINGS")]
    SETTINGS,
    #[strum(serialize = "UPDATE")]
    UPDATE,
    #[strum(serialize = "ADVANCED_SYSTEM_MONITOR")]
    ADVANCED_SYSTEM_MONITOR,
    #[strum(serialize = "ADVANCED_IMU")]
    ADVANCED_IMU,
    #[strum(serialize = "ADVANCED_MAGNETOMETER")]
    ADVANCED_MAGNETOMETER,
    #[strum(serialize = "ADVANCED_NETWORKING")]
    ADVANCED_NETWORKING,
    #[strum(serialize = "ADVANCED_SPECTRUM_ANALYZER")]
    ADVANCED_SPECTRUM_ANALYZER,
    #[strum(serialize = "ADVANCED_INS")]
    ADVANCED_INS,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum FusionStatus {
    #[strum(serialize = "UNKNOWN")]
    UNKNOWN,
    #[strum(serialize = "WARNING")]
    WARNING,
    #[strum(serialize = "OK")]
    OK,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum SbpLogging {
    #[strum(serialize = "SBP_JSON")]
    SBP_JSON,
    #[strum(serialize = "SBP")]
    SBP,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum CsvLogging {
    #[strum(serialize = "OFF")]
    OFF,
    #[strum(serialize = "ON")]
    ON,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum LogLevel {
    #[strum(serialize = "ERROR")]
    ERROR,
    #[strum(serialize = "WARNING")]
    WARNING,
    #[strum(serialize = "NOTICE")]
    NOTICE,
    #[strum(serialize = "INFO")]
    INFO,
    #[strum(serialize = "DEBUG")]
    DEBUG,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum Keys {
    #[strum(serialize = "POINTS")]
    POINTS,
    #[strum(serialize = "LABELS")]
    LABELS,
    #[strum(serialize = "CHECK_LABELS")]
    CHECK_LABELS,
    #[strum(serialize = "COLORS")]
    COLORS,
    #[strum(serialize = "MAX")]
    MAX,
    #[strum(serialize = "MIN")]
    MIN,
    #[strum(serialize = "AVAILABLE_UNITS")]
    AVAILABLE_UNITS,
    #[strum(serialize = "ENTRIES")]
    ENTRIES,
    #[strum(serialize = "LAT_MAX")]
    LAT_MAX,
    #[strum(serialize = "LAT_MIN")]
    LAT_MIN,
    #[strum(serialize = "LON_MAX")]
    LON_MAX,
    #[strum(serialize = "LON_MIN")]
    LON_MIN,
    #[strum(serialize = "N_MAX")]
    N_MAX,
    #[strum(serialize = "N_MIN")]
    N_MIN,
    #[strum(serialize = "E_MAX")]
    E_MAX,
    #[strum(serialize = "E_MIN")]
    E_MIN,
    #[strum(serialize = "CUR_POINTS")]
    CUR_POINTS,
    #[strum(serialize = "AVAILABLE_PORTS")]
    AVAILABLE_PORTS,
    #[strum(serialize = "AVAILABLE_BAUDRATES")]
    AVAILABLE_BAUDRATES,
    #[strum(serialize = "AVAILABLE_FLOWS")]
    AVAILABLE_FLOWS,
    #[strum(serialize = "REMOTE")]
    REMOTE,
    #[strum(serialize = "TOW")]
    TOW,
    #[strum(serialize = "WEEK")]
    WEEK,
    #[strum(serialize = "ROWS")]
    ROWS,
    #[strum(serialize = "PREVIOUS_HOSTS")]
    PREVIOUS_HOSTS,
    #[strum(serialize = "PREVIOUS_PORTS")]
    PREVIOUS_PORTS,
    #[strum(serialize = "PREVIOUS_FILES")]
    PREVIOUS_FILES,
    #[strum(serialize = "CONNECTION_STATE")]
    CONNECTION_STATE,
    #[strum(serialize = "PORT")]
    PORT,
    #[strum(serialize = "POS")]
    POS,
    #[strum(serialize = "RTK")]
    RTK,
    #[strum(serialize = "SATS")]
    SATS,
    #[strum(serialize = "CORR_AGE")]
    CORR_AGE,
    #[strum(serialize = "INS")]
    INS,
    #[strum(serialize = "DATA_RATE")]
    DATA_RATE,
    #[strum(serialize = "SOLID_CONNECTION")]
    SOLID_CONNECTION,
    #[strum(serialize = "PREVIOUS_FOLDERS")]
    PREVIOUS_FOLDERS,
    #[strum(serialize = "SBP_LOGGING")]
    SBP_LOGGING,
    #[strum(serialize = "SBP_LOGGING_FORMAT")]
    SBP_LOGGING_FORMAT,
    #[strum(serialize = "SBP_LOGGING_FORMAT_INDEX")]
    SBP_LOGGING_FORMAT_INDEX,
    #[strum(serialize = "CSV_LOGGING")]
    CSV_LOGGING,
    #[strum(serialize = "SBP_LOGGING_LABELS")]
    SBP_LOGGING_LABELS,
    #[strum(serialize = "LOG_LEVEL_LABELS")]
    LOG_LEVEL_LABELS,
    #[strum(serialize = "FIELDS_DATA")]
    FIELDS_DATA,
    #[strum(serialize = "XMIN_OFFSET")]
    XMIN_OFFSET,
    #[strum(serialize = "GNSSPOS")]
    GNSSPOS,
    #[strum(serialize = "GNSSVEL")]
    GNSSVEL,
    #[strum(serialize = "WHEELTICKS")]
    WHEELTICKS,
    #[strum(serialize = "SPEED")]
    SPEED,
    #[strum(serialize = "NHC")]
    NHC,
    #[strum(serialize = "ZEROVEL")]
    ZEROVEL,
    #[strum(serialize = "YMIN")]
    YMIN,
    #[strum(serialize = "YMAX")]
    YMAX,
    #[strum(serialize = "LOG_LEVEL")]
    LOG_LEVEL,
    #[strum(serialize = "CHANNEL")]
    CHANNEL,
    #[strum(serialize = "XMIN")]
    XMIN,
    #[strum(serialize = "XMAX")]
    XMAX,
    #[strum(serialize = "HARDWARE_REVISION")]
    HARDWARE_REVISION,
    #[strum(serialize = "FW_VERSION_CURRENT")]
    FW_VERSION_CURRENT,
    #[strum(serialize = "FW_VERSION_LATEST")]
    FW_VERSION_LATEST,
    #[strum(serialize = "FW_LOCAL_FILENAME")]
    FW_LOCAL_FILENAME,
    #[strum(serialize = "DIRECTORY")]
    DIRECTORY,
    #[strum(serialize = "DOWNLOADING")]
    DOWNLOADING,
    #[strum(serialize = "UPGRADING")]
    UPGRADING,
    #[strum(serialize = "FW_TEXT")]
    FW_TEXT,
    #[strum(serialize = "FILEIO_LOCAL_FILEPATH")]
    FILEIO_LOCAL_FILEPATH,
    #[strum(serialize = "FILEIO_DESTINATION_FILEPATH")]
    FILEIO_DESTINATION_FILEPATH,
    #[strum(serialize = "TITLE")]
    TITLE,
    #[strum(serialize = "IMPORT_STATUS")]
    IMPORT_STATUS,
    #[strum(serialize = "FW_OUTDATED")]
    FW_OUTDATED,
    #[strum(serialize = "FW_V2_OUTDATED")]
    FW_V2_OUTDATED,
    #[strum(serialize = "SERIAL_PROMPT")]
    SERIAL_PROMPT,
    #[strum(serialize = "CONSOLE_OUTDATED")]
    CONSOLE_OUTDATED,
    #[strum(serialize = "CONSOLE_VERSION_CURRENT")]
    CONSOLE_VERSION_CURRENT,
    #[strum(serialize = "CONSOLE_VERSION_LATEST")]
    CONSOLE_VERSION_LATEST,
    #[strum(serialize = "OBS_PERIOD")]
    OBS_PERIOD,
    #[strum(serialize = "OBS_LATENCY")]
    OBS_LATENCY,
    #[strum(serialize = "THREADS_TABLE")]
    THREADS_TABLE,
    #[strum(serialize = "ZYNQ_TEMP")]
    ZYNQ_TEMP,
    #[strum(serialize = "FE_TEMP")]
    FE_TEMP,
    #[strum(serialize = "RUNNING")]
    RUNNING,
    #[strum(serialize = "NETWORK_INFO")]
    NETWORK_INFO,
    #[strum(serialize = "IP_ADDRESS")]
    IP_ADDRESS,
    #[strum(serialize = "RECOMMENDED_INS_SETTINGS")]
    RECOMMENDED_INS_SETTINGS,
    #[strum(serialize = "NEW_INS_CONFIRMATON")]
    NEW_INS_CONFIRMATON,
    #[strum(serialize = "ANTENNA_STATUS")]
    ANTENNA_STATUS,
    #[strum(serialize = "RECORDING_START_TIME")]
    RECORDING_START_TIME,
    #[strum(serialize = "RECORDING_SIZE")]
    RECORDING_SIZE,
    #[strum(serialize = "LAST_USED_SERIAL_DEVICE")]
    LAST_USED_SERIAL_DEVICE,
    #[strum(serialize = "PREVIOUS_SERIAL_CONFIGS")]
    PREVIOUS_SERIAL_CONFIGS,
    #[strum(serialize = "RECORDING_FILENAME")]
    RECORDING_FILENAME,
    #[strum(serialize = "CONSOLE_VERSION")]
    CONSOLE_VERSION,
    #[strum(serialize = "PREVIOUS_CONNECTION_TYPE")]
    PREVIOUS_CONNECTION_TYPE,
    #[strum(serialize = "CONNECTION_MESSAGE")]
    CONNECTION_MESSAGE,
    #[strum(serialize = "NOTIFICATION")]
    NOTIFICATION,
    #[strum(serialize = "SOLUTION_LINE")]
    SOLUTION_LINE,
    #[strum(serialize = "NTRIP_DISPLAY")]
    NTRIP_DISPLAY,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ConnectionState {
    #[strum(serialize = "CLOSED")]
    CLOSED,
    #[strum(serialize = "CONNECTED")]
    CONNECTED,
    #[strum(serialize = "DISCONNECTED")]
    DISCONNECTED,
    #[strum(serialize = "CONNECTING")]
    CONNECTING,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ConnectionType {
    #[strum(serialize = "Tcp")]
    Tcp,
    #[strum(serialize = "File")]
    File,
    #[strum(serialize = "Serial")]
    Serial,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum QTKeys {
    #[strum(serialize = "QVariantList")]
    QVARIANTLIST,
    #[strum(serialize = "QVariant")]
    QVARIANT,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ApplicationMetadata {
    #[strum(serialize = "Swift Navigation")]
    ORGANIZATION_NAME,
    #[strum(serialize = "swiftnav.com")]
    ORGANIZATION_DOMAIN,
    #[strum(serialize = "Swift Console")]
    APPLICATION_NAME,
}
