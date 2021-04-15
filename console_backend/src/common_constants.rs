// cargo-deps: strum,strum_macros
extern crate strum;
extern crate strum_macros;
use strum_macros::{Display, EnumString};

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
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum ApplicationStates {
    #[strum(serialize = "CLOSE")]
    CLOSE,
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum MessageKeys {
    #[strum(serialize = "status")]
    STATUS,
    #[strum(serialize = "solutionPositionStatus")]
    SOLUTION_POSITION_STATUS,
    #[strum(serialize = "solutionTableStatus")]
    SOLUTION_TABLE_STATUS,
    #[strum(serialize = "solutionVelocityStatus")]
    SOLUTION_VELOCITY_STATUS,
    #[strum(serialize = "trackingSignalsStatus")]
    TRACKING_SIGNALS_STATUS,
}

#[derive(Clone, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum QTKeys {
    #[strum(serialize = "QVariantList")]
    QVARIANTLIST,
}
