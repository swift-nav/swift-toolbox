"""Advanced System Monitor QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def advanced_system_monitor_tab_update() -> Dict[str, Any]:
    return {
        Keys.OBS_PERIOD: [],
        Keys.OBS_LATENCY: [],
        Keys.THREADS_TABLE: [],
        Keys.CSAC_TELEM_LIST: [],
        Keys.ZYNQ_TEMP: 0.0,
        Keys.FE_TEMP: 0.0,
        Keys.CSAC_RECEIVED: False,
    }


class AdvancedSystemMonitorData(QObject):  # pylint: disable=too-many-instance-attributes
    _obs_period: List[List[Any]] = []
    _obs_latency: List[List[Any]] = []
    _threads_table: List[List[Any]] = []
    _csac_telem_list: List[List[str]] = []
    _zynq_temp: float = 0.0
    _fe_temp: float = 0.0
    _csac_received: bool = False
    _data_updated = Signal(dict)
    advanced_system_monitor_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_system_monitor_tab = advanced_system_monitor_tab_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.advanced_system_monitor_tab = update_data

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
        cp.set_obs_latency(cp.advanced_system_monitor_tab[Keys.OBS_LATENCY])
        cp.set_obs_period(cp.advanced_system_monitor_tab[Keys.OBS_PERIOD])
        cp.set_threads_table(cp.advanced_system_monitor_tab[Keys.THREADS_TABLE])
        cp.set_csac_telem_list(cp.advanced_system_monitor_tab[Keys.CSAC_TELEM_LIST])
        cp.set_fe_temp(cp.advanced_system_monitor_tab[Keys.FE_TEMP])
        cp.set_zynq_temp(cp.advanced_system_monitor_tab[Keys.ZYNQ_TEMP])
        cp.set_csac_received(cp.advanced_system_monitor_tab[Keys.CSAC_RECEIVED])
        return cp
