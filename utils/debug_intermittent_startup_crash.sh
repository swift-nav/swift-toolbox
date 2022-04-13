#!/bin/bash

scriptpath=`dirname $0`
scriptpath=`(cd "$scriptpath"; /bin/pwd)`

function uniqueFn () {
    name=$1
    ext=$2
    if [[ -e ${name}.$ext || -L ${name}/$ext ]] ; then
        i=0
        while [[ -e ${name}${i}.$ext || -L ${name}${i}.$ext ]] ; do
            let i++
        done
        name=${name}$i
    fi
    uniquefn="${name}.$ext"
    touch -- "$uniquefn"
    echo "$uniquefn"
}

if [ ! -d py39-dist ]; then
    cargo make build-dist
fi

# If we are not on windows, log this whole looped debugging session.
# Output file will be named debug_intermittent_startup_crash<num>.log
logfn=./$(uniqueFn "debug_intermittent_startup_crash" "log")
exec 3>&1 4>&2
exec 1>$logfn 2>&1

export SWIFT_CONSOLE_ARGS="--log-console --file ../console_backend/tests/data/ins_updates.sbp --exit-after-timeout=10"

trap 'exec 2>&4 1>&3; echo Ctrl-C pressed, exiting loop. ; exit' SIGINT SIGTERM

echo "Logging to $logfn"
echo "Starting looped debugging of app, 10s execution per iteration."

cd ${scriptpath}/../py39-dist
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
