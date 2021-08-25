# Do some basic purges
pushd py39-dist
fd -I --full-path '\\test\\' | %{ rm -recurse -ErrorAction SilentlyContinue $_ }
fd -I --full-path '\\examples\\' | %{ rm -recurse -ErrorAction SilentlyContinue $_ }
fd -I __pycache__ | %{ rm -recurse -ErrorAction SilentlyContinue $_ }
ls -Recurse -Include *.pdb | %{ rm -ErrorAction SilentlyContinue $_ }
rm -Recurse -ErrorAction SilentlyContinue tcl
mv .\Scripts\swiftnav-console.exe .
rm -Recurse .\Scripts\*
mv .\swiftnav-console.exe .\Scripts
cd .\Lib\site-packages\PySide2
rm .\Qt5WebEngineCore.dll
rm .\QtWeb*
rm *.exe # qt tools in python installation like rcc.exe
cd ..\..\..
.\python -m pip uninstall -y pip setuptools wheel
popd
