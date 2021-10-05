#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

ARCHIVE_NAME="console_pp"
DATE="$(date '+%Y-%m-%d')"
PYTHON_DIST = "py39-dist.tar.xz"

if [[ "$OS_NAME" == "macOS" ]]; then
    VERSION="$(git describe --always --tags --dirty)"
    BUILD_TRIPLET="$(gcc -dumpmachine)"
    OUTPUT_NAME = "${ARCHIVE_NAME}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz"

    mv ${PYTHON_DIST} ${OUTPUT_NAME}
    echo ${OUTPUT_NAME} >release-archive.filename

    mkdir "main.dist"
    cp -r console_backend/tests/data "main.dist"
    cp utils/bench_runner.py "main.dist"
    cd "main.dist"
    7z a -tzip "../$OS_NAME.zip" *
    cd ..
    echo "${OS_NAME}.zip" >bench.filename

    ls -l
fi

if [[ "$OS_NAME" == "Linux" ]]; then
    VERSION="$(git describe --always --tags --dirty)"
    BUILD_TRIPLET="$(gcc -dumpmachine)"
    OUTPUT_NAME = "${ARCHIVE_NAME}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz"

    mv ${PYTHON_DIST} ${OUTPUT_NAME}
    echo ${OUTPUT_NAME} >release-archive.filename

    cp -r console_backend/tests/data "target/swift_navigation_console"
    cp utils/bench_runner.py "target/swift_navigation_console"
    cd "target/swift_navigation_console"
    7z a -tzip "../../$OS_NAME.zip" *
    cd ../..
    echo "${OS_NAME}.zip" >bench.filename

    ls -l
fi

if [[ "$OS_NAME" == "Windows" ]]; then
    VERSION="$(git describe --always --tags)"
    BUILD_TRIPLET="x86_64-pc-windows-msvc"
    OUTPUT_NAME = "${ARCHIVE_NAME}-${VERSION}-windows-${BUILD_TRIPLET}-${DATE}.tar.gz"

    mv ${PYTHON_DIST} ${OUTPUT_NAME}
    echo ${OUTPUT_NAME} >release-archive.filename

    cp -r console_backend/tests/data "target/swift_navigation_console"
    cp utils/bench_runner.py "target/swift_navigation_console"
    cd "target/swift_navigation_console"
    7z a -tzip "../../$OS_NAME.zip" *
    cd ../..
    echo "${OS_NAME}.zip" >bench.filename

    ls -l
fi
