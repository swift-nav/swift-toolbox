from enum import Enum


class Keys(str, Enum):
    POINTS = "POINTS"
    LABELS = "LABELS"
    CHECK_LABELS = "CHECK_LABELS"
    COLORS = "COLORS"
    MAX = "MAX"
    MIN = "MIN"
    AVAILABLE_UNITS = "AVAILABLE_UNITS"
    ENTRIES = "ENTRIES"


class ApplicationStates(str, Enum):
    CLOSE = "CLOSE"


class MessageKeys(str, Enum):
    STATUS = "status"
    SOLUTION_TABLE_STATUS = "solutionTableStatus"
    SOLUTION_VELOCITY_STATUS = "solutionVelocityStatus"
    TRACKING_SIGNALS_STATUS = "trackingSignalsStatus"


class QTKeys(str, Enum):
    QVARIANTLIST = "QVariantList"
