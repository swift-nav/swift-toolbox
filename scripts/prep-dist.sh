#!/usr/bin/env bash
set -ex

rm -rf ./py39-dist
cp -r ./standalone-py/python/install ./py39-dist
./py39-dist/bin/python3 -m pip install .
