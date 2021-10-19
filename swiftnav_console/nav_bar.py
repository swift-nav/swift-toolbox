"""Nav Bar QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from .constants import Keys, LogLevel, QTKeys, ConnectionState

NAV_BAR: Dict[str, Any] = {
    Keys.AVAILABLE_PORTS: [],
    Keys.AVAILABLE_BAUDRATES: [],
    Keys.AVAILABLE_FLOWS: [],
    Keys.AVAILABLE_REFRESH_RATES: [],
    Keys.APPLICATION_STATE: ConnectionState.DISCONNECTED,
    Keys.PREVIOUS_HOSTS: [],
    Keys.PREVIOUS_PORTS: [],
    Keys.PREVIOUS_FILES: [],
    Keys.LOG_LEVEL_LABELS: [LogLevel.ERROR, LogLevel.WARNING, LogLevel.NOTICE, LogLevel.INFO, LogLevel.DEBUG],
    Keys.LOG_LEVEL: LogLevel.INFO,
}


class NavBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _available_ports: List[str] = []
    _available_baudrates: List[str] = []
    _available_flows: List[str] = []
    _available_refresh_rates: List[str] = []
    _conn_state: ConnectionState = ConnectionState.DISCONNECTED
    _previous_hosts: List[str] = []
    _previous_ports: List[str] = []
    _previous_files: List[str] = []
    _log_level_labels: List[str] = []
    _log_level: str

    def get_log_level_labels(self) -> List[str]:
        return self._log_level_labels

    def set_log_level_labels(self, log_level_labels: List[str]) -> None:
        self._log_level_labels = log_level_labels

    log_level_labels = Property(QTKeys.QVARIANTLIST, get_log_level_labels, set_log_level_labels)  # type: ignore

    def get_log_level(self) -> str:
        return self._log_level

    def set_log_level(self, log_level: str) -> None:
        self._log_level = log_level

    log_level = Property(str, get_log_level, set_log_level)

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

    def get_available_refresh_rates(self) -> List[str]:
        return self._available_refresh_rates

    def set_available_refresh_rates(self, available_refresh_rates: List[str]) -> None:
        self._available_refresh_rates = available_refresh_rates

    available_refresh_rates = Property(
        QTKeys.QVARIANTLIST, get_available_refresh_rates, set_available_refresh_rates  # type: ignore
    )

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


class NavBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(NavBarData)  # type: ignore
    def fill_data(self, cp: NavBarData) -> NavBarData:  # pylint:disable=no-self-use
        cp.set_available_ports(NAV_BAR[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(NAV_BAR[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(NAV_BAR[Keys.AVAILABLE_FLOWS])
        cp.set_available_refresh_rates(NAV_BAR[Keys.AVAILABLE_REFRESH_RATES])
        cp.set_conn_state(NAV_BAR[Keys.APPLICATION_STATE])
        cp.set_previous_hosts(NAV_BAR[Keys.PREVIOUS_HOSTS])
        cp.set_previous_ports(NAV_BAR[Keys.PREVIOUS_PORTS])
        cp.set_previous_files(NAV_BAR[Keys.PREVIOUS_FILES])
        cp.set_log_level_labels(NAV_BAR[Keys.LOG_LEVEL_LABELS])
        cp.set_log_level(NAV_BAR[Keys.LOG_LEVEL])
        return cp
