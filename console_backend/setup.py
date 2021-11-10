import sys

from setuptools import setup  # type: ignore
from setuptools_rust import RustExtension  # type: ignore


def get_py_version_cfgs():
    return ["--cfg=Py_3_10"]


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
