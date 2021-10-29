#![allow(unused_imports)]
use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam::channel;
use glob::glob;
use sbp::SbpIterExt;
use std::{
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
    thread, time,
};

extern crate console_backend;
use console_backend::{
    client_sender::ChannelSender,
    common_constants::ConnectionState,
    connection::{Connection, ConnectionManager},
    process_messages,
    shared_state::SharedState,
    types::RealtimeDelay,
};

const BENCH_FILEPATH: &str = "./tests/data/piksi-relay.sbp";
const BENCHMARK_TIME_LIMIT: u64 = 10000;
const BENCHMARK_SAMPLE_SIZE: usize = 50;
const FAILURE_CASE_SLEEP_MILLIS: u64 = 1000;
const BENCH_NAME_FAILURE: &str = "RPM_failure";
const BENCH_NAME_SUCCESS: &str = "RPM_success";

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("proc_messages");
    group.measurement_time(time::Duration::from_millis(BENCHMARK_TIME_LIMIT));
    group.sample_size(BENCHMARK_SAMPLE_SIZE);
    group.bench_function(BENCH_NAME_FAILURE, |b| {
        b.iter(|| run_process_messages(BENCH_FILEPATH, true))
    });
    group.bench_function(BENCH_NAME_SUCCESS, |b| {
        b.iter(|| run_process_messages(BENCH_FILEPATH, false))
    });
}

fn run_process_messages(file_in_name: &str, failure: bool) {
    let (client_recv_tx, client_recv_rx) = channel::unbounded::<channel::Receiver<Vec<u8>>>();
    let recv_thread = thread::spawn(move || {
        let client_recv = client_recv_rx.recv().unwrap();
        let mut iter_count = 0;
        loop {
            if client_recv.recv().is_err() {
                break;
            }
            iter_count += 1;
        }
        assert!(iter_count > 0);
    });
    {
        let (client_send, client_recv) = channel::unbounded::<Vec<u8>>();
        let client_send = ChannelSender::boxed(client_send);
        client_recv_tx
            .send(client_recv)
            .expect("sending client recv handle should succeed");
        if failure {
            thread::sleep(time::Duration::from_millis(FAILURE_CASE_SLEEP_MILLIS));
        }
        let shared_state = SharedState::new();
        shared_state.set_debug(true);
        let conn_manager = ConnectionManager::new(client_send, shared_state);
        conn_manager.connect_to_file(
            file_in_name.into(),
            RealtimeDelay::Off,
            /*close_when_done=*/ true,
        );
    }
    recv_thread.join().expect("join should succeed");
}

#[cfg(feature = "benches")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "benches")]
criterion_main!(benches);

#[cfg(not(feature = "benches"))]
fn main() {}
