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

import sys

from setuptools import setup  # type: ignore
from setuptools_rust import RustExtension  # type: ignore


def get_py_version_cfgs():
    # For now each Cfg Py_3_X flag is interpreted as "at least 3.X"
    version = sys.version_info[0:2]
    py3_min = 9
    out_cfg = []
    for minor in range(py3_min, version[1] + 1):
        out_cfg.append(f"--cfg=Py_3_{minor}")
    return out_cfg


def make_rust_extension(module_name):
    return RustExtension(module_name, "Cargo.toml", rustc_flags=get_py_version_cfgs())


setup(
    name="console-backend",
    version="0.1.0",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["console_backend"],
    rust_extensions=[
        make_rust_extension("console_backend.server"),
    ],
    include_package_data=True,
    zip_safe=False,
)
