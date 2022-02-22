#!/bin/sh

if [ ! -d py39-dist ]; then
    cargo make build-dist
fi
cd py39-dist

trap "echo Ctrl-C pressed, exiting loop. ; exit" SIGINT SIGTERM

for i in {1..1000}; do 
    if [ $(uname) == "Darwin" ]; then
        rust-lldb -o run -o quit -- ./swift-console --log-console --file ../console_backend/tests/data/ins_updates.sbp
    else
        rust-gdb -ex='set confirm on' -ex run -ex=quit --args swift-console --log-console --file ../console_backend/tests/data/ins_updates.sbp
    fi
    echo ${i}
done
