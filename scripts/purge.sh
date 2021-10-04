#!/usr/bin/env bash
set -ex

cd ./py39-dist

fd -I --full-path "/test/" | xargs -rd "\n" rm -rf
fd -I --full-path "/tests/" | xargs -rd "\n" rm -rf
fd -I --full-path "/examples/" | xargs -rd "\n" rm -rf
fd -I --full-path "/__pycache__/" | xargs -rd "\n" rm -rf

fd -I ".*\.pyc" | xargs -rd "\n" rm
fd -I ".*\.pyi" | xargs -rd "\n" rm
fd -I ".*\.pdb" | xargs -rd "\n" rm

# Remove-Item -Recurse -ErrorAction SilentlyContinue tcl

# Move-Item .\Scripts\swiftnav-console.exe .
# Remove-Item -Recurse .\Scripts\*
# Move-Item .\swiftnav-console.exe .\Scripts

# Remove-Item -Recurse -ErrorAction SilentlyContinue .\include
# Remove-Item .\DLLs\_test*.pyd
# Remove-Item .\DLLs\libcrypto*.dll
# Remove-Item .\DLLs\libssl*.dll
# Remove-Item .\DLLs\tcl*.dll
# Remove-Item .\DLLs\tk*.dll

# Remove-Item -Recurse -ErrorAction SilentlyContinue .\Lib\ensurepip

# # Purge Qt stuff
# Push-Location .\Lib\site-packages\PySide2

# Remove-Item -ErrorAction SilentlyContinue .\Qt5WebEngineCore.dll
# Remove-Item -ErrorAction SilentlyContinue .\Qt5DesignerComponents.dll
# Remove-Item -ErrorAction SilentlyContinue .\Qt5VirtualKeyboard.dll
# Remove-Item -ErrorAction SilentlyContinue .\Qt5Pdf.dll
# Remove-Item .\QtWeb*
# Remove-Item *.exe # qt tools in python installation like rcc.exe

# Remove-Item -Recurse .\translations\*
# Remove-Item -Recurse .\qml\QtWeb*
# Remove-Item -Recurse .\qml\QtBluetooth*
# Remove-Item -Recurse .\qml\QtNfc*
# Remove-Item -Recurse .\plugins\virtualkeyboard\*
# Remove-Item -Recurse .\plugins\sqldrivers\*
# Remove-Item -Recurse .\resources\*

# Pop-Location

# .\python -m pip uninstall -y pip setuptools wheel
# .\python -m compileall -b -f -o 1 -o 2 .

# List-Item -Recurse -Include *.py | ForEach-Object { Remove-Item -ErrorAction SilentlyContinue $_ }

# Pop-Location