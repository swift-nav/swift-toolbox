import json
import sys
from typing import Any, Optional

WINDOWS = "win32"
MACOS = "darwin"
LINUX = "linux"
NAME = "name"
FILE_PATH = "file_path"
KEY_LOCATION = "key_location"
EXPECTED = "expected"
ERROR_MARGIN_FRAC = "error_margin_frac"
SUCCESS = "success"

RUST_BENCHMARKS = {
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


def run_validate_benchmarks():
    """Runner for a suite of benchmark validations.
    """
    for os_, benchmarks in RUST_BENCHMARKS.items():
        if os_ != sys.platform:
            continue
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


if __name__ == "__main__":
    run_validate_benchmarks()
