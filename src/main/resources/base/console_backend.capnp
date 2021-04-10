@0xe7871c33e8243ee4;

struct FileinRequest {
    filename @0 :Text;
}

struct ConnectRequest {
    host @0 :Text;
    port @1 :UInt16;
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
        fileinRequest @3 :FileinRequest;
        trackingSignalsStatus @4 :TrackingSignalsStatus;
        trackingSignalsStatusFront @5 :TrackingSignalsStatusFront;
        solutionVelocityStatusFront @6 :SolutionVelocityStatusFront;
        solutionTableStatus @7 :SolutionTableStatus;
        solutionPositionStatus @8 :SolutionPositionStatus;
        solutionPositionStatusButtonFront @9 :SolutionPositionStatusButtonFront;
        solutionPositionStatusUnitFront @10 :SolutionPositionStatusUnitFront;
    }
}
