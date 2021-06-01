// cargo-deps: strum,strum_macros

#![allow(clippy::upper_case_acronyms)]
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
    #[strum(serialize = "ADVANCED")]
    ADVANCED,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum SbpLogging {
    #[strum(serialize = "OFF")]
    OFF,
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
    #[strum(serialize = "CUR_POINTS")]
    CUR_POINTS,
    #[strum(serialize = "AVAILABLE_PORTS")]
    AVAILABLE_PORTS,
    #[strum(serialize = "AVAILABLE_BAUDRATES")]
    AVAILABLE_BAUDRATES,
    #[strum(serialize = "AVAILABLE_FLOWS")]
    AVAILABLE_FLOWS,
    #[strum(serialize = "AVAILABLE_REFRESH_RATES")]
    AVAILABLE_REFRESH_RATES,
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
    #[strum(serialize = "CONNECTED")]
    CONNECTED,
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
    #[strum(serialize = "CSV_LOGGING")]
    CSV_LOGGING,
    #[strum(serialize = "SBP_LOGGING_LABELS")]
    SBP_LOGGING_LABELS,
    #[strum(serialize = "LOG_LEVEL_LABELS")]
    LOG_LEVEL_LABELS,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum ApplicationStates {
    #[strum(serialize = "CLOSE")]
    CLOSE,
    #[strum(serialize = "CONNECTED")]
    CONNECTED,
    #[strum(serialize = "DISCONNECTED")]
    DISCONNECTED,
}

#[derive(Clone, Debug, Display, EnumString, EnumVariantNames, Eq, Hash, PartialEq)]
pub enum QTKeys {
    #[strum(serialize = "QVariantList")]
    QVARIANTLIST,
}
