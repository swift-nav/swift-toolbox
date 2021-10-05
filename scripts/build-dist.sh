#!/usr/bin/env bash
set -ex

cd ./console_backend
../py39/bin/python3 setup.py bdist_wheel
cd ..

./py39/bin/python3 -m flit build

ls console_backend/dist

case "$(uname -s)" in
Darwin)
    wheel="./console_backend/dist/console_backend-0.1.0-cp39-cp39-apple-darwin_x86_64.whl"
    ;;
Linux)
    wheel="./console_backend/dist/console_backend-0.1.0-cp39-cp39-linux_x86_64.whl"
    ;;
*)
    echo "unknown os"
    exit 1
    ;;
esac

./py39-dist/bin/python3 -m pip install $wheel --force-reinstall
./py39-dist/bin/python3 -m pip install ./dist/swiftnav_console-0.1.0-py3-none-any.whl --force-reinstall

if [ ! -d "./py39-dist/lib/site-packages/resources" ]; then
    rsync -R -r src/main/resources py39-dist/lib/python3.9/site-packages
fi
