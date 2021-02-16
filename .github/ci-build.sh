#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

archive_name="console_pp"

if [[ "$OS_NAME" == "macOS" ]]; then
    tar -czf ${archive_name}_osx.tar.gz 'Swift Navigation Console.dmg';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_osx.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz" >release-archive.filename;
    ls -l;
fi

if [[ "$OS_NAME" == "Linux" ]]; then
    tar -C "target" -czf ${archive_name}_linux.tar.gz 'Swift Navigation Console.deb';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_linux.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz" >release-archive.filename;
    ls -l;
fi

if [[ "$OS_NAME" == "Windows" ]]; then
    VERSION="$(git describe --always --tags)";
    BUILD_TRIPLET="x86_64-pc-windows-msvc";
    mv 'target/Swift Navigation ConsoleSetup.exe' "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}.exe";
    echo "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}.exe" >release-archive.filename;
    ls -l;
fi
