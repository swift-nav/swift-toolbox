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
use clap::ValueEnum;
use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Display, ValueEnum, Eq, Hash, PartialEq)]
pub enum Tabs {
    TRACKING_SIGNALS,
    TRACKING_SKYPLOT,
    SOLUTION_POSITION,
    SOLUTION_VELOCITY,
    BASELINE,
    OBSERVATIONS,
    SETTINGS,
    UPDATE,
    ADVANCED_SYSTEM_MONITOR,
    ADVANCED_IMU,
    ADVANCED_MAGNETOMETER,
    ADVANCED_NETWORKING,
    ADVANCED_SPECTRUM_ANALYZER,
    ADVANCED_INS,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum FusionStatus {
    UNKNOWN,
    WARNING,
    OK,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, ValueEnum, Eq, Hash, PartialEq)]
pub enum SbpLogging {
    SBP_JSON,
    SBP,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum CsvLogging {
    OFF,
    ON,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, ValueEnum, Eq, Hash, PartialEq)]
pub enum LogLevel {
    ERROR,
    WARNING,
    NOTICE,
    INFO,
    DEBUG,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum Keys {
    POINTS,
    LABELS,
    CHECK_LABELS,
    COLORS,
    MAX,
    MIN,
    AVAILABLE_UNITS,
    ENTRIES,
    LAT_MAX,
    LAT_MIN,
    LON_MAX,
    LON_MIN,
    N_MAX,
    N_MIN,
    E_MAX,
    E_MIN,
    CUR_POINTS,
    AVAILABLE_PORTS,
    AVAILABLE_BAUDRATES,
    AVAILABLE_FLOWS,
    AVAILABLE_REFRESH_RATES,
    REMOTE,
    TOW,
    WEEK,
    ROWS,
    PREVIOUS_HOSTS,
    PREVIOUS_PORTS,
    PREVIOUS_FILES,
    CONNECTION_STATE,
    PORT,
    POS,
    RTK,
    SATS,
    CORR_AGE,
    INS,
    DATA_RATE,
    SOLID_CONNECTION,
    PREVIOUS_FOLDERS,
    SBP_LOGGING,
    SBP_LOGGING_FORMAT,
    CSV_LOGGING,
    SBP_LOGGING_LABELS,
    LOG_LEVEL_LABELS,
    FIELDS_DATA,
    XMIN_OFFSET,
    GNSSPOS,
    GNSSVEL,
    WHEELTICKS,
    SPEED,
    NHC,
    ZEROVEL,
    YMIN,
    YMAX,
    LOG_LEVEL,
    CHANNEL,
    XMIN,
    XMAX,
    HARDWARE_REVISION,
    FW_VERSION_CURRENT,
    FW_VERSION_LATEST,
    FW_LOCAL_FILENAME,
    DIRECTORY,
    DOWNLOADING,
    UPGRADING,
    FW_TEXT,
    FILEIO_LOCAL_FILEPATH,
    FILEIO_DESTINATION_FILEPATH,
    TITLE,
    IMPORT_STATUS,
    FW_OUTDATED,
    FW_V2_OUTDATED,
    SERIAL_PROMPT,
    CONSOLE_OUTDATED,
    CONSOLE_VERSION_CURRENT,
    CONSOLE_VERSION_LATEST,
    OBS_PERIOD,
    OBS_LATENCY,
    THREADS_TABLE,
    ZYNQ_TEMP,
    FE_TEMP,
    RUNNING,
    NETWORK_INFO,
    IP_ADDRESS,
    RECOMMENDED_INS_SETTINGS,
    NEW_INS_CONFIRMATON,
    ANTENNA_STATUS,
    RECORDING_DURATION_SEC,
    RECORDING_SIZE,
    LAST_USED_SERIAL_DEVICE,
    PREVIOUS_SERIAL_CONFIGS,
    RECORDING_FILENAME,
    CONSOLE_VERSION,
    PREVIOUS_CONNECTION_TYPE,
    CONNECTION_MESSAGE,
    NOTIFICATION,
    SOLUTION_LINE,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ConnectionState {
    CLOSED,
    CONNECTED,
    DISCONNECTED,
    CONNECTING,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ConnectionType {
    Tcp,
    File,
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
