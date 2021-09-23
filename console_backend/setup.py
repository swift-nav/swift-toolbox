import sys
import platform

from setuptools import setup  # type: ignore
from setuptools_rust import RustExtension  # type: ignore


def get_py_version_cfgs():
    # For now each Cfg Py_3_X flag is interpreted as "at least 3.X"
    version = sys.version_info[0:2]
    py3_min = 6
    out_cfg = []
    for minor in range(py3_min, version[1] + 1):
        out_cfg.append(f"--cfg=Py_3_{minor}")

    if platform.python_implementation() == "PyPy":
        out_cfg.append("--cfg=PyPy")

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
