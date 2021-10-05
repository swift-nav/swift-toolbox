#!/usr/bin/env bash
set -ex

tar -cvf ./py39-dist.tar ./py39-dist
rm -f ./py39-dist.tar.xz
xz -e -9 ./py39-dist.tar
