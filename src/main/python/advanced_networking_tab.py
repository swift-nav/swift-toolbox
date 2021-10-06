"""Advanced Networking QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, QTKeys

ADVANCED_NETWORKING_TAB: Dict[str, Any] = {
    Keys.NETWORK_INFO: [],
    Keys.RUNNING: False,
    Keys.IP_ADDRESS: "127.0.0.1",
    Keys.PORT: 13320,
}


class AdvancedNetworkingData(QObject):
    _network_info: List[List[str]] = []
    _running: bool = False
    _ip_address: str = ""
    _port: int = 0

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
    def fill_console_points(self, cp: AdvancedNetworkingData) -> AdvancedNetworkingData:  # pylint:disable=no-self-use
        cp.set_network_info(ADVANCED_NETWORKING_TAB[Keys.NETWORK_INFO])
        cp.set_running(ADVANCED_NETWORKING_TAB[Keys.RUNNING])
        cp.set_ip_address(ADVANCED_NETWORKING_TAB[Keys.IP_ADDRESS])
        cp.set_port(ADVANCED_NETWORKING_TAB[Keys.PORT])
        return cp
