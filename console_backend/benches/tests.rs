#![feature(test)]
use glob::glob;
use std::{fs, path::Path, sync::mpsc, time, thread};


const TEST_DATA_DIRECTORY: &str = "./src/tests/data/";
extern crate console_backend;
use console_backend::server;

// use crate::console_backend;
// use crate::capnp;


use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("**/**/*.sbp");
    let glob_pattern = glob_pattern.to_str().unwrap();
    
    for ele in glob(glob_pattern).expect("failed to read glob") {
        
        match ele {
            Ok(filename) => {
                println!("{:?}", filename.display());
                let file_in_name = filename.to_str().unwrap();
                // b.iter(|| run_process_messages(&file_in_name));
                c.bench_function("RPM", |b| b.iter(|| run_process_messages(&file_in_name)));
                
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
                // panic!("the disco!!");
                println!("Panic at the disco!");
            }
            thread::sleep(time::Duration::from_millis(1));
        }
        
    });
    let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
    server::process_messages(messages, client_send)
}
    


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


// #[cfg(test)]
// mod tests {
//     use super::*;
    

//     // use test::Bencher;
//     // #[bench]
//     // fn bench_readfile() {//b: &mut Bencher) {
//     //     let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("**/**/*.sbp");
//     //     let glob_pattern = glob_pattern.to_str().unwrap();
        
//     //     for ele in glob(glob_pattern).expect("failed to read glob") {
            
//     //         match ele {
//     //             Ok(filename) => {
//     //                 println!("{:?}", filename.display());
//     //                 let file_in_name = filename.to_str().unwrap();
//     //                 b.iter(|| run_process_messages(&file_in_name));
                    
//     //             }
//     //             Err(e) => println!("{:?}", e),
//     //         }
//     //     }
//     // }
//     pub fn criterion_benchmark(c: &mut Criterion) {
//         let glob_pattern = Path::new(&TEST_DATA_DIRECTORY).join("**/**/*.sbp");
//         let glob_pattern = glob_pattern.to_str().unwrap();
        
//         for ele in glob(glob_pattern).expect("failed to read glob") {
            
//             match ele {
//                 Ok(filename) => {
//                     println!("{:?}", filename.display());
//                     let file_in_name = filename.to_str().unwrap();
//                     // b.iter(|| run_process_messages(&file_in_name));
//                     c.bench_function("RPM", |b| b.iter(|| run_process_messages(&file_in_name)));
                    
//                 }
//                 Err(e) => println!("{:?}", e),
//             }
//         }
        
//     }
//     fn run_process_messages(file_in_name: &str) {
//         let (client_send, client_recv) = mpsc::channel::<Vec<u8>>();
//         thread::spawn(move|| {
//             loop {
//                 if let Err(_) = client_recv.recv() {
//                     // panic!("the disco!!");
//                     println!("Panic at the disco!");
//                 }
//                 thread::sleep(time::Duration::from_millis(1));
//             }
            
//         });
//         let messages = sbp::iter_messages(Box::new(fs::File::open(file_in_name).unwrap()));
//         server::process_messages(messages, client_send)
//     }

// }
