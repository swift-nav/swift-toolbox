use std::{fs::File, io::Write};

use async_logger::Writer;
use async_logger_log::Logger;
use capnp::message::Builder;
use chrono::Local;
use log::{debug, error, info, warn, LevelFilter, Record};
use sbp::messages::logging::MsgLog;
use serde::{Deserialize, Serialize};

use crate::client_sender::BoxedClientSender;
use crate::common_constants as cc;
use crate::constants::LOG_WRITER_BUFFER_MESSAGE_COUNT;
use crate::errors::CONSOLE_LOG_JSON_TO_STRING_FAILURE;
use crate::shared_state::SharedState;
use crate::utils::{serialize_capnproto_builder, OkOrLog};

const DEVICE: &str = "DEVICE";
const CONSOLE: &str = "CONSOLE";

pub type LogLevel = cc::LogLevel;

impl LogLevel {
    pub fn level_filter(&self) -> LevelFilter {
        match self {
            LogLevel::DEBUG => LevelFilter::Debug,
            LogLevel::INFO => LevelFilter::Info,
            LogLevel::NOTICE | LogLevel::WARNING => LevelFilter::Warn,
            LogLevel::ERROR => LevelFilter::Error,
        }
    }
}

pub fn handle_log_msg(msg: MsgLog) {
    let text = msg.text.to_string();
    let level: SbpMsgLevel = SbpMsgLevel::from(msg.level);
    match level {
        SbpMsgLevel::Emergency
        | SbpMsgLevel::Alert
        | SbpMsgLevel::Critical
        | SbpMsgLevel::Error => error!(target: DEVICE, "{}", text),
        SbpMsgLevel::Warning | SbpMsgLevel::Notice => warn!(target: DEVICE, "{}", text),
        SbpMsgLevel::Info => info!(target: DEVICE, "{}", text),
        _ => debug!(target: DEVICE, "{}", text),
    }
}

pub fn setup_logging(client_sender: BoxedClientSender, shared_state: SharedState) {
    let log_panel = LogPanelWriter::new(client_sender, shared_state);
    let logger = Logger::builder()
        .buf_size(LOG_WRITER_BUFFER_MESSAGE_COUNT)
        .formatter(splitable_log_formatter)
        .writer(Box::new(log_panel))
        .build()
        .unwrap();

    log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Warn);
}

#[derive(Debug)]
struct LogPanelWriter {
    client_sender: BoxedClientSender,
    shared_state: SharedState,
    log_file: Option<File>,
}

impl LogPanelWriter {
    pub fn new(client_sender: BoxedClientSender, shared_state: SharedState) -> LogPanelWriter {
        LogPanelWriter {
            log_file: init_log_file(&shared_state),
            client_sender,
            shared_state,
        }
    }
}

impl Writer<Box<String>> for LogPanelWriter {
    fn process_slice(&mut self, slice: &[Box<String>]) {
        if slice.is_empty() {
            return;
        }

        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut log_update = msg.init_log_append();
        log_update.set_log_level(&self.shared_state.log_level().to_string());

        let mut entries = log_update.init_entries(slice.len() as u32);
        for (idx, item) in slice.iter().enumerate() {
            let mut entry = entries.reborrow().get(idx as u32);
            entry.set_line(item);
        }
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));

        if let Some(ref mut f) = self.log_file {
            for item in slice {
                let packet = serde_json::from_str(item).expect(CONSOLE_LOG_JSON_TO_STRING_FAILURE);
                write_packet(packet, f);
            }
        }
    }

    fn flush(&mut self) {
        if let Some(ref mut f) = self.log_file {
            let _ = f.flush();
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ConsoleLogPacket<'a> {
    level: &'a str,
    timestamp: &'a str,
    msg: &'a str,
}

enum SbpMsgLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
    Other,
}

impl From<u8> for SbpMsgLevel {
    fn from(orig: u8) -> Self {
        match orig {
            0 => SbpMsgLevel::Emergency,
            1 => SbpMsgLevel::Alert,
            2 => SbpMsgLevel::Critical,
            3 => SbpMsgLevel::Error,
            4 => SbpMsgLevel::Warning,
            5 => SbpMsgLevel::Notice,
            6 => SbpMsgLevel::Info,
            7 => SbpMsgLevel::Debug,
            _ => SbpMsgLevel::Other,
        }
    }
}

fn init_log_file(shared_state: &SharedState) -> Option<File> {
    let filepath = shared_state
        .log_filename()
        .map(|f| shared_state.logging_directory().join(f));
    filepath.and_then(|p| {
        File::create(&p).ok_or_log(|e| {
            let fname = p.display();
            error!("issue creating console log file, {fname}, error, {e}");
        })
    })
}

fn write_packet(packet: ConsoleLogPacket, f: &mut File) {
    // Min one space plus the longest log level
    const MIN_SPACES: usize = "CONSOLE".len() + 1;

    let spaces = " ".repeat(MIN_SPACES - packet.level.len());
    let _ = writeln!(
        f,
        "{timestamp} {level}{spaces}{msg}",
        timestamp = packet.timestamp,
        level = packet.level,
        msg = packet.msg,
    );
}

// Custom formatting of `log::Record` to account for SbpLog values
fn splitable_log_formatter(record: &Record) -> String {
    let level = if record.target() != DEVICE {
        CONSOLE
    } else {
        let level = record.level().as_str();
        if level == "WARN" {
            "WARNING"
        } else {
            level
        }
    };
    let timestamp = Local::now().format("%b %d %Y %H:%M:%S").to_string();
    let mut msg = record.args().to_string();
    msg.retain(|c| c != '\0');
    let msg_packet = ConsoleLogPacket {
        level,
        timestamp: &timestamp,
        msg: &msg,
    };
    serde_json::to_string(&msg_packet).expect(CONSOLE_LOG_JSON_TO_STRING_FAILURE)
}
