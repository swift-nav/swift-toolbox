use std::{
    collections::HashMap,
    io::Write,
    time::{Duration, Instant},
};

use crossbeam::{atomic::AtomicCell, channel, scope, select, utils::Backoff};
use rand::Rng;
use sbp::messages::{
    file_io::{
        MsgFileioConfigReq, MsgFileioConfigResp, MsgFileioReadDirReq, MsgFileioReadDirResp,
        MsgFileioReadReq, MsgFileioReadResp, MsgFileioRemove, MsgFileioWriteReq,
        MsgFileioWriteResp,
    },
    SBP,
};

use crate::{
    broadcaster::Broadcaster,
    types::{MsgSender, Result},
};

const MAX_RETRIES: usize = 20;

const READDIR_TIMEOUT: Duration = Duration::from_secs(5);
const CONFIG_REQ_RETRY: Duration = Duration::from_millis(100);
const CONFIG_REQ_TIMEOUT: Duration = Duration::from_secs(10);
const CHECK_INTERVAL: Duration = Duration::from_millis(100);
const FILE_IO_TIMEOUT: Duration = Duration::from_secs(3);

const MAX_PAYLOAD_SIZE: usize = 255;
const READ_CHUNK_SIZE: usize = MAX_PAYLOAD_SIZE - 4;
const SEQUENCE_LEN: usize = 4;
const OFFSET_LEN: usize = 4;
const NULL_SEP_LEN: usize = 1;
const WRITE_REQ_OVERHEAD_LEN: usize = SEQUENCE_LEN + OFFSET_LEN + NULL_SEP_LEN;

pub struct Fileio<W> {
    broadcast: Broadcaster,
    sender: MsgSender<W>,
    config: Option<FileIoConfig>,
}

impl<W> Fileio<W>
where
    W: Write + Send,
{
    pub fn new(broadcast: Broadcaster, sender: MsgSender<W>) -> Self {
        Self {
            broadcast,
            sender,
            config: None,
        }
    }

    pub fn read(&mut self, path: String) -> Result<Vec<u8>> {
        let config = self.fetch_config();

        let sender = self.sender.clone();
        let send_msg = move |sequence, offset| {
            sender.send(SBP::from(MsgFileioReadReq {
                sender_id: Some(42),
                filename: path.clone().into(),
                chunk_size: READ_CHUNK_SIZE as u8,
                sequence,
                offset,
            }))
        };

        let (stop_req_tx, stop_req_rx) = channel::bounded(0);
        let (req_tx, req_rx) = channel::unbounded();

        let (res_tx, res_rx) = channel::unbounded();

        let open_requests = AtomicCell::new(0u32);

        scope(|s| {
            s.spawn(|_| {
                let mut sequence = new_sequence();
                let mut offset = 0;
                let backoff = Backoff::new();

                while stop_req_rx.try_recv().is_err() {
                    while open_requests.load() >= config.window_size {
                        backoff.snooze();
                    }
                    send_msg(sequence, offset)?;
                    req_tx.send((sequence, ReadReq::new(offset))).unwrap();
                    offset += READ_CHUNK_SIZE as u32;
                    sequence += 1;
                    open_requests.fetch_add(1);
                }

                sbp::Result::Ok(())
            });

            let (sub, key) = self.broadcast.subscribe::<MsgFileioReadResp>();
            s.spawn(move |_| {
                for res in sub.iter() {
                    res_tx.send(res).unwrap();
                }
            });

            let mut data: HashMap<u32, Vec<u8>> = HashMap::new();
            let mut pending: HashMap<u32, ReadReq> = HashMap::new();
            let mut last_sent = false;

            loop {
                select! {
                    recv(req_rx) -> msg => {
                        let (sequence, request) = msg?;
                        pending.insert(sequence, request);
                    },
                    recv(res_rx) -> msg => {
                        let msg = msg?;
                        let req = match pending.remove(&msg.sequence) {
                            Some(req) => req,
                            None => continue,
                        };
                        let bytes_read = msg.contents.len();
                        data.insert(req.offset, msg.contents);
                        open_requests.fetch_sub(1);
                        if !last_sent && bytes_read != READ_CHUNK_SIZE as usize {
                            last_sent = true;
                            stop_req_tx.send(true).unwrap();
                        }
                        if last_sent && open_requests.load() == 0 {
                            break
                        }
                    },
                    recv(channel::tick(CHECK_INTERVAL)) -> _ => {
                        for (seq, req) in pending.iter_mut() {
                            if req.expired() {
                                req.track_retry()?;
                                send_msg(*seq, req.offset)?;
                            }
                        }
                    }
                }
            }

            self.broadcast.unsubscribe(key);

            let mut data: Vec<_> = data.into_iter().collect();
            data.sort_by_key(|(seq, _)| *seq);
            let results = data.into_iter().fold(Vec::new(), |mut acc, (_, data)| {
                acc.extend(data);
                acc
            });

            Ok(results)
        })
        .unwrap()
    }

    pub fn write(&mut self, mut filename: String, data: &[u8]) -> Result<()> {
        let config = self.fetch_config();

        self.remove(filename.clone())?;

        let filename_len = filename.len();
        let data_len = data.len();
        filename.push(b'\x00' as char);

        let chunk_size = MAX_PAYLOAD_SIZE - WRITE_REQ_OVERHEAD_LEN - filename_len;

        let (req_tx, req_rx) = channel::unbounded();
        let (res_tx, res_rx) = channel::unbounded();

        let open_requests = AtomicCell::new(0u32);

        let sender = self.sender.clone();
        let send_msg = |sequence, offset, end_offset| {
            let data = data[offset..end_offset].to_vec();
            let msg = SBP::from(MsgFileioWriteReq {
                sender_id: Some(42),
                sequence,
                offset: offset as u32,
                filename: filename.clone().into(),
                data,
            });
            sender.send(msg)
        };

        scope(|s| {
            s.spawn(|_| {
                let mut sequence = new_sequence();
                let mut offset = 0;
                let backoff = Backoff::new();

                while offset < data_len {
                    while open_requests.load() >= config.window_size {
                        backoff.snooze();
                    }
                    let end_offset = std::cmp::min(offset + chunk_size, data_len);
                    let chunk_len = std::cmp::min(chunk_size, data_len - offset);
                    let is_last = chunk_len < chunk_size;
                    send_msg(sequence, offset, end_offset)?;
                    req_tx
                        .send((sequence, WriteReq::new(offset, end_offset), is_last))
                        .unwrap();
                    offset += chunk_len;
                    sequence += 1;
                    open_requests.fetch_add(1);
                }

                sbp::Result::Ok(())
            });

            let (sub, key) = self.broadcast.subscribe::<MsgFileioWriteResp>();
            s.spawn(move |_| {
                for res in sub.iter() {
                    res_tx.send(res).unwrap();
                }
            });

            let mut pending: HashMap<u32, WriteReq> = HashMap::new();
            let mut last_sent = false;

            loop {
                select! {
                    recv(req_rx) -> msg => {
                        let (sequence, req, is_last) = msg?;
                        if !last_sent && is_last {
                            last_sent = true;
                        }
                        pending.insert(sequence, req);
                    },
                    recv(res_rx) -> msg => {
                        let msg = msg?;
                        if pending.remove(&msg.sequence).is_none() {
                            continue
                        }
                        open_requests.fetch_sub(1);
                        if last_sent && open_requests.load() == 0 {
                            break;
                        }
                    },
                    recv(channel::tick(CHECK_INTERVAL)) -> _ => {
                        for (seq, req) in pending.iter_mut() {
                            if req.expired() {
                                req.track_retry()?;
                                send_msg(*seq, req.offset, req.end_offset)?;
                            }
                        }
                    }
                }
            }

            self.broadcast.unsubscribe(key);

            Ok(())
        })
        .unwrap()
    }

    pub fn readdir(&mut self, path: String) -> Result<Vec<String>> {
        let mut seq = new_sequence();
        let mut files = vec![];

        loop {
            self.sender.send(SBP::from(MsgFileioReadDirReq {
                sender_id: Some(42),
                sequence: seq,
                offset: files.len() as u32,
                dirname: path.clone().into(),
            }))?;

            let reply = self
                .broadcast
                .wait::<MsgFileioReadDirResp>(READDIR_TIMEOUT)?;

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

    pub fn remove(&self, filename: String) -> Result<()> {
        self.sender.send(SBP::from(MsgFileioRemove {
            sender_id: Some(42),
            filename: filename.into(),
        }))?;
        Ok(())
    }

    fn fetch_config(&mut self) -> FileIoConfig {
        if let Some(ref config) = self.config {
            return config.clone();
        }

        let sequence = new_sequence();
        let (tx, rx) = channel::bounded(0);

        let config = scope(|s| {
            s.spawn(|_| {
                while rx.try_recv().is_err() {
                    let _ = self.sender.send(SBP::from(MsgFileioConfigReq {
                        sender_id: Some(42),
                        sequence,
                    }));
                    std::thread::sleep(CONFIG_REQ_RETRY);
                }
            });

            let config = self
                .broadcast
                .wait::<MsgFileioConfigResp>(CONFIG_REQ_TIMEOUT)
                .map_or_else(|_| Default::default(), Into::into);

            tx.send(true).unwrap();

            config
        })
        .unwrap();

        self.config = Some(config);
        self.config.clone().unwrap()
    }
}

struct ReadReq {
    offset: u32,
    sent_at: Instant,
    retries: usize,
}

impl ReadReq {
    fn new(offset: u32) -> Self {
        Self {
            offset,
            sent_at: Instant::now(),
            retries: 0,
        }
    }

    fn expired(&self) -> bool {
        self.sent_at.elapsed() >= FILE_IO_TIMEOUT
    }

    fn track_retry(&mut self) -> Result<()> {
        self.retries += 1;
        self.sent_at = Instant::now();

        if self.retries >= MAX_RETRIES {
            Err("fileio send message timeout".into())
        } else {
            Ok(())
        }
    }
}

struct WriteReq {
    offset: usize,
    end_offset: usize,
    sent_at: Instant,
    retries: usize,
}

impl WriteReq {
    fn new(offset: usize, end_offset: usize) -> Self {
        Self {
            offset,
            end_offset,
            sent_at: Instant::now(),
            retries: 0,
        }
    }

    fn expired(&self) -> bool {
        self.sent_at.elapsed() >= FILE_IO_TIMEOUT
    }

    fn track_retry(&mut self) -> Result<()> {
        self.retries += 1;
        self.sent_at = Instant::now();

        if self.retries >= MAX_RETRIES {
            Err("fileio send message timeout".into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
struct FileIoConfig {
    window_size: u32,
    batch_size: u32,
}

impl From<MsgFileioConfigResp> for FileIoConfig {
    fn from(msg: MsgFileioConfigResp) -> Self {
        FileIoConfig {
            window_size: msg.window_size,
            batch_size: msg.batch_size,
        }
    }
}

impl Default for FileIoConfig {
    fn default() -> Self {
        FileIoConfig {
            window_size: 100,
            batch_size: 1,
        }
    }
}

fn new_sequence() -> u32 {
    rand::thread_rng().gen_range(0..0xfffffff)
}
