#!/usr/bin/env bash
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


set -e # Terminate with failure if any command returns nonzero
set -u # Terminate with failure any time an undefined variable is expanded

SCRIPT_DIR="$(cd -P "$(dirname "$BASH_SOURCE")" >/dev/null 2>&1 && pwd)"

pyproject_toml="${SCRIPT_DIR}/../pyproject.toml"
if [[ ! -f "$pyproject_toml" ]]; then
    echo >&2 "Couldn't find pyproject.toml."
    echo >&2 "Unable to determine PySide6 version. aborting."
    exit 0
fi

set +e
awk --version >/dev/null 2>&1
if [[ $? -ne 0 ]]; then
    echo >&2 "Couldn't find awk. Aborting."
    exit 1
fi
set -e

pysideversion=$(awk '/PySide6/{print(gensub(".*(PySide6 *== *([0-9]\\.[0-9]{1,2}\\.[0-9]{1,3})).*","\\2", "1")) }' "$pyproject_toml")

if [[ ! "$pysideversion" =~ [0-9]\.[0-9]{1,2}\.[0-9]{1,3} ]]; then
    echo >&2 "PySide version found is not a version number: $pysideversion"
    exit 1
fi

qt_archdir=""
case $OSTYPE in
    "linux-gnu"*)
        qt_archdir="gcc_64"
        ;;
    "darwin"*)
        qt_archdir="macos"
        ;;
    *)
        qt_archdir="msvc2019_64"
        ;;
esac

# Standard Qt install location
QTDIR="${HOME}/Qt/${pysideversion}/${qt_archdir}"

kept_args=()
for arg in "$@"; do
  case $arg in
    --qtdir=*)
        QTDIR="${arg#*=}"
        shift
        ;;
    -h|--help)
        echo >&2 "$(basename "$BASH_SOURCE") [--qtdir=<Path to Qt base dir>] [-h|--help]"
        echo >&2
        echo >&2 "    --qtdir=<Path to Qt base dir>"
        echo >&2 "        Use specified Qt dir to link Qt libraries into PySide6 installation."
        echo >&2 "    -h, --help"
        echo >&2 "        Show this help."
        exit 0
        ;;
    -*|--*)
        echo >&2 "Unknown option $arg."
        exit 1
        ;;
    *)
        kept_args+=($arg)
        ;;
  esac
done
set -- "${kept_args[@]}"


# Check if Qt is installed
if [[ ! -e "${QTDIR}/lib/libQt6Core.so.6" ]]; then
    echo >&2 "Could not find ${QTDIR}/lib/libQt6Core.so.6. aborting."
    exit 1
fi

# Check if PySide6 is installed
pyside6_qt_lib_dir=$(realpath "${SCRIPT_DIR}/../311/lib/python3.11/site-packages/PySide6/Qt/lib")
if [[ ! -e "$pyside6_qt_lib_dir" ]]; then
    echo >&2 "Could not find $pyside6_qt_lib_dir. aborting."
    exit 1
fi

cd "${QTDIR}/lib"
qtdir_libs=$(ls -1 libQt6*.so.6 | sort)
cd "${SCRIPT_DIR}/../311/lib/python3.11/site-packages/PySide6/Qt/lib"

bkup_dir_suffix=""
while [[ -e "original_libs${bkup_dir_suffix}" ]]; do
    if [[ -z "$bkup_dir_suffix" ]]; then
        bkup_dir_suffix=1
    else
        bkup_dir_suffix=$(($bkup_dir_suffix+1))
    fi
done
orig_libs_dir="original_libs${bkup_dir_suffix}"
mkdir "$orig_libs_dir"
mv $qtdir_libs "$orig_libs_dir"

for lib in $qtdir_libs ; do
    ln -s "${QTDIR}/lib/$lib" $lib
done

echo "Done."
echo "Qt libraries from
    $QTDIR
have been linked to
    $pyside6_qt_lib_dir"
