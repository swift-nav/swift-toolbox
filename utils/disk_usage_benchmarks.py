import subprocess
import sys

WINDOWS = "win32"
MACOS = "darwin"
LINUX = "linux"
NAME = "name"
FILE_PATH = "file_path"
EXPECTED = "expected"
ERROR_MARGIN_FRAC = "error_margin_frac"

DISK_USAGE_COMMAND = lambda file_path: f"du -ch {file_path} | grep total"

INSTALLER_BENCHMARKS = {
    WINDOWS: [
        {NAME: "Windows Installer", FILE_PATH: "release-archive.filename", EXPECTED: 55, ERROR_MARGIN_FRAC: 0.05,},
    ],
    MACOS: [{NAME: "macOS Installer", FILE_PATH: "release-archive.filename", EXPECTED: 95, ERROR_MARGIN_FRAC: 0.05,},],
    LINUX: [{NAME: "Linux Installer", FILE_PATH: "release-archive.filename", EXPECTED: 85, ERROR_MARGIN_FRAC: 0.05,},],
}


def run_disk_usage_benchmark():
    """Runner for disk usage benchmark validations.
    """
    for os_, benchmarks in INSTALLER_BENCHMARKS.items():
        if os_ != sys.platform:
            continue
        for bench in benchmarks:
            release_file = ""
            with open(bench[FILE_PATH]) as archive_file:
                release_file = archive_file.readline().rstrip()

            bench_command = f"{DISK_USAGE_COMMAND(release_file)}"
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


if __name__ == "__main__":
    run_disk_usage_benchmark()
