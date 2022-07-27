import pathlib

try:
    import PySide6  # type: ignore

    print(pathlib.Path(PySide6.__file__).parent)
except ImportError:
    pass
