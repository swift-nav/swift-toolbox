import argparse
import json
import subprocess
import os
import sys

HYPERFINE_COMMAND = lambda file_out: f"hyperfine --warmup 1 --runs 5 --show-output --export-json {file_out} "

WINDOWS = "win32"
MACOS = "darwin"
LINUX = "linux"
RESULTS = "results"
NAME = "name"
FILE_PATH = "file_path"
KEY_LOCATION = "key_location"
EXPECTED = "expected"
ERROR_MARGIN_FRAC = "error_margin_frac"

BENCHMARK_COMMAND_ARGS = lambda file_path: f"--file-in={file_path} -connect"

RUST_BENCHMARKS = {
    WINDOWS: [
        {
            NAME: "2020-09-04-BayLoop/M8L_BDS_ADR431_nativeODO_Swiftlets_OSR_prod_10Hz_AmotechL2/04-155800",
            FILE_PATH: "benches/data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    MACOS: [
        {
            NAME: "2020-09-04-BayLoop/M8L_BDS_ADR431_nativeODO_Swiftlets_OSR_prod_10Hz_AmotechL2/04-155800",
            FILE_PATH: "benches/data/202010224_192043.sbp",
            KEY_LOCATION: "mean",
            EXPECTED: 1.75,
            ERROR_MARGIN_FRAC: 0.05,
        },
    ],
    LINUX: [
        {
            NAME: "2020-09-04-BayLoop/M8L_BDS_ADR431_nativeODO_Swiftlets_OSR_prod_10Hz_AmotechL2/04-155800",
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
    for os_, benchmarks in RUST_BENCHMARKS.items():
        if os_ != sys.platform:
            continue
        for bench in benchmarks:
            bench_command = (
                f"{HYPERFINE_COMMAND('fileout.json')} \"{prepped_command} {BENCHMARK_COMMAND_ARGS(bench[FILE_PATH])}\""
            )
            print(bench_command)
            subprocess.call(bench_command, shell=True)
            with open("fileout.json") as fileo:
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
