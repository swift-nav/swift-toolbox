import os
CONDA_DIR = "/home/jm/miniconda3/envs/console_pp/"
SITEPACK_DIR = "lib/python3.9/site-packages/"
CONDA_PREFIX = "$CONDA_PREFIX/"


BLACKLIST = ["translations", "QtQuick3D", "QtWebEngine", "resources", ".png", ".qml", "WebEngine"]
for root, dirs, files in os.walk(f"{CONDA_DIR}{SITEPACK_DIR}PySide2/Qt", topdown=False):
    for name in files:
        path = os.path.relpath(os.path.join(root, name), f"{CONDA_DIR}{SITEPACK_DIR}")
        if any([x in path for x in BLACKLIST]):
            continue
        print(f"--include-data-file={CONDA_PREFIX}{SITEPACK_DIR}{path}={path} \\")

