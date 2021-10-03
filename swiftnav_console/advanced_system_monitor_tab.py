"""Baseline Plot QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from .constants import Keys, QTKeys

ADVANCED_SYSTEM_MONITOR_TAB: Dict[str, Any] = {
    Keys.OBS_PERIOD: [],
    Keys.OBS_LATENCY: [],
    Keys.THREADS_TABLE: [],
    Keys.CSAC_TELEM_LIST: [],
    Keys.ZYNQ_TEMP: 0.0,
    Keys.FE_TEMP: 0.0,
    Keys.CSAC_RECEIVED: False,
}


class AdvancedSystemMonitorData(QObject):
    _obs_period: List[List[Any]] = []
    _obs_latency: List[List[Any]] = []
    _threads_table: List[List[Any]] = []
    _csac_telem_list: List[List[str]] = []
    _zynq_temp: float = 0.0
    _fe_temp: float = 0.0
    _csac_received: bool = False

    def get_csac_telem_list(self) -> List[List[str]]:
        """Getter for _csac_telem_list."""
        return self._csac_telem_list

    def set_csac_telem_list(self, csac_telem_list: List[List[str]]) -> None:
        """Setter for _csac_telem_list."""
        self._csac_telem_list = csac_telem_list

    csac_telem_list = Property(QTKeys.QVARIANTLIST, get_csac_telem_list, set_csac_telem_list)  # type: ignore

    def get_threads_table(self) -> List[List[str]]:
        """Getter for _threads_table."""
        return self._threads_table

    def set_threads_table(self, threads_table: List[List[str]]) -> None:
        """Setter for _threads_table."""
        self._threads_table = threads_table

    threads_table = Property(QTKeys.QVARIANTLIST, get_threads_table, set_threads_table)  # type: ignore

    def get_obs_latency(self) -> List[List[str]]:
        """Getter for _obs_latency."""
        return self._obs_latency

    def set_obs_latency(self, obs_latency: List[List[str]]) -> None:
        """Setter for _obs_latency."""
        self._obs_latency = obs_latency

    obs_latency = Property(QTKeys.QVARIANTLIST, get_obs_latency, set_obs_latency)  # type: ignore

    def get_obs_period(self) -> List[List[str]]:
        """Getter for _obs_period."""
        return self._obs_period

    def set_obs_period(self, obs_period: List[List[str]]) -> None:
        """Setter for _obs_period."""
        self._obs_period = obs_period

    obs_period = Property(QTKeys.QVARIANTLIST, get_obs_period, set_obs_period)  # type: ignore

    def get_zynq_temp(self) -> float:
        """Getter for _zynq_temp."""
        return self._zynq_temp

    def set_zynq_temp(self, zynq_temp_: float) -> None:
        """Setter for _zynq_temp."""
        self._zynq_temp = zynq_temp_

    zynq_temp = Property(float, get_zynq_temp, set_zynq_temp)

    def get_fe_temp(self) -> float:
        """Getter for _fe_temp."""
        return self._fe_temp

    def set_fe_temp(self, fe_temp_: float) -> None:
        """Setter for _fe_temp."""
        self._fe_temp = fe_temp_

    fe_temp = Property(float, get_fe_temp, set_fe_temp)

    def get_csac_received(self) -> bool:
        """Getter for _csac_received."""
        return self._csac_received

    def set_csac_received(self, csac_received_: bool) -> None:
        """Setter for _csac_received."""
        self._csac_received = csac_received_

    csac_received = Property(bool, get_csac_received, set_csac_received)


class AdvancedSystemMonitorModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedSystemMonitorData)  # type: ignore
    def fill_console_points(  # pylint:disable=no-self-use
        self, cp: AdvancedSystemMonitorData
    ) -> AdvancedSystemMonitorData:
        cp.set_obs_latency(ADVANCED_SYSTEM_MONITOR_TAB[Keys.OBS_LATENCY])
        cp.set_obs_period(ADVANCED_SYSTEM_MONITOR_TAB[Keys.OBS_PERIOD])
        cp.set_threads_table(ADVANCED_SYSTEM_MONITOR_TAB[Keys.THREADS_TABLE])
        cp.set_csac_telem_list(ADVANCED_SYSTEM_MONITOR_TAB[Keys.CSAC_TELEM_LIST])
        cp.set_fe_temp(ADVANCED_SYSTEM_MONITOR_TAB[Keys.FE_TEMP])
        cp.set_zynq_temp(ADVANCED_SYSTEM_MONITOR_TAB[Keys.ZYNQ_TEMP])
        cp.set_csac_received(ADVANCED_SYSTEM_MONITOR_TAB[Keys.CSAC_RECEIVED])
        return cp
