#!/usr/bin/env bash
set -ex

cp -r ./standalone-py/python/install py39
./py39/bin/python3 ./get-pip.py
./py39/bin/python3 -m pip install flit
./py39/bin/python3 -m pip install .
./py39/bin/python3 -m pip install ".[test]"

