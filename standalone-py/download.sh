#!/usr/bin/env bash
set -ex

base_url="https://github.com/indygreg/python-build-standalone/releases/download/20210724"

case "$(uname -s)" in
Darwin)
    cpython_release="$base_url/cpython-3.9.6-x86_64-apple-darwin-lto-20210724T1424.tar.zst"
    ;;
Linux)
    cpython_release="$base_url/cpython-3.9.6-x86_64-unknown-linux-gnu-lto-20210724T1424.tar.zst"
    ;;
*)
    echo "unknown os"
    exit 1
    ;;
esac

wget $cpython_release -O py39.tar.zst

zstd -f -d py39.tar.zst -o py39.tar
tar -xvf py39.tar
