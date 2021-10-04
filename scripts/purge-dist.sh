#!/usr/bin/env bash
set -ex

cd ./py39-dist

fd -I --full-path "/test/" | xargs -rd "\n" rm -rf
fd -I --full-path "/tests/" | xargs -rd "\n" rm -rf
fd -I --full-path "/examples/" | xargs -rd "\n" rm -rf
fd -I --full-path "/__pycache__/" | xargs -rd "\n" rm -rf

fd -I -e pyc | xargs -rd "\n" rm
fd -I -e pyi | xargs -rd "\n" rm
fd -I -e pdb | xargs -rd "\n" rm
fd -I -e tcl | xargs -rd "\n" rm

rm -rf ./lib/Tix8.4.3/

# Purge Qt stuff
fd Qt5WebEngineCore | xargs -rd "\n" rm -rf
fd Qt5DesignerComponents | xargs -rd "\n" rm -rf
fd Qt5VirtualKeyboard | xargs -rd "\n" rm -rf
fd Qt5Pdf | xargs -rd "\n" rm -rf
fd "QtWeb.*" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/translations" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/qml/QtNfc.*" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/qml/QtBluetooth.*" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/plugins/virtualkeyboard.*" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/plugins/sqldrivers.*" | xargs -rd "\n" rm -rf
fd --full-path ".*Qt/resources.*" | xargs -rd "\n" rm -rf

./bin/python3 -m pip uninstall -y pip setuptools wheel
./bin/python3 -m compileall -b -f -o 1 -o 2 .

fd -I -e py | xargs -rd "\n" rm
