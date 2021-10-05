Push-Location .\console_backend
& ..\py39\python setup.py bdist_wheel
Pop-Location

.\py39\python -m flit build

.\py39-dist\python -m pip install .\console_backend\dist\console_backend-0.1.0-cp39-cp39-win_amd64.whl --force-reinstal
.\py39-dist\python -m pip install .\dist\swiftnav_console-0.1.0-py3-none-any.whl --force-reinstall

if (-not (Test-Path ".\py39-dist\lib\site-packages\resources")) {
	Copy-Item -recurse ./src/main/resources .\py39-dist\lib\site-packages
}
