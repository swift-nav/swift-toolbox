from types import SimpleNamespace
from typing import Any


class MockEndpoint:
    def send_message(self, message: Any) -> None:
        pass

    def shutdown(self):
        pass


def return_mock_endpoint():
    return MockEndpoint()


class BackendImporter:  # pylint: disable=too-few-public-methods
    def __init__(self, use_fake=False):
        if use_fake:
            self.Server = lambda: SimpleNamespace(start=return_mock_endpoint)
        else:
            import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module,import-outside-toplevel

            self.Server = console_backend.server.Server  # pylint: disable=no-member,c-extension-no-member
