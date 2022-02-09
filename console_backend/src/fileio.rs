use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail};
use crossbeam::{
    channel::{self, Receiver, Sender},
    scope, select,
    sync::Parker,
};
use log::{debug, trace, warn};
use parking_lot::Mutex;
use rand::Rng;
use sbp::{
    link::Link,
    messages::{
        file_io::{
            MsgFileioConfigReq, MsgFileioConfigResp, MsgFileioReadDirReq, MsgFileioReadDirResp,
            MsgFileioReadReq, MsgFileioReadResp, MsgFileioRemove, MsgFileioWriteReq,
            MsgFileioWriteResp,
        },
        ConcreteMessage,
    },
};

use crate::{
    errors::{CROSSBEAM_SCOPE_UNWRAP_FAILURE, FILEIO_CHANNEL_SEND_FAILURE, THREAD_START_FAILURE},
    types::{MsgSender, Result},
};

const MAX_RETRIES: usize = 20;

const READDIR_TIMEOUT: Duration = Duration::from_secs(5);
const CONFIG_REQ_RETRY: Duration = Duration::from_millis(100);
const CONFIG_REQ_TIMEOUT: Duration = Duration::from_secs(10);
const FILE_IO_TIMEOUT: Duration = Duration::from_secs(2);

const READ_CHUNK_SIZE: usize = sbp::MAX_PAYLOAD_LEN - 4;
const SEQUENCE_LEN: usize = 4;
const OFFSET_LEN: usize = 4;
const NULL_SEP_LEN: usize = 1;
const WRITE_REQ_OVERHEAD_LEN: usize = SEQUENCE_LEN + OFFSET_LEN + NULL_SEP_LEN;

pub struct Fileio {
    link: Link<'static, ()>,
    sender: MsgSender,
    config: Option<FileioConfig>,
}

impl Fileio {
    pub fn new(link: Link<'static, ()>, sender: MsgSender) -> Self {
        Self {
            link,
            sender,
            config: None,
        }
    }

    pub fn read(&mut self, path: String, dest: impl Write + Send) -> Result<()> {
        self.read_with_progress(path, dest, |_| ())
    }

    pub fn read_with_progress<F>(
        &mut self,
        path: String,
        mut dest: impl Write + Send,
        mut on_progress: F,
    ) -> Result<()>
    where
        F: FnMut(u64) + Send,
    {
        let mut sequence = new_sequence();
        let mut offset = 0;
        let (tx, rx) = channel::unbounded();
        let key = self.link.register(move |msg: MsgFileioReadResp| {
            let _ = tx.send(msg);
        });
        if let Err(err) = self.sender.send(MsgFileioReadReq {
            sender_id: None,
            filename: path.clone().into(),
            chunk_size: READ_CHUNK_SIZE as u8,
            sequence,
            offset,
        }) {
            self.link.unregister(key);
            return Err(err);
        };

        loop {
            select! {
                recv(rx) -> msg => {
                    let msg = msg?;
                    if let Err(err) = dest.write_all(&msg.contents) {
                        self.link.unregister(key);
                        return Err(err.into());
                    }
                    let bytes_read = msg.contents.len();
                    on_progress(bytes_read as u64);
                    if bytes_read != READ_CHUNK_SIZE {
                        break;
                    }
                    sequence += 1;
                    offset += READ_CHUNK_SIZE as u32;
                    if let Err(err) = self.sender.send(MsgFileioReadReq {
                        sender_id: None,
                        filename: path.clone().into(),
                        chunk_size: READ_CHUNK_SIZE as u8,
                        sequence,
                        offset,
                    }) {
                        self.link.unregister(key);
                        return Err(err);
                    };
                },
                recv(channel::tick(FILE_IO_TIMEOUT)) -> _ => {
                    self.link.unregister(key);
                    bail!("Timed out waiting for file read response. Ensure SBP FILEIO message  {} ({}) is enabled.", MsgFileioReadResp::MESSAGE_TYPE, MsgFileioReadResp::MESSAGE_NAME);
                }
            }
        }

        self.link.unregister(key);
        Ok(())
    }

    /// Deletes `filename` on the remote device (if it exists) and writes the contents of `data` to the file.
    /// This operation is NOT atomic. If the write fails and `filename` existed, it is gone forever.
    /// For more context see: https://github.com/swift-nav/swift-toolbox/pull/72#discussion_r654751414
    pub fn overwrite(&mut self, filename: String, data: impl Read + Send) -> Result<()> {
        self.overwrite_with_progress(filename, data, |_| ())
    }

    /// Deletes `filename` on the remote device (if it exists) and writes the contents of `data` to the file.
    /// This operation is NOT atomic. If the write fails and `filename` existed, it is gone forever.
    /// For more context see: https://github.com/swift-nav/swift-toolbox/pull/72#discussion_r654751414
    pub fn overwrite_with_progress<F>(
        &mut self,
        filename: String,
        data: impl Read + Send,
        mut on_progress: F,
    ) -> Result<()>
    where
        F: FnMut(u64) + Send,
    {
        self.remove(filename.clone())?;
        let config = self.fetch_config();
        let (tx, rx) = channel::bounded(config.batch_size);
        let chunk_sizes = Mutex::new(HashMap::with_capacity(config.window_size));
        let mut data = BufReader::new(data);
        let mut offset = 0;
        let (chunk_size, filename) = if filename.ends_with('\x00') {
            (
                sbp::MAX_PAYLOAD_LEN - WRITE_REQ_OVERHEAD_LEN - filename.len() - 1,
                filename,
            )
        } else {
            (
                sbp::MAX_PAYLOAD_LEN - WRITE_REQ_OVERHEAD_LEN - filename.len(),
                filename + "\x00",
            )
        };
        scope(move |scope| {
            let chunk_sizes = &chunk_sizes;
            scope.spawn(move |_| loop {
                let buf = match data.fill_buf() {
                    Ok(buf) => buf,
                    Err(e) => {
                        let _ = tx.send(Err(e));
                        return;
                    }
                };
                let data_len = buf.len();
                if data_len == 0 {
                    return;
                }
                let mut slice_offset = 0;
                while slice_offset < data_len {
                    let end_offset = std::cmp::min(slice_offset + chunk_size, data_len);
                    let chunk_len = std::cmp::min(chunk_size, data_len - slice_offset);
                    let req = MsgFileioWriteReq {
                        sender_id: None,
                        sequence: 0,
                        offset,
                        filename: filename.clone().into(),
                        data: buf[slice_offset..end_offset].to_vec(),
                    };
                    offset += chunk_len as u32;
                    slice_offset += chunk_len;
                    if tx.send(Ok(req)).is_err() {
                        return;
                    };
                }
                data.consume(data_len);
            });
            with_repeater(
                &self.sender,
                &self.link,
                config,
                move |sequence| {
                    let req = match rx.recv() {
                        Ok(req) => req,
                        Err(_) => return Ok(None),
                    };
                    match req {
                        Ok(mut msg) => {
                            msg.sequence = sequence;
                            chunk_sizes
                                .lock()
                                .insert(msg.sequence, msg.data.len() as u64);
                            Ok(Some(msg))
                        }
                        Err(e) => Err(e.into()),
                    }
                },
                move |msg: MsgFileioWriteResp| {
                    let chunk_size = match chunk_sizes.lock().remove(&msg.sequence) {
                        Some(chunk_size) => chunk_size,
                        None => {
                            debug!("unexpected message {:?}", msg);
                            return;
                        }
                    };
                    on_progress(chunk_size);
                },
            )
        })
        .expect(CROSSBEAM_SCOPE_UNWRAP_FAILURE)?;
        Ok(())
    }

    pub fn readdir(&mut self, path: String) -> Result<Vec<String>> {
        let mut seq = new_sequence();
        let mut files = vec![];

        let (tx, rx) = channel::unbounded();

        let key = self.link.register(move |msg: MsgFileioReadDirResp| {
            tx.send(msg).expect(FILEIO_CHANNEL_SEND_FAILURE);
        });

        self.sender.send(MsgFileioReadDirReq {
            sender_id: None,
            sequence: seq,
            offset: files.len() as u32,
            dirname: path.clone().into(),
        })?;

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
                    })?;
                },
                recv(channel::tick(READDIR_TIMEOUT)) -> _ => {
                    self.link.unregister(key);
                    bail!("Timed out waiting for directory read response. Ensure SBP FILEIO message  {} ({}) is enabled.", MsgFileioReadDirResp::MESSAGE_TYPE, MsgFileioReadDirResp::MESSAGE_NAME);
                }
            }
        }
    }

    pub fn remove(&self, filename: String) -> Result<()> {
        self.sender.send(MsgFileioRemove {
            sender_id: None,
            filename: filename.into(),
        })
    }

    fn fetch_config(&mut self) -> FileioConfig {
        if let Some(config) = self.config {
            return config;
        }

        let sequence = new_sequence();
        let (stop_tx, stop_rx) = channel::unbounded();
        let (tx, rx) = channel::unbounded();
        let stop_tx_clone = stop_tx.clone();
        let key = self.link.register(move |msg: MsgFileioConfigResp| {
            tx.send(FileioConfig::from(msg))
                .expect(FILEIO_CHANNEL_SEND_FAILURE);
            stop_tx_clone.send(true).expect(FILEIO_CHANNEL_SEND_FAILURE);
        });

        let sender = &self.sender;
        let config = scope(|s| {
            s.spawn(|_| {
                while stop_rx.try_recv().is_err() {
                    let _ = sender.send(MsgFileioConfigReq {
                        sender_id: None,
                        sequence,
                    });
                    std::thread::sleep(CONFIG_REQ_RETRY);
                }
            });

            let res = match rx.recv_timeout(CONFIG_REQ_TIMEOUT) {
                Ok(config) => config,
                Err(_) => {
                    warn!("Timed out waiting for fileio config response, continuing with defaults. Ensure SBP FILEIO message {} ({}) is enabled to receive the device's config.", MsgFileioConfigResp::MESSAGE_TYPE, MsgFileioConfigResp::MESSAGE_NAME);
                    Default::default()
                }
            };
            stop_tx.send(true).expect(FILEIO_CHANNEL_SEND_FAILURE);
            res
        })
        .expect(CROSSBEAM_SCOPE_UNWRAP_FAILURE);

        self.link.unregister(key);

        self.config = Some(config);
        self.config.unwrap()
    }
}

pub fn new_sequence() -> u32 {
    rand::thread_rng().gen_range(0..0xfffffff)
}

#[derive(Debug, Clone, Copy)]
struct FileioConfig {
    window_size: usize,
    batch_size: usize,
}

impl From<MsgFileioConfigResp> for FileioConfig {
    fn from(msg: MsgFileioConfigResp) -> Self {
        Self {
            window_size: msg.window_size as usize,
            batch_size: msg.batch_size as usize,
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

fn with_repeater<F, G>(
    sender: &MsgSender,
    link: &Link<'static, ()>,
    config: FileioConfig,
    req_gen: F,
    mut cb: G,
) -> Result<()>
where
    F: FnMut(u32) -> Result<Option<MsgFileioWriteReq>> + Send,
    G: FnMut(MsgFileioWriteResp) + Send,
{
    // maps sequences -> pending requests
    let pending_map = Mutex::new(HashMap::new());
    // each sequence gets sent to this channel. the timeout thread pulls from
    // it and checks the timeouts in order. On a retry the sequence gets pushed
    // back onto the queue
    let (pending_queue_tx, pending_queue_rx) = channel::unbounded();
    // workers send errors via this channel to propagate failure
    let (err_tx, err_rx) = channel::unbounded();
    // forwards messages from the link to the main select loop
    let (res_tx, res_rx) = channel::unbounded();
    // the request generating thread sends requests to the request making thread over this channel
    let (req_tx, req_rx) = channel::bounded(config.batch_size);
    let key = link.register(move |msg: MsgFileioWriteResp| {
        let _ = res_tx.send(msg);
    });
    // when the window is full the request thread goes to sleep. when we get a response
    // we can wake up the thread with this
    let send_parker = Parker::new();
    let send_unparker = send_parker.unparker().clone();
    // kills the timeout thread early if we finish uploading the file
    let timeout_parker = Parker::new();
    let timeout_unparker = timeout_parker.unparker().clone();
    let result = scope(|scope| {
        let err_tx = &err_tx;
        let pending_map = &pending_map;
        let pending_queue_rx = &pending_queue_rx;
        let pending_queue_tx = &pending_queue_tx;
        let res_rx = &res_rx;
        scope
            .builder()
            .name("request-generator".into())
            .spawn(move |_| {
                req_generator_thd(req_gen, req_tx);
            })
            .expect(THREAD_START_FAILURE);
        scope
            .builder()
            .name("request-maker".into())
            .spawn(move |_| {
                req_maker_thd(
                    config,
                    sender,
                    pending_map,
                    pending_queue_tx,
                    err_tx,
                    req_rx,
                    send_parker,
                );
            })
            .expect(THREAD_START_FAILURE);
        scope
            .builder()
            .name("timeout".into())
            .spawn(move |_| {
                timeout_thd(
                    pending_queue_rx,
                    pending_map,
                    err_tx,
                    sender,
                    pending_queue_tx,
                    timeout_parker,
                );
            })
            .expect(THREAD_START_FAILURE);
        loop {
            select! {
                recv(res_rx) -> msg => {
                    let res = msg.expect("response channel closed");
                    if pending_map.lock().remove(&res.sequence).is_none() {
                        trace!("duplicate response {}", res.sequence);
                        continue;
                    }
                    trace!("got response {}", res.sequence);
                    send_unparker.unpark();
                    cb(res);
                    if pending_map.lock().is_empty() {
                        let _ = pending_queue_tx.send(None);
                        timeout_unparker.unpark();
                        return Ok(());
                    }
                }
                recv(err_rx) -> msg => {
                    let err = msg.expect("error channel closed");
                    return Err(err);
                }
            }
        }
    })
    .expect(CROSSBEAM_SCOPE_UNWRAP_FAILURE);
    link.unregister(key);
    result
}

fn req_generator_thd<F: FnMut(u32) -> Result<Option<MsgFileioWriteReq>> + Send>(
    mut req_gen: F,
    req_tx: Sender<Result<Option<MsgFileioWriteReq>>>,
) {
    let sequence = new_sequence();
    for seq in sequence.. {
        let msg = req_gen(seq);
        let exit = msg.as_ref().map_or(true, Option::is_none);
        if req_tx.send(msg).is_err() || exit {
            break;
        }
    }
    debug!("request-generator thread finished");
}
fn req_maker_thd(
    config: FileioConfig,
    sender: &MsgSender,
    pending_map: &Mutex<HashMap<u32, PendingReq>>,
    pending_queue_tx: &Sender<Option<u32>>,
    err_tx: &Sender<anyhow::Error>,
    req_rx: Receiver<Result<Option<MsgFileioWriteReq>>>,
    send_parker: Parker,
) {
    let mut batch = Vec::with_capacity(config.batch_size);
    let send_batch = move |batch: &mut Vec<MsgFileioWriteReq>| -> bool {
        let mut guard = pending_map.lock();
        for req in batch.drain(..config.batch_size.min(batch.len())) {
            match sender.send(req.clone()) {
                Ok(_) => {
                    let req = PendingReq::new(req);
                    let seq = req.message.sequence;
                    guard.insert(seq, req);
                    let _ = pending_queue_tx.send(Some(seq));
                }
                Err(err) => {
                    let _ = err_tx.send(err);
                    return false;
                }
            }
        }
        true
    };
    for req in req_rx {
        let req = match req {
            Ok(Some(req)) => req,
            Err(err) => {
                let _ = err_tx.send(err);
                break;
            }
            _ => break,
        };
        batch.push(req);
        if batch.len() < config.batch_size {
            continue;
        }
        while !window_available(pending_map, config) {
            send_parker.park();
        }
        if !send_batch(&mut batch) {
            break;
        }
        trace!("batch sent");
    }
    debug!("flushing remaining messages");
    while !batch.is_empty() {
        while !window_available(pending_map, config) {
            send_parker.park();
        }
        if !send_batch(&mut batch) {
            break;
        }
    }
    debug!("request-maker thread finished");
}

fn timeout_thd(
    pending_queue_rx: &Receiver<Option<u32>>,
    pending_map: &Mutex<HashMap<u32, PendingReq>>,
    err_tx: &Sender<anyhow::Error>,
    sender: &MsgSender,
    pending_queue_tx: &Sender<Option<u32>>,
    timeout_parker: Parker,
) {
    for seq in pending_queue_rx.iter() {
        let seq = match seq {
            Some(seq) => seq,
            None => break,
        };
        let elapsed = match pending_map.lock().get(&seq) {
            Some(req) => req.sent_at.elapsed(),
            None => continue,
        };
        if elapsed < FILE_IO_TIMEOUT {
            timeout_parker.park_timeout(FILE_IO_TIMEOUT - elapsed);
        }
        let mut guard = pending_map.lock();
        if let Some(req) = guard.get_mut(&seq) {
            req.retries += 1;
            trace!("retry {} times {}", req.message.sequence, req.retries);
            if req.retries >= MAX_RETRIES {
                let _ = err_tx.send(anyhow!("Timed out waiting for file write response. Ensure SBP FILEIO message {} ({}) is enabled.", MsgFileioWriteResp::MESSAGE_TYPE, MsgFileioWriteResp::MESSAGE_NAME));
                break;
            }
            req.sent_at = Instant::now();
            let msg = req.message.clone();
            let seq = msg.sequence;
            drop(guard);
            if let Err(err) = sender.send(msg) {
                let _ = err_tx.send(err);
                break;
            };
            let _ = pending_queue_tx.send(Some(seq));
        }
    }
    debug!("timeout thread finished");
}

fn window_available(pending_map: &Mutex<HashMap<u32, PendingReq>>, config: FileioConfig) -> bool {
    pending_map.lock().len() + config.batch_size <= config.window_size
}

#[derive(Debug, Clone)]
struct PendingReq {
    message: MsgFileioWriteReq,
    retries: usize,
    sent_at: Instant,
}

impl PendingReq {
    fn new(message: MsgFileioWriteReq) -> Self {
        Self {
            message,
            retries: 0,
            sent_at: Instant::now(),
        }
    }
}
