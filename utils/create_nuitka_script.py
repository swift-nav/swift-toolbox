import os
import sys

CONDA_DIR = os.environ["CONDA_PREFIX"]
SITEPACK_DIR = "lib/python3.8/site-packages/"
CONDA_PREFIX = "$CONDA_PREFIX"
NUITKA_DIR = "src/main/nuitka"
FILENAME_POSTFIX = ".sh"

BLACKLIST = ["translations", "QtQuick3D", "Web", ".png"]
SYS_PLATFORM_PREFIXS = {"win32": "windows", "linux": "linux", "darwin": "mac"}

DEFAULT_NUITKA_COMMAND = (
    "python -m nuitka ./src/main/python/main.py "
    "--include-data-file src/main/resources/base/console_backend.capnp=console_backend.capnp "
    "--follow-imports --standalone --plugin-enable=qt-plugins "
)


def create_nuitka_script():
    out = ""
    for root, _, files in os.walk(f"{CONDA_DIR}/{SITEPACK_DIR}PySide2/Qt", topdown=False):
        for name in files:
            path = os.path.relpath(os.path.join(root, name), f"{CONDA_DIR}/{SITEPACK_DIR}")
            if any([x in path for x in BLACKLIST]):
                continue
            out += f"--include-data-file={CONDA_PREFIX}/{SITEPACK_DIR}{path}={path} "

    filename_prefix = SYS_PLATFORM_PREFIXS.get(sys.platform, "unknown")
    with open(os.path.join(NUITKA_DIR, filename_prefix + FILENAME_POSTFIX), "w") as filo:
        filo.write(DEFAULT_NUITKA_COMMAND + out[: -len(" ")])
        filo.write("\n")


if __name__ == "__main__":
    create_nuitka_script()
