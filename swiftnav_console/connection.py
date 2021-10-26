"""Connection QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from .constants import Keys, QTKeys, ConnectionState

CONNECTION: Dict[str, Any] = {
    Keys.AVAILABLE_PORTS: [],
    Keys.AVAILABLE_BAUDRATES: [],
    Keys.AVAILABLE_FLOWS: [],
    Keys.AVAILABLE_REFRESH_RATES: [],
    Keys.CONNECTION_STATE: ConnectionState.DISCONNECTED,
    Keys.PREVIOUS_HOSTS: [],
    Keys.PREVIOUS_PORTS: [],
    Keys.PREVIOUS_FILES: [],
    Keys.LAST_USED_SERIAL_DEVICE: None,
    Keys.PREVIOUS_SERIAL_CONFIGS: [],
}


class ConnectionData(QObject):  # pylint: disable=too-many-instance-attributes disable=too-many-public-methods

    _available_ports: List[str] = []
    _available_baudrates: List[str] = []
    _available_flows: List[str] = []
    _available_refresh_rates: List[str] = []
    _conn_state: ConnectionState = ConnectionState.DISCONNECTED
    _previous_hosts: List[str] = []
    _previous_ports: List[str] = []
    _previous_files: List[str] = []
    _last_used_serial_device: str
    _previous_serial_configs: List[List[Any]] = []

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

    def get_conn_state(self) -> ConnectionState:
        """Getter for _conn_state.

        Returns:
            ConnectionState: Whether a connection is live, disconnecting or disconnected.
        """
        return self._conn_state

    def set_conn_state(self, conn_state: ConnectionState) -> None:
        """Setter for _conn_state."""
        self._conn_state = conn_state

    conn_state = Property(str, get_conn_state, set_conn_state)

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

    def get_last_used_serial_device(self) -> str:
        return self._last_used_serial_device

    def set_last_used_serial_device(self, last_used_serial_device: str) -> None:
        self._last_used_serial_device = last_used_serial_device

    last_used_serial_device = Property(str, get_last_used_serial_device, set_last_used_serial_device)  # type: ignore

    def get_previous_serial_configs(self) -> List[List[Any]]:
        return self._previous_serial_configs

    def set_previous_serial_configs(self, previous_serial_configs: List[List[Any]]) -> None:
        self._previous_serial_configs = previous_serial_configs

    previous_serial_configs = Property(
        QTKeys.QVARIANTLIST, get_previous_serial_configs, set_previous_serial_configs  # type: ignore
    )


class ConnectionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(ConnectionData)  # type: ignore
    def fill_data(self, cp: ConnectionData) -> ConnectionData:  # pylint:disable=no-self-use
        cp.set_available_ports(CONNECTION[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(CONNECTION[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(CONNECTION[Keys.AVAILABLE_FLOWS])
        cp.set_conn_state(CONNECTION[Keys.CONNECTION_STATE])
        cp.set_previous_hosts(CONNECTION[Keys.PREVIOUS_HOSTS])
        cp.set_previous_ports(CONNECTION[Keys.PREVIOUS_PORTS])
        cp.set_previous_files(CONNECTION[Keys.PREVIOUS_FILES])
        cp.set_last_used_serial_device(CONNECTION[Keys.LAST_USED_SERIAL_DEVICE])
        cp.set_previous_serial_configs(CONNECTION[Keys.PREVIOUS_SERIAL_CONFIGS])
        return cp
