import os
import sys

FBS_DIR = "src/main/fbs"
FILENAME_POSTFIX = "-remove-negligibles.sh"
INSTALLER_DIR = "target/swift_navigation_console"

BLACKLIST = ["translations", "QtQuick3D", "Web"]
SYS_PLATFORM_PREFIXS = {"win32": "windows", "linux": "linux"}


def create_remove_negligibles_file():
    """Create a platform specific bash script to remove undesirables before fbs
    installer command operation.
    """
    out = ""
    for root, _, files in os.walk(f"{INSTALLER_DIR}", topdown=False):
        for name in files:
            path = os.path.relpath(os.path.join(root, name), f"{INSTALLER_DIR}")
            if not [x in path for x in BLACKLIST]:
                continue
            out += f'rm -rf "{INSTALLER_DIR}/{path}" && '
    filename_prefix = SYS_PLATFORM_PREFIXS.get(sys.platform, "unknown")
    with open(os.path.join(FBS_DIR, filename_prefix + FILENAME_POSTFIX), "w", encoding="utf-8") as filo:
        filo.write(out[: -len("&& ")])
        filo.write("\n")


if __name__ == "__main__":
    create_remove_negligibles_file()
