"""Connection QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from .constants import Keys, QTKeys

CONNECTION: Dict[str, Any] = {
    Keys.AVAILABLE_PORTS: [],
    Keys.AVAILABLE_BAUDRATES: [],
    Keys.AVAILABLE_FLOWS: [],
    Keys.AVAILABLE_REFRESH_RATES: [],
    Keys.CONNECTED: False,
    Keys.PREVIOUS_HOSTS: [],
    Keys.PREVIOUS_PORTS: [],
    Keys.PREVIOUS_FILES: [],
}


class ConnectionData(QObject):  # pylint: disable=too-many-instance-attributes

    _available_ports: List[str] = []
    _available_baudrates: List[str] = []
    _available_flows: List[str] = []
    _available_refresh_rates: List[str] = []
    _connected: bool = False
    _previous_hosts: List[str] = []
    _previous_ports: List[str] = []
    _previous_files: List[str] = []

    def get_available_ports(self) -> List[str]:
        return self._available_ports

    def set_available_ports(self, available_ports: List[str]) -> None:
        self._available_ports = available_ports

    available_ports = Property(QTKeys.QVARIANTLIST, get_available_ports, set_available_ports)  # type: ignore

    def get_available_baudrates(self) -> List[str]:
        return self._available_baudrates

    def set_available_baudrates(self, available_baudrates: List[str]) -> None:
        self._available_baudrates = available_baudrates

    available_baudrates = Property(
        QTKeys.QVARIANTLIST, get_available_baudrates, set_available_baudrates  # type: ignore
    )

    def get_available_flows(self) -> List[str]:
        return self._available_flows

    def set_available_flows(self, available_flows: List[str]) -> None:
        self._available_flows = available_flows

    available_flows = Property(QTKeys.QVARIANTLIST, get_available_flows, set_available_flows)  # type: ignore

    def get_connected(self) -> bool:
        """Getter for _connected.

        Returns:
            bool: Whether a connection is live or not.
        """
        return self._connected

    def set_connected(self, connected: bool) -> None:
        """Setter for _connected."""
        self._connected = connected

    connected = Property(bool, get_connected, set_connected)

    def get_previous_hosts(self) -> List[str]:
        return self._previous_hosts

    def set_previous_hosts(self, previous_hosts: List[str]) -> None:
        self._previous_hosts = previous_hosts

    previous_hosts = Property(QTKeys.QVARIANTLIST, get_previous_hosts, set_previous_hosts)  # type: ignore

    def get_previous_ports(self) -> List[str]:
        return self._previous_ports

    def set_previous_ports(self, previous_ports: List[str]) -> None:
        self._previous_ports = previous_ports

    previous_ports = Property(QTKeys.QVARIANTLIST, get_previous_ports, set_previous_ports)  # type: ignore

    def get_previous_files(self) -> List[str]:
        return self._previous_files

    def set_previous_files(self, previous_files: List[str]) -> None:
        self._previous_files = previous_files

    previous_files = Property(QTKeys.QVARIANTLIST, get_previous_files, set_previous_files)  # type: ignore


class ConnectionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(ConnectionData)  # type: ignore
    def fill_data(self, cp: ConnectionData) -> ConnectionData:  # pylint:disable=no-self-use
        cp.set_available_ports(CONNECTION[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(CONNECTION[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(CONNECTION[Keys.AVAILABLE_FLOWS])
        cp.set_connected(CONNECTION[Keys.CONNECTED])
        cp.set_previous_hosts(CONNECTION[Keys.PREVIOUS_HOSTS])
        cp.set_previous_ports(CONNECTION[Keys.PREVIOUS_PORTS])
        cp.set_previous_files(CONNECTION[Keys.PREVIOUS_FILES])
        return cp
