# Do some basic purges
pushd py39-dist

fd -I --full-path '\\test\\' | %{ rm -recurse -ErrorAction SilentlyContinue $_ }
fd -I --full-path '\\tests\\' | %{ rm -recurse -ErrorAction SilentlyContinue $_ }
fd -I --full-path '\\examples\\' | %{ rm -recurse -ErrorAction SilentlyContinue $_ }

ls -recurse -include __pycache__ | %{ rm -recurse $_ }

ls -Recurse -Include *.pyc | %{ rm -ErrorAction SilentlyContinue $_ }
ls -Recurse -Include *.pdb | %{ rm -ErrorAction SilentlyContinue $_ }

rm -Recurse -ErrorAction SilentlyContinue tcl

mv .\Scripts\swiftnav-console.exe .
rm -Recurse .\Scripts\*
mv .\swiftnav-console.exe .\Scripts

rm -Recurse -ErrorAction SilentlyContinue .\include
rm .\DLLs\_test*.pyd
rm .\DLLs\libcrypto*.dll
rm .\DLLs\libssl*.dll
rm .\DLLs\tcl*.dll
rm .\DLLs\tk*.dll

# Purge Qt stuff
pushd .\Lib\site-packages\PySide2

rm -ErrorAction SilentlyContinue .\Qt5WebEngineCore.dll
rm -ErrorAction SilentlyContinue .\Qt5DesignerComponents.dll
rm -ErrorAction SilentlyContinue .\Qt5VirtualKeyboard.dll
rm -ErrorAction SilentlyContinue .\Qt5Pdf.dll
rm .\QtWeb*
rm *.exe # qt tools in python installation like rcc.exe

rm -Recurse .\translations\*
rm -Recurse .\qml\QtWeb*
rm -Recurse .\qml\QtBluetooth*
rm -Recurse .\qml\QtNfc*

popd

.\python -m pip uninstall -y pip setuptools wheel
.\python -m compileall -b -f -o 1 -o 2 .

popd
