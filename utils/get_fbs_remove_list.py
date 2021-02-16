import os

INSTALLER_DIR = "target/Swift Navigation Console"


BLACKLIST = ["translations", "QtQuick3D", "resources", ".png", ".qml", "Web"]
out = ""
for root, dirs, files in os.walk(f"{INSTALLER_DIR}", topdown=False):
    for name in files:
        path = os.path.relpath(os.path.join(root, name), f"{INSTALLER_DIR}")
        if not any([x in path for x in BLACKLIST]):
            continue
        out += f'rm -rf "{INSTALLER_DIR}/{path}" && '
with open("./linux.sh", "w") as filo:
    filo.write(out[: -len("&& ")])
    filo.write("\n")
