Copy-Item -Recurse .\standalone-py\python\install py39
.\py39\python ".\get-pip.py"
.\py39\python -m pip install flit
.\py39\python -m pip install .
.\py39\python -m pip install ".[test]"
