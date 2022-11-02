#!/bin/bash
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
