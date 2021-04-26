use capnp::message::Builder;
use capnp::serialize;

use crate::types::*;

use crate::console_backend_capnp as m;

use async_logger::Writer;
use chrono::Local;
use log::Record;

// Custom formatting of `log::Record`
pub fn splitable_log_formatter(record: &Record) -> String {
    // TODO (JV): Timestamp should be "remote"
    format!(
        "{} {} {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        record.level(),
        record.args()
    )
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
