# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
