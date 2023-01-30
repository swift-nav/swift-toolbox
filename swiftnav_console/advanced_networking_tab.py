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

"""Advanced Networking QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def advanced_networking_tab_update() -> Dict[str, Any]:
    return {
        Keys.NETWORK_INFO: [],
        Keys.RUNNING: False,
        Keys.IP_ADDRESS: "127.0.0.1",
        Keys.PORT: 13320,
    }


ADVANCED_NETWORKING_TAB: List[Dict[str, Any]] = [advanced_networking_tab_update()]


class AdvancedNetworkingData(QObject):
    _instance: "AdvancedNetworkingData"
    _network_info: List[List[str]] = []
    _running: bool = False
    _ip_address: str = ""
    _port: int = 0
    _data_updated = Signal()
    advanced_networking_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_networking_tab = ADVANCED_NETWORKING_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        ADVANCED_NETWORKING_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.advanced_networking_tab = ADVANCED_NETWORKING_TAB[0]

    def get_network_info(self) -> List[List[str]]:
        """Getter for _network_info."""
        return self._network_info

    def set_network_info(self, network_info: List[List[str]]) -> None:
        """Setter for _network_info."""
        self._network_info = network_info

    network_info = Property(QTKeys.QVARIANTLIST, get_network_info, set_network_info)  # type: ignore

    def get_running(self) -> bool:
        """Getter for _running."""
        return self._running

    def set_running(self, running_: bool) -> None:
        """Setter for _running."""
        self._running = running_

    running = Property(bool, get_running, set_running)

    def get_ip_address(self) -> str:
        """Getter for _ip_address."""
        return self._ip_address

    def set_ip_address(self, ip_address: str) -> None:
        """Setter for _ip_address."""
        self._ip_address = ip_address

    ip_address = Property(str, get_ip_address, set_ip_address)

    def get_port(self) -> int:
        """Getter for _port."""
        return self._port

    def set_port(self, port: int) -> None:
        """Setter for _port."""
        self._port = port

    port = Property(int, get_port, set_port)


class AdvancedNetworkingModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedNetworkingData)  # type: ignore
    def fill_console_points(self, cp: AdvancedNetworkingData) -> AdvancedNetworkingData:
        cp.set_network_info(cp.advanced_networking_tab[Keys.NETWORK_INFO])
        cp.set_running(cp.advanced_networking_tab[Keys.RUNNING])
        cp.set_ip_address(cp.advanced_networking_tab[Keys.IP_ADDRESS])
        cp.set_port(cp.advanced_networking_tab[Keys.PORT])
        return cp
