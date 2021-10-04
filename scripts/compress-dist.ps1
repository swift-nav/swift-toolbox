tar -C ".\py39-dist" -cvf ".\py39-dist.tar" .
Remove-Item -ErrorAction SilentlyContinue "py39-dist.tar.xz"
xz -e -9 ".\py39-dist.tar"
