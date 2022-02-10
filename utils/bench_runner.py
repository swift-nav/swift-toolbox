import argparse
import json
import subprocess
import sys
import time
from multiprocessing.pool import ThreadPool
from typing import Any, Dict, List, Optional, Tuple

import psutil  # type: ignore

# sys.platform keys.
LINUX = "linux"
MACOS = "darwin"
WINDOWS = "win32"

# Benchmark keys.
EXPECTED = "expected"
ERROR_MARGIN_FRAC = "error_margin_frac"
FILE_PATH = "file_path"
KEY_LOCATION = "key_location"
NAME = "name"
SUCCESS = "success"
RESULTS = "results"

## Benchmark Specific Functions and Containers.
# Installer Disk Usage Benchmark.
DISK_USAGE_COMMAND = lambda file_path: f"du -ch {file_path} | grep total"

INSTALLER_MAX_SIZE = 100
INSTALLER_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "Windows Installer",
            FILE_PATH: "release-archive.filename",
            EXPECTED: 55,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    MACOS: [
        {
            NAME: "macOS Installer",
            FILE_PATH: "release-archive.filename",
            EXPECTED: 115,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    LINUX: [
        {
            NAME: "Linux Installer",
            FILE_PATH: "release-archive.filename",
            EXPECTED: 85,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
}

# Backend CPU Benchmark.
BACKEND_CPU_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_success/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 77500000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: True,
        },
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_failure/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 77500000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: False,
        },
    ],
    MACOS: [
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_success/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 90000000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: True,
        },
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_failure/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 110000000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: False,
        },
    ],
    LINUX: [
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_success/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 77500000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: True,
        },
        {
            NAME: "piksi-relay.sbp",
            FILE_PATH: "target/criterion/proc_messages/RPM_failure/base/estimates.json",
            KEY_LOCATION: "mean.point_estimate",
            EXPECTED: 77500000,
            ERROR_MARGIN_FRAC: 0.05,
            SUCCESS: False,
        },
    ],
}

# Frontend CPU Benchmarks.
DEFAULT_JSON_FILEPATH = "fileout.json"
BENCHMARK_COMMAND_ARGS = lambda file_path: f" --exit-after --file {file_path}"
HYPERFINE_COMMAND = (
    lambda file_out: f'hyperfine --warmup 1 --runs 5 --cleanup "sleep 1" --show-output --export-json {file_out} '
)

FRONTEND_CPU_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 50.0,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    MACOS: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 12.0,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    LINUX: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 80.0,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
}

# Frontend MEM Benchmarks.
MAXIMUM_RATE_OF_MAX_STD = "maximum_rate_of_max_std"
MAXIMUM_RATE_OF_MAX_MEAN = "maximum_rate_of_max_mean"
MAXIMUM_MEAN_MB = "maximum_mean_mb"

BYTES_TO_MB = lambda x: float(x) / (1 << 20)
ABSOLUTE_MINIMUM_MEMORY_MB = 1
ABSOLUTE_MINIMUM_READINGS = 200
THREAD_TIMEOUT_SEC = 180
RUN_COUNT = 5

FRONTEND_MEM_BENCHMARKS: Dict[str, List[Dict[str, Any]]] = {
    WINDOWS: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            MAXIMUM_MEAN_MB: 350,
            MAXIMUM_RATE_OF_MAX_MEAN: 0.05,
            MAXIMUM_RATE_OF_MAX_STD: 0.4,
        },
    ],
    MACOS: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            MAXIMUM_MEAN_MB: 300,
            MAXIMUM_RATE_OF_MAX_MEAN: 0.05,
            MAXIMUM_RATE_OF_MAX_STD: 0.3,
        },
    ],
    LINUX: [
        {
            NAME: "piksi-relay-5sec",
            FILE_PATH: "data/piksi-relay-5sec.sbp",
            MAXIMUM_MEAN_MB: 400,
            MAXIMUM_RATE_OF_MAX_MEAN: 0.05,
            MAXIMUM_RATE_OF_MAX_STD: 0.4,
        },
    ],
}


def get_nested_key(nested_dict: dict, key_path: str) -> Optional[Any]:
    """Extract a key in nested dict/json assuming stringified key_path.

    Assuming `key_path` format: <key1>.<key2>.<key3>

    Args:
        nested_dict (dict): The nested dictionary containing the desired key.
        key_path (str): The stringified nested dictionary key location.

    Returns:
        Optional[Any]: A value corresponding to the key_path in the nested dictionary.
            Otherwise, None if not found.
    """
    current_key, *next_keys = key_path.split(".", 1)
    value = nested_dict.get(current_key, None)
    return value if not isinstance(value, dict) and len(next_keys) != 1 else get_nested_key(value, next_keys[0])


def run_disk_usage_benchmark():
    """Runner for disk usage benchmark validations."""
    os_ = sys.platform
    benchmarks = INSTALLER_BENCHMARKS.get(os_, [])
    for bench in benchmarks:
        release_file = ""
        with open(bench[FILE_PATH], "r", encoding="utf-8") as archive_file:
            release_file = archive_file.readline().rstrip()

        bench_command = DISK_USAGE_COMMAND(release_file)
        disk_usage = subprocess.run(bench_command, shell=True, text=True, check=True, capture_output=True)
        print(disk_usage)
        disk_usage, _ = disk_usage.stdout.split("\t")
        print(disk_usage)

        disk_usage = float(disk_usage.rstrip("M"))
        assert disk_usage is not None, f"Test:{bench[NAME]} retrieved bench value None."
        assert disk_usage >= bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (
            f"Test:{bench[NAME]} Bench Value:{disk_usage} not larger than "
            f"{bench[ERROR_MARGIN_FRAC]*bench[EXPECTED]}MB."
        )
        assert (
            disk_usage <= INSTALLER_MAX_SIZE
        ), f"Test:{bench[NAME]} Bench Value:{disk_usage} not less than {INSTALLER_MAX_SIZE}"  # type: ignore
        print(f"PASS - {os_}:{bench[NAME]} MARGIN={disk_usage - bench[EXPECTED]}")


def run_backend_cpu_validate_benchmarks():
    """Runner for a suite of cpu benchmark validations."""
    os_ = sys.platform
    benchmarks = BACKEND_CPU_BENCHMARKS.get(os_, [])
    for bench in benchmarks:
        with open(bench[FILE_PATH], "r", encoding="utf-8") as fileo:
            bench_result = json.load(fileo)
            bench_value = get_nested_key(bench_result, bench[KEY_LOCATION])
            assert bench_value is not None, f"Test:{bench[NAME]} retrieved bench value None."
            if bench[SUCCESS]:
                assert bench_value - bench[EXPECTED] <= bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (
                    f"Success Test:{bench[NAME]} Bench Value:{bench_value} not within "
                    f"{bench[ERROR_MARGIN_FRAC]} of {bench[EXPECTED]}."
                )
                print(f"PASS - {os_}:{bench[NAME]} MARGIN={bench_value - bench[EXPECTED]}")
            else:
                assert bench_value - bench[EXPECTED] > bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (
                    f"Failure Test:{bench[NAME]} Bench Value:{bench_value} not outside of "
                    f"{bench[ERROR_MARGIN_FRAC]} of {bench[EXPECTED]}."
                )
                print(f"PASS(Fail Test) - {os_}:{bench[NAME]} MARGIN={bench_value - bench[EXPECTED]}")


def run_frontend_cpu_benchmark(executable: str):
    """Runner for a suite of frontend cpu benchmark validations.
    Args:
        executable (str): Path to the executable location to run the benchmark on.
    """
    prepped_command = f"{executable}"
    os_ = sys.platform
    benchmarks = FRONTEND_CPU_BENCHMARKS.get(sys.platform, [])
    for bench in benchmarks:
        bench_command = (
            f'{HYPERFINE_COMMAND(DEFAULT_JSON_FILEPATH)} "{prepped_command} '
            f'{BENCHMARK_COMMAND_ARGS(bench[FILE_PATH])}"'
        )
        subprocess.call(bench_command, shell=True)
        with open(DEFAULT_JSON_FILEPATH, "r", encoding="utf-8") as fileo:
            bench_result = json.load(fileo)
            bench_value = bench_result[RESULTS][0].get(bench[KEY_LOCATION], None)
            assert bench_value is not None, f"Test:{bench[NAME]} retrieved bench value None."
            assert bench_value - bench[EXPECTED] <= bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (  # type: ignore
                f"Test:{bench[NAME]} Bench Value:{bench_value} not within "
                f"{bench[ERROR_MARGIN_FRAC]} of {bench[EXPECTED]}."
            )
            print(f"PASS - {os_}:{bench[NAME]} MARGIN={bench_value - bench[EXPECTED]}")


def collect_memory_readings(pid: str) -> List[float]:
    """Collect a series of memory readings for a running process.

    If a child process is present, switch focus to the child process.

    Args:
        pid (str): The PID of the process for which to read memory usage.

    Returns:
        List[float]: All memory readings collected with zeros filtered out.
    """
    memory_readings = []
    proc = psutil.Process(pid)
    time.sleep(1)  # Give the process time to start up.
    try:
        while proc.status() != psutil.STATUS_ZOMBIE:
            total_mem_usage = 0
            for pid_ in [proc] + proc.children(recursive=True):
                total_mem_usage += pid_.memory_info().rss
            memory_readings.append(total_mem_usage)
    except (psutil.AccessDenied, psutil.NoSuchProcess):
        pass
    return [float(reading) for reading in memory_readings if reading != 0]


def get_mean_and_pop_stdev(values: List[float]) -> Tuple[float, float]:
    """Get the mean and population standard deviation.

    Args:
        values (List[float]): The list of values to parse for statistics.

    Returns:
        Tuple[float, float]: The mean and population standard deviation.
    """
    lenn = float(len(values))
    mean = sum(values) / lenn
    std = ((1.0 / lenn) * sum([(val - mean) ** 2 for val in values])) ** (1 / 2)
    return (mean, std)


def run_frontend_mem_benchmark(executable: str):
    """Runner for a suite of frontend cpu benchmark validations.
    Args:
        executable (str): Path to the executable location to run the benchmark on.
    """
    os_ = sys.platform
    benchmarks = FRONTEND_MEM_BENCHMARKS.get(os_, [])
    for bench in benchmarks:
        bench_command = f"{executable} {BENCHMARK_COMMAND_ARGS(bench[FILE_PATH])}"
        for _ in range(RUN_COUNT):
            pool = ThreadPool(processes=1)
            with subprocess.Popen(bench_command.split()) as process:
                mem_readings = pool.apply_async(collect_memory_readings, (process.pid,)).get(THREAD_TIMEOUT_SEC)
            mean_bytes, std_bytes = get_mean_and_pop_stdev(mem_readings)
            mean_mb = BYTES_TO_MB(mean_bytes)
            std_mb = BYTES_TO_MB(std_bytes)
            print(f"Mean: {mean_mb:.2f}MB, Stdev: {std_mb:.2f}MB")
            assert (
                len(mem_readings) >= ABSOLUTE_MINIMUM_READINGS
            ), f"Not enough readings recorded {len(mem_readings)} < {ABSOLUTE_MINIMUM_READINGS}"
            mean_std_max_diff = mean_mb + std_mb - bench[MAXIMUM_MEAN_MB]
            max_diff_allowed = bench[MAXIMUM_MEAN_MB] * bench[MAXIMUM_RATE_OF_MAX_MEAN]
            assert (
                mean_std_max_diff <= max_diff_allowed
            ), f"mean + std - max, {mean_std_max_diff}, > max * max_rate, {max_diff_allowed}"

            max_std_allowed = bench[MAXIMUM_MEAN_MB] * bench[MAXIMUM_RATE_OF_MAX_STD]
            assert std_mb <= max_std_allowed, f"std, {std_mb}, > max_std_allowed, {max_std_allowed}"

            print(f"PASS - {os_}:{bench[NAME]} MARGIN={mean_std_max_diff}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--disk_usage", action="store_true", help="Run Installer Disk Usage Benchmark.")
    group.add_argument("--backend_cpu", action="store_true", help="Validate Backend CPU Benchmark.")
    group.add_argument("--frontend_cpu", action="store_true", help="Run Frontend CPU Benchmark.")
    group.add_argument("--frontend_mem", action="store_true", help="Run Frontend MEM Benchmark.")
    parser.add_argument("--executable", help="Path to executable required to run Frontend CPU or MEM Benchmark.")
    args = parser.parse_args()
    if args.frontend_cpu or args.frontend_mem:
        assert (
            args.executable is not None
        ), "'--executable=<path/to/console/executable>' is required to run the Frontend CPU or MEM Benchmark."
        if args.frontend_cpu:
            run_frontend_cpu_benchmark(args.executable)
        else:
            run_frontend_mem_benchmark(args.executable)
    if args.disk_usage:
        run_disk_usage_benchmark()
    if args.backend_cpu:
        run_backend_cpu_validate_benchmarks()
