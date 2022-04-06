# Debugging Swift Toolbox 🧰

## Install Qt

To build PySide, you need a copy of the Qt libraries, headers, and tools that
matches the version of PySide2/6 that you are wanting to install.

### Get Qt installer (Commercial Qt License holder)
* Point a browser to https://account.qt.io/ and create or log into your Qt account
* From the `Downloads` link, pick your license. Likely you only have one choice.
  If you have a choice of licenses, pick "Qt for Device Creation" or the one for
  desktop application development.
* For product, choose `Qt Online Installer`
* Leave version untouched.
* Click the download button.

### Get Qt installer (LGPLv3 or trial installation)
* Make sure that you will be in compliance with the terms of the LGPLv3.
* Point a browser to https://qt.io/download
* Click "Download. Try." at the top.
* Fill out the form that comes up. The Qt Company will not sell your personal
  information to third parties. You may be contacted by a sales person, however.
* The next page after form submission will have links to download the installer.
* Click the download button.

### Installing Qt
* On *nix operating systems, make sure you `chmod u+x qt-unified-os-n.n.n-n.run`
  to give the installer ability to be run as an executable.
* Run the installer .exe or .run package.
* Log in if you're a license holder.
* Choose "Add or Remove components" if you encounter that choice.
* On the component selections screen, enable the "Archive" and "LTS" filter
  choices and disable the "Latest releases" and "Preview" filter choices on the
  right hand side and click "Filter". This will take some time.
* Open the Qt->Qt 5.15.2 tree view component branches.
* In this grouping, select the following:
  * The component that matches your compiler and architecture (for example,
    gcc_64 or MSVC 2019 64-bit)
  * Qt Charts
  * Sources
  * Qt Debug Information Files
* Click Next, and review and agree to any licenses.
* Click "Install".
* Qt will then install to the default location. Makefile.toml assumes Qt is
  installed to the default installation, so if you do modify that, review
  the pyside targets in Makefile.toml and update the paths accordingly.

## Get the swift-console ready for building

* Follow README.md


## Build the build-dist target

```
cargo make prep-debug-pyside
cargo make build-dist
```

## Debugging an intermittent crash on startup

For all platforms, just use the `utils/debug_intermittent_startup_crash.sh` script.
