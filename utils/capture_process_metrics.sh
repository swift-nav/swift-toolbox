#!/bin/bash
# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

set -e -u

sleepSecs=1

for cmd in date pgrep ps sleep ; do
    set +e
    $cmd --version >/dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "'$cmd' command is not present. aborting."
        exit 1
    fi
    set -e
done

set +e
kill -l >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "'kill' command is not present. aborting."
    exit 1
fi
set -e

outfile=""
swift_date=$(date)
if [[ $# -ge 1 && "$1" == -O ]]; then
    outfile=$(date +%Y%m%d-%H%M%S.swiftnav-process-metrics.log)
    echo "Saving output to $outfile"
fi

pgrep_cmd="pgrep -f swiftnav_console.main"
swift_pid=$($pgrep_cmd | head -1)
if [[ -z $swift_pid ]]; then
    echo "Swift Toolbox not running, waiting 10 seconds for it to start."
    for i in {10..0}; do
        if [[ $i -eq 0 ]]; then
            echo "exiting."
            exit 1
        fi
        swift_pid=$($pgrep_cmd | head -1)
        if [[ -n $swift_pid ]]; then
            break;
        fi
        sleep 1
    done
fi

echo $swift_date
if [[ -n $outfile ]]; then
    echo $swift_date > $outfile
fi

cmd="ps -p $swift_pid -o etimes,%cpu,%mem"
$cmd
kill -s 0 $swift_pid 2>/dev/null
while [[ $? -eq 0 ]]; do
    sleep $sleepSecs
    if [[ -n $outfile ]]; then
        $cmd --no-headers | tee -a $outfile
    else
        $cmd --no-headers
    fi
    kill -s 0 $swift_pid 2>/dev/null
done
