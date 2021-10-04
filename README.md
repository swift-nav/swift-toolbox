# Console++ Prototype

## Setup

Install Rust: https://rustup.rs/ 

Install *cargo-make*: `cargo install --force cargo-make`.

Download and install miniconda3:
- https://docs.conda.io/en/latest/miniconda.html

Create Python new environment:

```
conda env create -f conda.yml
```

Activate the environment and install poetry:

```
conda activate console_pp
pip install poetry
```

Install cmake, clang, and capnp in your respective OS.

```
# Windows - install with installer, or via chocolatey
choco install cmake llvm capnproto

# Mac
xcode-select install
brew install cmake capnp

# Linux
apt-get install cmake libclang-dev capnproto
```

Install development dependencies (On Windows make sure you're using Adminstrator shell).

# For the below commands, we do not want to be in the console_pp conda environment.
# Bad things will happen if you are.

```
conda deactivate
cargo make pip-install-dev
git lfs pull
```

# Building for macos

The module used for generating rust bindings for native libraries; `rust-bindgen` 
has been observed to fail to find system headers (i.e. `assert.h`, `math.h`) on 
newer versions of macos. Fortunately we can add include search paths to pass to 
clang by setting an environment variable:

```
export BINDGEN_EXTRA_CLANG_ARGS=-I$(xcrun --show-sdk-path)/usr/include
```

## Running

To run the app (in dev mode):

```
cargo make run
```

Or in "prod" mode (compiles a wheel for the backend):

```
cargo make prod-run
```

## Building the installer

To build the installer:

```
cargo make prod-installer
```

## Running the benchmarks

Prerequisites:

- Windows
  - NSIS - Nullsoft Scriptable Install System
  - NSIS ShellExecAsUser plugin
- All
  - `cargo install hyperfine`

To run the frontend benchmarks:

```
git lfs pull
cargo make prod-installer
cargo make frontend-cpu-bench
```

## Contributing

After making changes, run to tasks to ensure the code is ready for submission

```
cargo make check-all
cargo make tests
```

## Technologies

### Rust

Rust is used for the "backend" logic of the application. The library [pyo3][]
(and companion library [setuptools-rust][]) are used to implement a native
Python extension.

[pyo3]: https://docs.rs/pyo3/0.13.1/pyo3/
[setuptools-rust]: https://github.com/PyO3/setuptools-rust

### PyInstaller (fbs)

PyInstaller (via [fbs]) is used to bundle the application and create an installer.
Things that fbs trivially solves:

- creating an installer: *fbs* uses [fpm] to create installers for unix/mac, it uses
  makensis to create a windows installer.
  - The Windows installer looks to be "decent", it doesn't seem to do a good job cleaning up old installations though: https://github.com/mherrmann/fbs/pull/29
  - It would be ideal if it could use WiX instead of NSIS to generate an MSI: https://github.com/mherrmann/fbs/issues/8

- managing resources: the fbs runtime handles packaging of application resources, particularly things like .capnp files and QML files

[fbs]: https://build-system.fman.io/
[fpm]: https://github.com/jordansissel/fpm

### PySide2

We're using Qt 5.15.2 via PySide2 (the official Python bindings for Qt).  Made possible via fork of fbs: https://github.com/silverjam/fbs

### QML

QML (QtQuick Mark-up Language) is used to model the UI.

### Python 3.9 (via Miniconda)

Miniconda builds of Python provide a consistent (and reliable) build of Python across different platforms.  We use Python 3.9 (which requires PyInstaller 4.2).

### Poetry

Poetry is used to manage our Python environment: https://python-poetry.org/

## Design Philosophy

### Installers, resources and packaging

One of the things that makes the current console difficult to maintain is the
number of Python dependencies that are required to implement the console. In
particular things like the TraitsUI library bind us to particular versions of
PyQt that work with traits. Additionally, if we want to use new libraries,
PyInstaller's "hooks" need to be up-to-date enough to work with these
libraries: https://github.com/pyinstaller/pyinstaller/tree/develop/PyInstaller/hooks.

PyInstaller (via fbs) ends up being a great way to package a massive
framework like Qt.  Otherwise we would have to rely on Qt being present
on Unix like systems, or using tools like [macdeployqt][] and [windeployqt][].

[macdeployqt]: https://doc.qt.io/qt-5/macos-deployment.html#the-bundle
[windeployqt]: https://doc.qt.io/qt-5/windows-deployment.html

To this end, it makes sense to minimize the number of dependencies that we
use for Python in order to avoid this problem.  This ends up "dovetailing"
well with the usage of Rust as the UI backend.

Resource management is another concern, for non-code assets like pictures,
protocol definitions (`.capnp` files) and UI mark-up files (`.qml`) we need a
system to bundle these -- the `fbs` tool currently handles this, though other
tools like [qrc][] and Rust's [`include_bytes!`][rust_include] could
potentially be used for this too.

[qrc]: https://doc.qt.io/qt-5/resources.html
[rust_include]: https://doc.rust-lang.org/std/macro.include_bytes.html

To this end the prototype attempts to impose these constraints:

+ Minimal dependencies in Python: only Qt (PySide2) and Capnproto (pycapnp)
+ All other necessary external libraries should be include via Rust libraries

[ui-javascript]: https://github.com/swift-nav/console_pp/blob/main/src/main/resources/base/view.qml#L57

### QML - a path to mobile

The QtQuick Mark-up Language (QML) is used to code the UI.  QML provides an
abstraction layer for describing the UI that doesn't require the UI to be hand
coded in any particular language.  While Qt already has support for this
through their UIC files, QML also provides support for lightweight logic to be
directly embedded in the QML file via JavaScript - this helps with the
portability of the QML since UI logic doesn't need to be encoded in the
language hostign the QML.

To this end the prototype attempts to imposes these constraints:

+ No UI logic in Python if possible, UI logic should be encoded in small bits
  of JavaScript ([example][ui-javascript])
+ Python code should focus on "data binding" and "message passing" only
  - For "data binding", this means Python code should be what's minimally
    necessary for Qt to fetch display data
  - For "message passing", to implement features, Python code will need to
    be written to pass data to the backend, and to marshal data from the
    backend into the data binding objects.

#### pyqtdeploy

The *pyqtdeploy* project: https://pypi.org/project/pyqtdeploy/ -- from the
same people that maintain PyQt5 (not Qt/Nokia) is designed to allow PyQt5
applications to deploy to desktop and mobile environments, however the
project page emphasizes that the project was designed specifically around
allowing the PyQt5 apps to be deployed to iOS and Android.

#### C++ "shell"

It's possible that we can build a C++ shell that hosts the QML, and
re-implements the "data binding" and "message passing" code if deploying with
something like *pyqtdeploy* is not sucessful, or proves to be prohibitive.

### Rust backend

The rust backend gives us a statically type checked, modern programming
language with (probably) enough library support to support our development
activities.  The "big" libraries that the current console uses are
*numpy* and *pyserial*.

The equivalents for Rust are:

+ serialport: https://docs.rs/serialport/4.0.0/serialport/
+ ndarray: https://docs.rs/ndarray/0.14.0/ndarray/

Using Rust should give us a head start on speed and resource usage issues.
However, we don't want to be bound to tightly with any particular FFI system.
To that end, we integrate into Python with [pyo3][] but we try to minimize
the dependence here by driving most of the interaction with the back-end via
message passing. This gives the backend greater portability should we need to
move to different "shell" (C++) for Qt/QML or a different UI system
altogether (e.g. we could build native UIs for Android/iOS).

To this end the prototype attempts to imposes these constraints:

+ Most interaction with the backend should be via message passing
+ FFI with the host language (Python) should focus on start/stopping the
  backend and exchanging lightweight IPC messages (capnproto)

### Message passing

We use [capnproto][] for message passing since it does not require parsing,
the in memory representation is accessed directly by the library, without an
unpacking/parsing step-- this makes it suitable for "high speed" scenarios like IPC
(where it's preferable to not pay the compute cost of parsing just to cross
language barriers). Other formats like protobufs require a "parse" phase,
capnproto also has very ergonomic Python bindings. Other formats such as
[flatbuffers][] achieve similar goals as *capnproto* but do not have good
Python support.

[capnproto]: https://capnproto.org/
[flatbuffers]: https://google.github.io/flatbuffers/
