#![allow(unused_imports)]
use criterion::{criterion_group, criterion_main, Criterion};
use glob::glob;
use std::{fs, path::Path, sync::mpsc, thread, time};

extern crate console_backend;
use console_backend::process_messages;

const TEST_DATA_DIRECTORY: &str = "./benches/data/";
const BENCHMARK_TIME_LIMIT: u64 = 10000;
const BENCHMARK_SAMPLE_SIZE: usize = 50;

pub fn criterion_benchmark_success(c: &mut Criterion) {
    let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("*.sbp");
    let glob_pattern = glob_pattern.to_str().unwrap();

    for ele in glob(glob_pattern).expect("failed to read glob") {
        match ele {
            Ok(filename) => {
                println!("{:?}", filename.display());
                let file_in_name = filename.to_str().unwrap();
                let mut group = c.benchmark_group("proc_messages");
                group.measurement_time(time::Duration::from_millis(BENCHMARK_TIME_LIMIT));
                group.sample_size(BENCHMARK_SAMPLE_SIZE);
                group.bench_function("RPM_success", |b| {
                    b.iter(|| run_process_messages(file_in_name))
                });
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
pub fn criterion_benchmark_failure(c: &mut Criterion) {
    let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("*.sbp");
    let glob_pattern = glob_pattern.to_str().unwrap();

    for ele in glob(glob_pattern).expect("failed to read glob") {
        match ele {
            Ok(filename) => {
                println!("{:?}", filename.display());
                let file_in_name = filename.to_str().unwrap();
                let mut group = c.benchmark_group("proc_messages");
                group.measurement_time(time::Duration::from_millis(BENCHMARK_TIME_LIMIT));
                group.sample_size(BENCHMARK_SAMPLE_SIZE);
                group.bench_function("RPM_failure", |b| {
                    b.iter(|| run_process_messages_failure(file_in_name))
                });
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn run_process_messages(file_in_name: &str) {
    use std::sync::mpsc::Receiver;
    let (client_recv_tx, client_recv_rx) = mpsc::channel::<Receiver<Vec<u8>>>();
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
        let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
        client_recv_tx
            .send(client_recv)
            .expect("sending client recv handle should succeed");

        let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
        process_messages::process_messages(messages, client_send);
    }
    recv_thread.join().expect("join should succeed");
}

fn run_process_messages_failure(file_in_name: &str) {
    use std::sync::mpsc::Receiver;
    let (client_recv_tx, client_recv_rx) = mpsc::channel::<Receiver<Vec<u8>>>();
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
        let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
        client_recv_tx
            .send(client_recv)
            .expect("sending client recv handle should succeed");
        thread::sleep(time::Duration::from_millis(1000));
        let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
        process_messages::process_messages(messages, client_send);
    }
    recv_thread.join().expect("join should succeed");
}

#[cfg(feature = "criterion_bench")]
criterion_group!(
    benches,
    criterion_benchmark_success,
    criterion_benchmark_failure
);
#[cfg(feature = "criterion_bench")]
criterion_main!(benches);

#[cfg(not(feature = "criterion_bench"))]
fn main() {}
