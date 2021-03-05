import argparse
import json
import subprocess
import sys
from typing import Any, Optional

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

INSTALLER_BENCHMARKS = {
    WINDOWS: [
        {NAME: "Windows Installer", FILE_PATH: "release-archive.filename", EXPECTED: 55, ERROR_MARGIN_FRAC: 0.05,},
    ],
    MACOS: [{NAME: "macOS Installer", FILE_PATH: "release-archive.filename", EXPECTED: 95, ERROR_MARGIN_FRAC: 0.05,},],
    LINUX: [{NAME: "Linux Installer", FILE_PATH: "release-archive.filename", EXPECTED: 85, ERROR_MARGIN_FRAC: 0.05,},],
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

# Frontend Benchmarks.
DEFAULT_JSON_FILEPATH = "fileout.json"
BENCHMARK_COMMAND_ARGS = lambda file_path: f"--file-in={file_path} --connect"
HYPERFINE_COMMAND = lambda file_out: f"hyperfine --warmup 1 --runs 5 --show-output --export-json {file_out} "

FRONTEND_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    MACOS: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    LINUX: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
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
    """Runner for disk usage benchmark validations.
    """
    os_ = sys.platform
    benchmarks = INSTALLER_BENCHMARKS.get(os_, [])
    for bench in benchmarks:
        release_file = ""
        with open(bench[FILE_PATH]) as archive_file:
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
        assert disk_usage - bench[EXPECTED] <= bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (  # type: ignore
            f"Test:{bench[NAME]} Bench Value:{disk_usage} not within "
            f"{bench[ERROR_MARGIN_FRAC]} of {bench[EXPECTED]}."
        )
        print(f"PASS - {os_}:{bench[NAME]} MARGIN={disk_usage - bench[EXPECTED]}")


def run_backend_cpu_validate_benchmarks():
    """Runner for a suite of cpu benchmark validations.
    """
    os_ = sys.platform
    benchmarks = BACKEND_CPU_BENCHMARKS.get(os_, [])
    for bench in benchmarks:
        with open(bench[FILE_PATH]) as fileo:
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
    benchmarks = FRONTEND_BENCHMARKS.get(sys.platform, [])
    for bench in benchmarks:
        bench_command = (
            f'{HYPERFINE_COMMAND(DEFAULT_JSON_FILEPATH)} "{prepped_command} '
            f'{BENCHMARK_COMMAND_ARGS(bench[FILE_PATH])}"'
        )
        subprocess.call(bench_command, shell=True)
        with open(DEFAULT_JSON_FILEPATH) as fileo:
            bench_result = json.load(fileo)
            bench_value = bench_result[RESULTS][0].get(bench[KEY_LOCATION], None)
            assert bench_value is not None, f"Test:{bench[NAME]} retrieved bench value None."
            assert bench_value - bench[EXPECTED] <= bench[ERROR_MARGIN_FRAC] * bench[EXPECTED], (  # type: ignore
                f"Test:{bench[NAME]} Bench Value:{bench_value} not within "
                f"{bench[ERROR_MARGIN_FRAC]} of {bench[EXPECTED]}."
            )
            print(f"PASS - {os_}:{bench[NAME]} MARGIN={bench_value - bench[EXPECTED]}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--disk_usage", action="store_true", help="Run Installer Disk Usage Benchmark.")
    group.add_argument("--backend_cpu", action="store_true", help="Validate Backend CPU Benchmark.")
    group.add_argument("--frontend_cpu", action="store_true", help="Run Frontend CPU Benchmark.")
    parser.add_argument("--executable", help="Path to executable required to run Frontend CPU Benchmark.")
    args = parser.parse_args()
    if args.frontend_cpu:
        assert (
            args.executable is not None
        ), "'--executable=<path/to/console/executable>' is required to run the Frontend CPU Benchmark."
        run_frontend_cpu_benchmark(args.executable)
    if args.disk_usage:
        run_disk_usage_benchmark()
    if args.backend_cpu:
        run_backend_cpu_validate_benchmarks()
