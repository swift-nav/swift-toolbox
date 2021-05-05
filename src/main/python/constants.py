from enum import Enum


class Tabs(str, Enum):
    TRACKING_SIGNALS = "TRACKING_SIGNALS"
    TRACKING_SKYPLOT = "TRACKING_SKYPLOT"
    SOLUTION_POSITION = "SOLUTION_POSITION"
    SOLUTION_VELOCITY = "SOLUTION_VELOCITY"
    BASELINE = "BASELINE"
    OBSERVATIONS = "OBSERVATIONS"
    SETTINGS = "SETTINGS"
    UPDATE = "UPDATE"
    ADVANCED = "ADVANCED"


class Keys(str, Enum):
    POINTS = "POINTS"
    LABELS = "LABELS"
    CHECK_LABELS = "CHECK_LABELS"
    COLORS = "COLORS"
    MAX = "MAX"
    MIN = "MIN"
    AVAILABLE_UNITS = "AVAILABLE_UNITS"
    ENTRIES = "ENTRIES"
    LAT_MAX = "LAT_MAX"
    LAT_MIN = "LAT_MIN"
    LON_MAX = "LON_MAX"
    LON_MIN = "LON_MIN"
    CUR_POINTS = "CUR_POINTS"
    AVAILABLE_PORTS = "AVAILABLE_PORTS"
    AVAILABLE_BAUDRATES = "AVAILABLE_BAUDRATES"
    AVAILABLE_FLOWS = "AVAILABLE_FLOWS"
    AVAILABLE_REFRESH_RATES = "AVAILABLE_REFRESH_RATES"
    REMOTE = "REMOTE"
    TOW = "TOW"
    WEEK = "WEEK"
    ROWS = "ROWS"
    CONNECTED = "CONNECTED"


class ApplicationStates(str, Enum):
    CLOSE = "CLOSE"
    CONNECTED = "CONNECTED"
    DISCONNECTED = "DISCONNECTED"


class MessageKeys(str, Enum):
    STATUS = "status"
    CONNECTED_STATUS = "connectedStatus"
    BOTTOM_NAVBAR_STATUS = "bottomNavbarStatus"
    SOLUTION_POSITION_STATUS = "solutionPositionStatus"
    SOLUTION_TABLE_STATUS = "solutionTableStatus"
    SOLUTION_VELOCITY_STATUS = "solutionVelocityStatus"
    TRACKING_SIGNALS_STATUS = "trackingSignalsStatus"
    OBSERVATION_STATUS = "observationStatus"
    CONNECT_REQUEST = "connectRequest"
    FILE_REQUEST = "fileRequest"
    TCP_REQUEST = "tcpRequest"
    SERIAL_REQUEST = "serialRequest"
    DISCONNECT_REQUEST = "disconnectRequest"
    PAUSE_REQUEST = "pauseRequest"
    SERIAL_REFRESH_REQUEST = "serialRefreshRequest"
    TRACKING_SIGNALS_STATUS_FRONT = "trackingSignalsStatusFront"
    SOLUTION_VELOCITY_STATUS_FRONT = "solutionVelocityStatusFront"
    SOLUTION_POSITION_STATUS_UNIT_FRONT = "solutionPositionStatusUnitFront"
    SOLUTION_POSITION_STATUS_BUTTON_FRONT = "solutionPositionStatusButtonFront"
    LOG_APPEND = "logAppend"


class QTKeys(str, Enum):
    QVARIANTLIST = "QVariantList"
