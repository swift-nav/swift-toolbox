# To use this code, make sure you
#
#     import json
#
# and then, to convert JSON from a string, do
#
#     result = message_from_dict(json.loads(json_string))

from typing import List, Any, Optional, TypeVar, Callable, Type, cast
from enum import Enum


T = TypeVar("T")
EnumT = TypeVar("EnumT", bound=Enum)


def from_list(f: Callable[[Any], T], x: Any) -> List[T]:
    assert isinstance(x, list)
    return [f(y) for y in x]


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def from_float(x: Any) -> float:
    assert isinstance(x, (float, int)) and not isinstance(x, bool)
    return float(x)


def to_float(x: Any) -> float:
    assert isinstance(x, float)
    return x


def from_int(x: Any) -> int:
    assert isinstance(x, int) and not isinstance(x, bool)
    return x


def to_class(c: Type[T], x: Any) -> dict:
    assert isinstance(x, c)
    return cast(Any, x).to_dict()


def from_bool(x: Any) -> bool:
    assert isinstance(x, bool)
    return x


def to_enum(c: Type[EnumT], x: Any) -> EnumT:
    assert isinstance(x, c)
    return x.value


def from_none(x: Any) -> Any:
    assert x is None
    return x


def from_union(fs, x):
    for f in fs:
        try:
            return f(x)
        except:
            pass
    assert False


class AdvancedInsStatus:
    data: List[List[str]]
    fields_data: List[float]

    def __init__(self, data: List[List[str]], fields_data: List[float]) -> None:
        self.data = data
        self.fields_data = fields_data

    @staticmethod
    def from_dict(obj: Any) -> 'AdvancedInsStatus':
        assert isinstance(obj, dict)
        data = from_list(lambda x: from_list(from_str, x), obj.get("data"))
        fields_data = from_list(from_float, obj.get("fields_data"))
        return AdvancedInsStatus(data, fields_data)

    def to_dict(self) -> dict:
        result: dict = {}
        result["data"] = from_list(lambda x: from_list(from_str, x), self.data)
        result["fields_data"] = from_list(to_float, self.fields_data)
        return result


class AdvancedMagnetometerStatus:
    data: List[List[str]]
    ymax: float
    ymin: float

    def __init__(self, data: List[List[str]], ymax: float, ymin: float) -> None:
        self.data = data
        self.ymax = ymax
        self.ymin = ymin

    @staticmethod
    def from_dict(obj: Any) -> 'AdvancedMagnetometerStatus':
        assert isinstance(obj, dict)
        data = from_list(lambda x: from_list(from_str, x), obj.get("data"))
        ymax = from_float(obj.get("ymax"))
        ymin = from_float(obj.get("ymin"))
        return AdvancedMagnetometerStatus(data, ymax, ymin)

    def to_dict(self) -> dict:
        result: dict = {}
        result["data"] = from_list(lambda x: from_list(from_str, x), self.data)
        result["ymax"] = to_float(self.ymax)
        result["ymin"] = to_float(self.ymin)
        return result


class Point:
    x: float
    y: float

    def __init__(self, x: float, y: float) -> None:
        self.x = x
        self.y = y

    @staticmethod
    def from_dict(obj: Any) -> 'Point':
        assert isinstance(obj, dict)
        x = from_float(obj.get("x"))
        y = from_float(obj.get("y"))
        return Point(x, y)

    def to_dict(self) -> dict:
        result: dict = {}
        result["x"] = to_float(self.x)
        result["y"] = to_float(self.y)
        return result


class AdvancedSpectrumAnalyzerStatus:
    channel: int
    data: List[Point]
    xmax: float
    ymax: float
    ymin: float

    def __init__(self, channel: int, data: List[Point], xmax: float, ymax: float, ymin: float) -> None:
        self.channel = channel
        self.data = data
        self.xmax = xmax
        self.ymax = ymax
        self.ymin = ymin

    @staticmethod
    def from_dict(obj: Any) -> 'AdvancedSpectrumAnalyzerStatus':
        assert isinstance(obj, dict)
        channel = from_int(obj.get("channel"))
        data = from_list(Point.from_dict, obj.get("data"))
        xmax = from_float(obj.get("xmax"))
        ymax = from_float(obj.get("ymax"))
        ymin = from_float(obj.get("ymin"))
        return AdvancedSpectrumAnalyzerStatus(channel, data, xmax, ymax, ymin)

    def to_dict(self) -> dict:
        result: dict = {}
        result["channel"] = from_int(self.channel)
        result["data"] = from_list(lambda x: to_class(Point, x), self.data)
        result["xmax"] = to_float(self.xmax)
        result["ymax"] = to_float(self.ymax)
        result["ymin"] = to_float(self.ymin)
        return result


class AdvancedSpectrumAnalyzerStatusFront:
    channel: int

    def __init__(self, channel: int) -> None:
        self.channel = channel

    @staticmethod
    def from_dict(obj: Any) -> 'AdvancedSpectrumAnalyzerStatusFront':
        assert isinstance(obj, dict)
        channel = from_int(obj.get("channel"))
        return AdvancedSpectrumAnalyzerStatusFront(channel)

    def to_dict(self) -> dict:
        result: dict = {}
        result["channel"] = from_int(self.channel)
        return result


class BaselinePlotStatus:
    cur_data: List[List[Point]]
    data: List[List[Point]]
    e_max: float
    e_min: float
    n_max: float
    n_min: float

    def __init__(self, cur_data: List[List[Point]], data: List[List[Point]], e_max: float, e_min: float, n_max: float, n_min: float) -> None:
        self.cur_data = cur_data
        self.data = data
        self.e_max = e_max
        self.e_min = e_min
        self.n_max = n_max
        self.n_min = n_min

    @staticmethod
    def from_dict(obj: Any) -> 'BaselinePlotStatus':
        assert isinstance(obj, dict)
        cur_data = from_list(lambda x: from_list(Point.from_dict, x), obj.get("cur_data"))
        data = from_list(lambda x: from_list(Point.from_dict, x), obj.get("data"))
        e_max = from_float(obj.get("e_max"))
        e_min = from_float(obj.get("e_min"))
        n_max = from_float(obj.get("n_max"))
        n_min = from_float(obj.get("n_min"))
        return BaselinePlotStatus(cur_data, data, e_max, e_min, n_max, n_min)

    def to_dict(self) -> dict:
        result: dict = {}
        result["cur_data"] = from_list(lambda x: from_list(lambda x: to_class(Point, x), x), self.cur_data)
        result["data"] = from_list(lambda x: from_list(lambda x: to_class(Point, x), x), self.data)
        result["e_max"] = to_float(self.e_max)
        result["e_min"] = to_float(self.e_min)
        result["n_max"] = to_float(self.n_max)
        result["n_min"] = to_float(self.n_min)
        return result


class BaselinePlotStatusButtonFront:
    clear: bool
    pause: bool
    reset_filters: bool

    def __init__(self, clear: bool, pause: bool, reset_filters: bool) -> None:
        self.clear = clear
        self.pause = pause
        self.reset_filters = reset_filters

    @staticmethod
    def from_dict(obj: Any) -> 'BaselinePlotStatusButtonFront':
        assert isinstance(obj, dict)
        clear = from_bool(obj.get("clear"))
        pause = from_bool(obj.get("pause"))
        reset_filters = from_bool(obj.get("reset_filters"))
        return BaselinePlotStatusButtonFront(clear, pause, reset_filters)

    def to_dict(self) -> dict:
        result: dict = {}
        result["clear"] = from_bool(self.clear)
        result["pause"] = from_bool(self.pause)
        result["reset_filters"] = from_bool(self.reset_filters)
        return result


class KeyValuePair:
    key: str
    pair: str

    def __init__(self, key: str, pair: str) -> None:
        self.key = key
        self.pair = pair

    @staticmethod
    def from_dict(obj: Any) -> 'KeyValuePair':
        assert isinstance(obj, dict)
        key = from_str(obj.get("key"))
        pair = from_str(obj.get("pair"))
        return KeyValuePair(key, pair)

    def to_dict(self) -> dict:
        result: dict = {}
        result["key"] = from_str(self.key)
        result["pair"] = from_str(self.pair)
        return result


class BaselineTableStatus:
    data: List[KeyValuePair]

    def __init__(self, data: List[KeyValuePair]) -> None:
        self.data = data

    @staticmethod
    def from_dict(obj: Any) -> 'BaselineTableStatus':
        assert isinstance(obj, dict)
        data = from_list(KeyValuePair.from_dict, obj.get("data"))
        return BaselineTableStatus(data)

    def to_dict(self) -> dict:
        result: dict = {}
        result["data"] = from_list(lambda x: to_class(KeyValuePair, x), self.data)
        return result


class DisconnectRequest:
    disconnect: bool

    def __init__(self, disconnect: bool) -> None:
        self.disconnect = disconnect

    @staticmethod
    def from_dict(obj: Any) -> 'DisconnectRequest':
        assert isinstance(obj, dict)
        disconnect = from_bool(obj.get("disconnect"))
        return DisconnectRequest(disconnect)

    def to_dict(self) -> dict:
        result: dict = {}
        result["disconnect"] = from_bool(self.disconnect)
        return result


class FileRequest:
    filename: str

    def __init__(self, filename: str) -> None:
        self.filename = filename

    @staticmethod
    def from_dict(obj: Any) -> 'FileRequest':
        assert isinstance(obj, dict)
        filename = from_str(obj.get("filename"))
        return FileRequest(filename)

    def to_dict(self) -> dict:
        result: dict = {}
        result["filename"] = from_str(self.filename)
        return result


class FusionStatusFlagsStatus:
    gnsspos: str
    gnssvel: str
    nhc: str
    speedd: str
    wheelticks: str
    zerovel: str

    def __init__(self, gnsspos: str, gnssvel: str, nhc: str, speedd: str, wheelticks: str, zerovel: str) -> None:
        self.gnsspos = gnsspos
        self.gnssvel = gnssvel
        self.nhc = nhc
        self.speedd = speedd
        self.wheelticks = wheelticks
        self.zerovel = zerovel

    @staticmethod
    def from_dict(obj: Any) -> 'FusionStatusFlagsStatus':
        assert isinstance(obj, dict)
        gnsspos = from_str(obj.get("gnsspos"))
        gnssvel = from_str(obj.get("gnssvel"))
        nhc = from_str(obj.get("nhc"))
        speedd = from_str(obj.get("speedd"))
        wheelticks = from_str(obj.get("wheelticks"))
        zerovel = from_str(obj.get("zerovel"))
        return FusionStatusFlagsStatus(gnsspos, gnssvel, nhc, speedd, wheelticks, zerovel)

    def to_dict(self) -> dict:
        result: dict = {}
        result["gnsspos"] = from_str(self.gnsspos)
        result["gnssvel"] = from_str(self.gnssvel)
        result["nhc"] = from_str(self.nhc)
        result["speedd"] = from_str(self.speedd)
        result["wheelticks"] = from_str(self.wheelticks)
        result["zerovel"] = from_str(self.zerovel)
        return result


class LogLevel(Enum):
    DEBUG = "Debug"
    ERROR = "Error"
    INFO = "Info"
    TRACE = "Trace"
    WARN = "Warn"


class LogEntry:
    level: LogLevel
    line: str
    timestamp: str

    def __init__(self, level: LogLevel, line: str, timestamp: str) -> None:
        self.level = level
        self.line = line
        self.timestamp = timestamp

    @staticmethod
    def from_dict(obj: Any) -> 'LogEntry':
        assert isinstance(obj, dict)
        level = LogLevel(obj.get("level"))
        line = from_str(obj.get("line"))
        timestamp = from_str(obj.get("timestamp"))
        return LogEntry(level, line, timestamp)

    def to_dict(self) -> dict:
        result: dict = {}
        result["level"] = to_enum(LogLevel, self.level)
        result["line"] = from_str(self.line)
        result["timestamp"] = from_str(self.timestamp)
        return result


class LogAppend:
    entries: List[LogEntry]

    def __init__(self, entries: List[LogEntry]) -> None:
        self.entries = entries

    @staticmethod
    def from_dict(obj: Any) -> 'LogAppend':
        assert isinstance(obj, dict)
        entries = from_list(LogEntry.from_dict, obj.get("entries"))
        return LogAppend(entries)

    def to_dict(self) -> dict:
        result: dict = {}
        result["entries"] = from_list(lambda x: to_class(LogEntry, x), self.entries)
        return result


class LogLevelFront:
    log_level: str

    def __init__(self, log_level: str) -> None:
        self.log_level = log_level

    @staticmethod
    def from_dict(obj: Any) -> 'LogLevelFront':
        assert isinstance(obj, dict)
        log_level = from_str(obj.get("log_level"))
        return LogLevelFront(log_level)

    def to_dict(self) -> dict:
        result: dict = {}
        result["log_level"] = from_str(self.log_level)
        return result


class LoggingBarFront:
    csv_logging: bool
    directory: str
    sbp_logging: str

    def __init__(self, csv_logging: bool, directory: str, sbp_logging: str) -> None:
        self.csv_logging = csv_logging
        self.directory = directory
        self.sbp_logging = sbp_logging

    @staticmethod
    def from_dict(obj: Any) -> 'LoggingBarFront':
        assert isinstance(obj, dict)
        csv_logging = from_bool(obj.get("csv_logging"))
        directory = from_str(obj.get("directory"))
        sbp_logging = from_str(obj.get("sbp_logging"))
        return LoggingBarFront(csv_logging, directory, sbp_logging)

    def to_dict(self) -> dict:
        result: dict = {}
        result["csv_logging"] = from_bool(self.csv_logging)
        result["directory"] = from_str(self.directory)
        result["sbp_logging"] = from_str(self.sbp_logging)
        return result


class LoggingBarStatus:
    csv_logging: bool
    previous_folders: List[str]
    sbp_logging: str

    def __init__(self, csv_logging: bool, previous_folders: List[str], sbp_logging: str) -> None:
        self.csv_logging = csv_logging
        self.previous_folders = previous_folders
        self.sbp_logging = sbp_logging

    @staticmethod
    def from_dict(obj: Any) -> 'LoggingBarStatus':
        assert isinstance(obj, dict)
        csv_logging = from_bool(obj.get("csv_logging"))
        previous_folders = from_list(from_str, obj.get("previous_folders"))
        sbp_logging = from_str(obj.get("sbp_logging"))
        return LoggingBarStatus(csv_logging, previous_folders, sbp_logging)

    def to_dict(self) -> dict:
        result: dict = {}
        result["csv_logging"] = from_bool(self.csv_logging)
        result["previous_folders"] = from_list(from_str, self.previous_folders)
        result["sbp_logging"] = from_str(self.sbp_logging)
        return result


class NavBarStatus:
    available_baudrates: List[int]
    available_flows: List[str]
    available_ports: List[str]
    available_refresh_rates: List[int]
    log_level: str
    previous_files: List[str]
    previous_hosts: List[str]
    previous_ports: List[int]

    def __init__(self, available_baudrates: List[int], available_flows: List[str], available_ports: List[str], available_refresh_rates: List[int], log_level: str, previous_files: List[str], previous_hosts: List[str], previous_ports: List[int]) -> None:
        self.available_baudrates = available_baudrates
        self.available_flows = available_flows
        self.available_ports = available_ports
        self.available_refresh_rates = available_refresh_rates
        self.log_level = log_level
        self.previous_files = previous_files
        self.previous_hosts = previous_hosts
        self.previous_ports = previous_ports

    @staticmethod
    def from_dict(obj: Any) -> 'NavBarStatus':
        assert isinstance(obj, dict)
        available_baudrates = from_list(from_int, obj.get("available_baudrates"))
        available_flows = from_list(from_str, obj.get("available_flows"))
        available_ports = from_list(from_str, obj.get("available_ports"))
        available_refresh_rates = from_list(from_int, obj.get("available_refresh_rates"))
        log_level = from_str(obj.get("log_level"))
        previous_files = from_list(from_str, obj.get("previous_files"))
        previous_hosts = from_list(from_str, obj.get("previous_hosts"))
        previous_ports = from_list(from_int, obj.get("previous_ports"))
        return NavBarStatus(available_baudrates, available_flows, available_ports, available_refresh_rates, log_level, previous_files, previous_hosts, previous_ports)

    def to_dict(self) -> dict:
        result: dict = {}
        result["available_baudrates"] = from_list(from_int, self.available_baudrates)
        result["available_flows"] = from_list(from_str, self.available_flows)
        result["available_ports"] = from_list(from_str, self.available_ports)
        result["available_refresh_rates"] = from_list(from_int, self.available_refresh_rates)
        result["log_level"] = from_str(self.log_level)
        result["previous_files"] = from_list(from_str, self.previous_files)
        result["previous_hosts"] = from_list(from_str, self.previous_hosts)
        result["previous_ports"] = from_list(from_int, self.previous_ports)
        return result


class ObservationTableRow:
    carrer_phase: float
    cn0: float
    computed_doppler: float
    flags: int
    lock: int
    measured_doppler: float
    prn: str
    pseudo_range: float

    def __init__(self, carrer_phase: float, cn0: float, computed_doppler: float, flags: int, lock: int, measured_doppler: float, prn: str, pseudo_range: float) -> None:
        self.carrer_phase = carrer_phase
        self.cn0 = cn0
        self.computed_doppler = computed_doppler
        self.flags = flags
        self.lock = lock
        self.measured_doppler = measured_doppler
        self.prn = prn
        self.pseudo_range = pseudo_range

    @staticmethod
    def from_dict(obj: Any) -> 'ObservationTableRow':
        assert isinstance(obj, dict)
        carrer_phase = from_float(obj.get("carrer_phase"))
        cn0 = from_float(obj.get("cn0"))
        computed_doppler = from_float(obj.get("computed_doppler"))
        flags = from_int(obj.get("flags"))
        lock = from_int(obj.get("lock"))
        measured_doppler = from_float(obj.get("measured_doppler"))
        prn = from_str(obj.get("prn"))
        pseudo_range = from_float(obj.get("pseudo_range"))
        return ObservationTableRow(carrer_phase, cn0, computed_doppler, flags, lock, measured_doppler, prn, pseudo_range)

    def to_dict(self) -> dict:
        result: dict = {}
        result["carrer_phase"] = to_float(self.carrer_phase)
        result["cn0"] = to_float(self.cn0)
        result["computed_doppler"] = to_float(self.computed_doppler)
        result["flags"] = from_int(self.flags)
        result["lock"] = from_int(self.lock)
        result["measured_doppler"] = to_float(self.measured_doppler)
        result["prn"] = from_str(self.prn)
        result["pseudo_range"] = to_float(self.pseudo_range)
        return result


class ObservationStatus:
    is_remote: bool
    rows: List[ObservationTableRow]
    tow: float
    week: int

    def __init__(self, is_remote: bool, rows: List[ObservationTableRow], tow: float, week: int) -> None:
        self.is_remote = is_remote
        self.rows = rows
        self.tow = tow
        self.week = week

    @staticmethod
    def from_dict(obj: Any) -> 'ObservationStatus':
        assert isinstance(obj, dict)
        is_remote = from_bool(obj.get("is_remote"))
        rows = from_list(ObservationTableRow.from_dict, obj.get("rows"))
        tow = from_float(obj.get("tow"))
        week = from_int(obj.get("week"))
        return ObservationStatus(is_remote, rows, tow, week)

    def to_dict(self) -> dict:
        result: dict = {}
        result["is_remote"] = from_bool(self.is_remote)
        result["rows"] = from_list(lambda x: to_class(ObservationTableRow, x), self.rows)
        result["tow"] = to_float(self.tow)
        result["week"] = from_int(self.week)
        return result


class PauseRequest:
    pause: bool

    def __init__(self, pause: bool) -> None:
        self.pause = pause

    @staticmethod
    def from_dict(obj: Any) -> 'PauseRequest':
        assert isinstance(obj, dict)
        pause = from_bool(obj.get("pause"))
        return PauseRequest(pause)

    def to_dict(self) -> dict:
        result: dict = {}
        result["pause"] = from_bool(self.pause)
        return result


class SerialRefreshRequest:
    refresh: bool

    def __init__(self, refresh: bool) -> None:
        self.refresh = refresh

    @staticmethod
    def from_dict(obj: Any) -> 'SerialRefreshRequest':
        assert isinstance(obj, dict)
        refresh = from_bool(obj.get("refresh"))
        return SerialRefreshRequest(refresh)

    def to_dict(self) -> dict:
        result: dict = {}
        result["refresh"] = from_bool(self.refresh)
        return result


class SerialRequest:
    baudrate: int
    device: str
    flow_control: str

    def __init__(self, baudrate: int, device: str, flow_control: str) -> None:
        self.baudrate = baudrate
        self.device = device
        self.flow_control = flow_control

    @staticmethod
    def from_dict(obj: Any) -> 'SerialRequest':
        assert isinstance(obj, dict)
        baudrate = from_int(obj.get("baudrate"))
        device = from_str(obj.get("device"))
        flow_control = from_str(obj.get("flow_control"))
        return SerialRequest(baudrate, device, flow_control)

    def to_dict(self) -> dict:
        result: dict = {}
        result["baudrate"] = from_int(self.baudrate)
        result["device"] = from_str(self.device)
        result["flow_control"] = from_str(self.flow_control)
        return result


class SolutionPositionStatus:
    available_units: List[str]
    cur_data: List[List[Point]]
    data: List[List[Point]]
    lat_max: float
    lat_min: float
    lon_max: float
    lon_min: float

    def __init__(self, available_units: List[str], cur_data: List[List[Point]], data: List[List[Point]], lat_max: float, lat_min: float, lon_max: float, lon_min: float) -> None:
        self.available_units = available_units
        self.cur_data = cur_data
        self.data = data
        self.lat_max = lat_max
        self.lat_min = lat_min
        self.lon_max = lon_max
        self.lon_min = lon_min

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionPositionStatus':
        assert isinstance(obj, dict)
        available_units = from_list(from_str, obj.get("available_units"))
        cur_data = from_list(lambda x: from_list(Point.from_dict, x), obj.get("cur_data"))
        data = from_list(lambda x: from_list(Point.from_dict, x), obj.get("data"))
        lat_max = from_float(obj.get("lat_max"))
        lat_min = from_float(obj.get("lat_min"))
        lon_max = from_float(obj.get("lon_max"))
        lon_min = from_float(obj.get("lon_min"))
        return SolutionPositionStatus(available_units, cur_data, data, lat_max, lat_min, lon_max, lon_min)

    def to_dict(self) -> dict:
        result: dict = {}
        result["available_units"] = from_list(from_str, self.available_units)
        result["cur_data"] = from_list(lambda x: from_list(lambda x: to_class(Point, x), x), self.cur_data)
        result["data"] = from_list(lambda x: from_list(lambda x: to_class(Point, x), x), self.data)
        result["lat_max"] = to_float(self.lat_max)
        result["lat_min"] = to_float(self.lat_min)
        result["lon_max"] = to_float(self.lon_max)
        result["lon_min"] = to_float(self.lon_min)
        return result


class SolutionPositionStatusButtonFront:
    solution_position_center: bool
    solution_position_clear: bool
    solution_position_pause: bool
    solution_position_zoom: bool

    def __init__(self, solution_position_center: bool, solution_position_clear: bool, solution_position_pause: bool, solution_position_zoom: bool) -> None:
        self.solution_position_center = solution_position_center
        self.solution_position_clear = solution_position_clear
        self.solution_position_pause = solution_position_pause
        self.solution_position_zoom = solution_position_zoom

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionPositionStatusButtonFront':
        assert isinstance(obj, dict)
        solution_position_center = from_bool(obj.get("solution_position_center"))
        solution_position_clear = from_bool(obj.get("solution_position_clear"))
        solution_position_pause = from_bool(obj.get("solution_position_pause"))
        solution_position_zoom = from_bool(obj.get("solution_position_zoom"))
        return SolutionPositionStatusButtonFront(solution_position_center, solution_position_clear, solution_position_pause, solution_position_zoom)

    def to_dict(self) -> dict:
        result: dict = {}
        result["solution_position_center"] = from_bool(self.solution_position_center)
        result["solution_position_clear"] = from_bool(self.solution_position_clear)
        result["solution_position_pause"] = from_bool(self.solution_position_pause)
        result["solution_position_zoom"] = from_bool(self.solution_position_zoom)
        return result


class SolutionPositionStatusUnitFront:
    solution_position_unit: str

    def __init__(self, solution_position_unit: str) -> None:
        self.solution_position_unit = solution_position_unit

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionPositionStatusUnitFront':
        assert isinstance(obj, dict)
        solution_position_unit = from_str(obj.get("solution_position_unit"))
        return SolutionPositionStatusUnitFront(solution_position_unit)

    def to_dict(self) -> dict:
        result: dict = {}
        result["solution_position_unit"] = from_str(self.solution_position_unit)
        return result


class SolutionTableStatus:
    data: List[KeyValuePair]

    def __init__(self, data: List[KeyValuePair]) -> None:
        self.data = data

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionTableStatus':
        assert isinstance(obj, dict)
        data = from_list(KeyValuePair.from_dict, obj.get("data"))
        return SolutionTableStatus(data)

    def to_dict(self) -> dict:
        result: dict = {}
        result["data"] = from_list(lambda x: to_class(KeyValuePair, x), self.data)
        return result


class SolutionVelocityStatus:
    available_units: List[str]
    colors: List[str]
    max: float
    min: float

    def __init__(self, available_units: List[str], colors: List[str], max: float, min: float) -> None:
        self.available_units = available_units
        self.colors = colors
        self.max = max
        self.min = min

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionVelocityStatus':
        assert isinstance(obj, dict)
        available_units = from_list(from_str, obj.get("available_units"))
        colors = from_list(from_str, obj.get("colors"))
        max = from_float(obj.get("max"))
        min = from_float(obj.get("min"))
        return SolutionVelocityStatus(available_units, colors, max, min)

    def to_dict(self) -> dict:
        result: dict = {}
        result["available_units"] = from_list(from_str, self.available_units)
        result["colors"] = from_list(from_str, self.colors)
        result["max"] = to_float(self.max)
        result["min"] = to_float(self.min)
        return result


class SolutionVelocityStatusFront:
    solution_velocity_units: str

    def __init__(self, solution_velocity_units: str) -> None:
        self.solution_velocity_units = solution_velocity_units

    @staticmethod
    def from_dict(obj: Any) -> 'SolutionVelocityStatusFront':
        assert isinstance(obj, dict)
        solution_velocity_units = from_str(obj.get("solution_velocity_units"))
        return SolutionVelocityStatusFront(solution_velocity_units)

    def to_dict(self) -> dict:
        result: dict = {}
        result["solution_velocity_units"] = from_str(self.solution_velocity_units)
        return result


class Status:
    text: str

    def __init__(self, text: str) -> None:
        self.text = text

    @staticmethod
    def from_dict(obj: Any) -> 'Status':
        assert isinstance(obj, dict)
        text = from_str(obj.get("text"))
        return Status(text)

    def to_dict(self) -> dict:
        result: dict = {}
        result["text"] = from_str(self.text)
        return result


class StatusBarStatus:
    corr_age: str
    data_rate: str
    ins: str
    port: str
    pos: str
    rtk: str
    sats: str
    solid_connection: bool

    def __init__(self, corr_age: str, data_rate: str, ins: str, port: str, pos: str, rtk: str, sats: str, solid_connection: bool) -> None:
        self.corr_age = corr_age
        self.data_rate = data_rate
        self.ins = ins
        self.port = port
        self.pos = pos
        self.rtk = rtk
        self.sats = sats
        self.solid_connection = solid_connection

    @staticmethod
    def from_dict(obj: Any) -> 'StatusBarStatus':
        assert isinstance(obj, dict)
        corr_age = from_str(obj.get("corr_age"))
        data_rate = from_str(obj.get("data_rate"))
        ins = from_str(obj.get("ins"))
        port = from_str(obj.get("port"))
        pos = from_str(obj.get("pos"))
        rtk = from_str(obj.get("rtk"))
        sats = from_str(obj.get("sats"))
        solid_connection = from_bool(obj.get("solid_connection"))
        return StatusBarStatus(corr_age, data_rate, ins, port, pos, rtk, sats, solid_connection)

    def to_dict(self) -> dict:
        result: dict = {}
        result["corr_age"] = from_str(self.corr_age)
        result["data_rate"] = from_str(self.data_rate)
        result["ins"] = from_str(self.ins)
        result["port"] = from_str(self.port)
        result["pos"] = from_str(self.pos)
        result["rtk"] = from_str(self.rtk)
        result["sats"] = from_str(self.sats)
        result["solid_connection"] = from_bool(self.solid_connection)
        return result


class TCPRequest:
    host: str
    port: int

    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    @staticmethod
    def from_dict(obj: Any) -> 'TCPRequest':
        assert isinstance(obj, dict)
        host = from_str(obj.get("host"))
        port = from_int(obj.get("port"))
        return TCPRequest(host, port)

    def to_dict(self) -> dict:
        result: dict = {}
        result["host"] = from_str(self.host)
        result["port"] = from_int(self.port)
        return result


class TrackingSignalsStatus:
    check_labels: List[str]
    colors: List[str]
    data: List[List[Point]]
    labels: List[str]
    xmin_offset: float

    def __init__(self, check_labels: List[str], colors: List[str], data: List[List[Point]], labels: List[str], xmin_offset: float) -> None:
        self.check_labels = check_labels
        self.colors = colors
        self.data = data
        self.labels = labels
        self.xmin_offset = xmin_offset

    @staticmethod
    def from_dict(obj: Any) -> 'TrackingSignalsStatus':
        assert isinstance(obj, dict)
        check_labels = from_list(from_str, obj.get("check_labels"))
        colors = from_list(from_str, obj.get("colors"))
        data = from_list(lambda x: from_list(Point.from_dict, x), obj.get("data"))
        labels = from_list(from_str, obj.get("labels"))
        xmin_offset = from_float(obj.get("xmin_offset"))
        return TrackingSignalsStatus(check_labels, colors, data, labels, xmin_offset)

    def to_dict(self) -> dict:
        result: dict = {}
        result["check_labels"] = from_list(from_str, self.check_labels)
        result["colors"] = from_list(from_str, self.colors)
        result["data"] = from_list(lambda x: from_list(lambda x: to_class(Point, x), x), self.data)
        result["labels"] = from_list(from_str, self.labels)
        result["xmin_offset"] = to_float(self.xmin_offset)
        return result


class TrackingSignalsStatusFront:
    tracking_signals_check_visibility: List[str]

    def __init__(self, tracking_signals_check_visibility: List[str]) -> None:
        self.tracking_signals_check_visibility = tracking_signals_check_visibility

    @staticmethod
    def from_dict(obj: Any) -> 'TrackingSignalsStatusFront':
        assert isinstance(obj, dict)
        tracking_signals_check_visibility = from_list(from_str, obj.get("tracking_signals_check_visibility"))
        return TrackingSignalsStatusFront(tracking_signals_check_visibility)

    def to_dict(self) -> dict:
        result: dict = {}
        result["tracking_signals_check_visibility"] = from_list(from_str, self.tracking_signals_check_visibility)
        return result


class Message:
    tcp_request: Optional[TCPRequest]
    file_request: Optional[FileRequest]
    serial_request: Optional[SerialRequest]
    serial_refresh_request: Optional[SerialRefreshRequest]
    pause_request: Optional[PauseRequest]
    disconnect_request: Optional[DisconnectRequest]
    solution_table_status: Optional[SolutionTableStatus]
    nav_bar_status: Optional[NavBarStatus]
    status_bar_status: Optional[StatusBarStatus]
    baseline_plot_status: Optional[BaselinePlotStatus]
    baseline_table_status: Optional[BaselineTableStatus]
    observation_status: Optional[ObservationStatus]
    solution_position_status: Optional[SolutionPositionStatus]
    solution_velocity_status: Optional[SolutionVelocityStatus]
    tracking_signals_status: Optional[TrackingSignalsStatus]
    advanced_ins_status: Optional[AdvancedInsStatus]
    status: Optional[Status]
    tracking_signals_status_front: Optional[TrackingSignalsStatusFront]
    solution_velocity_status_front: Optional[SolutionVelocityStatusFront]
    solution_position_status_button_front: Optional[SolutionPositionStatusButtonFront]
    solution_position_status_unit_front: Optional[SolutionPositionStatusUnitFront]
    log_append: Optional[LogAppend]
    logging_bar_front: Optional[LoggingBarFront]
    logging_bar_status: Optional[LoggingBarStatus]
    log_level_front: Optional[LogLevelFront]
    fusion_status_flags_status: Optional[FusionStatusFlagsStatus]
    advanced_magnetometer_status: Optional[AdvancedMagnetometerStatus]
    baseline_plot_status_button_front: Optional[BaselinePlotStatusButtonFront]
    advanced_spectrum_analyzer_status: Optional[AdvancedSpectrumAnalyzerStatus]
    advanced_spectrum_analyzer_status_front: Optional[AdvancedSpectrumAnalyzerStatusFront]

    def __init__(self, tcp_request: Optional[TCPRequest], file_request: Optional[FileRequest], serial_request: Optional[SerialRequest], serial_refresh_request: Optional[SerialRefreshRequest], pause_request: Optional[PauseRequest], disconnect_request: Optional[DisconnectRequest], solution_table_status: Optional[SolutionTableStatus], nav_bar_status: Optional[NavBarStatus], status_bar_status: Optional[StatusBarStatus], baseline_plot_status: Optional[BaselinePlotStatus], baseline_table_status: Optional[BaselineTableStatus], observation_status: Optional[ObservationStatus], solution_position_status: Optional[SolutionPositionStatus], solution_velocity_status: Optional[SolutionVelocityStatus], tracking_signals_status: Optional[TrackingSignalsStatus], advanced_ins_status: Optional[AdvancedInsStatus], status: Optional[Status], tracking_signals_status_front: Optional[TrackingSignalsStatusFront], solution_velocity_status_front: Optional[SolutionVelocityStatusFront], solution_position_status_button_front: Optional[SolutionPositionStatusButtonFront], solution_position_status_unit_front: Optional[SolutionPositionStatusUnitFront], log_append: Optional[LogAppend], logging_bar_front: Optional[LoggingBarFront], logging_bar_status: Optional[LoggingBarStatus], log_level_front: Optional[LogLevelFront], fusion_status_flags_status: Optional[FusionStatusFlagsStatus], advanced_magnetometer_status: Optional[AdvancedMagnetometerStatus], baseline_plot_status_button_front: Optional[BaselinePlotStatusButtonFront], advanced_spectrum_analyzer_status: Optional[AdvancedSpectrumAnalyzerStatus], advanced_spectrum_analyzer_status_front: Optional[AdvancedSpectrumAnalyzerStatusFront]) -> None:
        self.tcp_request = tcp_request
        self.file_request = file_request
        self.serial_request = serial_request
        self.serial_refresh_request = serial_refresh_request
        self.pause_request = pause_request
        self.disconnect_request = disconnect_request
        self.solution_table_status = solution_table_status
        self.nav_bar_status = nav_bar_status
        self.status_bar_status = status_bar_status
        self.baseline_plot_status = baseline_plot_status
        self.baseline_table_status = baseline_table_status
        self.observation_status = observation_status
        self.solution_position_status = solution_position_status
        self.solution_velocity_status = solution_velocity_status
        self.tracking_signals_status = tracking_signals_status
        self.advanced_ins_status = advanced_ins_status
        self.status = status
        self.tracking_signals_status_front = tracking_signals_status_front
        self.solution_velocity_status_front = solution_velocity_status_front
        self.solution_position_status_button_front = solution_position_status_button_front
        self.solution_position_status_unit_front = solution_position_status_unit_front
        self.log_append = log_append
        self.logging_bar_front = logging_bar_front
        self.logging_bar_status = logging_bar_status
        self.log_level_front = log_level_front
        self.fusion_status_flags_status = fusion_status_flags_status
        self.advanced_magnetometer_status = advanced_magnetometer_status
        self.baseline_plot_status_button_front = baseline_plot_status_button_front
        self.advanced_spectrum_analyzer_status = advanced_spectrum_analyzer_status
        self.advanced_spectrum_analyzer_status_front = advanced_spectrum_analyzer_status_front

    @staticmethod
    def from_dict(obj: Any) -> 'Message':
        assert isinstance(obj, dict)
        tcp_request = from_union([TCPRequest.from_dict, from_none], obj.get("TcpRequest"))
        file_request = from_union([FileRequest.from_dict, from_none], obj.get("FileRequest"))
        serial_request = from_union([SerialRequest.from_dict, from_none], obj.get("SerialRequest"))
        serial_refresh_request = from_union([SerialRefreshRequest.from_dict, from_none], obj.get("SerialRefreshRequest"))
        pause_request = from_union([PauseRequest.from_dict, from_none], obj.get("PauseRequest"))
        disconnect_request = from_union([DisconnectRequest.from_dict, from_none], obj.get("DisconnectRequest"))
        solution_table_status = from_union([SolutionTableStatus.from_dict, from_none], obj.get("SolutionTableStatus"))
        nav_bar_status = from_union([NavBarStatus.from_dict, from_none], obj.get("NavBarStatus"))
        status_bar_status = from_union([StatusBarStatus.from_dict, from_none], obj.get("StatusBarStatus"))
        baseline_plot_status = from_union([BaselinePlotStatus.from_dict, from_none], obj.get("BaselinePlotStatus"))
        baseline_table_status = from_union([BaselineTableStatus.from_dict, from_none], obj.get("BaselineTableStatus"))
        observation_status = from_union([ObservationStatus.from_dict, from_none], obj.get("ObservationStatus"))
        solution_position_status = from_union([SolutionPositionStatus.from_dict, from_none], obj.get("SolutionPositionStatus"))
        solution_velocity_status = from_union([SolutionVelocityStatus.from_dict, from_none], obj.get("SolutionVelocityStatus"))
        tracking_signals_status = from_union([TrackingSignalsStatus.from_dict, from_none], obj.get("TrackingSignalsStatus"))
        advanced_ins_status = from_union([AdvancedInsStatus.from_dict, from_none], obj.get("AdvancedInsStatus"))
        status = from_union([Status.from_dict, from_none], obj.get("Status"))
        tracking_signals_status_front = from_union([TrackingSignalsStatusFront.from_dict, from_none], obj.get("TrackingSignalsStatusFront"))
        solution_velocity_status_front = from_union([SolutionVelocityStatusFront.from_dict, from_none], obj.get("SolutionVelocityStatusFront"))
        solution_position_status_button_front = from_union([SolutionPositionStatusButtonFront.from_dict, from_none], obj.get("SolutionPositionStatusButtonFront"))
        solution_position_status_unit_front = from_union([SolutionPositionStatusUnitFront.from_dict, from_none], obj.get("SolutionPositionStatusUnitFront"))
        log_append = from_union([LogAppend.from_dict, from_none], obj.get("LogAppend"))
        logging_bar_front = from_union([LoggingBarFront.from_dict, from_none], obj.get("LoggingBarFront"))
        logging_bar_status = from_union([LoggingBarStatus.from_dict, from_none], obj.get("LoggingBarStatus"))
        log_level_front = from_union([LogLevelFront.from_dict, from_none], obj.get("LogLevelFront"))
        fusion_status_flags_status = from_union([FusionStatusFlagsStatus.from_dict, from_none], obj.get("FusionStatusFlagsStatus"))
        advanced_magnetometer_status = from_union([AdvancedMagnetometerStatus.from_dict, from_none], obj.get("AdvancedMagnetometerStatus"))
        baseline_plot_status_button_front = from_union([BaselinePlotStatusButtonFront.from_dict, from_none], obj.get("BaselinePlotStatusButtonFront"))
        advanced_spectrum_analyzer_status = from_union([AdvancedSpectrumAnalyzerStatus.from_dict, from_none], obj.get("AdvancedSpectrumAnalyzerStatus"))
        advanced_spectrum_analyzer_status_front = from_union([AdvancedSpectrumAnalyzerStatusFront.from_dict, from_none], obj.get("AdvancedSpectrumAnalyzerStatusFront"))
        return Message(tcp_request, file_request, serial_request, serial_refresh_request, pause_request, disconnect_request, solution_table_status, nav_bar_status, status_bar_status, baseline_plot_status, baseline_table_status, observation_status, solution_position_status, solution_velocity_status, tracking_signals_status, advanced_ins_status, status, tracking_signals_status_front, solution_velocity_status_front, solution_position_status_button_front, solution_position_status_unit_front, log_append, logging_bar_front, logging_bar_status, log_level_front, fusion_status_flags_status, advanced_magnetometer_status, baseline_plot_status_button_front, advanced_spectrum_analyzer_status, advanced_spectrum_analyzer_status_front)

    def to_dict(self) -> dict:
        result: dict = {}
        result["TcpRequest"] = from_union([lambda x: to_class(TCPRequest, x), from_none], self.tcp_request)
        result["FileRequest"] = from_union([lambda x: to_class(FileRequest, x), from_none], self.file_request)
        result["SerialRequest"] = from_union([lambda x: to_class(SerialRequest, x), from_none], self.serial_request)
        result["SerialRefreshRequest"] = from_union([lambda x: to_class(SerialRefreshRequest, x), from_none], self.serial_refresh_request)
        result["PauseRequest"] = from_union([lambda x: to_class(PauseRequest, x), from_none], self.pause_request)
        result["DisconnectRequest"] = from_union([lambda x: to_class(DisconnectRequest, x), from_none], self.disconnect_request)
        result["SolutionTableStatus"] = from_union([lambda x: to_class(SolutionTableStatus, x), from_none], self.solution_table_status)
        result["NavBarStatus"] = from_union([lambda x: to_class(NavBarStatus, x), from_none], self.nav_bar_status)
        result["StatusBarStatus"] = from_union([lambda x: to_class(StatusBarStatus, x), from_none], self.status_bar_status)
        result["BaselinePlotStatus"] = from_union([lambda x: to_class(BaselinePlotStatus, x), from_none], self.baseline_plot_status)
        result["BaselineTableStatus"] = from_union([lambda x: to_class(BaselineTableStatus, x), from_none], self.baseline_table_status)
        result["ObservationStatus"] = from_union([lambda x: to_class(ObservationStatus, x), from_none], self.observation_status)
        result["SolutionPositionStatus"] = from_union([lambda x: to_class(SolutionPositionStatus, x), from_none], self.solution_position_status)
        result["SolutionVelocityStatus"] = from_union([lambda x: to_class(SolutionVelocityStatus, x), from_none], self.solution_velocity_status)
        result["TrackingSignalsStatus"] = from_union([lambda x: to_class(TrackingSignalsStatus, x), from_none], self.tracking_signals_status)
        result["AdvancedInsStatus"] = from_union([lambda x: to_class(AdvancedInsStatus, x), from_none], self.advanced_ins_status)
        result["Status"] = from_union([lambda x: to_class(Status, x), from_none], self.status)
        result["TrackingSignalsStatusFront"] = from_union([lambda x: to_class(TrackingSignalsStatusFront, x), from_none], self.tracking_signals_status_front)
        result["SolutionVelocityStatusFront"] = from_union([lambda x: to_class(SolutionVelocityStatusFront, x), from_none], self.solution_velocity_status_front)
        result["SolutionPositionStatusButtonFront"] = from_union([lambda x: to_class(SolutionPositionStatusButtonFront, x), from_none], self.solution_position_status_button_front)
        result["SolutionPositionStatusUnitFront"] = from_union([lambda x: to_class(SolutionPositionStatusUnitFront, x), from_none], self.solution_position_status_unit_front)
        result["LogAppend"] = from_union([lambda x: to_class(LogAppend, x), from_none], self.log_append)
        result["LoggingBarFront"] = from_union([lambda x: to_class(LoggingBarFront, x), from_none], self.logging_bar_front)
        result["LoggingBarStatus"] = from_union([lambda x: to_class(LoggingBarStatus, x), from_none], self.logging_bar_status)
        result["LogLevelFront"] = from_union([lambda x: to_class(LogLevelFront, x), from_none], self.log_level_front)
        result["FusionStatusFlagsStatus"] = from_union([lambda x: to_class(FusionStatusFlagsStatus, x), from_none], self.fusion_status_flags_status)
        result["AdvancedMagnetometerStatus"] = from_union([lambda x: to_class(AdvancedMagnetometerStatus, x), from_none], self.advanced_magnetometer_status)
        result["BaselinePlotStatusButtonFront"] = from_union([lambda x: to_class(BaselinePlotStatusButtonFront, x), from_none], self.baseline_plot_status_button_front)
        result["AdvancedSpectrumAnalyzerStatus"] = from_union([lambda x: to_class(AdvancedSpectrumAnalyzerStatus, x), from_none], self.advanced_spectrum_analyzer_status)
        result["AdvancedSpectrumAnalyzerStatusFront"] = from_union([lambda x: to_class(AdvancedSpectrumAnalyzerStatusFront, x), from_none], self.advanced_spectrum_analyzer_status_front)
        return result


def message_from_dict(s: Any) -> Message:
    return Message.from_dict(s)


def message_to_dict(x: Message) -> Any:
    return to_class(Message, x)
