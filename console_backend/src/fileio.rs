use std::{collections::HashMap, io::Write, time::Duration};

use crossbeam::{channel, queue::ArrayQueue};
use rand::Rng;
use sbp::messages::{
    file_io::{
        MsgFileioConfigReq, MsgFileioConfigResp, MsgFileioReadDirReq, MsgFileioReadDirResp,
        MsgFileioReadReq, MsgFileioReadResp, MsgFileioWriteReq,
    },
    SBP,
};

use crate::{
    types::Result,
    utils::{MsgDispatcher, MsgSender},
};

const WINDOW_SIZE: u8 = 100;
const MAX_PAYLOAD_SIZE: u8 = 255;
const READ_CHUNK_SIZE: u8 = MAX_PAYLOAD_SIZE - 4;
const READDIR_TIMEOUT_SECS: u64 = 5;
const CONFIG_REQ_RETRY_MS: u64 = 100;
const CONFIG_REQ_TIMEOUT_SECS: u64 = 10;
const FILE_IO_TIMEOUT_SECS: u64 = 3;

// MsgFileioConfigResp {
//     sender_id: Some(
//         31183,
//     ),
//     sequence: 119337718,
//     window_size: 256,
//     batch_size: 32,
//     fileio_version: 0,
// }


pub struct FileIo<W> {
    dispatcher: MsgDispatcher,
    sender: MsgSender<W>,
    config: Option<MsgFileioConfigResp>,
    request_pool: ArrayQueue<SBP>,
    batched_messages: Vec<SBP>,
}

impl<W: Write + Send> FileIo<W> {

    // fn send(&mut self, msg: SBP) {
    //     self.batched_messages.push(msg);
    //     if self.batched_messages.len() >= {

    //     }
    // }

    // fn window_available(&self) -> bool {
    //     self.request_pool.len()
    // }

    pub fn new(dispatcher: MsgDispatcher, sender: MsgSender<W>) -> Self {
        Self {
            dispatcher,
            sender,
            config: None,
            request_pool: ArrayQueue::new(2),
            batched_messages: vec![],
        }
    }

    pub fn read(&mut self, path: String) -> Result<()> {
        let mut seq = new_sequence();
        let mut offset = 0;
        let mut data: HashMap<u32, Vec<u8>> = HashMap::new();

        let (rx, key) = self.dispatcher.on::<MsgFileioReadResp>();

        loop {
            // add pending
            self.sender.send(&SBP::from(MsgFileioReadReq {
                sender_id: Some(42),
                filename: path.clone().into(),
                sequence: seq,
                chunk_size: READ_CHUNK_SIZE,
                offset,
            }));
            offset += READ_CHUNK_SIZE as u32;
        }
    }

    pub fn readdir(&mut self, path: String) -> Result<Vec<String>> {
        dbg!(self.fetch_config());
        let mut seq = new_sequence();
        let mut files = vec![];

        loop {
            self.sender.send(&SBP::from(MsgFileioReadDirReq {
                sender_id: Some(42),
                sequence: seq,
                offset: files.len() as u32,
                dirname: path.clone().into(),
            }));

            let reply = self
                .dispatcher
                .wait::<MsgFileioReadDirResp>(READDIR_TIMEOUT_SECS)?;

            if reply.sequence != seq {
                return Err(format!(
                    "MsgFileioReadDirResp didn't match request ({} vs {})",
                    reply.sequence, seq
                )
                .into());
            }

            let mut contents = reply.contents;
            if contents.is_empty() {
                return Ok(files);
            }
            if contents[contents.len() - 1] == b'\0' {
                contents.remove(contents.len() - 1);
            }
            for f in contents.split(|b| b == &b'\0') {
                files.push(String::from_utf8_lossy(f).into_owned());
            }
            seq += 1;
        }
    }

    pub fn write(&self, path: String, bytes: &[u8]) -> Result<()> {
        todo!()
    }

    pub fn remove(&self, path: String) -> Result<()> {
        todo!()
    }

    fn fetch_config(&mut self) -> &MsgFileioConfigResp {
        if let Some(ref config) = self.config {
            return config;
        }

        let sequence = new_sequence();
        let (tx, rx) = channel::bounded::<()>(1);

        let config = crossbeam::thread::scope(|s| {
            s.spawn(|_| {
                while rx.try_recv().is_err() {
                    self.sender.send(&SBP::from(MsgFileioConfigReq {
                        sender_id: Some(42),
                        sequence,
                    }));
                    std::thread::sleep(Duration::from_millis(CONFIG_REQ_RETRY_MS));
                }
            });

            let config = match self
                .dispatcher
                .wait::<MsgFileioConfigResp>(CONFIG_REQ_TIMEOUT_SECS)
            {
                Ok(msg) => msg,
                Err(_) => MsgFileioConfigResp {
                    sender_id: None,
                    sequence: 0,
                    window_size: 100,
                    batch_size: 1,
                    fileio_version: 0,
                },
            };

            tx.send(()).unwrap();

            config
        })
        .unwrap();

        self.config = Some(config);
        self.config.as_ref().unwrap()
    }
}

fn new_sequence() -> u32 {
    rand::thread_rng().gen_range(0..0xfffffff)
}
