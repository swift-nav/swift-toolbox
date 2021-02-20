#![feature(test)]
use glob::glob;
use std::{fs, path::Path, sync::mpsc, time, thread};


const TEST_DATA_DIRECTORY: &str = "./benches/data/";
extern crate console_backend;
use console_backend::process_messages;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("**/**/*.sbp");
    let glob_pattern = glob_pattern.to_str().unwrap();
    
    for ele in glob(glob_pattern).expect("failed to read glob") {
        
        match ele {
            Ok(filename) => {
                println!("{:?}", filename.display());
                let file_in_name = filename.to_str().unwrap();
                let mut group = c.benchmark_group("proc_messages");
                group.sample_size(10);
                group.bench_function("RPM", |b| b.iter(|| run_process_messages(file_in_name.clone())));
                
            }
            Err(e) => println!("{:?}", e),
        }
    }
    
}
fn run_process_messages(file_in_name: &str) {
    let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move|| {
        loop {
            if let Err(_) = client_recv.recv() {
            }
            thread::sleep(time::Duration::from_millis(1));
        }
        
    });
    let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
    process_messages::process_messages(messages, client_send)
}
    

#[cfg(feature = "criterion_bench")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "criterion_bench")]
criterion_main!(benches);

#[cfg(not(feature = "criterion_bench"))]
fn main() {}
