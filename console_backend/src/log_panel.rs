use async_logger_log::Logger;
use sbp::messages::logging::MsgLog;

use capnp::message::Builder;

use crate::common_constants as cc;
use crate::constants::LOG_WRITER_BUFFER_MESSAGE_COUNT;
use crate::errors::CONSOLE_LOG_JSON_TO_STRING_FAILURE;
use crate::shared_state::SharedState;
use crate::types::{ArcBool, CapnProtoSender, ClientSender};
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
    let timestamp = Local::now().format("%b %d %Y %H:%M:%S");
    let mut msg = record.args().to_string();
    msg.retain(|c| c != '\0');
    let msg_packet = ConsoleLogPacket {
        level: level.to_string(),
        timestamp: timestamp.to_string(),
        msg,
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

pub fn setup_logging(client_sender: ClientSender, shared_state: SharedState) {
    let log_panel = LogPanelWriter::new(client_sender, shared_state);
    let logger = Logger::builder()
        .buf_size(LOG_WRITER_BUFFER_MESSAGE_COUNT)
        .formatter(splitable_log_formatter)
        .writer(Box::new(log_panel))
        .build()
        .unwrap();

    log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Debug);
}

#[derive(Debug)]
pub struct LogPanelWriter<S: CapnProtoSender> {
    pub client_sender: S,
    shared_state: SharedState,
    pub log_to_std: ArcBool,
}

impl<S: CapnProtoSender> LogPanelWriter<S> {
    pub fn new(client_sender: S, shared_state: SharedState) -> LogPanelWriter<S> {
        LogPanelWriter {
            client_sender,
            log_to_std: shared_state.log_to_std(),
            shared_state,
        }
    }
}

impl<S: CapnProtoSender> Writer<Box<String>> for LogPanelWriter<S> {
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
            let packet: ConsoleLogPacket =
                serde_json::from_str(item).expect(CONSOLE_LOG_JSON_TO_STRING_FAILURE);
            if self.log_to_std.get() {
                eprintln!("{}\t{}\t{}", packet.timestamp, packet.level, packet.msg);
            }
            let mut entry = entries.reborrow().get(idx as u32);

            entry.set_line(item);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }

    fn flush(&mut self) {}
}
