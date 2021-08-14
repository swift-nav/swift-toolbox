use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct TcpRequest {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct FileRequest {
    pub filename: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SerialRequest {
    pub device: String,
    pub baudrate: u32,
    pub flow_control: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SerialRefreshRequest {
    pub refresh: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct PauseRequest {
    pub pause: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct DisconnectRequest {
    pub disconnect: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct LogLevelFront {
    pub log_level: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub line: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct LogAppend {
    pub entries: Vec<LogEntry>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct KeyValuePair {
    pub key: String,
    pub pair: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionTableStatus {
    pub data: Vec<KeyValuePair>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct NavBarStatus {
    pub available_baudrates: Vec<u32>,
    pub available_ports: Vec<String>,
    pub available_flows: Vec<String>,
    pub previous_hosts: Vec<String>,
    pub available_refresh_rates: Vec<u8>,
    pub previous_ports: Vec<u16>,
    pub previous_files: Vec<String>,
    pub log_level: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct StatusBarStatus {
    pub port: String,
    pub pos: String,
    pub rtk: String,
    pub sats: String,
    pub corr_age: String,
    pub ins: String,
    pub data_rate: String,
    pub solid_connection: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct BaselinePlotStatus {
    pub data: Vec<Vec<Point>>,
    pub n_min: f64,
    pub n_max: f64,
    pub e_min: f64,
    pub e_max: f64,
    pub cur_data: Vec<Vec<Point>>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct BaselineTableStatus {
    pub data: Vec<KeyValuePair>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct ObservationTableRow {
    pub prn: String,
    pub pseudo_range: f64,
    pub carrer_phase: f64,
    pub cn0: f64,
    pub measured_doppler: f64,
    pub computed_doppler: f64,
    pub lock: u16,
    pub flags: u8,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct ObservationStatus {
    pub is_remote: bool,
    pub tow: f64,
    pub week: u16,
    pub rows: Vec<ObservationTableRow>,
}
 
#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionPositionStatus {
    pub data: Vec<Vec<Point>>,
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub cur_data: Vec<Vec<Point>>,
    pub available_units: Vec<String>,
}
 
#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionVelocityStatus {
    pub min: f64,
    pub max: f64,
    pub available_units: Vec<String>,
    pub colors: Vec<String>,
}
 
#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct TrackingSignalsStatus {
    pub xmin_offset: f64,
    pub labels: Vec<String>,
    pub data: Vec<Vec<Point>>,
    pub colors: Vec<String>,
    pub check_labels: Vec<String>,
}
 
#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct AdvancedInsStatus {
    pub data: Vec<Vec<Point>>,
    pub fields_data: Vec<f64>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct AdvancedMagnetometerStatus {
    pub data: Vec<Vec<Point>>,
    pub ymin: f64,
    pub ymax: f64,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct FusionStatusFlagsStatus {
    pub gnsspos: String,
    pub gnssvel: String,
    pub wheelticks: String,
    pub speedd: String,
    pub nhc: String,
    pub zerovel: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct AdvancedSpectrumAnalyzerStatus {
    pub ymin: f32,
    pub ymax: f32,
    pub xmax: f32,
    pub data: Vec<Point>,
    pub channel: u16,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct LoggingBarFront {
    pub csv_logging: bool,
    pub sbp_logging: String,
    pub directory: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct LoggingBarStatus {
    pub previous_folders: Vec<String>,
    pub csv_logging: bool,
    pub sbp_logging: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct TrackingSignalsStatusFront {
    pub tracking_signals_check_visibility: Vec<String>,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionVelocityStatusFront {
    pub solution_velocity_units: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct AdvancedSpectrumAnalyzerStatusFront {
    pub channel: u16,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionPositionStatusUnitFront {
    pub solution_position_unit: String,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct SolutionPositionStatusButtonFront {
    pub solution_position_center: bool,
    pub solution_position_zoom: bool,
    pub solution_position_clear: bool,
    pub solution_position_pause: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct BaselinePlotStatusButtonFront {
    pub clear: bool,
    pub pause: bool,
    pub reset_filters: bool,
}

#[derive(Deserialize, JsonSchema, Serialize, Default)]
pub struct Status {
    pub text: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub enum Message {
    TcpRequest(TcpRequest),
    FileRequest(FileRequest),
    SerialRequest(SerialRequest),
    SerialRefreshRequest(SerialRefreshRequest),
    PauseRequest(PauseRequest),
    DisconnectRequest(DisconnectRequest),
    SolutionTableStatus(SolutionTableStatus),
    NavBarStatus(NavBarStatus),
    StatusBarStatus(StatusBarStatus),
    BaselinePlotStatus(BaselinePlotStatus),
    BaselineTableStatus(BaselineTableStatus),
    ObservationStatus(ObservationStatus),
    SolutionPositionStatus(SolutionPositionStatus),
    SolutionVelocityStatus(SolutionVelocityStatus),
    TrackingSignalsStatus(TrackingSignalsStatus),
    AdvancedInsStatus(AdvancedInsStatus),
    Status(Status),
    TrackingSignalsStatusFront(TrackingSignalsStatusFront),
    SolutionVelocityStatusFront(SolutionVelocityStatusFront),
    SolutionPositionStatusButtonFront(SolutionPositionStatusButtonFront),
    SolutionPositionStatusUnitFront(SolutionPositionStatusUnitFront),
    LogAppend(LogAppend),
    LoggingBarFront(LoggingBarFront),
    LoggingBarStatus(LoggingBarStatus),
    LogLevelFront(LogLevelFront),
    FusionStatusFlagsStatus(FusionStatusFlagsStatus),
    AdvancedMagnetometerStatus(AdvancedMagnetometerStatus),
    BaselinePlotStatusButtonFront(BaselinePlotStatusButtonFront),
    AdvancedSpectrumAnalyzerStatus(AdvancedSpectrumAnalyzerStatus),
    AdvancedSpectrumAnalyzerStatusFront(AdvancedSpectrumAnalyzerStatusFront),
}