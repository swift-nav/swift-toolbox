#![allow(dead_code)]
#![allow(unused_imports)]
use ndarray::{ArrayView, Axis};
use std::{
    fs,
    sync::{mpsc, Arc, Mutex},
    thread, time,
};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
extern crate console_backend;
use console_backend::process_messages;

const BENCH_FILEPATH: &str = "./tests/data/piksi-relay-1min.sbp";
const MINIMUM_MEM_READINGS: usize = 20;

const DIFF_THRESHOLD: f32 = 0.05;
const MAXIMUM_MEM_USAGE_KB: f32 = 20000.0;
const ABSOLUTE_MINIMUM_MEM_USAGE: f32 = 1000.0;
const MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM: f32 = 0.2;

#[test]
#[cfg(feature = "benches")]
#[ignore]
fn test_run_process_messages() {
    use std::sync::mpsc::Receiver;
    let (client_recv_tx, client_recv_rx) = mpsc::channel::<Receiver<Vec<u8>>>();
    let pid = get_current_pid().unwrap();
    println!("PID: {}", pid);
    let is_running = Arc::new(Mutex::new(true));
    let is_running_recv = Arc::clone(&is_running);
    let is_running_mem = Arc::clone(&is_running);
    let mem_read_thread = thread::spawn(move || {
        let mut sys = System::new();
        let mut mem_readings_kb: Vec<f32> = vec![];
        let mut cpu_readings: Vec<f32> = vec![];
        loop {
            sys.refresh_process(pid);
            let proc = sys.get_process(pid).unwrap();
            mem_readings_kb.push(proc.memory() as f32);
            cpu_readings.push(proc.cpu_usage());
            let is_running_mem = is_running_mem.lock().unwrap();
            if !*is_running_mem {
                break;
            }
        }
        let mems = ArrayView::from_shape(mem_readings_kb.len(), &mem_readings_kb).unwrap();
        let cpus = ArrayView::from_shape(cpu_readings.len(), &cpu_readings).unwrap();
        assert!(
            mem_readings_kb.len() >= MINIMUM_MEM_READINGS,
            format!(
                "Benchmark does not meet minimum samples collected {} requires {}",
                mem_readings_kb.len(),
                MINIMUM_MEM_READINGS
            )
        );
        let mem_usage_mean = mems.mean_axis(Axis(0)).unwrap();
        let mem_usage_std = mems.std_axis(Axis(0), 0.0);
        println!(
            "Memory Usage: {:.2}kB ~ +/- {:.2}kB",
            mem_usage_mean, mem_usage_std
        );
        let mem_usage_mean = mem_usage_mean.into_owned();
        let mem_usage_mean = mem_usage_mean.first().unwrap();
        let mem_usage_std = mem_usage_std.into_owned();
        let mem_usage_std = mem_usage_std.first().unwrap();

        let mem_usage_max = mem_usage_mean + mem_usage_std;

        let mem_usage_min = mem_usage_mean - mem_usage_std;

        assert!((mem_usage_max - MAXIMUM_MEM_USAGE_KB)<=MAXIMUM_MEM_USAGE_KB*DIFF_THRESHOLD, format!(
            "Worst Case Memory Usage: {:.2}kB was {:.2}kb greater than threshold margin {:.2}kB where max is {:.2}kB.", mem_usage_max, (
                mem_usage_max - MAXIMUM_MEM_USAGE_KB), MAXIMUM_MEM_USAGE_KB*DIFF_THRESHOLD, MAXIMUM_MEM_USAGE_KB));
        assert!(*mem_usage_std <= MAXIMUM_MEM_USAGE_KB*MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM, format!(
            "Memory Standard Deviation {:.2}kB was greater than {:.2}kB which is {:.2} of the maximum memory usage {:.2}kB.", *mem_usage_std, (
                MAXIMUM_MEM_USAGE_KB*MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM), MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM, MAXIMUM_MEM_USAGE_KB
        ));
        assert!(
            mem_usage_min >= ABSOLUTE_MINIMUM_MEM_USAGE,
            format!(
                "Best Case Memory Usage: {:.2}kB was less than absolute minimum {:.2}kB.",
                mem_usage_min, ABSOLUTE_MINIMUM_MEM_USAGE
            )
        );
        println!(
            "CPU Usage: {:.2}% ~ +/- {:.2}%",
            cpus.mean_axis(Axis(0)).unwrap(),
            cpus.std_axis(Axis(0), 0.0)
        );
    });
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
        let mut is_running_recv = is_running_recv.lock().unwrap();
        *is_running_recv = false;
    });

    {
        let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
        client_recv_tx
            .send(client_recv)
            .expect("sending client recv handle should succeed");

        let messages = sbp::iter_messages(Box::new(fs::File::open(BENCH_FILEPATH).unwrap()));
        process_messages::process_messages(messages, client_send);
    }
    recv_thread.join().expect("join should succeed");
    mem_read_thread.join().expect("join should succeed");
}
