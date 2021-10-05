#!/usr/bin/env bash
set -ex

gnuxargs=$(xargs --version 2>&1 |grep -s GNU >/dev/null && echo true || echo false)

# deal with inconsistencies between gnu xargs and the version that ships with osx
# https://stackoverflow.com/questions/8803987/what-is-the-equivalent-to-xargs-r-under-osx:
function xargs_r() {
  if $gnuxargs ; then
    cat - | xargs -r "$@"
  else
    cat - | xargs "$@"
  fi
}

cd ./py39-dist

fd -I --full-path "/test/" | xargs_r -n1 rm -rf
fd -I --full-path "/tests/" | xargs_r -n1 rm -rf
fd -I --full-path "/examples/" | xargs_r -n1 rm -rf
fd -I --full-path "/__pycache__/" | xargs_r -n1 rm -rf

fd -I -e pyc | xargs_r -n1 rm
fd -I -e pyi | xargs_r -n1 rm
fd -I -e pdb | xargs_r -n1 rm
fd -I -e tcl | xargs_r -n1 rm

rm -rf ./lib/Tix8.4.3/

# Purge Qt stuff
fd Qt5WebEngineCore | xargs_r -n1 rm -rf
fd Qt5DesignerComponents | xargs_r -n1 rm -rf
fd Qt5VirtualKeyboad | xargs_r -n1 rm -rf
fd Qt5Pdf | xargs_r -n1 rm -rf
fd "QtWeb.*" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/translations" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/qml/QtNfc.*" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/qml/QtBluetooth.*" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/plugins/virtualkeyboad.*" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/plugins/sqldrivers.*" | xargs_r -n1 rm -rf
fd --full-path ".*Qt/resources.*" | xargs_r -n1 rm -rf

./bin/python3 -m pip uninstall -y pip setuptools wheel
./bin/python3 -m compileall -b -f -o 1 -o 2 .

fd -I -e py | xargs_r -n1 rm
