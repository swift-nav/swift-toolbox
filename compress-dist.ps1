tar -C ".\py39-dist" -cvf ".\py39-dist.tar" .
rm -ErrorAction SilentlyContinue "py39-dist.tar.xz"
xz -e -9 ".\py39-dist.tar"
