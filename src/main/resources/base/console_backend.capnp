@0xe7871c33e8243ee4;

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
    labels @2 :List(Text);
    data @3 :List(List(Point));
    colors @4 :List(Text);
    checkLabels @5 :List(Text);
}

struct Status {
    text @0 :Text;
}

struct Message {
    union {
        connectRequest @0 :ConnectRequest;
        velocityStatus @1 :VelocityStatus;
        status @2 :Status;
        fileinRequest @3 :FileinRequest;
        trackingStatus @4 :TrackingStatus;
    }
}
