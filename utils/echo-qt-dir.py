import pathlib

try:
    import qt5_applications  # type: ignore

    print(pathlib.Path(qt5_applications.__file__).parent)
except ImportError:
    pass
