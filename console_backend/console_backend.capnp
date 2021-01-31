@0xc3bf8e2ae9033064;

struct FileinRequest {
    filename @0 :Text;
}

struct ConnectRequest {
    host @0 :Text;
    port @1 :UInt16;
}

struct Point {
    x @0 :Float64;
    y @1 :Float64;
}

struct VelocityStatus {
    min @0 :Float64;
    max @1 :Float64;
    hpoints @2 :List(Point);
    vpoints @3 :List(Point);
}

struct TrackingStatus {
    min @0 :Float64;
    max @1 :Float64;
    headers @2 :List(UInt8);
    data @3 :List(List(Point));
}

struct Status {
    text @0 :Text;
}

struct Message {
    union {
        connectRequest @0 :ConnectRequest;
        fileinRequest @1 :FileinRequest;
        velocityStatus @2 :VelocityStatus;
        status @3 :Status;
        trackingStatus @4 :TrackingStatus;
    }
}