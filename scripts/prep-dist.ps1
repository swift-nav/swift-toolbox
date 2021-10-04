Remove-Item -Recurse -ErrorAction SilentlyContinue .\py39-dist
Copy-Item -Recurse .\standalone-py\python\install .\py39-dist
.\py39-dist\python .\get-pip.py
.\py39-dist\python -m pip install .
