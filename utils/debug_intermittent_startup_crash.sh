#!/bin/bash

if [ ! -d py39-dist ]; then
    cargo make build-dist
fi
cd py39-dist

set SWIFT_CONSOLE_ARGS="--log-console --file ../console_backend/tests/data/ins_updates.sbp"

trap "echo Ctrl-C pressed, exiting loop. ; exit" SIGINT SIGTERM

for i in {1..1000}; do
    if [ $(uname) == "Darwin" ]; then
        rust-lldb -o run -o quit -- ./swift-console $SWIFT_CONSOLE_ARGS
    elif [ $(uname -o) == "Msys" ]; then
        # With Visual Studio installed, if there is a crash, a dialog will pop
        # up prompting if you want to debug.
        ./swift-console $SWIFT_CONSOLE_ARGS
    else
        rust-gdb -iex='set pagination off' -ex='set confirm on' -ex run -ex=quit --args swift-console $SWIFT_CONSOLE_ARGS
    fi
    echo ${i}
done
