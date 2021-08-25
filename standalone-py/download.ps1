iwr -Uri https://github.com/indygreg/python-build-standalone/releases/download/20210724/cpython-3.9.6-x86_64-pc-windows-msvc-shared-pgo-20210724T1424.tar.zst -Outfile py39.tar.zst
zstd -d py39.tar.zst -o py39.tar
tar -xvf py39.tar
