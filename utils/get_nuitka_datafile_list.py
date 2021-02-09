import os

CONDA_DIR = os.environ["CONDA_PREFIX"]
SITEPACK_DIR = "lib/python3.9/site-packages/"
CONDA_PREFIX = "$CONDA_PREFIX"


BLACKLIST = ["translations", "QtQuick3D", "resources", ".png", ".qml", "WebEngine"]
out = ""
for root, dirs, files in os.walk(f"{CONDA_DIR}/{SITEPACK_DIR}PySide2/Qt", topdown=False):
    for name in files:
        path = os.path.relpath(os.path.join(root, name), f"{CONDA_DIR}/{SITEPACK_DIR}")
        if any([x in path for x in BLACKLIST]):
            continue
        out += f"--include-data-file={CONDA_PREFIX}/{SITEPACK_DIR}{path}={path} "

print(out)
