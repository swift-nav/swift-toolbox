use crate::common_constants as cc;
use crate::constants::*;
use crate::errors::*;
use crate::process_messages::{process_messages, Messages, StopToken};
use crate::shared_state::SharedState;
use crate::types::*;
use crate::watch::Watched;
use anyhow::anyhow;
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

#[derive(Debug)]
pub struct ConnectionManager {
    conn: Watched<Option<Connection>>,
    handle: Option<JoinHandle<()>>,
}

impl ConnectionManager {
    pub fn new(client_send: ClientSender, shared_state: SharedState) -> ConnectionManager {
        let conn = Watched::new(None);
        let handle = Some(thread::spawn({
            let conn = conn.clone();
            move || ConnectionManager::start_thd(client_send, shared_state, conn)
        }));
        ConnectionManager { conn, handle }
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
        self.conn.send(Some(conn));
    }

    /// Helper function for attempting to open a tcp connection and process SBP messages from it.
    ///
    /// # Parameters
    /// - `host`: The host portion of the TCP stream to open.
    /// - `port`: The port to be used to open a TCP stream.
    pub fn connect_to_host(&self, host: String, port: u16) {
        let conn = Connection::tcp(host, port);
        self.conn.send(Some(conn));
    }

    /// Helper function for attempting to open a serial port and process SBP messages from it.
    ///
    /// # Parameters
    /// - `device`: The string path corresponding to the serial device to connect with.
    /// - `baudrate`: The baudrate to use when communicating with the serial device.
    /// - `flow`: The flow control mode to use when communicating with the serial device.
    pub fn connect_to_serial(&self, device: String, baudrate: u32, flow: FlowControl) {
        let conn = Connection::serial(device, baudrate, flow);
        self.conn.send(Some(conn));
    }

    /// Send disconnect signal to server state loop.
    pub fn disconnect(&self) {
        self.conn.send(None);
    }

    fn start_thd(
        mut client_send: ClientSender,
        shared_state: SharedState,
        conn_info: Watched<Option<Connection>>,
    ) {
        let mut pm_thd: Option<JoinHandle<()>> = None;
        let join = |thd: &mut Option<JoinHandle<()>>| {
            if let Some(thd) = thd.take() {
                thd.join().expect("process_messages thread panicked");
            }
        };
        let mut recv = conn_info.watch();

        info!("Console started...");

        while let Ok(conn) = recv.wait() {
            let conn = match conn {
                Some(conn) => conn,
                None => {
                    shared_state.set_connection(ConnectionState::Disconnected, &mut client_send);
                    join(&mut pm_thd);
                    info!("Disconnected successfully.");
                    log::logger().flush();
                    continue;
                }
            };

            let ((stop_token, messages), msg_sender) = match conn.try_connect(Some(&shared_state)) {
                Ok((r, w)) => (Messages::new(r), MsgSender::new(w)),
                Err(err) => {
                    error!("Unable to connect: {}", err);
                    log::logger().flush();
                    continue;
                }
            };

            let conn_watch = conn_info.clone();
            let conn = conn.clone();
            let shared_state = shared_state.clone();
            let mut client_send = client_send.clone();

            shared_state.set_connection(
                ConnectionState::Connected {
                    conn: conn.clone(),
                    stop_token,
                },
                &mut client_send,
            );

            pm_thd = Some(thread::spawn(move || {
                process_messages(
                    messages,
                    msg_sender,
                    conn.clone(),
                    shared_state,
                    client_send,
                );
                if conn.close_when_done() {
                    conn_watch.close();
                }
            }));
        }

        log::logger().flush();
        shared_state.set_connection(ConnectionState::Closed, &mut client_send);
        join(&mut pm_thd);
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) {
        // breaks the `while let Ok(conn) ...` loop
        self.conn.close();
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    /// App is shut down
    Closed,

    /// Running but disconnected
    Disconnected,

    /// Running and connected
    Connected {
        conn: Connection,
        stop_token: StopToken,
    },
}

impl ConnectionState {
    pub fn is_disconnected(&self) -> bool {
        matches!(self, Self::Disconnected)
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Closed => write!(f, "{}", cc::ConnectionState::CLOSED),
            ConnectionState::Disconnected => write!(f, "{}", cc::ConnectionState::DISCONNECTED),
            ConnectionState::Connected { .. } => {
                write!(f, "{}", cc::ConnectionState::CONNECTED)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Connection {
    Tcp(TcpConnection),
    Serial(SerialConnection),
    File(FileConnection),
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

    pub fn name(&self) -> String {
        match self {
            Connection::Tcp(conn) => conn.name(),
            Connection::File(conn) => conn.name(),
            Connection::Serial(conn) => conn.name(),
        }
    }

    pub fn close_when_done(&self) -> bool {
        match self {
            Connection::File(conn) => conn.close_when_done(),
            Connection::Tcp(_) | Connection::Serial(_) => false,
        }
    }

    pub fn realtime_delay(&self) -> RealtimeDelay {
        match self {
            Connection::File(conn) => conn.realtime_delay(),
            Connection::Tcp(_) | Connection::Serial(_) => RealtimeDelay::Off,
        }
    }

    pub fn settings_enabled(&self) -> bool {
        matches!(self, Connection::Tcp(_) | Connection::Serial(_))
    }

    pub fn is_serial(&self) -> bool {
        matches!(self, Connection::Serial(_))
    }

    pub fn try_connect(
        &self,
        shared_state: Option<&SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        match self {
            Connection::Tcp(conn) => conn.clone().try_connect(shared_state),
            Connection::File(conn) => conn.clone().try_connect(shared_state),
            Connection::Serial(conn) => conn.clone().try_connect(shared_state),
        }
    }
}

#[derive(Debug, Clone)]
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

    fn name(&self) -> String {
        self.name.clone()
    }

    fn socket_addrs(name: String) -> Result<SocketAddr> {
        let socket = &mut name.to_socket_addrs()?;
        if let Some(s) = socket.next() {
            Ok(s)
        } else {
            Err(anyhow!("{}", TCP_CONNECTION_PARSING_FAILURE))
        }
    }

    fn try_connect(
        self,
        shared_state: Option<&SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let socket = TcpConnection::socket_addrs(self.name.clone())?;
        let rdr =
            TcpStream::connect_timeout(&socket, Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS))?;
        rdr.set_read_timeout(Some(Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS)))?;
        let writer = rdr.try_clone()?;
        info!("Connected to tcp stream!");
        if let Some(shared_state) = shared_state {
            shared_state.update_tcp_history(self.host, self.port);
        }
        Ok((Box::new(rdr), Box::new(writer)))
    }
}

#[derive(Debug, Clone)]
pub struct SerialConnection {
    name: String,
    device: String,
    baudrate: u32,
    flow: FlowControl,
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
        _shared_state: Option<&SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let rdr = serialport::new(self.device.clone(), self.baudrate)
            .flow_control(*self.flow)
            .timeout(Duration::from_millis(SERIALPORT_READ_TIMEOUT_MS))
            .open()?;
        let writer = rdr.try_clone()?;
        info!("Opened serial port successfully!");

        if let Some(shared_state) = _shared_state {
            shared_state.update_serial_history(self.device, self.baudrate, *self.flow);
        }

        Ok((Box::new(rdr), Box::new(writer)))
    }
}

#[derive(Debug, Clone)]
pub struct FileConnection {
    name: String,
    filepath: PathBuf,
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
        shared_state: Option<&SharedState>,
    ) -> Result<(Box<dyn io::Read + Send>, Box<dyn io::Write + Send>)> {
        let rdr = fs::File::open(&self.filepath)?;
        let writer = io::sink();
        info!("Opened file successfully!");
        if let Some(shared_state) = shared_state {
            shared_state.update_file_history(self.filepath.to_string_lossy().to_string());
        }
        Ok((Box::new(rdr), Box::new(writer)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{backup_file, filename, restore_backup_file};
    use crossbeam::channel;
    use serial_test::serial;
    use std::{
        str::FromStr,
        thread::sleep,
        time::{Duration, SystemTime},
    };
    const TEST_FILEPATH: &str = "./tests/data/piksi-relay-1min.sbp";
    const TEST_SHORT_FILEPATH: &str = "./tests/data/piksi-relay.sbp";
    const SBP_FILE_SHORT_DURATION: Duration = Duration::from_millis(27100);
    const DELAY_BEFORE_CHECKING_APP_STARTED: Duration = Duration::from_millis(1000);

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

    fn receive_thread(client_recv: channel::Receiver<Vec<u8>>) -> JoinHandle<()> {
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
        shared_state.set_debug(true);
        let (client_send_, client_receive) = channel::unbounded::<Vec<u8>>();
        let client_send = ClientSender::new(client_send_);
        let conn_manager = ConnectionManager::new(client_send, shared_state.clone());
        let filename = TEST_SHORT_FILEPATH.to_string();
        receive_thread(client_receive);
        assert!(!shared_state.connection().is_connected());
        conn_manager.connect_to_file(
            filename,
            RealtimeDelay::On,
            /*close_when_done = */ true,
        );
        sleep(DELAY_BEFORE_CHECKING_APP_STARTED);
        assert!(shared_state.connection().is_connected());
        // TODO: [CPP-272] Reassess timing on pause unittest for Windows
        sleep(SBP_FILE_SHORT_DURATION + Duration::from_secs(1));
        drop(conn_manager);
        assert!(!shared_state.connection().is_connected());
        restore_backup_file(bfilename);
    }

    #[test]
    #[serial]
    fn disconnect_via_connect_to_file_test() {
        let bfilename = filename();
        backup_file(bfilename.clone());
        let shared_state = SharedState::new();
        shared_state.set_debug(true);
        let (client_send_, client_receive) = channel::unbounded::<Vec<u8>>();
        let client_send = ClientSender::new(client_send_);
        let conn_manager = ConnectionManager::new(client_send.clone(), shared_state.clone());
        let filename = TEST_FILEPATH.to_string();
        let expected_duration = Duration::from_secs(1);
        let handle = receive_thread(client_receive);
        assert!(!shared_state.connection().is_connected());
        conn_manager.connect_to_file(
            filename,
            RealtimeDelay::On,
            /*close_when_done = */ true,
        );
        sleep(Duration::from_millis(50));
        assert!(shared_state.connection().is_connected());
        let now = SystemTime::now();
        sleep(Duration::from_millis(1));
        conn_manager.disconnect();
        sleep(Duration::from_millis(10));
        assert!(!shared_state.connection().is_connected());
        drop(client_send);
        drop(conn_manager);
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
