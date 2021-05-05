#![allow(clippy::all)]
#![allow(unknown_lints)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
// cargo-deps: strum,strum_macros
extern crate strum;
extern crate strum_macros;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
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
    #[strum(serialize = "CONNECTED")]
    CONNECTED,
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum ApplicationStates {
    #[strum(serialize = "CLOSE")]
    CLOSE,
    #[strum(serialize = "CONNECTED")]
    CONNECTED,
    #[strum(serialize = "DISCONNECTED")]
    DISCONNECTED,
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum MessageKeys {
    #[strum(serialize = "status")]
    STATUS,
    #[strum(serialize = "connectedStatus")]
    CONNECTED_STATUS,
    #[strum(serialize = "bottomNavbarStatus")]
    BOTTOM_NAVBAR_STATUS,
    #[strum(serialize = "solutionPositionStatus")]
    SOLUTION_POSITION_STATUS,
    #[strum(serialize = "solutionTableStatus")]
    SOLUTION_TABLE_STATUS,
    #[strum(serialize = "solutionVelocityStatus")]
    SOLUTION_VELOCITY_STATUS,
    #[strum(serialize = "trackingSignalsStatus")]
    TRACKING_SIGNALS_STATUS,
    #[strum(serialize = "observationStatus")]
    OBSERVATION_STATUS,
    #[strum(serialize = "connectRequest")]
    CONNECT_REQUEST,
    #[strum(serialize = "fileRequest")]
    FILE_REQUEST,
    #[strum(serialize = "tcpRequest")]
    TCP_REQUEST,
    #[strum(serialize = "serialRequest")]
    SERIAL_REQUEST,
    #[strum(serialize = "disconnectRequest")]
    DISCONNECT_REQUEST,
    #[strum(serialize = "pauseRequest")]
    PAUSE_REQUEST,
    #[strum(serialize = "serialRefreshRequest")]
    SERIAL_REFRESH_REQUEST,
    #[strum(serialize = "trackingSignalsStatusFront")]
    TRACKING_SIGNALS_STATUS_FRONT,
    #[strum(serialize = "solutionVelocityStatusFront")]
    SOLUTION_VELOCITY_STATUS_FRONT,
    #[strum(serialize = "solutionPositionStatusUnitFront")]
    SOLUTION_POSITION_STATUS_UNIT_FRONT,
    #[strum(serialize = "solutionPositionStatusButtonFront")]
    SOLUTION_POSITION_STATUS_BUTTON_FRONT,
    #[strum(serialize = "logAppend")]
    LOG_APPEND,
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum QTKeys {
    #[strum(serialize = "QVariantList")]
    QVARIANTLIST,
}
