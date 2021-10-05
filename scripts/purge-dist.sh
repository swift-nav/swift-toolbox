#!/usr/bin/env bash
set -ex

cd ./py39-n1ist

fd -I --full-path "/test/" | xargs -n1 rm -rf
fd -I --full-path "/tests/" | xargs -n1 rm -rf
fd -I --full-path "/examples/" | xargs -n1 rm -rf
fd -I --full-path "/__pycache__/" | xargs -n1 rm -rf

fd -I -e pyc | xargs -n1 rm
fd -I -e pyi | xargs -n1 rm
fd -I -e pdb | xargs -n1 rm
fd -I -e tcl | xargs -n1 rm

rm -rf ./lib/Tix8.4.3/

# Purge Qt stuff
fd Qt5WebEngineCore | xargs -n1 rm -rf
fd Qt5DesignerComponents | xargs -n1 rm -rf
fd Qt5VirtualKeyboad | xargs -n1 rm -rf
fd Qt5Pdf | xargs -n1 rm -rf
fd "QtWeb.*" | xargs -n1 rm -rf
fd --full-path ".*Qt/translations" | xargs -n1 rm -rf
fd --full-path ".*Qt/qml/QtNfc.*" | xargs -n1 rm -rf
fd --full-path ".*Qt/qml/QtBluetooth.*" | xargs -n1 rm -rf
fd --full-path ".*Qt/plugins/virtualkeyboad.*" | xargs -n1 rm -rf
fd --full-path ".*Qt/plugins/sqldrivers.*" | xargs -n1 rm -rf
fd --full-path ".*Qt/resources.*" | xargs -n1 rm -rf

./bin/python3 -m pip uninstall -y pip setuptools wheel
./bin/python3 -m compileall -b -f -o 1 -o 2 .

fd -I -e py | xargs -n1 rm
