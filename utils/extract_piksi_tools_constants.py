import requests


def main():
    r = requests.get("https://raw.githubusercontent.com/swift-nav/piksi_tools/master/piksi_tools/console/utils.py")
    lines = r.text.splitlines()

    emit = False
    resume = "CODE_GPS_L1CA"
    resume_after = None
    for line in lines:
        if resume and line.startswith(resume):
            emit = True
            resume_after = resume = None
        elif resume_after and line.startswith(resume_after):
            emit = True
            resume_after = resume = None
            continue
        # These three codes have been given improved strings
        # https://snav.slack.com/archives/C02USAD03/p1618522328058800
        elif line.startswith("GAL_E8I_STR = "):
            print('GAL_E8I_STR = "GAL E5ab I"')
            continue
        elif line.startswith("GAL_E8Q_STR = "):
            print('GAL_E8Q_STR = "GAL E5ab Q"')
            continue
        elif line.startswith("GAL_E8X_STR = "):
            print('GAL_E8X_STR = "GAL E5ab I+Q"')
            continue
        elif line.startswith("GUI_CODES = {"):
            emit = False
            resume = "GPS_L1CA_STR"
        elif line.startswith("color_dict"):
            emit = False
            resume = "gps_codes ="
        elif line.startswith("def code_is_"):
            emit = False
            resume_after = "    return code in "
        elif line.startswith("def get_mode"):
            break

        if emit:
            print(line.rstrip())


if __name__ == "__main__":
    main()
