#!/bin/bash

set -e
set -x
set -o errexit
set -o pipefail

archive_name="console_pp"

if [[ "$OS_NAME" == "osx" ]]; then
    (strip 'Swift Navigation Console.dmg');
    tar -czf ${archive_name}_osx.tar.gz 'Swift Navigation Console.dmg';
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
