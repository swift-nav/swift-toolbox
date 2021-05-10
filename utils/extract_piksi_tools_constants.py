import requests


def main():
    r = requests.get("https://raw.githubusercontent.com/swift-nav/piksi_tools/master/piksi_tools/console/utils.py")
    lines = r.text.splitlines()

    emit = False
    resume = "CODE_GPS_L1CA"
    resume_after = None
    for line in lines:
        if line.startswith("def ") and line.endswith("(code):"):
            # help the py2many type inference system
            # https://github.com/adsharma/py2many/issues/246
            line = line.replace("(code):", "(code: int):")

        if resume and line.strip().startswith(resume):
            emit = True
            resume_after = resume = None
        elif resume_after and line.startswith(resume_after):
            emit = True
            resume_after = resume = None
            continue
        elif line.startswith("color_dict"):
            # Depends on https://github.com/adsharma/py2many/issues/171
            emit = False
            resume = "def code_to_str"
        elif line.startswith("def get_mode"):
            # get_mode uses SBP codes from an import, and the `msg` class.
            # The functions which follow `get_mode` utilise `datetime` or
            # use str.format https://github.com/adsharma/py2many/issues/73
            emit = False

        if emit:
            print(line.rstrip())


if __name__ == "__main__":
    main()
