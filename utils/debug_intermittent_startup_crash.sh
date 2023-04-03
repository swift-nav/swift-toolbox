#!/bin/bash

# This script will aid the debugging of application crashes that occur
# intermittently, so long as the app can be configured to run without human
# intervention. Especially useful if the crash happens early on in app startup.
# Your app is required to exit after some predetermined period of time - the
# shorter the runtime, the more iterations that can be done per minute/hour,
# and the higher likelihood you'll be able to catch the crash.

# You can try this out with this sample c++ code:
##include <iostream>
##include <string>
##include <cstdlib>
##include <ctime>
#
#using namespace std;
#
#int main(int argc, char**argv)
#{
#
#    std::srand(std::time(nullptr));
#    int r = rand() % 100;
#    cout << "Hello, World " << r << endl;
#    if (argc <= 1 && r < 50 ) {
#        string *strp = nullptr;
#        cout << *strp << endl;
#    }
#
#    return 0;
#}


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

#if [ ! -d py311-dist ]; then
#    cargo make build-dist
#fi

function join () {
    local delim=${1-}
    local tojoin=${2-}
    if shift 2; then
        printf %s "$tojoin" "${@/#/$delim}"
    fi
}


# Log this whole looped debugging session.
# Output file will be named debug_intermittent_startup_crash<num>.log
logfn=./$(uniqueFn "debug_intermittent_startup_crash" "log")
exec 3>&1 4>&2
exec 1> >(tee $logfn) 2>&1

export SWIFT_CONSOLE_ARGS="--log-console --file ../console_backend/tests/data/ins_updates.sbp --exit-after-timeout=10"

trap 'exec 2>&4 1>&3; echo Ctrl-C pressed, exiting loop. ; exit' SIGINT SIGTERM

echo "Logging to $logfn"

cd ${scriptpath}/../py311-dist
# macOS (or any platform using lldb) works differently.
# The looping is done within the debugger in a python script.
# This is faster, as the debugger doesn't have to shutdown and restart,
# and the executable does not have to be repeatedly loaded.
# The --batch -o run options to lldb using a shell loop could be used
# instead of this, but is slower.
if [ $(uname) == "Darwin" ]; then
    tmpdir=${TMPDIR:-.}
    lldbpy_path=$(mktemp -u $tmpdir/intmt_startup_crashXXXXX)
    echo "lldbpy_path == $lldbpy_path"
    awk '/[P]LUGH/{p=1; next}p' ${scriptpath}/$(basename $0) >${lldbpy_path}.py

    lldbcmdtorun="script $(basename ${lldbpy_path}).run(\"$(join "\", \"" $SWIFT_CONSOLE_ARGS)\")"
    echo -e "\n\nOnce lldb starts, run the following (it will be in your clipboard so you can just paste):"
    echo -e "$lldbcmdtorun\n\n"
    exit 0
    echo -n "$lldbcmdtorun" | pbcopy
    rust-lldb -o "command script import ${lldbpy_path}.py" -- ./swift-console
    #rust-lldb --batch -o run -- ./swift-console
    exec 2>&4 1>&3
    rm ${lldbpy_path}.py
    trap SIGINT SIGTERM
    exit 0
fi

echo "Starting looped debugging of app, 10s execution per iteration."

uname_o=$(uname -o 2>/dev/null)
for i in {1..1000}; do
    if [ ${uname_o} == "Msys" ]; then
        # With Visual Studio installed, if there is a crash, a dialog will pop
        # up prompting if you want to debug.
        ./swift-console $SWIFT_CONSOLE_ARGS
    else
        rust-gdb -iex='set pagination off' -ex='set confirm on' -ex run -ex=quit --args swift-console $SWIFT_CONSOLE_ARGS
    fi
    echo ${i}
done
exit 0
PLUGH
import lldb
import os
from time import sleep

print("intmt_startup_crash.py")
# If trying to run at startup/import from command line,
# the `lldb` object does not exist - only in an interactive python
# session will it exist.
# Without lldb existing - there seems to be no way to
# get the current target - so creation and launch of the target
# would need to happen entirely in python. There is also
# no access to the arguments to lldb - so we can't get the name
# of the process to debug from the lldb command line. sys.argv
# is empty.
#lldb.SBDebugger.Initialize()
#debugger = lldb.SBDebugger.Create()
#target = debugger.CreateTargetWithFileAndArch(exe, lldb,LLDB_ARCH_DEFAULT)

# Instead, provide a function for the user to execute with a target
# already loaded.

state_map = {
    lldb.eStateInvalid   : "eStateInvalid",
    lldb.eStateUnloaded  : "eStateUnloaded",
    lldb.eStateConnected : "eStateConnected",
    lldb.eStateAttaching : "eStateAttaching",
    lldb.eStateLaunching : "eStateLaunching",
    lldb.eStateStopped   : "eStateStopped",
    lldb.eStateRunning   : "eStateRunning",
    lldb.eStateStepping  : "eStateStepping",
    lldb.eStateCrashed   : "eStateCrashed",
    lldb.eStateDetached  : "eStateDetached",
    lldb.eStateExited    : "eStateExited",
    lldb.eStateSuspended : "eStateSuspended",
}

# Restart the target upon seeing a normal exit, until an abnormal state
# is reached (breakpoint is hit, or there is an exception)
# NOTE: This requires an application that will exit normally after a
# predetermined amount of time without user input.
#
# To use this, run the following commands in lldb:
# command script import intmt_startup_crash.py
# python script intmt_startup_crash.run("./hello_world", "arg1", "arg2", ...)
#
# This can also be added to your .lldbinit file using the command script
# import line above.
def run(*args, target=None):
    if target is None:
        target = lldb.target
    args = list(args)

    if target == None or target.LaunchSimple == None:
        print("error: no target provided")
        return -1
    else:
        process = lldb.target.LaunchSimple(args, None, os.getcwd())
        while (process.state == lldb.eStateRunning or
               process.state == lldb.eStateExited) :
            sleep(0.1)
            if process.state == lldb.eStateExited:
                process = lldb.target.LaunchSimple(args, None, os.getcwd())

    print("process state: " + state_map[process.state])
    return 0
