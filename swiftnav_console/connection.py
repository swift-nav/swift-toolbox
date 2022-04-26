"""Connection QObjects.
"""
from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys, ConnectionState, ConnectionType


def connection_update() -> Dict[str, Any]:
    return {
        Keys.AVAILABLE_PORTS: [],
        Keys.AVAILABLE_BAUDRATES: [],
        Keys.AVAILABLE_FLOWS: [],
        Keys.AVAILABLE_REFRESH_RATES: [],
        Keys.PREVIOUS_HOSTS: [],
        Keys.PREVIOUS_PORTS: [],
        Keys.PREVIOUS_FILES: [],
        Keys.LAST_USED_SERIAL_DEVICE: None,
        Keys.PREVIOUS_SERIAL_CONFIGS: [],
        Keys.CONSOLE_VERSION: str,
        Keys.PREVIOUS_CONNECTION_TYPE: "",
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
    _console_version: str = ""
    _previous_connection_type: ConnectionType = ConnectionType.Serial
    _connection_message: str = ""
    _data_updated = Signal(dict)
    _state_updated = Signal(ConnectionState)
    _message_updated = Signal(str)
    connection: Dict[str, Any] = {}
    connection_state: ConnectionState = ConnectionState.DISCONNECTED
    connection_msg: str = ""

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.connection = connection_update()
        self.connection_state = ConnectionState.DISCONNECTED
        self.connection_msg = ""
        self._data_updated.connect(self.handle_data_updated)
        self._state_updated.connect(self.handle_state_updated)
        self._message_updated.connect(self.handle_message_updated)

    @classmethod
    def post_connection_state_update(cls, update_data: ConnectionState) -> None:
        cls._instance._state_updated.emit(update_data)

    @classmethod
    def post_connection_message_update(cls, update_data: str) -> None:
        cls._instance._message_updated.emit(update_data)

    @classmethod
    def post_connection_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(ConnectionState)  # type: ignore
    def handle_state_updated(self, data: ConnectionState) -> None:
        self.connection_state = data

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.connection = update_data

    @Slot(str)  # type: ignore
    def handle_message_updated(self, data: str) -> None:
        self.connection_msg = data

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

    def get_console_version(self) -> str:
        return self._console_version

    def set_console_version(self, console_version: str) -> None:
        self._console_version = console_version

    console_version = Property(str, get_console_version, set_console_version)

    def get_previous_connection_type(self) -> ConnectionType:
        return self._previous_connection_type

    def set_previous_connection_type(self, previous_connection_type: ConnectionType) -> None:
        self._previous_connection_type = previous_connection_type

    previous_connection_type = Property(str, get_previous_connection_type, set_previous_connection_type)

    def get_connection_message(self) -> str:
        return self._connection_message

    def set_connection_message(self, connection_message: str) -> None:
        self._connection_message = connection_message

    connection_message = Property(str, get_connection_message, set_connection_message)


class ConnectionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(ConnectionData)  # type: ignore
    def fill_data(self, cp: ConnectionData) -> ConnectionData:  # pylint:disable=no-self-use
        cp.set_available_ports(cp.connection[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(cp.connection[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(cp.connection[Keys.AVAILABLE_FLOWS])
        cp.set_conn_state(cp.connection_state)
        cp.set_previous_hosts(cp.connection[Keys.PREVIOUS_HOSTS])
        cp.set_previous_ports(cp.connection[Keys.PREVIOUS_PORTS])
        cp.set_previous_files(cp.connection[Keys.PREVIOUS_FILES])
        cp.set_last_used_serial_device(cp.connection[Keys.LAST_USED_SERIAL_DEVICE])
        cp.set_previous_serial_configs(cp.connection[Keys.PREVIOUS_SERIAL_CONFIGS])
        cp.set_console_version(cp.connection[Keys.CONSOLE_VERSION])
        cp.set_previous_connection_type(cp.connection[Keys.PREVIOUS_CONNECTION_TYPE])
        cp.set_connection_message(cp.connection_msg)
        cp.connection_msg = ""
        return cp
