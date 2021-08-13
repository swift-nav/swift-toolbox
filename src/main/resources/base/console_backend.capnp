@0xe7871c33e8243ee4;

struct TcpRequest {
    host @0 :Text;
    port @1 :UInt16;
}

struct FileRequest {
    filename @0 :Text;
}

struct SerialRequest {
    device @0 :Text;
    baudrate @1 :UInt32;
    flowControl @2 :Text;
}

struct SerialRefreshRequest {
    refresh @0 :Void = void;
}

struct PauseRequest {
    pause @0 :Bool;
}

struct DisconnectRequest {
    disconnect @0 :Void = void;
}

struct LogLevelFront {
    logLevel @0 :Text;
}

enum LogLevel {
    error @0;
    warn @1;
    info @2;
    debug @3;
    trace @4;
}

struct LogEntry {
    timestamp @0 :Text;
    level @1 :LogLevel;
    line @2 :Text;
}

struct LogAppend {
    entries @0 :List(LogEntry);
}

struct KeyValPair {
    key @0 :Text;
    val @1 :Text;
}

struct SolutionTableStatus {
    data @0 :List(KeyValPair);
}

struct Point {
    x @0 :Float64;
    y @1 :Float64;
}

struct NavBarStatus {
    availableBaudrates @0 : List(UInt32);
    availablePorts @1 : List(Text);
    availableFlows @2 : List(Text);
    previousHosts @3: List(Text);
    availableRefreshRates @4 : List(UInt8);
    previousPorts @5: List(UInt16);
    previousFiles @6: List(Text);
    logLevel @7: Text;
}

struct StatusBarStatus {
    port @0 : Text;
    pos @1 : Text;
    rtk @2 : Text;
    sats @3: Text;
    corrAge @4 : Text;
    ins @5: Text;
    dataRate @6: Text;
    solidConnection @7: Bool;
}

struct BaselinePlotStatus {
    data @0 :List(List(Point));
    nMin @1 :Float64;
    nMax @2 :Float64;
    eMin @3 :Float64;
    eMax @4 :Float64;
    curData @5 :List(List(Point));
}

struct BaselineTableStatus {
    data @0 :List(KeyValPair);
}

struct ObservationTableRow {
    prn @0 :Text;
    pseudoRange @1 :Float64;
    carrierPhase @2 :Float64;
    cn0 @3 :Float64;
    measuredDoppler @4 :Float64;
    computedDoppler @5 :Float64;
    lock @6 :UInt16;
    flags @7 :UInt8;
}

struct ObservationStatus {
    isRemote @0 :Bool;
    tow @1 :Float64;
    week @2 :UInt16;
    rows @3 :List(ObservationTableRow);
}

struct SolutionPositionStatus {
    data @0 :List(List(Point));
    latMin @1 :Float64;
    latMax @2 :Float64;
    lonMin @3 :Float64;
    lonMax @4 :Float64;
    curData @5 :List(List(Point));
    availableUnits @6 : List(Text);
}

struct SolutionVelocityStatus {
    min @0 :Float64;
    max @1 :Float64;
    data @2 :List(List(Point));
    availableUnits @3 : List(Text);
    colors @4 :List(Text);
}

struct TrackingSignalsStatus {
    xminOffset @0 :Float64;
    labels @1 :List(Text);
    data @2 :List(List(Point));
    colors @3 :List(Text);
    checkLabels @4 :List(Text);
}

struct AdvancedInsStatus {
    data @0 :List(List(Point));
    fieldsData @1 :List(Float64);
}

struct AdvancedMagnetometerStatus {
    data @0 :List(List(Point));
    ymin @1 :Float64;
    ymax @2 :Float64;
}

struct FusionStatusFlagsStatus {
    gnsspos @0 :Text;
    gnssvel @1 :Text;
    wheelticks @2 :Text;
    speed @3 :Text;
    nhc @4 :Text;
    zerovel @5 :Text;
}

struct AdvancedSpectrumAnalyzerStatus {
    ymin @0 :Float32;
    ymax @1 :Float32;
    xmin @2 :Float32;
    xmax @3 :Float32;
    data @4 :List(Point);
    channel @5 : UInt16;
}

struct LoggingBarFront {
    csvLogging @0 :Bool;
    sbpLogging @1 :Text;
    directory @2 :Text;
}

struct LoggingBarStatus {
    previousFolders @0 : List(Text);
    csvLogging @1 :Bool;
    sbpLogging @2 :Text;
}

struct TrackingSignalsStatusFront {
    trackingSignalsCheckVisibility @0 :List(Text);
}

struct SolutionVelocityStatusFront {
    solutionVelocityUnit @0 :Text;
}

struct AdvancedSpectrumAnalyzerStatusFront {
    channel @0 :UInt16;
}

struct SolutionPositionStatusUnitFront {
    solutionPositionUnit @0 :Text;
}
struct SolutionPositionStatusButtonFront {
    solutionPositionCenter @0 :Bool;
    solutionPositionZoom @1 :Bool;
    solutionPositionClear @2 :Bool;
    solutionPositionPause @3 :Bool;
}

struct BaselinePlotStatusButtonFront {
    clear @0 :Bool;
    pause @1 :Bool;
    resetFilters @2 :Bool;
}

struct Status {
    text @0 :Text;
}

struct Message {
    union {
        solutionVelocityStatus @0 :SolutionVelocityStatus;
        status @1 :Status;
        trackingSignalsStatus @2 :TrackingSignalsStatus;
        trackingSignalsStatusFront @3 :TrackingSignalsStatusFront;
        solutionVelocityStatusFront @4 :SolutionVelocityStatusFront;
        solutionTableStatus @5 :SolutionTableStatus;
        solutionPositionStatus @6 :SolutionPositionStatus;
        solutionPositionStatusButtonFront @7 :SolutionPositionStatusButtonFront;
        solutionPositionStatusUnitFront @8 :SolutionPositionStatusUnitFront;
        tcpRequest @9 :TcpRequest;
        fileRequest @10 :FileRequest;
        serialRequest @11 :SerialRequest;
        pauseRequest @12 :PauseRequest;
        disconnectRequest @13 :DisconnectRequest;
        navBarStatus @14 :NavBarStatus;
        serialRefreshRequest @15 :SerialRefreshRequest;
        logAppend @16 :LogAppend;
        observationStatus @17 :ObservationStatus;
        statusBarStatus @18 :StatusBarStatus;
        loggingBarFront @19 :LoggingBarFront;
        loggingBarStatus @20 :LoggingBarStatus;
        logLevelFront @21 :LogLevelFront;
        advancedInsStatus @22 :AdvancedInsStatus;
        fusionStatusFlagsStatus @23 :FusionStatusFlagsStatus;
        advancedMagnetometerStatus @24 :AdvancedMagnetometerStatus;
        baselinePlotStatus @25 :BaselinePlotStatus;
        baselineTableStatus @26 :BaselineTableStatus;
        baselinePlotStatusButtonFront @27 :BaselinePlotStatusButtonFront;
        advancedSpectrumAnalyzerStatus @28:AdvancedSpectrumAnalyzerStatus;
        advancedSpectrumAnalyzerStatusFront @29:AdvancedSpectrumAnalyzerStatusFront;
    }
}
