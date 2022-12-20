#!/bin/bash

# Simpler script to reproduce intermittent crash with rustless console

# should aid in generating a core file to inspect.
# No interactive debugger is invoked for this script - just crash and
# generate core file for later inspection.

if [ "$core_pat" != "core" ]; then
    echo "/proc/sys/kernel/core_pattern incorrect: '$core_pat', should be 'core'"
    echo "Please run the following as root:"
    echo "$ echo core  > /proc/sys/kernel/core_pattern"
    exit 1
fi
ulimit -c unlimited
ret = $?
if [$ret -ne 0]; then
    exit $ret
fi

I=0
while [ true ]
do
    echo "++++++++++++++++++++++" $I
python -m swiftnav_console.main --read-capnp-recording console_backend/tests/data/console-capnp-20220419-033358.pickle
if [ $? -ne 0 ]; then
    break
fi
    let I=I+1
done
