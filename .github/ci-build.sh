#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

archive_name="console_pp"
DATE="$(date '+%Y-%m-%d')";

if [[ "$OS_NAME" == "macOS" ]]; then
    tar -czf ${archive_name}_osx.tar.gz 'swift_navigation_console.dmg';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_osx.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz" >release-archive.filename;

    cp -r console_backend/tests/data 'main.dist' && cp utils/bench_runner.py 'main.dist';
    cd 'main.dist';
    7z a -tzip "../$OS_NAME.zip" *;
    cd ..;
    echo "${OS_NAME}.zip" >bench.filename;
    ls -l;
fi

if [[ "$OS_NAME" == "Linux" ]]; then
    tar -C "target" -czf ${archive_name}_linux.tar.gz 'swift_navigation_console.deb';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_linux.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}-${DATE}.tar.gz" >release-archive.filename;

    cp -r console_backend/tests/data 'target/swift_navigation_console' && cp utils/bench_runner.py 'target/swift_navigation_console';
    cd 'target/swift_navigation_console';
    7z a -tzip "../../$OS_NAME.zip" *;
    cd ../..;
    echo "${OS_NAME}.zip" >bench.filename;

    ls -l;
fi

if [[ "$OS_NAME" == "Windows" ]]; then
    VERSION="$(git describe --always --tags)";
    BUILD_TRIPLET="x86_64-pc-windows-msvc";
    ls target
    mv 'target/swift_navigation_consoleSetup.exe' "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}-${DATE}.exe";
    echo "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}-${DATE}.exe" >release-archive.filename;

    cp -r console_backend/tests/data 'target/swift_navigation_console' && cp utils/bench_runner.py 'target/swift_navigation_console';
    cd 'target/swift_navigation_console';
    ls -l;
    7z a -tzip "../../$OS_NAME.zip" *;
    cd ../..;
    echo "${OS_NAME}.zip" >bench.filename;
    ls -l;
fi
