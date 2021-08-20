// To parse this data:
//
//   import { Convert, Message } from "./file";
//
//   const message = Convert.toMessage(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export class Message {
    TcpRequest?:                          TcpRequest;
    FileRequest?:                         FileRequest;
    SerialRequest?:                       SerialRequest;
    SerialRefreshRequest?:                SerialRefreshRequest;
    PauseRequest?:                        PauseRequest;
    DisconnectRequest?:                   DisconnectRequest;
    SolutionTableStatus?:                 SolutionTableStatus;
    NavBarStatus?:                        NavBarStatus;
    StatusBarStatus?:                     StatusBarStatus;
    BaselinePlotStatus?:                  BaselinePlotStatus;
    BaselineTableStatus?:                 BaselineTableStatus;
    ObservationStatus?:                   ObservationStatus;
    SolutionPositionStatus?:              SolutionPositionStatus;
    SolutionVelocityStatus?:              SolutionVelocityStatus;
    TrackingSignalsStatus?:               TrackingSignalsStatus;
    AdvancedInsStatus?:                   AdvancedInsStatus;
    Status?:                              Status;
    TrackingSignalsStatusFront?:          TrackingSignalsStatusFront;
    SolutionVelocityStatusFront?:         SolutionVelocityStatusFront;
    SolutionPositionStatusButtonFront?:   SolutionPositionStatusButtonFront;
    SolutionPositionStatusUnitFront?:     SolutionPositionStatusUnitFront;
    LogAppend?:                           LogAppend;
    LoggingBarFront?:                     LoggingBarFront;
    LoggingBarStatus?:                    LoggingBarStatus;
    LogLevelFront?:                       LogLevelFront;
    FusionStatusFlagsStatus?:             FusionStatusFlagsStatus;
    AdvancedMagnetometerStatus?:          AdvancedMagnetometerStatus;
    BaselinePlotStatusButtonFront?:       BaselinePlotStatusButtonFront;
    AdvancedSpectrumAnalyzerStatus?:      AdvancedSpectrumAnalyzerStatus;
    AdvancedSpectrumAnalyzerStatusFront?: AdvancedSpectrumAnalyzerStatusFront;
}

export class AdvancedInsStatus {
    data:        Array<Point[]>;
    fields_data: number[];
}

export class Point {
    x: number;
    y: number;
}

export class AdvancedMagnetometerStatus {
    data: Array<Point[]>;
    ymax: number;
    ymin: number;
}

export class AdvancedSpectrumAnalyzerStatus {
    channel: number;
    data:    Point[];
    xmax:    number;
    ymax:    number;
    ymin:    number;
}

export class AdvancedSpectrumAnalyzerStatusFront {
    channel: number;
}

export class BaselinePlotStatus {
    cur_data: Array<Point[]>;
    data:     Array<Point[]>;
    e_max:    number;
    e_min:    number;
    n_max:    number;
    n_min:    number;
}

export class BaselinePlotStatusButtonFront {
    clear:         boolean;
    pause:         boolean;
    reset_filters: boolean;
}

export class BaselineTableStatus {
    data: KeyValuePair[];
}

export class KeyValuePair {
    key:  string;
    pair: string;
}

export class DisconnectRequest {
    disconnect: boolean;
}

export class FileRequest {
    filename: string;
}

export class FusionStatusFlagsStatus {
    gnsspos:    string;
    gnssvel:    string;
    nhc:        string;
    speedd:     string;
    wheelticks: string;
    zerovel:    string;
}

export class LogAppend {
    entries: LogEntry[];
}

export class LogEntry {
    level:     LogLevel;
    line:      string;
    timestamp: string;
}

export enum LogLevel {
    Debug = "Debug",
    Error = "Error",
    Info = "Info",
    Trace = "Trace",
    Warn = "Warn",
}

export class LogLevelFront {
    log_level: string;
}

export class LoggingBarFront {
    csv_logging: boolean;
    directory:   string;
    sbp_logging: string;
}

export class LoggingBarStatus {
    csv_logging:      boolean;
    previous_folders: string[];
    sbp_logging:      string;
}

export class NavBarStatus {
    available_baudrates:     number[];
    available_flows:         string[];
    available_ports:         string[];
    available_refresh_rates: number[];
    log_level:               string;
    previous_files:          string[];
    previous_hosts:          string[];
    previous_ports:          number[];
}

export class ObservationStatus {
    is_remote: boolean;
    rows:      ObservationTableRow[];
    tow:       number;
    week:      number;
}

export class ObservationTableRow {
    carrer_phase:     number;
    cn0:              number;
    computed_doppler: number;
    flags:            number;
    lock:             number;
    measured_doppler: number;
    prn:              string;
    pseudo_range:     number;
}

export class PauseRequest {
    pause: boolean;
}

export class SerialRefreshRequest {
    refresh: boolean;
}

export class SerialRequest {
    baudrate:     number;
    device:       string;
    flow_control: string;
}

export class SolutionPositionStatus {
    available_units: string[];
    cur_data:        Array<Point[]>;
    data:            Array<Point[]>;
    lat_max:         number;
    lat_min:         number;
    lon_max:         number;
    lon_min:         number;
}

export class SolutionPositionStatusButtonFront {
    solution_position_center: boolean;
    solution_position_clear:  boolean;
    solution_position_pause:  boolean;
    solution_position_zoom:   boolean;
}

export class SolutionPositionStatusUnitFront {
    solution_position_unit: string;
}

export class SolutionTableStatus {
    data: KeyValuePair[];
}

export class SolutionVelocityStatus {
    available_units: string[];
    colors:          string[];
    max:             number;
    min:             number;
}

export class SolutionVelocityStatusFront {
    solution_velocity_units: string;
}

export class Status {
    text: string;
}

export class StatusBarStatus {
    corr_age:         string;
    data_rate:        string;
    ins:              string;
    port:             string;
    pos:              string;
    rtk:              string;
    sats:             string;
    solid_connection: boolean;
}

export class TcpRequest {
    host: string;
    port: number;
}

export class TrackingSignalsStatus {
    check_labels: string[];
    colors:       string[];
    data:         Array<Point[]>;
    labels:       string[];
    xmin_offset:  number;
}

export class TrackingSignalsStatusFront {
    tracking_signals_check_visibility: string[];
}

// Converts JSON types to/from your types
// and asserts the results at runtime
export class Convert {
    public static toMessage(json: any): Message {
        return cast(json, r("Message"));
    }

    public static messageToJson(value: Message): any {
        return uncast(value, r("Message"));
    }
}

function invalidValue(typ: any, val: any, key: any = ''): never {
    if (key) {
        throw Error(`Invalid value for key "${key}". Expected type ${JSON.stringify(typ)} but got ${JSON.stringify(val)}`);
    }
    throw Error(`Invalid value ${JSON.stringify(val)} for type ${JSON.stringify(typ)}`, );
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) {}
        }
        return invalidValue(typs, val);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases, val);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue("array", val);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue("Date", val);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue("object", val);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, prop.key);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val);
    }
    if (typ === false) return invalidValue(typ, val);
    while (typeof typ === "object" && typ.ref !== undefined) {
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems")    ? transformArray(typ.arrayItems, val)
            : typ.hasOwnProperty("props")         ? transformObject(getProps(typ), typ.additional, val)
            : invalidValue(typ, val);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "Message": o([
        { json: "TcpRequest", js: "TcpRequest", typ: u(undefined, r("TcpRequest")) },
        { json: "FileRequest", js: "FileRequest", typ: u(undefined, r("FileRequest")) },
        { json: "SerialRequest", js: "SerialRequest", typ: u(undefined, r("SerialRequest")) },
        { json: "SerialRefreshRequest", js: "SerialRefreshRequest", typ: u(undefined, r("SerialRefreshRequest")) },
        { json: "PauseRequest", js: "PauseRequest", typ: u(undefined, r("PauseRequest")) },
        { json: "DisconnectRequest", js: "DisconnectRequest", typ: u(undefined, r("DisconnectRequest")) },
        { json: "SolutionTableStatus", js: "SolutionTableStatus", typ: u(undefined, r("SolutionTableStatus")) },
        { json: "NavBarStatus", js: "NavBarStatus", typ: u(undefined, r("NavBarStatus")) },
        { json: "StatusBarStatus", js: "StatusBarStatus", typ: u(undefined, r("StatusBarStatus")) },
        { json: "BaselinePlotStatus", js: "BaselinePlotStatus", typ: u(undefined, r("BaselinePlotStatus")) },
        { json: "BaselineTableStatus", js: "BaselineTableStatus", typ: u(undefined, r("BaselineTableStatus")) },
        { json: "ObservationStatus", js: "ObservationStatus", typ: u(undefined, r("ObservationStatus")) },
        { json: "SolutionPositionStatus", js: "SolutionPositionStatus", typ: u(undefined, r("SolutionPositionStatus")) },
        { json: "SolutionVelocityStatus", js: "SolutionVelocityStatus", typ: u(undefined, r("SolutionVelocityStatus")) },
        { json: "TrackingSignalsStatus", js: "TrackingSignalsStatus", typ: u(undefined, r("TrackingSignalsStatus")) },
        { json: "AdvancedInsStatus", js: "AdvancedInsStatus", typ: u(undefined, r("AdvancedInsStatus")) },
        { json: "Status", js: "Status", typ: u(undefined, r("Status")) },
        { json: "TrackingSignalsStatusFront", js: "TrackingSignalsStatusFront", typ: u(undefined, r("TrackingSignalsStatusFront")) },
        { json: "SolutionVelocityStatusFront", js: "SolutionVelocityStatusFront", typ: u(undefined, r("SolutionVelocityStatusFront")) },
        { json: "SolutionPositionStatusButtonFront", js: "SolutionPositionStatusButtonFront", typ: u(undefined, r("SolutionPositionStatusButtonFront")) },
        { json: "SolutionPositionStatusUnitFront", js: "SolutionPositionStatusUnitFront", typ: u(undefined, r("SolutionPositionStatusUnitFront")) },
        { json: "LogAppend", js: "LogAppend", typ: u(undefined, r("LogAppend")) },
        { json: "LoggingBarFront", js: "LoggingBarFront", typ: u(undefined, r("LoggingBarFront")) },
        { json: "LoggingBarStatus", js: "LoggingBarStatus", typ: u(undefined, r("LoggingBarStatus")) },
        { json: "LogLevelFront", js: "LogLevelFront", typ: u(undefined, r("LogLevelFront")) },
        { json: "FusionStatusFlagsStatus", js: "FusionStatusFlagsStatus", typ: u(undefined, r("FusionStatusFlagsStatus")) },
        { json: "AdvancedMagnetometerStatus", js: "AdvancedMagnetometerStatus", typ: u(undefined, r("AdvancedMagnetometerStatus")) },
        { json: "BaselinePlotStatusButtonFront", js: "BaselinePlotStatusButtonFront", typ: u(undefined, r("BaselinePlotStatusButtonFront")) },
        { json: "AdvancedSpectrumAnalyzerStatus", js: "AdvancedSpectrumAnalyzerStatus", typ: u(undefined, r("AdvancedSpectrumAnalyzerStatus")) },
        { json: "AdvancedSpectrumAnalyzerStatusFront", js: "AdvancedSpectrumAnalyzerStatusFront", typ: u(undefined, r("AdvancedSpectrumAnalyzerStatusFront")) },
    ], false),
    "AdvancedInsStatus": o([
        { json: "data", js: "data", typ: a(a(r("Point"))) },
        { json: "fields_data", js: "fields_data", typ: a(3.14) },
    ], "any"),
    "Point": o([
        { json: "x", js: "x", typ: 3.14 },
        { json: "y", js: "y", typ: 3.14 },
    ], "any"),
    "AdvancedMagnetometerStatus": o([
        { json: "data", js: "data", typ: a(a(r("Point"))) },
        { json: "ymax", js: "ymax", typ: 3.14 },
        { json: "ymin", js: "ymin", typ: 3.14 },
    ], "any"),
    "AdvancedSpectrumAnalyzerStatus": o([
        { json: "channel", js: "channel", typ: 0 },
        { json: "data", js: "data", typ: a(r("Point")) },
        { json: "xmax", js: "xmax", typ: 3.14 },
        { json: "ymax", js: "ymax", typ: 3.14 },
        { json: "ymin", js: "ymin", typ: 3.14 },
    ], "any"),
    "AdvancedSpectrumAnalyzerStatusFront": o([
        { json: "channel", js: "channel", typ: 0 },
    ], "any"),
    "BaselinePlotStatus": o([
        { json: "cur_data", js: "cur_data", typ: a(a(r("Point"))) },
        { json: "data", js: "data", typ: a(a(r("Point"))) },
        { json: "e_max", js: "e_max", typ: 3.14 },
        { json: "e_min", js: "e_min", typ: 3.14 },
        { json: "n_max", js: "n_max", typ: 3.14 },
        { json: "n_min", js: "n_min", typ: 3.14 },
    ], "any"),
    "BaselinePlotStatusButtonFront": o([
        { json: "clear", js: "clear", typ: true },
        { json: "pause", js: "pause", typ: true },
        { json: "reset_filters", js: "reset_filters", typ: true },
    ], "any"),
    "BaselineTableStatus": o([
        { json: "data", js: "data", typ: a(r("KeyValuePair")) },
    ], "any"),
    "KeyValuePair": o([
        { json: "key", js: "key", typ: "" },
        { json: "pair", js: "pair", typ: "" },
    ], "any"),
    "DisconnectRequest": o([
        { json: "disconnect", js: "disconnect", typ: true },
    ], "any"),
    "FileRequest": o([
        { json: "filename", js: "filename", typ: "" },
    ], "any"),
    "FusionStatusFlagsStatus": o([
        { json: "gnsspos", js: "gnsspos", typ: "" },
        { json: "gnssvel", js: "gnssvel", typ: "" },
        { json: "nhc", js: "nhc", typ: "" },
        { json: "speedd", js: "speedd", typ: "" },
        { json: "wheelticks", js: "wheelticks", typ: "" },
        { json: "zerovel", js: "zerovel", typ: "" },
    ], "any"),
    "LogAppend": o([
        { json: "entries", js: "entries", typ: a(r("LogEntry")) },
    ], "any"),
    "LogEntry": o([
        { json: "level", js: "level", typ: r("LogLevel") },
        { json: "line", js: "line", typ: "" },
        { json: "timestamp", js: "timestamp", typ: "" },
    ], "any"),
    "LogLevelFront": o([
        { json: "log_level", js: "log_level", typ: "" },
    ], "any"),
    "LoggingBarFront": o([
        { json: "csv_logging", js: "csv_logging", typ: true },
        { json: "directory", js: "directory", typ: "" },
        { json: "sbp_logging", js: "sbp_logging", typ: "" },
    ], "any"),
    "LoggingBarStatus": o([
        { json: "csv_logging", js: "csv_logging", typ: true },
        { json: "previous_folders", js: "previous_folders", typ: a("") },
        { json: "sbp_logging", js: "sbp_logging", typ: "" },
    ], "any"),
    "NavBarStatus": o([
        { json: "available_baudrates", js: "available_baudrates", typ: a(0) },
        { json: "available_flows", js: "available_flows", typ: a("") },
        { json: "available_ports", js: "available_ports", typ: a("") },
        { json: "available_refresh_rates", js: "available_refresh_rates", typ: a(0) },
        { json: "log_level", js: "log_level", typ: "" },
        { json: "previous_files", js: "previous_files", typ: a("") },
        { json: "previous_hosts", js: "previous_hosts", typ: a("") },
        { json: "previous_ports", js: "previous_ports", typ: a(0) },
    ], "any"),
    "ObservationStatus": o([
        { json: "is_remote", js: "is_remote", typ: true },
        { json: "rows", js: "rows", typ: a(r("ObservationTableRow")) },
        { json: "tow", js: "tow", typ: 3.14 },
        { json: "week", js: "week", typ: 0 },
    ], "any"),
    "ObservationTableRow": o([
        { json: "carrer_phase", js: "carrer_phase", typ: 3.14 },
        { json: "cn0", js: "cn0", typ: 3.14 },
        { json: "computed_doppler", js: "computed_doppler", typ: 3.14 },
        { json: "flags", js: "flags", typ: 0 },
        { json: "lock", js: "lock", typ: 0 },
        { json: "measured_doppler", js: "measured_doppler", typ: 3.14 },
        { json: "prn", js: "prn", typ: "" },
        { json: "pseudo_range", js: "pseudo_range", typ: 3.14 },
    ], "any"),
    "PauseRequest": o([
        { json: "pause", js: "pause", typ: true },
    ], "any"),
    "SerialRefreshRequest": o([
        { json: "refresh", js: "refresh", typ: true },
    ], "any"),
    "SerialRequest": o([
        { json: "baudrate", js: "baudrate", typ: 0 },
        { json: "device", js: "device", typ: "" },
        { json: "flow_control", js: "flow_control", typ: "" },
    ], "any"),
    "SolutionPositionStatus": o([
        { json: "available_units", js: "available_units", typ: a("") },
        { json: "cur_data", js: "cur_data", typ: a(a(r("Point"))) },
        { json: "data", js: "data", typ: a(a(r("Point"))) },
        { json: "lat_max", js: "lat_max", typ: 3.14 },
        { json: "lat_min", js: "lat_min", typ: 3.14 },
        { json: "lon_max", js: "lon_max", typ: 3.14 },
        { json: "lon_min", js: "lon_min", typ: 3.14 },
    ], "any"),
    "SolutionPositionStatusButtonFront": o([
        { json: "solution_position_center", js: "solution_position_center", typ: true },
        { json: "solution_position_clear", js: "solution_position_clear", typ: true },
        { json: "solution_position_pause", js: "solution_position_pause", typ: true },
        { json: "solution_position_zoom", js: "solution_position_zoom", typ: true },
    ], "any"),
    "SolutionPositionStatusUnitFront": o([
        { json: "solution_position_unit", js: "solution_position_unit", typ: "" },
    ], "any"),
    "SolutionTableStatus": o([
        { json: "data", js: "data", typ: a(r("KeyValuePair")) },
    ], "any"),
    "SolutionVelocityStatus": o([
        { json: "available_units", js: "available_units", typ: a("") },
        { json: "colors", js: "colors", typ: a("") },
        { json: "max", js: "max", typ: 3.14 },
        { json: "min", js: "min", typ: 3.14 },
    ], "any"),
    "SolutionVelocityStatusFront": o([
        { json: "solution_velocity_units", js: "solution_velocity_units", typ: "" },
    ], "any"),
    "Status": o([
        { json: "text", js: "text", typ: "" },
    ], "any"),
    "StatusBarStatus": o([
        { json: "corr_age", js: "corr_age", typ: "" },
        { json: "data_rate", js: "data_rate", typ: "" },
        { json: "ins", js: "ins", typ: "" },
        { json: "port", js: "port", typ: "" },
        { json: "pos", js: "pos", typ: "" },
        { json: "rtk", js: "rtk", typ: "" },
        { json: "sats", js: "sats", typ: "" },
        { json: "solid_connection", js: "solid_connection", typ: true },
    ], "any"),
    "TcpRequest": o([
        { json: "host", js: "host", typ: "" },
        { json: "port", js: "port", typ: 0 },
    ], "any"),
    "TrackingSignalsStatus": o([
        { json: "check_labels", js: "check_labels", typ: a("") },
        { json: "colors", js: "colors", typ: a("") },
        { json: "data", js: "data", typ: a(a(r("Point"))) },
        { json: "labels", js: "labels", typ: a("") },
        { json: "xmin_offset", js: "xmin_offset", typ: 3.14 },
    ], "any"),
    "TrackingSignalsStatusFront": o([
        { json: "tracking_signals_check_visibility", js: "tracking_signals_check_visibility", typ: a("") },
    ], "any"),
    "LogLevel": [
        "Debug",
        "Error",
        "Info",
        "Trace",
        "Warn",
    ],
};
