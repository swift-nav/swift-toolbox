@0xe7871c33e8243ee4;

struct ConnectRequest {
}

struct Point {
    x @0 :Float64;
    y @1 :Float64;
}

struct VelocityStatus {
    min @0 :Float64;
    max @1 :Float64;
    points @2 :List(Point);
}

struct Status {
    text @0 :Text;
}

struct Message {
    union {
        connectRequest @0 :ConnectRequest;
        velocityStatus @1 :VelocityStatus;
        status @2 :Status;
    }
}