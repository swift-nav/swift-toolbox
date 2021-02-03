#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

# cargo test --all-features --all-targets --release -vv
# cargo build --all-features --all-targets --release -vv

cargo make prod-installer

archive_name="console_pp"

# if [[ "$OS_NAME" == "windows" ]]; then
#     cd target/release;
#     strip.exe sbp-chopper.exe;
#     7z a -tzip ../../${archive_name}_windows.zip sbp-chopper.exe;
#     cd ../..;
#     VERSION="$(git describe --always --tags)";
#     BUILD_TRIPLET="x86_64-pc-windows-msvc";
#     mv ${archive_name}_windows.zip "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}.zip";
#     echo "${archive_name}-${VERSION}-windows-${BUILD_TRIPLET}.zip" >release-archive.filename;
#     ls -l;
# fi

if [[ "$OS_NAME" == "osx" ]]; then
    (cd target; strip 'Swift Navigation Console.dmg');
    tar -C "target" -czf ${archive_name}_osx.tar.gz 'Swift Navigation Console.dmg';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_osx.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz" >release-archive.filename;
    ls -l;
fi

if [[ "$OS_NAME" == "linux" ]]; then
    (cd target; strip 'Swift Navigation Console.deb');
    tar -C "target" -czf ${archive_name}_linux.tar.gz 'Swift Navigation Console.deb';
    VERSION="$(git describe --always --tags --dirty)";
    BUILD_TRIPLET="$(gcc -dumpmachine)";
    mv ${archive_name}_linux.tar.gz "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz";
    echo "${archive_name}-${VERSION}-${BUILD_TRIPLET}.tar.gz" >release-archive.filename;
    ls -l;
fi
