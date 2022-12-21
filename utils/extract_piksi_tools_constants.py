# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
