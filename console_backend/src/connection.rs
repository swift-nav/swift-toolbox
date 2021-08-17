use crate::constants::*;
use crate::errors::*;
use crate::process_messages::process_messages;
use crate::types::*;
use chrono::{DateTime, Utc};
use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{error, info};
use std::{
    fmt::Debug,
    fs, io,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    ops::Drop,
    path::{Path, PathBuf},
    thread,
    thread::JoinHandle,
    time::Duration,
};

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub type UtcDateTime = DateTime<Utc>;

#[derive(Clone)]
pub struct TcpConnection {
    name: String,
    host: String,
    port: u16,
}
impl TcpConnection {
    fn new(host: String, port: u16) -> Self {
        let name = format!("{}:{}", host, port);
        Self { name, host, port }
    }
    fn socket_addrs(name: String) -> Result<SocketAddr> {
        let socket = &mut name.to_socket_addrs()?;
        let socket = if let Some(socket_) = socket.next() {
            socket_
        } else {
            let e: Box<dyn std::error::Error> = String::from(TCP_CONNECTION_PARSING_FAILURE).into();
            return Err(e);
        };
        Ok(socket)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn try_connect(
        self,
        shared_state: Option<SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let socket = TcpConnection::socket_addrs(self.name.clone())?;
        let rdr =
            TcpStream::connect_timeout(&socket, Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS))?;
        rdr.set_read_timeout(Some(Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS)))?;
        let wtr = rdr.try_clone()?;
        info!("Connected to tcp stream!");
        if let Some(shared_state_) = shared_state {
            shared_state_.update_tcp_history(self.host, self.port);
        }
        Ok((Box::new(rdr), Box::new(wtr)))
    }
}

#[derive(Clone)]
pub struct SerialConnection {
    pub name: String,
    pub device: String,
    pub baudrate: u32,
    pub flow: FlowControl,
}
impl SerialConnection {
    fn new(device: String, baudrate: u32, flow: FlowControl) -> Self {
        Self {
            name: format!("{} @{}", device, baudrate),
            device,
            baudrate,
            flow,
        }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn try_connect(
        self,
        _shared_state: Option<SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let rdr = serialport::new(self.device, self.baudrate)
            .flow_control(*self.flow)
            .timeout(Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS))
            .open()?;
        let wtr = rdr.try_clone()?;
        info!("Opened serial port successfully!");
        Ok((Box::new(rdr), Box::new(wtr)))
    }
}

#[derive(Clone)]
pub struct FileConnection {
    pub name: String,
    pub filepath: PathBuf,
    close_when_done: bool,
    realtime_delay: RealtimeDelay,
}
impl FileConnection {
    fn new<P: AsRef<Path>>(
        filepath: P,
        close_when_done: bool,
        realtime_delay: RealtimeDelay,
    ) -> Self {
        let filepath = PathBuf::from(filepath.as_ref());
        let name = if let Some(path) = filepath.file_name() {
            path
        } else {
            filepath.as_os_str()
        };
        let name: &str = &*name.to_string_lossy();
        Self {
            name: String::from(name),
            filepath,
            close_when_done,
            realtime_delay,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn close_when_done(&self) -> bool {
        self.close_when_done
    }
    fn realtime_delay(&self) -> RealtimeDelay {
        self.realtime_delay
    }
    fn try_connect(
        self,
        shared_state: Option<SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let rdr = fs::File::open(&self.filepath)?;
        let wtr = io::sink();
        info!("Opened file successfully!");
        if let Some(shared_state_) = shared_state {
            shared_state_.update_file_history(self.filepath.to_string_lossy().to_string());
        }
        Ok((Box::new(rdr), Box::new(wtr)))
    }
}

#[derive(Clone)]
pub enum Connection {
    Tcp(TcpConnection),
    File(FileConnection),
    Serial(SerialConnection),
}
impl Connection {
    pub fn tcp(host: String, port: u16) -> Self {
        Connection::Tcp(TcpConnection::new(host, port))
    }

    pub fn serial(device: String, baudrate: u32, flow: FlowControl) -> Self {
        Connection::Serial(SerialConnection::new(device, baudrate, flow))
    }

    pub fn file(filename: String, realtime_delay: RealtimeDelay, close_when_done: bool) -> Self {
        Connection::File(FileConnection::new(
            filename,
            close_when_done,
            realtime_delay,
        ))
    }
    pub fn close_when_done(&self) -> bool {
        match self {
            Connection::File(conn) => conn.close_when_done(),
            Connection::Tcp(_) | Connection::Serial(_) => false,
        }
    }
    pub fn name(&self) -> String {
        match self {
            Connection::Tcp(conn) => conn.name(),
            Connection::File(conn) => conn.name(),
            Connection::Serial(conn) => conn.name(),
        }
    }
    pub fn realtime_delay(&self) -> RealtimeDelay {
        match self {
            Connection::File(conn) => conn.realtime_delay(),
            Connection::Tcp(_) | Connection::Serial(_) => RealtimeDelay::Off,
        }
    }
    pub fn try_connect(
        &self,
        shared_state: Option<SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        match self {
            Connection::Tcp(conn) => conn.clone().try_connect(shared_state),
            Connection::File(conn) => conn.clone().try_connect(shared_state),
            Connection::Serial(conn) => conn.clone().try_connect(shared_state),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionState {
    pub handler: Option<JoinHandle<()>>,
    shared_state: SharedState,
    sender: Sender<Option<Connection>>,
    receiver: Receiver<Option<Connection>>,
}
impl ConnectionState {
    pub fn new(client_send: ClientSender, shared_state: SharedState) -> ConnectionState {
        let (sender, receiver) = unbounded();

        ConnectionState {
            handler: Some(ConnectionState::connect_thread(
                client_send,
                shared_state.clone(),
                receiver.clone(),
            )),
            shared_state,
            sender,
            receiver,
        }
    }

    fn connect_thread(
        client_send: ClientSender,
        shared_state: SharedState,
        receiver: Receiver<Option<Connection>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut conn = None;
            info!("Console started...");
            while shared_state.clone().is_server_running() {
                if let Ok(conn_option) = receiver.recv_timeout(Duration::from_secs_f64(
                    SERVER_STATE_CONNECTION_LOOP_TIMEOUT_SEC,
                )) {
                    match conn_option {
                        Some(conn_) => {
                            conn = Some(conn_);
                        }
                        None => {
                            conn = None;
                            info!("Disconnected successfully.");
                        }
                    }
                }
                if let Some(conn_) = conn.clone() {
                    if let Err(e) =
                        process_messages(conn_, shared_state.clone(), client_send.clone())
                    {
                        error!("unable to process messages, {}", e);
                    }
                    if !shared_state.is_running() {
                        conn = None;
                    }
                    shared_state.set_running(false, client_send.clone());
                }
                log::logger().flush();
            }
        })
    }

    /// Helper function for attempting to open a file and process SBP messages from it.
    ///
    /// # Parameters
    /// - `filename`: The path to the filename to be read for SBP messages.
    pub fn connect_to_file(
        &self,
        filename: String,
        realtime_delay: RealtimeDelay,
        close_when_done: bool,
    ) {
        let conn = Connection::file(filename, realtime_delay, close_when_done);
        self.connect(conn);
    }

    /// Helper function for attempting to open a tcp connection and process SBP messages from it.
    ///
    /// # Parameters
    /// - `host`: The host portion of the TCP stream to open.
    /// - `port`: The port to be used to open a TCP stream.
    pub fn connect_to_host(&self, host: String, port: u16) {
        let conn = Connection::tcp(host, port);
        self.connect(conn);
    }

    /// Helper function for attempting to open a serial port and process SBP messages from it.
    ///
    /// # Parameters
    /// - `device`: The string path corresponding to the serial device to connect with.
    /// - `baudrate`: The baudrate to use when communicating with the serial device.
    /// - `flow`: The flow control mode to use when communicating with the serial device.
    pub fn connect_to_serial(&self, device: String, baudrate: u32, flow: FlowControl) {
        let conn = Connection::serial(device, baudrate, flow);
        self.connect(conn);
    }

    /// Send disconnect signal to server state loop.
    pub fn disconnect<S: IpcSender>(&self, client_send: S) {
        self.shared_state.set_running(false, client_send);
        if let Err(err) = self.sender.try_send(None) {
            error!("{}, {}", SERVER_STATE_DISCONNECT_FAILURE, err);
        }
    }

    /// Helper function to send connection object to server state loop.
    fn connect(&self, conn: Connection) {
        if let Err(err) = self.sender.try_send(Some(conn)) {
            error!("{}, {}", SERVER_STATE_NEW_CONNECTION_FAILURE, err);
        }
    }
}

impl Drop for ConnectionState {
    fn drop(&mut self) {
        self.shared_state.stop_server_running();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{backup_file, filename, restore_backup_file};
    use serial_test::serial;
    use std::{
        str::FromStr,
        thread::sleep,
        time::{Duration, SystemTime},
    };
    const TEST_FILEPATH: &str = "./tests/data/piksi-relay-1min.sbp";
    const TEST_SHORT_FILEPATH: &str = "./tests/data/piksi-relay.sbp";
    const SBP_FILE_SHORT_DURATION_SEC: f64 = 27.1;
    const DELAY_BEFORE_CHECKING_APP_STARTED_IN_MS: u64 = 150;

    #[test]
    fn create_tcp() {
        let host = String::from("0.0.0.0");
        let port = 55555;
        let conn = Connection::tcp(host.clone(), port);
        assert_eq!(conn.name(), format!("{}:{}", host, port));
        assert!(!conn.close_when_done());
        assert_eq!(conn.realtime_delay(), RealtimeDelay::Off);
    }

    #[test]
    fn create_file() {
        let filepath = String::from(TEST_FILEPATH);
        let realtime_delay_on = RealtimeDelay::On;
        let realtime_delay_off = RealtimeDelay::Off;
        let close_when_done_false = false;
        let close_when_done_true = true;
        let conn = Connection::file(filepath.clone(), realtime_delay_on, close_when_done_true);
        assert_eq!(conn.name(), String::from("piksi-relay-1min.sbp"));
        assert!(conn.close_when_done());
        assert_eq!(conn.realtime_delay(), RealtimeDelay::On);
        let conn = Connection::file(filepath, realtime_delay_off, close_when_done_false);
        assert!(!conn.close_when_done());
        assert_eq!(conn.realtime_delay(), RealtimeDelay::Off);
    }

    #[test]
    fn create_serial() {
        let device = String::from("/dev/ttyUSB0");
        let baudrate = 115200;
        let flow = FlowControl::from_str(FLOW_CONTROL_NONE).unwrap();
        let conn = Connection::serial(device.clone(), baudrate, flow);
        assert_eq!(conn.name(), format!("{} @{}", device, baudrate));
        assert!(!conn.close_when_done());
        assert_eq!(conn.realtime_delay(), RealtimeDelay::Off);
    }

    fn receive_thread(client_recv: crossbeam::channel::Receiver<(u8, Vec<u8>)>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut iter_count = 0;

            loop {
                if client_recv.recv().is_err() {
                    break;
                }

                iter_count += 1;
            }
            assert!(iter_count > 0);
        })
    }

    #[test]
    #[serial]
    fn connect_to_file_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());
        let shared_state = SharedState::new();
        let (client_send_, client_receive) = crossbeam::channel::unbounded::<(u8, Vec<u8>)>();
        let client_send = ClientSender::new(client_send_);
        let connection_state = ConnectionState::new(client_send, shared_state.clone());
        let filename = TEST_SHORT_FILEPATH.to_string();
        receive_thread(client_receive);
        assert!(!shared_state.is_running());
        connection_state.connect_to_file(
            filename,
            RealtimeDelay::On,
            /*close_when_done = */ true,
        );
        sleep(Duration::from_millis(
            DELAY_BEFORE_CHECKING_APP_STARTED_IN_MS,
        ));
        assert!(shared_state.is_running());
        // TODO: [CPP-272] Reassess timing on pause unittest for Windows
        sleep(Duration::from_secs_f64(SBP_FILE_SHORT_DURATION_SEC + 1.0));
        assert!(!shared_state.is_running());
        restore_backup_file(bfilename);
    }

    #[test]
    #[serial]
    fn pause_via_connect_to_file_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());
        let shared_state = SharedState::new();
        let (client_send_, client_receive) = crossbeam::channel::unbounded::<(u8, Vec<u8>)>();
        let client_send = ClientSender::new(client_send_);
        let connection_state = ConnectionState::new(client_send, shared_state.clone());
        let filename = TEST_SHORT_FILEPATH.to_string();
        receive_thread(client_receive);
        assert!(!shared_state.is_running());
        connection_state.connect_to_file(
            filename,
            RealtimeDelay::On,
            /*close_when_done = */ true,
        );
        sleep(Duration::from_millis(
            DELAY_BEFORE_CHECKING_APP_STARTED_IN_MS,
        ));
        assert!(shared_state.is_running());
        shared_state.set_paused(true);
        sleep(Duration::from_secs_f64(SBP_FILE_SHORT_DURATION_SEC));
        assert!(shared_state.is_running());
        shared_state.set_paused(false);
        // TODO: [CPP-272] Reassess timing on pause unittest for Windows
        sleep(Duration::from_secs_f64(SBP_FILE_SHORT_DURATION_SEC + 1.0));
        assert!(!shared_state.is_running());
        restore_backup_file(bfilename);
    }

    #[test]
    #[serial]
    fn disconnect_via_connect_to_file_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());
        let shared_state = SharedState::new();
        let (client_send_, client_receive) = crossbeam::channel::unbounded::<(u8, Vec<u8>)>();
        let client_send = ClientSender::new(client_send_);
        let connection_state = ConnectionState::new(client_send.clone(), shared_state.clone());
        let filename = TEST_FILEPATH.to_string();
        let expected_duration = Duration::from_secs_f64(SERVER_STATE_CONNECTION_LOOP_TIMEOUT_SEC)
            + Duration::from_millis(100);
        let handle = receive_thread(client_receive);
        assert!(!shared_state.is_running());
        {
            connection_state.connect_to_file(
                filename,
                RealtimeDelay::On,
                /*close_when_done = */ true,
            );
        }

        sleep(Duration::from_millis(5));
        assert!(shared_state.is_running());
        let now = SystemTime::now();
        sleep(Duration::from_millis(1));
        connection_state.disconnect(client_send.clone());
        sleep(Duration::from_millis(10));
        assert!(!shared_state.is_running());
        shared_state.stop_server_running();
        drop(client_send);
        assert!(handle.join().is_ok());

        match now.elapsed() {
            Ok(elapsed) => {
                assert!(
                    elapsed < expected_duration,
                    "Time elapsed for disconnect test {:?}, expecting {:?}ms",
                    elapsed,
                    expected_duration
                );
            }
            Err(e) => {
                panic!("unknown error {}", e);
            }
        }
        restore_backup_file(bfilename);
    }

    // TODO(johnmichael.burke@) [CPP-111] Need to implement unittest for TCPStream.
    // #[test]
    // fn connect_to_host_test() {
    // }

    // TODO(johnmichael.burke@) [CPP-111] Need to implement unittest for serial.
    // #[test]
    // fn connect_to_serial_test() {
    // }
}
