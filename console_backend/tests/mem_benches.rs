#[cfg(feature = "benches")]
mod mem_bench_impl {
    use std::{error::Error, result::Result, thread};

    use crossbeam::channel;
    use ndarray::{ArrayView, Axis, Dim};
    use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

    use console_backend::{
        client_sender::ChannelSender,
        connection::ConnectionManager,
        shared_state::{ConnectionState, SharedState},
        types::RealtimeDelay,
    };

    const BENCH_FILEPATH: &str = "./tests/data/piksi-relay-1min.sbp";
    const MINIMUM_MEM_READINGS: usize = 20;

    const DIFF_THRESHOLD: f32 = 0.25;
    const MAXIMUM_MEM_USAGE_BYTES: f32 = 30e6;
    const ABSOLUTE_MINIMUM_MEM_USAGE: f32 = 1e6;
    const MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM: f32 = 0.5;

    /// Convert a 1D Vector to an ArrayView.
    ///
    /// Parameters:
    ///   - `vec_1d`: The generic vector for which to get a read-only ArrayView.
    ///
    /// Returns:
    ///   - `Ok`: ArrayView of the vector passed in.
    ///   - `Err`: If the ArrayView was not converted correctly.
    #[allow(clippy::type_complexity)]
    fn vec_1d_to_array<'a, T>(
        vec_1d: &[T],
    ) -> Result<ArrayView<T, Dim<[usize; 1]>>, Box<dyn Error + 'a>> {
        Ok(ArrayView::from_shape(vec_1d.len(), vec_1d)?)
    }

    /// Get updated information from a running process.
    ///
    /// Asserts:
    ///   - num_mem_readings >= min_allowed_mem_readings
    ///   - mean + std - max_allowed <= max_allowed * threshold_rate
    ///   - std <= max_allowed * max_allowed_std_dev_rate
    ///   - mean - std >= absolute_mean
    #[test]
    fn test_run_process_messages() {
        let pid = get_current_pid().unwrap();
        println!("PID: {}", pid);

        let (client_send, client_recv) = channel::unbounded::<Vec<u8>>();
        let (mem_stop_send, mem_stop_recv) = channel::bounded(1);
        let shared_state = SharedState::new();
        shared_state.set_debug(true);

        let mem_read_thread = thread::spawn(move || {
            let mut sys = System::new();
            let mut mem_readings_bytes: Vec<f32> = vec![];
            let mut cpu_readings: Vec<f32> = vec![];
            loop {
                sys.refresh_process(pid);
                let proc = sys.process(pid).unwrap();
                mem_readings_bytes.push(proc.memory() as f32);
                cpu_readings.push(proc.cpu_usage());
                if mem_stop_recv.try_recv().is_ok() {
                    break;
                }
                // The file is roughly a minute long so storing memory usage
                // every half second is sufficient.
                thread::sleep(std::time::Duration::from_millis(500));
            }
            validate_memory_benchmark(&mem_readings_bytes, &cpu_readings);
        });

        let recv_state = shared_state.clone();
        let recv_thread = thread::spawn(move || {
            let mut iter_count = 0;
            loop {
                if client_recv.recv().is_err() || recv_state.connection().is_disconnected() {
                    break;
                }
                iter_count += 1;
            }
            assert!(iter_count > 0);
            mem_stop_send.send(()).unwrap();
        });

        let mut conn_watch = shared_state.watch_connection();
        let client_send = ChannelSender::boxed(client_send);
        let conn_manager = ConnectionManager::new(client_send, shared_state);

        conn_manager.connect_to_file(
            BENCH_FILEPATH.into(),
            RealtimeDelay::On,
            /*close_when_done=*/ true,
        );
        conn_watch
            .wait_while(ConnectionState::is_disconnected)
            .unwrap();
        conn_watch
            .wait_while(ConnectionState::is_connected)
            .unwrap();
        drop(conn_manager);
        recv_thread.join().expect("join should succeed");
        mem_read_thread.join().expect("join should succeed");
    }

    /// Validate the results of running the memory benchmark.
    ///
    /// Parameters:
    ///   - `mem_readings`: The vector containing all memory readings acquired during benchmark.
    ///   - `cpu_readings`: The vector containing all cpu percentage readings acquired during benchmark.
    fn validate_memory_benchmark(mem_readings: &[f32], cpu_readings: &[f32]) {
        let mems = vec_1d_to_array(mem_readings).unwrap();
        let cpus = vec_1d_to_array(cpu_readings).unwrap();
        assert!(
            mem_readings.len() >= MINIMUM_MEM_READINGS,
            "This benchmark requires {} samples to be collected for analysis and only {} samples were collected.",
            mem_readings.len(),
            MINIMUM_MEM_READINGS
        );
        let mem_usage_mean = mems.mean_axis(Axis(0)).unwrap();
        let mem_usage_std = mems.std_axis(Axis(0), 0.0);
        println!(
            "Memory Usage: {:.2}bytes ~ +/- {:.2}bytes",
            mem_usage_mean, mem_usage_std
        );
        let mem_usage_mean = mem_usage_mean.into_owned();
        let mem_usage_mean = mem_usage_mean.first().unwrap();
        let mem_usage_std = mem_usage_std.into_owned();
        let mem_usage_std = mem_usage_std.first().unwrap();

        let mem_usage_max = mem_usage_mean + mem_usage_std;

        let mem_usage_min = mem_usage_mean - mem_usage_std;

        let mem_usage_over_amount = mem_usage_max - MAXIMUM_MEM_USAGE_BYTES;
        let mem_usage_threshold = MAXIMUM_MEM_USAGE_BYTES * DIFF_THRESHOLD;
        let worst_case_message = format!("Worst Case Memory Usage:\nThe mean memory usage, {:.2}bytes, is added to the stdev, {:.2}bytes, equaling {:.2}bytes.", mem_usage_mean, mem_usage_std, mem_usage_max);
        let worst_case_message = format!("{}\nThen this value is subtracted by the ideal maximum memory usage {:.2}bytes equaling {:.2}bytes.", worst_case_message, MAXIMUM_MEM_USAGE_BYTES, mem_usage_over_amount);
        let worst_case_message = format!(
            "{}\nThis amount is greater than {:.2}bytes which is {:.2} of the maximum amount {:.2}.",
            worst_case_message, mem_usage_threshold, DIFF_THRESHOLD, MAXIMUM_MEM_USAGE_BYTES
        );
        assert!(
            (mem_usage_max - MAXIMUM_MEM_USAGE_BYTES) <= MAXIMUM_MEM_USAGE_BYTES * DIFF_THRESHOLD,
            "{}",
            worst_case_message
        );
        assert!(*mem_usage_std <= MAXIMUM_MEM_USAGE_BYTES*MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM,
                "Memory Standard Deviation, {:.2}bytes, was greater than {:.2}bytes which is {:.2} of the maximum memory usage {:.2}bytes.", *mem_usage_std, (
                    MAXIMUM_MEM_USAGE_BYTES*MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM), MAXIMUM_STANDARD_DEV_RATE_OF_MAXIMUM_MEM, MAXIMUM_MEM_USAGE_BYTES
        );
        assert!(
            mem_usage_min >= ABSOLUTE_MINIMUM_MEM_USAGE,
            "Best Case Memory Usage: {:.2}bytes was less than absolute minimum {:.2}bytes.",
            mem_usage_min,
            ABSOLUTE_MINIMUM_MEM_USAGE
        );
        println!(
            "CPU Usage: {:.2}% ~ +/- {:.2}%",
            cpus.mean_axis(Axis(0)).unwrap(),
            cpus.std_axis(Axis(0), 0.0)
        );
    }
}
