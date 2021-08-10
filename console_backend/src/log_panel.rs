use async_logger_log::Logger;
use sbp::messages::logging::MsgLog;

use capnp::message::Builder;

use crate::common_constants as cc;
use crate::constants::LOG_WRITER_BUFFER_MESSAGE_COUNT;
use crate::errors::CONSOLE_LOG_JSON_TO_STRING_FAILURE;
use crate::types::*;
use crate::utils::serialize_capnproto_builder;

use async_logger::Writer;
use chrono::Local;
use log::{debug, error, info, warn, LevelFilter, Record};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ConsoleLogPacket {
    level: String,
    timestamp: String,
    msg: String,
}

const DEVICE: &str = "DEVICE";
const CONSOLE: &str = "CONSOLE";

pub type LogLevel = cc::LogLevel;
impl LogLevel {
    pub fn level_filter(&self) -> LevelFilter {
        match self {
            cc::LogLevel::DEBUG => LevelFilter::Debug,
            cc::LogLevel::INFO => LevelFilter::Info,
            cc::LogLevel::NOTICE | cc::LogLevel::WARNING => LevelFilter::Warn,
            cc::LogLevel::ERROR => LevelFilter::Error,
        }
    }
}

// Custom formatting of `log::Record` to account for SbpLog values
pub fn splitable_log_formatter(record: &Record) -> String {
    let level = if record.target() != DEVICE {
        CONSOLE
    } else {
        record.level().as_str()
    };
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S");
    let msg = record.args();
    let msg_packet = ConsoleLogPacket {
        level: level.to_string(),
        timestamp: timestamp.to_string(),
        msg: msg.to_string(),
    };
    serde_json::to_string(&msg_packet).expect(CONSOLE_LOG_JSON_TO_STRING_FAILURE)
}

enum SbpMsgLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warn = 4,
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
            4 => SbpMsgLevel::Warn,
            5 => SbpMsgLevel::Notice,
            6 => SbpMsgLevel::Info,
            7 => SbpMsgLevel::Debug,
            _ => SbpMsgLevel::Other,
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
        SbpMsgLevel::Warn | SbpMsgLevel::Notice => warn!(target: DEVICE, "{}", text),
        SbpMsgLevel::Info => info!(target: DEVICE, "{}", text),
        _ => debug!(target: DEVICE, "{}", text),
    }
}

pub fn setup_logging(client_sender: ClientSender) {
    let log_panel = LogPanelWriter::new(client_sender);
    let logger = Logger::builder()
        .buf_size(LOG_WRITER_BUFFER_MESSAGE_COUNT)
        .formatter(splitable_log_formatter)
        .writer(Box::new(log_panel))
        .build()
        .unwrap();

    log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
}

#[derive(Debug)]
pub struct LogPanelWriter<S: CapnProtoSender> {
    pub client_sender: S,
}

impl<S: CapnProtoSender> LogPanelWriter<S> {
    pub fn new(client_sender: S) -> LogPanelWriter<S> {
        LogPanelWriter { client_sender }
    }
}

impl<S: CapnProtoSender> Writer<Box<String>> for LogPanelWriter<S> {
    fn process_slice(&mut self, slice: &[Box<String>]) {
        if slice.is_empty() {
            return;
        }

        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let log_update = msg.init_log_append();
        let mut entries = log_update.init_entries(slice.len() as u32);

        for (idx, item) in slice.iter().enumerate() {
            let mut entry = entries.reborrow().get(idx as u32);

            entry.set_line(&**item);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn flush(&mut self) {}
}
