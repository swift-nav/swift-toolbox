@0xe7871c33e8243ee4;

struct ConnectRequest(RequestType) {
    request @0 :RequestType;
}

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

struct BottomNavbarStatus {
    availableBaudrates @0 : List(UInt32);
    availablePorts @1 : List(Text);
    availableFlows @2 : List(Text);
}

struct ObservationTableRow {
    prn @0 :Text;
    pseudoRange @1 :Float64;
    carrierPhase @2 :Float64;
    cn0 @3 :Float64;
    measuredDoppler @4 :Float64;
    computedDoppler @5 :Float64;
    lock @6 :UInt16;
    flags @7 :Text;
}

struct ObservationStatus {
    isRemote @0 :Bool;
    tow @1 :Float64;
    week @2 :UInt16;
    rows @3 :List(ObservationTableRow);
}

struct SolutionPositionStatus {
    data @0 :List(List(Point));
    labels @1 :List(Text);
    colors @2 :List(Text);
    latMin @3 :Float64;
    latMax @4 :Float64;
    lonMin @5 :Float64;
    lonMax @6 :Float64;
    curData @7 :List(List(Point));
    availableUnits @8 : List(Text);
}

struct SolutionVelocityStatus {
    min @0 :Float64;
    max @1 :Float64;
    data @2 :List(List(Point));
    availableUnits @3 : List(Text);
    colors @4 :List(Text);
}

struct TrackingSignalsStatus {
    min @0 :Float64;
    max @1 :Float64;
    labels @2 :List(Text);
    data @3 :List(List(Point));
    colors @4 :List(Text);
    checkLabels @5 :List(Text);
}

struct TrackingSignalsStatusFront {
    trackingSignalsCheckVisibility @0 :List(Text);
}

struct SolutionVelocityStatusFront {
    solutionVelocityUnit @0 :Text;
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

struct Status {
    text @0 :Text;
}

struct Message {
    union {
        connectRequest @0 :ConnectRequest;
        solutionVelocityStatus @1 :SolutionVelocityStatus;
        status @2 :Status;
        trackingSignalsStatus @3 :TrackingSignalsStatus;
        trackingSignalsStatusFront @4 :TrackingSignalsStatusFront;
        solutionVelocityStatusFront @5 :SolutionVelocityStatusFront;
        solutionTableStatus @6 :SolutionTableStatus;
        solutionPositionStatus @7 :SolutionPositionStatus;
        solutionPositionStatusButtonFront @8 :SolutionPositionStatusButtonFront;
        solutionPositionStatusUnitFront @9 :SolutionPositionStatusUnitFront;
        tcpRequest @10 :TcpRequest;
        fileRequest @11 :FileRequest;
        serialRequest @12 :SerialRequest;
        pauseRequest @13 :PauseRequest;
        disconnectRequest @14 :DisconnectRequest;
        bottomNavbarStatus @15 :BottomNavbarStatus;
        serialRefreshRequest @16 :SerialRefreshRequest;
        logAppend @17 :LogAppend;
        observationStatus @18 :ObservationStatus;
    }
}
