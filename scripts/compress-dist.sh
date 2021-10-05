#!/usr/bin/env bash
set -ex

tar -C ./py39-dist -cvf ./py39-dist.tar *
rm -f ./py39-dist.tar.xz
xz -e -9 ./py39-dist.tar
