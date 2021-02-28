import argparse
import json
import subprocess
import os
import sys

WINDOWS = "win32"
MACOS = "darwin"
LINUX = "linux"
RESULTS = "results"
NAME = "name"
FILE_PATH = "file_path"
KEY_LOCATION = "key_location"
EXPECTED = "expected"
ERROR_MARGIN_FRAC = "error_margin_frac"

DEFAULT_JSON_FILEPATH = "fileout.json"
BENCHMARK_COMMAND_ARGS = lambda file_path: f"--file-in={file_path} --connect"
HYPERFINE_COMMAND = lambda file_out: f"hyperfine --warmup 1 --runs 5 --show-output --export-json {file_out} "

FRONTEND_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "benches/data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    MACOS: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "benches/data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    LINUX: [
        {
            NAME: "202010224_192043",
            FILE_PATH: "benches/data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
}


def run_frontend_benchmark(binary: str):
    """Runner for a suite of benchmark validations.
    Args:
        binary (str): Path to the binary location to run the benchmark on.
    """
    prepped_command = f"{binary}"
    for os_, benchmarks in FRONTEND_BENCHMARKS.items():
        if os_ != sys.platform:
            continue
        for bench in benchmarks:
            bench_command = (
                f"{HYPERFINE_COMMAND(DEFAULT_JSON_FILEPATH)} \"{prepped_command} "
                f"{BENCHMARK_COMMAND_ARGS(bench[FILE_PATH])}\""
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
    parser.add_argument("--binary", required=True, help="The path to the Swift Console binary.")

    args = parser.parse_args()

    assert os.path.isfile(args.binary), f"The path provided {args.binary} is not a file."
    run_frontend_benchmark(args.binary)
