import os.path

import plistlib


def create_main_plist(app_path):
    data = {
        "CFBundleIconFile": "SwiftNavConsole.icns",
    }
    plist_path = os.path.join(app_path, "Contents", "Info.plist")
    with open(plist_path, "wb") as fp:
        plistlib.dump(data, fp)


create_main_plist("main.app")
