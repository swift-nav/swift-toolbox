#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

DIST=$(find . -maxdepth 1 -iname "*.tar.xz")
echo ${DIST} >release-archive.filename

if [[ "$OS_NAME" == "macOS" ]]; then
    mkdir "main.dist"
    cp -r console_backend/tests/data "main.dist"
    cp utils/bench_runner.py "main.dist"
    cd "main.dist"
    7z a -tzip "../$OS_NAME.zip" *
    cd ..
    echo "${OS_NAME}.zip" >bench.filename
else
    cp -r console_backend/tests/data "target/swift_navigation_console"
    cp utils/bench_runner.py "target/swift_navigation_console"
    cd "target/swift_navigation_console"
    7z a -tzip "../../$OS_NAME.zip" *
    cd ../..
    echo "${OS_NAME}.zip" >bench.filename
fi
