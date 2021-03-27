from enum import Enum

class Keys(str, Enum):
    POINTS = "POINTS"
    LABELS = "LABELS"
    CHECK_LABELS = "CHECK_LABELS"
    COLORS = "COLORS"
    MAX = "MAX"
    MIN = "MIN"

class ApplicationStates(str, Enum):
    CLOSE = "CLOSE"

class MessageKeys(str, Enum):
    STATUS = "status"
    VELOCITY_STATUS = "velocityStatus"
    TRACKING_STATUS = "trackingStatus"

class QTKeys(str, Enum):
    QVARIANTLIST = "QVariantList"
