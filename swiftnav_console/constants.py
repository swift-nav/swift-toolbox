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
    ADVANCED_SYSTEM_MONITOR = "ADVANCED_SYSTEM_MONITOR"
    ADVANCED_IMU = "ADVANCED_IMU"
    ADVANCED_MAGNETOMETER = "ADVANCED_MAGNETOMETER"
    ADVANCED_NETWORKING = "ADVANCED_NETWORKING"
    ADVANCED_SPECTRUM_ANALYZER = "ADVANCED_SPECTRUM_ANALYZER"


class FusionStatus(str, Enum):
    UNKNOWN = "UNKNOWN"
    WARNING = "WARNING"
    OK = "OK"


class SbpLogging(str, Enum):
    SBP_JSON = "SBP_JSON"
    SBP = "SBP"


class CsvLogging(str, Enum):
    OFF = "OFF"
    ON = "ON"


class LogLevel(str, Enum):
    ERROR = "ERROR"
    WARNING = "WARNING"
    NOTICE = "NOTICE"
    INFO = "INFO"
    DEBUG = "DEBUG"


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
    N_MAX = "N_MAX"
    N_MIN = "N_MIN"
    E_MAX = "E_MAX"
    E_MIN = "E_MIN"
    CUR_POINTS = "CUR_POINTS"
    AVAILABLE_PORTS = "AVAILABLE_PORTS"
    AVAILABLE_BAUDRATES = "AVAILABLE_BAUDRATES"
    AVAILABLE_FLOWS = "AVAILABLE_FLOWS"
    AVAILABLE_REFRESH_RATES = "AVAILABLE_REFRESH_RATES"
    REMOTE = "REMOTE"
    TOW = "TOW"
    WEEK = "WEEK"
    ROWS = "ROWS"
    PREVIOUS_HOSTS = "PREVIOUS_HOSTS"
    PREVIOUS_PORTS = "PREVIOUS_PORTS"
    PREVIOUS_FILES = "PREVIOUS_FILES"
    CONNECTION_STATE = "CONNECTION_STATE"
    PORT = "PORT"
    POS = "POS"
    RTK = "RTK"
    SATS = "SATS"
    CORR_AGE = "CORR_AGE"
    INS = "INS"
    DATA_RATE = "DATA_RATE"
    SOLID_CONNECTION = "SOLID_CONNECTION"
    PREVIOUS_FOLDERS = "PREVIOUS_FOLDERS"
    SBP_LOGGING = "SBP_LOGGING"
    SBP_LOGGING_FORMAT = "SBP_LOGGING_FORMAT"
    CSV_LOGGING = "CSV_LOGGING"
    SBP_LOGGING_LABELS = "SBP_LOGGING_LABELS"
    LOG_LEVEL_LABELS = "LOG_LEVEL_LABELS"
    FIELDS_DATA = "FIELDS_DATA"
    XMIN_OFFSET = "XMIN_OFFSET"
    GNSSPOS = "GNSSPOS"
    GNSSVEL = "GNSSVEL"
    WHEELTICKS = "WHEELTICKS"
    SPEED = "SPEED"
    NHC = "NHC"
    ZEROVEL = "ZEROVEL"
    YMIN = "YMIN"
    YMAX = "YMAX"
    LOG_LEVEL = "LOG_LEVEL"
    CHANNEL = "CHANNEL"
    XMIN = "XMIN"
    XMAX = "XMAX"
    HARDWARE_REVISION = "HARDWARE_REVISION"
    FW_VERSION_CURRENT = "FW_VERSION_CURRENT"
    FW_VERSION_LATEST = "FW_VERSION_LATEST"
    FW_LOCAL_FILENAME = "FW_LOCAL_FILENAME"
    DIRECTORY = "DIRECTORY"
    DOWNLOADING = "DOWNLOADING"
    UPGRADING = "UPGRADING"
    FW_TEXT = "FW_TEXT"
    FILEIO_LOCAL_FILEPATH = "FILEIO_LOCAL_FILEPATH"
    FILEIO_DESTINATION_FILEPATH = "FILEIO_DESTINATION_FILEPATH"
    TITLE = "TITLE"
    IMPORT_STATUS = "IMPORT_STATUS"
    FW_OUTDATED = "FW_OUTDATED"
    FW_V2_OUTDATED = "FW_V2_OUTDATED"
    SERIAL_PROMPT = "SERIAL_PROMPT"
    CONSOLE_OUTDATED = "CONSOLE_OUTDATED"
    CONSOLE_VERSION_CURRENT = "CONSOLE_VERSION_CURRENT"
    CONSOLE_VERSION_LATEST = "CONSOLE_VERSION_LATEST"
    OBS_PERIOD = "OBS_PERIOD"
    OBS_LATENCY = "OBS_LATENCY"
    THREADS_TABLE = "THREADS_TABLE"
    CSAC_TELEM_LIST = "CSAC_TELEM_LIST"
    ZYNQ_TEMP = "ZYNQ_TEMP"
    FE_TEMP = "FE_TEMP"
    CSAC_RECEIVED = "CSAC_RECEIVED"
    RUNNING = "RUNNING"
    NETWORK_INFO = "NETWORK_INFO"
    IP_ADDRESS = "IP_ADDRESS"
    RECOMMENDED_INS_SETTINGS = "RECOMMENDED_INS_SETTINGS"
    NEW_INS_CONFIRMATON = "NEW_INS_CONFIRMATON"
    ANTENNA_STATUS = "ANTENNA_STATUS"
    RECORDING_DURATION_SEC = "RECORDING_DURATION_SEC"
    RECORDING_SIZE = "RECORDING_SIZE"
    LAST_USED_SERIAL_DEVICE = "LAST_USED_SERIAL_DEVICE"
    PREVIOUS_SERIAL_CONFIGS = "PREVIOUS_SERIAL_CONFIGS"
    RECORDING_FILENAME = "RECORDING_FILENAME"
    CONSOLE_VERSION = "CONSOLE_VERSION"


class ConnectionState(str, Enum):
    CLOSED = "CLOSED"
    CONNECTED = "CONNECTED"
    DISCONNECTED = "DISCONNECTED"


class QTKeys(str, Enum):
    QVARIANTLIST = "QVariantList"
    QVARIANT = "QVariant"


class ApplicationMetadata(str, Enum):
    ORGANIZATION_NAME = "Swift Navigation"
    ORGANIZATION_DOMAIN = "swiftnav.com"
    APPLICATION_NAME = "console_pp"
