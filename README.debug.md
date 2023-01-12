# Debugging Swift Toolbox 🧰

## Install Qt

To build PySide, you need a copy of the Qt libraries, headers, and tools that
matches the version of PySide2/6 that you are wanting to install. To debug, you
additionally need the Qt sources for the version of Qt you are installing.

### Get Qt installer (Commercial Qt License holder)
* Point a browser to https://account.qt.io/ and create or log into your Qt account.
* From the [Downloads link](https://account.qt.io/s/downloads) on the left,
  pick your license. It is likely that you have only one choice.
  If you have a choice of licenses, pick "Qt for Device Creation" or the one for
  desktop application development.
* For product, choose `Qt Online Installer`
* Leave version untouched.
* Click the download button.

### Get Qt installer (LGPLv3 or trial installation)
* Make sure that you will be in compliance with the terms of the LGPLv3.
* Point a browser to https://qt.io/download
* In the "Try Qt Framework and Tools" section, Click "Download Qt".
* Fill out the form that comes up. The Qt Company will not sell your personal
  information to third parties. You may be contacted by a sales person, however.
* The next page after form submission will have links to download the installer.
* Click the "Download" button.

### Installing Qt
* On *nix operating systems, make sure you `chmod u+x qt-unified-os-n.n.n-n.run`
  to give the installer ability to be run as an executable.
* Run the installer .exe or .run package.
* Log in if you're a license holder.
* Choose "Add or Remove components" if you encounter that choice.
* On the component selections screen, enable the "Archive" and "LTS" filter
  choices and disable the "Latest releases" and "Preview" filter choices on the
  right hand side and click "Filter". This will take some time.
* Open the Qt->Qt 6.x.x tree view component branches.
* In this grouping, select the following:
  * The component that matches your compiler and architecture (for example,
    gcc_64 or MSVC 2019 64-bit)
  * Qt Charts
  * Sources
  * Qt Debug Information Files
* Click Next, and review and agree to any licenses.
* Click "Install".
* Add ~/Qt/6.\<your Qt version>/\<arch>/bin to the path. (C:\Qt on windows)

## Get the swift-console ready for building

* run `cargo make prep-debug-pyside` to clone the PySide6 sources.
* Follow [README.md](README.md).
* The standard targets that README.md directs you to make will detect the cloned
  pyside-setup repository and use that as an indication to build and install
  PySide6 as debug from sources.

## Debug with gdb

```
cargo make gdb-run [<args>]
```

* The toolbox will start running under rust-gdb. Press Ctrl-C to get to a GDB
  command prompt.
* To be able to see Qt sources, you need to add a substitute path so that the
  paths to Qt sources embedded in the debug symbol files can be mapped to the
  Qt source path on your machine. To do this:
* `(gdb) set substitute-path /home/qt/work/qt ~/Qt/<Qt version>/Src`
* `(gdb) set substitute-path /home/qt/work/install/include ~/Qt/<Qt version>/gcc_64/include`
* Now you can set breakpoints in Qt and PySide6 code that should properly break
  when the code is executed.

## Build build-dist target and run without rust

Sometimes you want to eliminate rust being a culprit in a bug. This allows you
to run the console without the rust backend, replaying rust-generated data from
a prior run of the application. Follow these instructions to run without the
rust backend:

```
cargo make prep-debug-pyside
cargo make build-dist
py39-dist/bin/python3 -m swiftnav_console.main --read-capnp-recording console_backend/tests/data/console-capnp-20220419-033358.pickle
```

## Debugging an intermittent crash on startup

For all platforms, just use the [utils/debug_intermittent_startup_crash.sh](utils/debug_intermittent_startup_crash.sh) script.
