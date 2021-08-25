tar -cvf .\py39-dist.tar .\py39-dist
rm py39-dist.xz
arc compress .\py39-dist.tar .\py39-dist.xz
mv .\py39-dist.xz .\swift-console.txz
