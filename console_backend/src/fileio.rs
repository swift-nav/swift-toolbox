use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail};
use crossbeam::{atomic::AtomicCell, channel, scope, select, utils::Backoff};
use rand::Rng;
use sbp::link::Link;
use sbp::messages::file_io::{
    MsgFileioConfigReq, MsgFileioConfigResp, MsgFileioReadDirReq, MsgFileioReadDirResp,
    MsgFileioReadReq, MsgFileioReadResp, MsgFileioRemove, MsgFileioWriteReq, MsgFileioWriteResp,
};

use crate::errors::FILEIO_CHANNEL_SEND_FAILURE;
use crate::types::{MsgSender, Result};

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

pub struct Fileio<'a> {
    link: Link<'a, ()>,
    sender: MsgSender,
    config: Option<FileioConfig>,
}

impl<'a> Fileio<'a> {
    pub fn new(link: Link<'a, ()>, sender: MsgSender) -> Self {
        Self {
            link,
            sender,
            config: None,
        }
    }

    pub fn read(&mut self, path: String, mut dest: impl Write) -> Result<()> {
        let config = self.config_to_default();

        let sender = self.sender.clone();
        let send_msg = move |sequence, offset| {
            sender.send(
                MsgFileioReadReq {
                    sender_id: None,
                    filename: path.clone().into(),
                    chunk_size: READ_CHUNK_SIZE as u8,
                    sequence,
                    offset,
                }
                .into(),
            )
        };

        let (stop_req_tx, stop_req_rx) = channel::bounded(0);
        let (req_tx, req_rx) = channel::unbounded();

        let (res_tx, res_rx) = channel::unbounded();

        let open_requests = AtomicCell::new(0u32);

        let mut sequence = new_sequence();
        // sequence number of the request we need to write to `dest` next
        let mut current_sequence = sequence;
        // holds data while we wait for out of order requests
        let mut data: HashMap<u32, Vec<u8>> = HashMap::new();
        let mut pending: HashMap<u32, ReadReq> = HashMap::new();
        let mut last_sent = false;

        scope(|s| {
            s.spawn(|_| {
                let mut offset = 0;
                let backoff = Backoff::new();

                while stop_req_rx.try_recv().is_err() {
                    while open_requests.load() >= config.window_size {
                        backoff.snooze();
                    }
                    send_msg(sequence, offset)?;
                    req_tx
                        .send((sequence, ReadReq::new(offset)))
                        .expect(FILEIO_CHANNEL_SEND_FAILURE);
                    offset += READ_CHUNK_SIZE as u32;
                    sequence += 1;
                    open_requests.fetch_add(1);
                }

                Result::Ok(())
            });

            let key = self.link.register(move |msg: MsgFileioReadResp| {
                res_tx.send(msg).expect(FILEIO_CHANNEL_SEND_FAILURE);
            });

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
                        if msg.sequence == current_sequence {
                            dest.write_all(&msg.contents)?;
                            current_sequence += 1;
                            while let Some(d) = data.remove(&current_sequence) {
                                dest.write_all(&d)?;
                                current_sequence += 1;
                            }
                        } else {
                            data.insert(req.offset, msg.contents);
                        }
                        open_requests.fetch_sub(1);
                        if !last_sent && bytes_read != READ_CHUNK_SIZE as usize {
                            last_sent = true;
                            stop_req_tx.send(true).expect(FILEIO_CHANNEL_SEND_FAILURE);
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

            self.link.unregister(key);

            Ok(())
        })
        .unwrap()
    }

    /// Deletes `filename` on the remote device (if it exists) and writes the contents of `data` to the file.
    /// This operation is NOT atomic. If the write fails and `filename` existed, it is gone forever.
    /// For more context see: https://github.com/swift-nav/console_pp/pull/72#discussion_r654751414
    pub fn overwrite(&mut self, filename: String, data: impl Read) -> Result<()> {
        self.overwrite_with_progress(filename, data, |_| ())
    }

    /// Deletes `filename` on the remote device (if it exists) and writes the contents of `data` to the file.
    /// This operation is NOT atomic. If the write fails and `filename` existed, it is gone forever.
    /// For more context see: https://github.com/swift-nav/console_pp/pull/72#discussion_r654751414
    pub fn overwrite_with_progress<'b, F>(
        &mut self,
        filename: String,
        data: impl Read,
        mut on_progress: F,
    ) -> Result<()>
    where
        F: FnMut(usize) + 'b,
    {
        self.remove(filename.clone())?;

        let mut data = BufReader::new(data);
        let mut state = WriteState::new(filename);

        loop {
            let buf = data.fill_buf()?;
            let bytes_read = buf.len();
            if bytes_read == 0 {
                break;
            }
            state = self.write_slice(state, buf, &mut on_progress)?;
            data.consume(bytes_read);
        }

        Ok(())
    }

    fn write_slice<'b, F>(
        &mut self,
        mut state: WriteState,
        data: &[u8],
        on_progress: &mut F,
    ) -> Result<WriteState>
    where
        F: FnMut(usize) + 'b,
    {
        let config = self.fetch_config();

        let (req_tx, req_rx) = channel::unbounded();
        let (res_tx, res_rx) = channel::unbounded();

        let open_requests = AtomicCell::new(0u32);

        let sender = self.sender.clone();
        let send_msg = |state: &WriteState, req: &WriteReq| {
            sender.send(
                MsgFileioWriteReq {
                    sender_id: None,
                    sequence: state.sequence,
                    offset: state.offset as u32,
                    filename: state.filename().into(),
                    data: data[req.offset..req.end_offset].to_vec(),
                }
                .into(),
            )
        };

        let data_len = data.len();

        scope(|s| {
            let key = self.link.register(move |msg: MsgFileioWriteResp| {
                res_tx.send(msg).expect(FILEIO_CHANNEL_SEND_FAILURE);
            });

            s.spawn(|_| {
                let backoff = Backoff::new();
                let mut slice_offset = 0;

                while slice_offset < data_len {
                    while open_requests.load() >= config.window_size {
                        backoff.snooze();
                    }
                    let end_offset = std::cmp::min(slice_offset + state.chunk_size, data_len);
                    let chunk_len = std::cmp::min(state.chunk_size, data_len - slice_offset);
                    let req = WriteReq::new(slice_offset, end_offset, chunk_len < state.chunk_size);
                    send_msg(&state, &req)?;
                    req_tx
                        .send((state.clone(), req))
                        .expect(FILEIO_CHANNEL_SEND_FAILURE);
                    state.update(chunk_len);
                    slice_offset += chunk_len;
                    open_requests.fetch_add(1);
                }

                Result::Ok(())
            });

            let mut pending: HashMap<u32, (WriteState, WriteReq)> = HashMap::new();
            let mut last_sent = false;

            loop {
                select! {
                    recv(req_rx) -> msg => {
                        let (req_state, req) = msg?;
                        if req.is_last {
                            last_sent = true;
                        }
                        pending.insert(req_state.sequence, (req_state, req));
                    },
                    recv(res_rx) -> msg => {
                        let msg = msg?;
                        let req = match pending.remove(&msg.sequence) {
                            Some((_, req)) => req,
                            _ => continue,
                        };
                        on_progress(req.end_offset - req.offset);
                        open_requests.fetch_sub(1);
                        if last_sent && open_requests.load() == 0 {
                            break;
                        }
                    },
                    recv(channel::tick(CHECK_INTERVAL)) -> _ => {
                        for (req_state, req) in pending.values_mut() {
                            if req.expired() {
                                req.track_retry()?;
                                send_msg(req_state, req)?;
                            }
                        }
                    }
                }
            }

            self.link.unregister(key);

            Result::Ok(())
        })
        .unwrap()?;

        Ok(state)
    }

    pub fn readdir(&mut self, path: String) -> Result<Vec<String>> {
        let mut seq = new_sequence();
        let mut files = vec![];

        let (tx, rx) = channel::unbounded();

        let key = self.link.register(move |msg: MsgFileioReadDirResp| {
            tx.send(msg).expect(FILEIO_CHANNEL_SEND_FAILURE);
        });

        self.sender.send(
            MsgFileioReadDirReq {
                sender_id: None,
                sequence: seq,
                offset: files.len() as u32,
                dirname: path.clone().into(),
            }
            .into(),
        )?;

        loop {
            select! {
                recv(rx) -> msg => {
                    let msg = msg?;
                    if msg.sequence != seq {
                        self.link.unregister(key);
                        bail!(
                            "MsgFileioReadDirResp didn't match request ({} vs {})",
                            msg.sequence, seq
                        );
                    }
                    let mut contents = msg.contents;
                    if contents.is_empty() {
                        self.link.unregister(key);
                        return Ok(files);
                    }
                    if contents[contents.len() - 1] == b'\0' {
                        contents.remove(contents.len() - 1);
                    }
                    for f in contents.split(|b| b == &b'\0') {
                        files.push(String::from_utf8_lossy(f).into_owned());
                    }
                    seq += 1;
                    self.sender.send(MsgFileioReadDirReq {
                        sender_id: None,
                        sequence: seq,
                        offset: files.len() as u32,
                        dirname: path.clone().into(),
                    }.into())?;
                },
                recv(channel::tick(READDIR_TIMEOUT)) -> _ => {
                    self.link.unregister(key);
                    bail!("MsgFileioReadDirReq timed out");
                }
            }
        }
    }

    pub fn remove(&self, filename: String) -> Result<()> {
        self.sender.send(
            MsgFileioRemove {
                sender_id: None,
                filename: filename.into(),
            }
            .into(),
        )?;
        Ok(())
    }

    fn config_to_default(&mut self) -> FileioConfig {
        self.config = Some(Default::default());
        self.config.as_ref().unwrap().clone()
    }

    fn fetch_config(&mut self) -> FileioConfig {
        if let Some(ref config) = self.config {
            return config.clone();
        }

        let sequence = new_sequence();
        let (stop_tx, stop_rx) = channel::bounded(1);
        let (tx, rx) = channel::bounded(1);
        let stop_tx_clone = stop_tx.clone();
        let key = self.link.register(move |msg: MsgFileioConfigResp| {
            tx.send(FileioConfig::new(msg))
                .expect(FILEIO_CHANNEL_SEND_FAILURE);
            stop_tx_clone.send(true).expect(FILEIO_CHANNEL_SEND_FAILURE);
        });

        let sender = &self.sender;
        let config = scope(|s| {
            s.spawn(|_| {
                while stop_rx.try_recv().is_err() {
                    let _ = sender.send(
                        MsgFileioConfigReq {
                            sender_id: None,
                            sequence,
                        }
                        .into(),
                    );
                    std::thread::sleep(CONFIG_REQ_RETRY);
                }
            });

            let res = match rx.recv_timeout(CONFIG_REQ_TIMEOUT) {
                Ok(config) => config,
                Err(_) => Default::default(),
            };
            stop_tx.send(true).expect(FILEIO_CHANNEL_SEND_FAILURE);
            res
        })
        .unwrap();

        self.link.unregister(key);

        self.config = Some(config);
        self.config.clone().unwrap()
    }
}

/// State that spans an entire call to `write` (i.e. potentially multiple `write_slice` calls)
#[derive(Debug, Clone)]
struct WriteState {
    sequence: u32,
    /// Offset into the file (not the current slice of data)
    offset: usize,
    filename: String,
    chunk_size: usize,
}

impl WriteState {
    fn new(filename: String) -> Self {
        let (chunk_size, filename) = if filename.ends_with('\x00') {
            (
                MAX_PAYLOAD_SIZE - WRITE_REQ_OVERHEAD_LEN - filename.len() - 1,
                filename,
            )
        } else {
            (
                MAX_PAYLOAD_SIZE - WRITE_REQ_OVERHEAD_LEN - filename.len(),
                filename + "\x00",
            )
        };
        Self {
            sequence: new_sequence(),
            offset: 0,
            filename,
            chunk_size,
        }
    }

    fn filename(&self) -> String {
        self.filename.clone().to_string()
    }

    fn update(&mut self, chunk_len: usize) {
        self.offset += chunk_len;
        self.sequence += 1;
    }
}

#[derive(Debug)]
struct FileioRequest {
    sent_at: Instant,
    retries: usize,
}

impl FileioRequest {
    fn new() -> Self {
        Self {
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
            Err(anyhow!("fileio send message timeout"))
        } else {
            Ok(())
        }
    }
}

impl Default for FileioRequest {
    fn default() -> Self {
        Self::new()
    }
}

struct ReadReq {
    offset: u32,
    req: FileioRequest,
}

impl ReadReq {
    fn new(offset: u32) -> Self {
        Self {
            offset,
            req: FileioRequest::new(),
        }
    }

    fn expired(&self) -> bool {
        self.req.expired()
    }

    fn track_retry(&mut self) -> Result<()> {
        self.req.track_retry()
    }
}

#[derive(Debug)]
struct WriteReq {
    /// Offset start into current slice of data
    offset: usize,
    /// Offset end into current slice of data
    end_offset: usize,
    /// Is this the last request for this chunk of data
    is_last: bool,
    req: FileioRequest,
}

impl WriteReq {
    fn new(offset: usize, end_offset: usize, is_last: bool) -> Self {
        Self {
            offset,
            end_offset,
            is_last,
            req: FileioRequest::new(),
        }
    }

    fn expired(&self) -> bool {
        self.req.expired()
    }

    fn track_retry(&mut self) -> Result<()> {
        self.req.track_retry()
    }
}

#[derive(Debug, Clone)]
struct FileioConfig {
    window_size: u32,
    batch_size: u32,
}

impl FileioConfig {
    fn new(msg: MsgFileioConfigResp) -> Self {
        Self {
            window_size: msg.window_size,
            batch_size: msg.batch_size,
        }
    }
}

impl Default for FileioConfig {
    fn default() -> Self {
        FileioConfig {
            window_size: 100,
            batch_size: 1,
        }
    }
}

pub fn new_sequence() -> u32 {
    rand::thread_rng().gen_range(0..0xfffffff)
}
