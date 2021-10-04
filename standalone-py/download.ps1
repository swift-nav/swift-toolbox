$base_url = "https://github.com/indygreg/python-build-standalone/releases/download/20210724"
$cpython_release ="${base_url}/cpython-3.9.6-x86_64-pc-windows-msvc-shared-pgo-20210724T1424.tar.zst"

Invoke-WebRequest -Uri $cpython_release -Outfile py39.tar.zst

zstd -f -d py39.tar.zst -o py39.tar
tar -xvf py39.tar