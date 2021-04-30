use sbp::messages::logging::MsgLog;

use capnp::message::Builder;
use capnp::serialize;

use crate::types::*;

use crate::console_backend_capnp as m;

use async_logger::Writer;
use chrono::Local;
use log::{debug, error, info, warn, Record};

// Custom formatting of `log::Record` to account for SbpLog values
pub fn splitable_log_formatter(record: &Record) -> String {
    // TODO (JV): CPP-117 Extract SbpLog timestamp and level from message
    format!(
        "{} {} {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        record.level(),
        record.args()
    )
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
    // TODO(JV): CPP-117 Include log level and remote timestamp in text message
    match level {
        SbpMsgLevel::Emergency
        | SbpMsgLevel::Alert
        | SbpMsgLevel::Critical
        | SbpMsgLevel::Error => error!("{}", text),
        SbpMsgLevel::Warn | SbpMsgLevel::Notice => warn!("{}", text),
        SbpMsgLevel::Info => info!("{}", text),
        _ => debug!("{}", text),
    }
}

#[derive(Debug)]
pub struct LogPanelWriter<S: MessageSender> {
    pub client_sender: S,
}

impl<S: MessageSender> LogPanelWriter<S> {
    pub fn new(client_sender: S) -> LogPanelWriter<S> {
        LogPanelWriter { client_sender }
    }
}

impl<S: MessageSender> Writer<Box<String>> for LogPanelWriter<S> {
    fn process_slice(&mut self, slice: &[Box<String>]) {
        if slice.is_empty() {
            return;
        }

        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let log_update = msg.init_log_append();
        let mut entries = log_update.init_entries(slice.len() as u32);

        for (idx, item) in slice.iter().enumerate() {
            let mut entry = entries.reborrow().get(idx as u32);

            //TODO: split line into timestamp, level and text
            entry.set_line(&**item);
        }

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        self.client_sender.send_data(msg_bytes);
    }

    fn flush(&mut self) {}
}
