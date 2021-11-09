"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys
import threading

from typing import List, Any, Optional

import capnp  # type: ignore

from PySide2.QtWidgets import QApplication  # type: ignore

from PySide2.QtCore import QObject, QUrl, QPointF, Slot
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtGui import QFontDatabase

from PySide2.QtQml import QQmlComponent, qmlRegisterType

import swiftnav_console.console_resources  # type: ignore # pylint: disable=unused-import

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

from .constants import ApplicationMetadata, ConnectionState, Keys, Tabs, QTKeys

from .log_panel import (
    LOG_PANEL,
    log_panel_lock,
    LogPanelData,
    LogPanelModel,
)

from .connection import (
    CONNECTION,
    ConnectionData,
    ConnectionModel,
)

from .logging_bar import (
    LOGGING_BAR,
    LoggingBarData,
    LoggingBarModel,
)

from .advanced_imu_tab import (
    AdvancedImuModel,
    AdvancedImuPoints,
    ADVANCED_IMU_TAB,
)

from .advanced_magnetometer_tab import (
    AdvancedMagnetometerModel,
    AdvancedMagnetometerPoints,
    ADVANCED_MAGNETOMETER_TAB,
)

from .advanced_networking_tab import (
    AdvancedNetworkingModel,
    AdvancedNetworkingData,
    ADVANCED_NETWORKING_TAB,
)

from .advanced_spectrum_analyzer_tab import (
    AdvancedSpectrumAnalyzerModel,
    AdvancedSpectrumAnalyzerPoints,
    ADVANCED_SPECTRUM_ANALYZER_TAB,
)

from .advanced_system_monitor_tab import (
    AdvancedSystemMonitorModel,
    AdvancedSystemMonitorData,
    ADVANCED_SYSTEM_MONITOR_TAB,
)

from .fusion_status_flags import (
    FusionStatusFlagsModel,
    FusionStatusFlagsData,
    FUSION_STATUS_FLAGS,
)

from .baseline_plot import (
    BaselinePlotModel,
    BaselinePlotPoints,
    BASELINE_PLOT,
)

from .baseline_table import (
    BaselineTableEntries,
    BaselineTableModel,
    BASELINE_TABLE,
)

from .observation_tab import (
    ObservationTableModel,
    REMOTE_OBSERVATION_TAB,
    LOCAL_OBSERVATION_TAB,
    obs_rows_to_json,
)

from .settings_tab import (
    SettingsTabModel,
    SettingsTabData,
    SettingsTableEntries,
    SettingsTableModel,
    SETTINGS_TAB,
    SETTINGS_TABLE,
    settings_rows_to_json,
)

from .solution_position_tab import (
    SolutionPositionModel,
    SolutionPositionPoints,
    SOLUTION_POSITION_TAB,
)

from .solution_table import (
    SolutionTableEntries,
    SolutionTableModel,
    SOLUTION_TABLE,
)

from .solution_velocity_tab import (
    SolutionVelocityModel,
    SolutionVelocityPoints,
    SOLUTION_VELOCITY_TAB,
)

from .status_bar import (
    STATUS_BAR,
    StatusBarData,
    StatusBarModel,
)

from .tracking_signals_tab import (
    TrackingSignalsPoints,
    TRACKING_SIGNALS_TAB,
)

from .tracking_sky_plot_tab import (
    TrackingSkyPlotModel,
    TrackingSkyPlotPoints,
    TRACKING_SKY_PLOT_TAB,
)

from .update_tab import (
    UPDATE_TAB,
    UpdateTabData,
    UpdateTabModel,
)

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555


MAIN_INDEX = "MAIN_INDEX"
SUB_INDEX = "SUB_INDEX"

TAB_LAYOUT = {
    Tabs.TRACKING_SIGNALS: {
        MAIN_INDEX: 0,
        SUB_INDEX: 0,
    },
    Tabs.TRACKING_SKYPLOT: {
        MAIN_INDEX: 0,
        SUB_INDEX: 1,
    },
    Tabs.SOLUTION_POSITION: {
        MAIN_INDEX: 1,
        SUB_INDEX: 0,
    },
    Tabs.SOLUTION_VELOCITY: {
        MAIN_INDEX: 1,
        SUB_INDEX: 1,
    },
    Tabs.BASELINE: {
        MAIN_INDEX: 2,
        SUB_INDEX: 0,
    },
    Tabs.OBSERVATIONS: {
        MAIN_INDEX: 3,
        SUB_INDEX: 0,
    },
    Tabs.SETTINGS: {
        MAIN_INDEX: 4,
        SUB_INDEX: 0,
    },
    Tabs.UPDATE: {
        MAIN_INDEX: 5,
        SUB_INDEX: 0,
    },
    Tabs.ADVANCED_SYSTEM_MONITOR: {
        MAIN_INDEX: 6,
        SUB_INDEX: 0,
    },
    Tabs.ADVANCED_IMU: {
        MAIN_INDEX: 6,
        SUB_INDEX: 1,
    },
    Tabs.ADVANCED_MAGNETOMETER: {
        MAIN_INDEX: 6,
        SUB_INDEX: 2,
    },
    Tabs.ADVANCED_NETWORKING: {
        MAIN_INDEX: 6,
        SUB_INDEX: 3,
    },
    Tabs.ADVANCED_SPECTRUM_ANALYZER: {
        MAIN_INDEX: 6,
        SUB_INDEX: 4,
    },
}


capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        if not buffer:
            print("terminating GUI loop", file=sys.stderr)
            break
        Message = messages.Message
        m = Message.from_bytes(buffer)
        if m.which == Message.Union.Status:
            app_state = ConnectionState(m.status.text)
            if app_state == ConnectionState.CLOSED:
                return app_.quit()
            CONNECTION[Keys.CONNECTION_STATE] = app_state

        elif m.which == Message.Union.SolutionPositionStatus:
            SOLUTION_POSITION_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.data[idx]]
                for idx in range(len(m.solutionPositionStatus.data))
            ]
            SOLUTION_POSITION_TAB[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.curData[idx]]
                for idx in range(len(m.solutionPositionStatus.curData))
            ]
            SOLUTION_POSITION_TAB[Keys.LAT_MAX] = m.solutionPositionStatus.latMax
            SOLUTION_POSITION_TAB[Keys.LAT_MIN] = m.solutionPositionStatus.latMin
            SOLUTION_POSITION_TAB[Keys.LON_MAX] = m.solutionPositionStatus.lonMax
            SOLUTION_POSITION_TAB[Keys.LON_MIN] = m.solutionPositionStatus.lonMin
            SOLUTION_POSITION_TAB[Keys.AVAILABLE_UNITS][:] = m.solutionPositionStatus.availableUnits
        elif m.which == Message.Union.SolutionTableStatus:
            SOLUTION_TABLE[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.solutionTableStatus.data]
        elif m.which == Message.Union.SolutionVelocityStatus:
            SOLUTION_VELOCITY_TAB[Keys.COLORS][:] = m.solutionVelocityStatus.colors
            SOLUTION_VELOCITY_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionVelocityStatus.data[idx]]
                for idx in range(len(m.solutionVelocityStatus.data))
            ]
            SOLUTION_VELOCITY_TAB[Keys.MAX] = m.solutionVelocityStatus.max
            SOLUTION_VELOCITY_TAB[Keys.MIN] = m.solutionVelocityStatus.min
            SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS][:] = m.solutionVelocityStatus.availableUnits
        elif m.which == Message.Union.BaselinePlotStatus:
            BASELINE_PLOT[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.data[idx]]
                for idx in range(len(m.baselinePlotStatus.data))
            ]
            BASELINE_PLOT[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.curData[idx]]
                for idx in range(len(m.baselinePlotStatus.curData))
            ]
            BASELINE_PLOT[Keys.N_MAX] = m.baselinePlotStatus.nMax
            BASELINE_PLOT[Keys.N_MIN] = m.baselinePlotStatus.nMin
            BASELINE_PLOT[Keys.E_MAX] = m.baselinePlotStatus.eMax
            BASELINE_PLOT[Keys.E_MIN] = m.baselinePlotStatus.eMin
        elif m.which == Message.Union.BaselineTableStatus:
            BASELINE_TABLE[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.baselineTableStatus.data]
        elif m.which == Message.Union.AdvancedImuStatus:
            ADVANCED_IMU_TAB[Keys.FIELDS_DATA][:] = m.advancedImuStatus.fieldsData
            ADVANCED_IMU_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedImuStatus.data[idx]]
                for idx in range(len(m.advancedImuStatus.data))
            ]
        elif m.which == Message.Union.AdvancedSpectrumAnalyzerStatus:
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.CHANNEL] = m.advancedSpectrumAnalyzerStatus.channel
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.POINTS][:] = [
                QPointF(point.x, point.y) for point in m.advancedSpectrumAnalyzerStatus.data
            ]
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMAX] = m.advancedSpectrumAnalyzerStatus.ymax
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMIN] = m.advancedSpectrumAnalyzerStatus.ymin
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMAX] = m.advancedSpectrumAnalyzerStatus.xmax
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMIN] = m.advancedSpectrumAnalyzerStatus.xmin
        elif m.which == Message.Union.AdvancedNetworkingStatus:
            ADVANCED_NETWORKING_TAB[Keys.RUNNING] = m.advancedNetworkingStatus.running
            ADVANCED_NETWORKING_TAB[Keys.IP_ADDRESS] = m.advancedNetworkingStatus.ipAddress
            ADVANCED_NETWORKING_TAB[Keys.PORT] = m.advancedNetworkingStatus.port
            ADVANCED_NETWORKING_TAB[Keys.NETWORK_INFO][:] = [
                [entry.interfaceName, entry.ipv4Address, entry.running, entry.txUsage, entry.rxUsage]
                for entry in m.advancedNetworkingStatus.networkInfo
            ]
        elif m.which == Message.Union.AdvancedSystemMonitorStatus:
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.OBS_LATENCY][:] = [
                [entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsLatency
            ]
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.OBS_PERIOD][:] = [
                [entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsPeriod
            ]
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.THREADS_TABLE][:] = [
                [entry.name, entry.cpu, entry.stackFree] for entry in m.advancedSystemMonitorStatus.threadsTable
            ]
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.CSAC_TELEM_LIST][:] = [
                [entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.csacTelemList
            ]
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.CSAC_RECEIVED] = m.advancedSystemMonitorStatus.csacReceived
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.ZYNQ_TEMP] = m.advancedSystemMonitorStatus.zynqTemp
            ADVANCED_SYSTEM_MONITOR_TAB[Keys.FE_TEMP] = m.advancedSystemMonitorStatus.feTemp
        elif m.which == Message.Union.AdvancedMagnetometerStatus:
            ADVANCED_MAGNETOMETER_TAB[Keys.YMAX] = m.advancedMagnetometerStatus.ymax
            ADVANCED_MAGNETOMETER_TAB[Keys.YMIN] = m.advancedMagnetometerStatus.ymin
            ADVANCED_MAGNETOMETER_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedMagnetometerStatus.data[idx]]
                for idx in range(len(m.advancedMagnetometerStatus.data))
            ]
        elif m.which == Message.Union.FusionStatusFlagsStatus:
            FUSION_STATUS_FLAGS[Keys.GNSSPOS] = m.fusionStatusFlagsStatus.gnsspos
            FUSION_STATUS_FLAGS[Keys.GNSSVEL] = m.fusionStatusFlagsStatus.gnssvel
            FUSION_STATUS_FLAGS[Keys.WHEELTICKS] = m.fusionStatusFlagsStatus.wheelticks
            FUSION_STATUS_FLAGS[Keys.SPEED] = m.fusionStatusFlagsStatus.speed
            FUSION_STATUS_FLAGS[Keys.NHC] = m.fusionStatusFlagsStatus.nhc
            FUSION_STATUS_FLAGS[Keys.ZEROVEL] = m.fusionStatusFlagsStatus.zerovel
        elif m.which == Message.Union.TrackingSignalsStatus:
            TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS][:] = m.trackingSignalsStatus.checkLabels
            TRACKING_SIGNALS_TAB[Keys.LABELS][:] = m.trackingSignalsStatus.labels
            TRACKING_SIGNALS_TAB[Keys.COLORS][:] = m.trackingSignalsStatus.colors
            TRACKING_SIGNALS_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.trackingSignalsStatus.data[idx]]
                for idx in range(len(m.trackingSignalsStatus.data))
            ]
            TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET] = m.trackingSignalsStatus.xminOffset
        elif m.which == Message.Union.TrackingSkyPlotStatus:
            TRACKING_SKY_PLOT_TAB[Keys.SATS][:] = [
                [QPointF(point.az, point.el) for point in m.trackingSkyPlotStatus.sats[idx]]
                for idx in range(len(m.trackingSkyPlotStatus.sats))
            ]
            TRACKING_SKY_PLOT_TAB[Keys.LABELS][:] = [
                list(m.trackingSkyPlotStatus.labels[idx]) for idx in range(len(m.trackingSkyPlotStatus.labels))
            ]
        elif m.which == Message.Union.ObservationStatus:
            if m.observationStatus.isRemote:
                REMOTE_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                REMOTE_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                REMOTE_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
            else:
                LOCAL_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                LOCAL_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                LOCAL_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
        elif m.which == Message.Union.StatusBarStatus:
            STATUS_BAR[Keys.POS] = m.statusBarStatus.pos
            STATUS_BAR[Keys.RTK] = m.statusBarStatus.rtk
            STATUS_BAR[Keys.SATS] = m.statusBarStatus.sats
            STATUS_BAR[Keys.CORR_AGE] = m.statusBarStatus.corrAge
            STATUS_BAR[Keys.INS] = m.statusBarStatus.ins
            STATUS_BAR[Keys.DATA_RATE] = m.statusBarStatus.dataRate
            STATUS_BAR[Keys.SOLID_CONNECTION] = m.statusBarStatus.solidConnection
            STATUS_BAR[Keys.TITLE] = m.statusBarStatus.title
            STATUS_BAR[Keys.ANTENNA_STATUS] = m.statusBarStatus.antennaStatus
        elif m.which == Message.Union.ConnectionStatus:
            CONNECTION[Keys.AVAILABLE_PORTS][:] = m.connectionStatus.availablePorts
            CONNECTION[Keys.AVAILABLE_BAUDRATES][:] = m.connectionStatus.availableBaudrates
            CONNECTION[Keys.AVAILABLE_FLOWS][:] = m.connectionStatus.availableFlows
            CONNECTION[Keys.PREVIOUS_HOSTS][:] = m.connectionStatus.previousHosts
            CONNECTION[Keys.PREVIOUS_PORTS][:] = m.connectionStatus.previousPorts
            CONNECTION[Keys.PREVIOUS_FILES][:] = m.connectionStatus.previousFiles
            CONNECTION[Keys.LAST_USED_SERIAL_DEVICE] = (
                m.connectionStatus.lastSerialDevice.port
                if m.connectionStatus.lastSerialDevice.which() == "port"
                else None
            )
            CONNECTION[Keys.PREVIOUS_SERIAL_CONFIGS][:] = [
                [entry.device, entry.baudrate, entry.flowControl] for entry in m.connectionStatus.previousSerialConfigs
            ]
            CONNECTION[Keys.CONSOLE_VERSION] = m.connectionStatus.consoleVersion
        elif m.which == Message.Union.LoggingBarStatus:
            LOGGING_BAR[Keys.PREVIOUS_FOLDERS][:] = m.loggingBarStatus.previousFolders
            LOGGING_BAR[Keys.CSV_LOGGING] = m.loggingBarStatus.csvLogging
            LOGGING_BAR[Keys.SBP_LOGGING] = m.loggingBarStatus.sbpLogging
            LOGGING_BAR[Keys.SBP_LOGGING_FORMAT] = m.loggingBarStatus.sbpLoggingFormat
        elif m.which == Message.Union.LoggingBarRecordingStatus:
            LOGGING_BAR[Keys.RECORDING_DURATION_SEC] = m.loggingBarRecordingStatus.recordingDurationSec
            LOGGING_BAR[Keys.RECORDING_SIZE] = m.loggingBarRecordingStatus.recordingSize
            LOGGING_BAR[Keys.RECORDING_FILENAME] = (
                m.loggingBarRecordingStatus.recordingFilename.filename
                if m.loggingBarRecordingStatus.recordingFilename.which() == "filename"
                else ""
            )
        elif m.which == Message.Union.UpdateTabStatus:
            UPDATE_TAB[Keys.HARDWARE_REVISION] = m.updateTabStatus.hardwareRevision
            UPDATE_TAB[Keys.FW_VERSION_CURRENT] = m.updateTabStatus.fwVersionCurrent
            UPDATE_TAB[Keys.FW_VERSION_LATEST] = m.updateTabStatus.fwVersionLatest
            UPDATE_TAB[Keys.FW_LOCAL_FILENAME] = m.updateTabStatus.fwLocalFilename
            UPDATE_TAB[Keys.DIRECTORY] = m.updateTabStatus.directory
            UPDATE_TAB[Keys.DOWNLOADING] = m.updateTabStatus.downloading
            UPDATE_TAB[Keys.UPGRADING] = m.updateTabStatus.upgrading
            UPDATE_TAB[Keys.FW_TEXT] = m.updateTabStatus.fwText
            UPDATE_TAB[Keys.FILEIO_LOCAL_FILEPATH] = m.updateTabStatus.fileioLocalFilepath
            UPDATE_TAB[Keys.FILEIO_DESTINATION_FILEPATH] = m.updateTabStatus.fileioDestinationFilepath
            UPDATE_TAB[Keys.FW_OUTDATED] = m.updateTabStatus.fwOutdated
            UPDATE_TAB[Keys.FW_V2_OUTDATED] = m.updateTabStatus.fwV2Outdated
            UPDATE_TAB[Keys.SERIAL_PROMPT] = m.updateTabStatus.serialPrompt
            UPDATE_TAB[Keys.CONSOLE_OUTDATED] = m.updateTabStatus.consoleOutdated
            UPDATE_TAB[Keys.CONSOLE_VERSION_CURRENT] = m.updateTabStatus.consoleVersionCurrent
            UPDATE_TAB[Keys.CONSOLE_VERSION_LATEST] = m.updateTabStatus.consoleVersionLatest
        elif m.which == Message.Union.LogAppend:
            log_panel_lock.lock()
            LOG_PANEL[Keys.ENTRIES] += [entry.line for entry in m.logAppend.entries]
            log_panel_lock.unlock()
            LOG_PANEL[Keys.LOG_LEVEL] = m.logAppend.logLevel
        elif m.which == Message.Union.SettingsTableStatus:
            SETTINGS_TABLE[Keys.ENTRIES][:] = settings_rows_to_json(m.settingsTableStatus.data)
        elif m.which == Message.Union.SettingsImportResponse:
            SETTINGS_TAB[Keys.IMPORT_STATUS] = m.settingsImportResponse.status
        elif m.which == Message.Union.InsSettingsChangeResponse:
            SETTINGS_TAB[Keys.RECOMMENDED_INS_SETTINGS][:] = [
                [entry.settingName, entry.currentValue, entry.recommendedValue]
                for entry in m.insSettingsChangeResponse.recommendedSettings
            ]
            SETTINGS_TAB[Keys.NEW_INS_CONFIRMATON] = True
        else:
            pass


class DataModel(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    endpoint: console_backend.server.ServerEndpoint  # pylint: disable=no-member
    messages: Any

    def __init__(self, endpoint, messages):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages

    @Slot()  # type: ignore
    def connect(self) -> None:
        self.connect_tcp(PIKSI_HOST, PIKSI_PORT)

    @Slot(str)  # type: ignore
    def connect_file(self, filename: str) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.fileRequest = msg.init(Message.Union.FileRequest)
        msg.fileRequest.filename = str(filename)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int)  # type: ignore
    def connect_tcp(self, host: str, port: int) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.tcpRequest = msg.init(Message.Union.TcpRequest)
        msg.tcpRequest.host = str(host)
        msg.tcpRequest.port = int(port)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int, str)  # type: ignore
    def connect_serial(self, device: str, baudrate: int, flow_control: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRequest = msg.init(Message.Union.SerialRequest)
        msg.serialRequest.device = str(device)
        msg.serialRequest.baudrate = int(baudrate)
        msg.serialRequest.flowControl = str(flow_control)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def disconnect(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.disconnectRequest = msg.init(Message.Union.DisconnectRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def serial_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRefreshRequest = msg.init(Message.Union.SerialRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsRefreshRequest = msg.init(Message.Union.SettingsRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_reset_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsResetRequest = msg.init(Message.Union.SettingsResetRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_save_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsSaveRequest = msg.init(Message.Union.SettingsSaveRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_export_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsExportRequest = msg.init(Message.Union.SettingsExportRequest)
        msg.settingsExportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_import_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsImportRequest = msg.init(Message.Union.SettingsImportRequest)
        msg.settingsImportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, str, str)  # type: ignore
    def settings_write_request(self, group: str, name: str, value: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsWriteRequest = msg.init(Message.Union.SettingsWriteRequest)
        msg.settingsWriteRequest.group = group
        msg.settingsWriteRequest.name = name
        msg.settingsWriteRequest.value = value
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def reset_device(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.advancedSystemMonitorStatusFront = msg.init(Message.Union.AdvancedSystemMonitorStatusFront)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def confirm_ins_change(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.confirmInsChange = msg.init(Message.Union.ConfirmInsChange)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def tracking_signals_check_visibility(self, checks: List[str]) -> None:
        Message = self.messages.Message
        m = Message()
        m.trackingSignalsStatusFront = m.init(Message.Union.TrackingSignalsStatusFront)
        m.trackingSignalsStatusFront.trackingSignalsCheckVisibility = checks
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_velocity_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionVelocityStatusFront = m.init(Message.Union.SolutionVelocityStatusFront)
        m.solutionVelocityStatusFront.solutionVelocityUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(int)  # type: ignore
    def advanced_spectrum_analyzer_channel(self, channel: int) -> None:
        Message = self.messages.Message
        m = Message()
        m.advancedSpectrumAnalyzerStatusFront = m.init(Message.Union.AdvancedSpectrumAnalyzerStatusFront)
        m.advancedSpectrumAnalyzerStatusFront.channel = channel
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT)  # type: ignore
    def advanced_networking(
        self, buttons: list, all_messages_toggle: Optional[bool], ipv4_address: Optional[str], port: Optional[int]
    ) -> None:
        Message = self.messages.Message
        m = Message()
        m.advancedNetworkingStatusFront = m.init(Message.Union.AdvancedNetworkingStatusFront)
        m.advancedNetworkingStatusFront.refresh = buttons[0]
        m.advancedNetworkingStatusFront.start = buttons[1]
        m.advancedNetworkingStatusFront.stop = buttons[2]
        if all_messages_toggle is not None:
            m.advancedNetworkingStatusFront.allMessages.toggle = all_messages_toggle
        else:
            m.advancedNetworkingStatusFront.allMessages.none = None
        if ipv4_address is not None:
            m.advancedNetworkingStatusFront.ipv4Address.address = ipv4_address
        else:
            m.advancedNetworkingStatusFront.ipv4Address.none = None
        if port is not None:
            m.advancedNetworkingStatusFront.port.port = int(port)
        else:
            m.advancedNetworkingStatusFront.port.none = None
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_position_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusUnitFront = m.init(Message.Union.SolutionPositionStatusUnitFront)
        m.solutionPositionStatusUnitFront.solutionPositionUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def solution_position(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusButtonFront = m.init(Message.Union.SolutionPositionStatusButtonFront)
        m.solutionPositionStatusButtonFront.solutionPositionPause = buttons[0]
        m.solutionPositionStatusButtonFront.solutionPositionClear = buttons[1]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def baseline_plot(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.baselinePlotStatusButtonFront = m.init(Message.Union.BaselinePlotStatusButtonFront)
        m.baselinePlotStatusButtonFront.pause = buttons[0]
        m.baselinePlotStatusButtonFront.clear = buttons[1]
        m.baselinePlotStatusButtonFront.resetFilters = buttons[2]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT)  # type: ignore
    def update_tab(
        self,
        buttons: list,
        update_local_filepath: Optional[str],
        download_directory: Optional[str],
        fileio_local_filepath: Optional[str],
        fileio_destination_filepath: Optional[str],
        update_local_filename: Optional[str],
    ) -> None:
        Message = self.messages.Message
        m = Message()
        m.updateTabStatusFront = m.init(Message.Union.UpdateTabStatusFront)
        if update_local_filepath is not None:
            m.updateTabStatusFront.updateLocalFilepath.filepath = str(update_local_filepath)
        else:
            m.updateTabStatusFront.updateLocalFilepath.none = None

        if download_directory is not None:
            m.updateTabStatusFront.downloadDirectory.directory = str(download_directory)
        else:
            m.updateTabStatusFront.downloadDirectory.none = None
        if fileio_local_filepath is not None:
            m.updateTabStatusFront.fileioLocalFilepath.filepath = str(fileio_local_filepath)
        else:
            m.updateTabStatusFront.fileioLocalFilepath.none = None

        if fileio_destination_filepath is not None:
            m.updateTabStatusFront.fileioDestinationFilepath.filepath = str(fileio_destination_filepath)
        else:
            m.updateTabStatusFront.fileioDestinationFilepath.none = None

        if update_local_filename is not None:
            m.updateTabStatusFront.updateLocalFilename.filepath = str(update_local_filename)
        else:
            m.updateTabStatusFront.updateLocalFilename.none = None

        m.updateTabStatusFront.downloadLatestFirmware = buttons[0]
        m.updateTabStatusFront.updateFirmware = buttons[1]
        m.updateTabStatusFront.sendFileToDevice = buttons[2]
        m.updateTabStatusFront.serialPromptConfirm = buttons[3]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, str)  # type: ignore
    def logging_bar(self, buttons, directory) -> None:
        Message = self.messages.Message
        m = Message()
        m.loggingBarFront = m.init(Message.Union.LoggingBarFront)
        m.loggingBarFront.csvLogging = buttons[0]
        m.loggingBarFront.sbpLogging = buttons[1]
        m.loggingBarFront.sbpLoggingFormat = buttons[2]
        m.loggingBarFront.directory = directory
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def log_level(self, log_level) -> None:
        Message = self.messages.Message
        m = Message()
        m.logLevelFront = m.init(Message.Union.LogLevelFront)
        m.logLevelFront.logLevel = str(log_level)
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def auto_survey_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.autoSurveyRequest = msg.init(Message.Union.AutoSurveyRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)


def is_frozen() -> bool:
    """Check whether the application is frozen.

    Returns:
        bool: Whether the application is frozen.
    """
    me = os.path.dirname(sys.executable)
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "") != ""
    path_frozen = os.path.exists(os.path.join(me, ".frozen"))
    return var_frozen or path_frozen


def get_app_dir() -> str:
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "")
    if var_frozen != "":
        return var_frozen
    return os.path.dirname(sys.executable)


def get_capnp_path() -> str:
    """Get the path to the capnp file based on current installer.

    Returns:
        str: The path to the capnp file.
    """
    d = get_app_dir()
    path = ""
    if is_frozen():
        path = os.path.join(d, "resources/base", CONSOLE_BACKEND_CAPNP_PATH)
    else:
        path = os.path.join(
            os.path.dirname(os.path.dirname(__file__)), "src/main/resources/base", CONSOLE_BACKEND_CAPNP_PATH
        )
    return path


def handle_cli_arguments(args: argparse.Namespace, globals_: QObject):
    if args.show_fileio:
        globals_.setProperty("showFileio", True)  # type: ignore
    if args.no_opengl:
        globals_.setProperty("useOpenGL", False)  # type: ignore
    if args.refresh_rate is not None:
        globals_.setProperty("currentRefreshRate", args.refresh_rate)  # type: ignore
    if args.tab is not None:
        layout_idxs = TAB_LAYOUT[args.tab]
        globals_.setProperty("initialMainTabIndex", layout_idxs[MAIN_INDEX])  # type: ignore
        globals_.setProperty("initialSubTabIndex", layout_idxs[SUB_INDEX])  # type: ignore
    if args.show_csv_log:
        globals_.setProperty("showCsvLog", True)  # type: ignore
    if args.height:
        min_height = globals_.property("minimumHeight")  # type: ignore
        if args.height < min_height:
            print(
                f"WARNING: --height value: {args.height}, is less than minimum: {min_height}. Input will be ignored.",
                file=sys.stderr,
            )
        else:
            globals_.setProperty("height", args.height)  # type: ignore
    if args.width:
        min_width = globals_.property("minimumWidth")  # type: ignore
        if args.width < min_width:
            print(
                f"WARNING: --width value: {args.width}, is less than minimum: {min_width}. Input will be ignored.",
                file=sys.stderr,
            )
        else:
            globals_.setProperty("width", args.width)  # type: ignore


def main():
    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)
    parser.add_argument("--show-fileio", action="store_true")
    parser.add_argument("--no-opengl", action="store_false")
    parser.add_argument("--refresh-rate", type=int)
    parser.add_argument("--tab")
    parser.add_argument("--show-csv-log", action="store_true")
    parser.add_argument("--height", type=int)
    parser.add_argument("--width", type=int)

    args_main, _ = parser.parse_known_args()

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication(sys.argv)
    app.setOrganizationName(ApplicationMetadata.ORGANIZATION_NAME)
    app.setOrganizationDomain(ApplicationMetadata.ORGANIZATION_DOMAIN)
    app.setApplicationName(ApplicationMetadata.APPLICATION_NAME)
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Regular.ttf")
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Bold.ttf")

    qmlRegisterType(LogPanelData, "SwiftConsole", 1, 0, "LogPanelData")  # type: ignore
    qmlRegisterType(ConnectionData, "SwiftConsole", 1, 0, "ConnectionData")  # type: ignore
    qmlRegisterType(LoggingBarData, "SwiftConsole", 1, 0, "LoggingBarData")  # type: ignore
    qmlRegisterType(AdvancedImuPoints, "SwiftConsole", 1, 0, "AdvancedImuPoints")  # type: ignore
    qmlRegisterType(AdvancedMagnetometerPoints, "SwiftConsole", 1, 0, "AdvancedMagnetometerPoints")  # type: ignore
    qmlRegisterType(AdvancedNetworkingData, "SwiftConsole", 1, 0, "AdvancedNetworkingData")  # type: ignore
    qmlRegisterType(
        AdvancedSpectrumAnalyzerPoints, "SwiftConsole", 1, 0, "AdvancedSpectrumAnalyzerPoints"  # type: ignore
    )
    qmlRegisterType(AdvancedSystemMonitorData, "SwiftConsole", 1, 0, "AdvancedSystemMonitorData")  # type: ignore
    qmlRegisterType(FusionStatusFlagsData, "SwiftConsole", 1, 0, "FusionStatusFlagsData")  # type: ignore
    qmlRegisterType(BaselinePlotPoints, "SwiftConsole", 1, 0, "BaselinePlotPoints")  # type: ignore
    qmlRegisterType(BaselineTableEntries, "SwiftConsole", 1, 0, "BaselineTableEntries")  # type: ignore
    qmlRegisterType(SettingsTabData, "SwiftConsole", 1, 0, "SettingsTabData")  # type: ignore
    qmlRegisterType(SettingsTableEntries, "SwiftConsole", 1, 0, "SettingsTableEntries")  # type: ignore
    qmlRegisterType(SolutionPositionPoints, "SwiftConsole", 1, 0, "SolutionPositionPoints")  # type: ignore
    qmlRegisterType(SolutionTableEntries, "SwiftConsole", 1, 0, "SolutionTableEntries")  # type: ignore
    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
    qmlRegisterType(StatusBarData, "SwiftConsole", 1, 0, "StatusBarData")  # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore
    qmlRegisterType(TrackingSkyPlotPoints, "SwiftConsole", 1, 0, "TrackingSkyPlotPoints")  # type: ignore
    qmlRegisterType(ObservationTableModel, "SwiftConsole", 1, 0, "ObservationTableModel")  # type: ignore
    qmlRegisterType(UpdateTabData, "SwiftConsole", 1, 0, "UpdateTabData")  # type: ignore

    engine = QtQml.QQmlApplicationEngine()
    qml_object_created = [False]

    def handle_qml_load_errors(obj, _url):
        qml_object_created[0] = obj is not None

    engine.objectCreated.connect(handle_qml_load_errors)  # pylint: disable=no-member

    capnp_path = get_capnp_path()

    engine.addImportPath("PySide2")
    engine.load(QUrl("qrc:/view.qml"))
    if not qml_object_created[0]:
        return 1

    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member

    backend_main = console_backend.server.Server()  # pylint: disable=no-member
    endpoint_main = backend_main.start()

    data_model = DataModel(endpoint_main, messages_main)
    log_panel_model = LogPanelModel()
    connection_model = ConnectionModel()
    advanced_imu_model = AdvancedImuModel()
    advanced_magnetometer_model = AdvancedMagnetometerModel()
    advanced_networking_model = AdvancedNetworkingModel()
    advanced_spectrum_analyzer_model = AdvancedSpectrumAnalyzerModel()
    advanced_system_monitor_model = AdvancedSystemMonitorModel()
    fusion_engine_flags_model = FusionStatusFlagsModel()
    baseline_plot_model = BaselinePlotModel()
    baseline_table_model = BaselineTableModel()
    settings_tab_model = SettingsTabModel()
    settings_table_model = SettingsTableModel()
    solution_position_model = SolutionPositionModel()
    solution_table_model = SolutionTableModel()
    solution_velocity_model = SolutionVelocityModel()
    status_bar_model = StatusBarModel()
    logging_bar_model = LoggingBarModel()
    tracking_sky_plot_model = TrackingSkyPlotModel()
    update_tab_model = UpdateTabModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("log_panel_model", log_panel_model)
    root_context.setContextProperty("connection_model", connection_model)
    root_context.setContextProperty("advanced_imu_model", advanced_imu_model)
    root_context.setContextProperty("advanced_magnetometer_model", advanced_magnetometer_model)
    root_context.setContextProperty("advanced_networking_model", advanced_networking_model)
    root_context.setContextProperty("advanced_spectrum_analyzer_model", advanced_spectrum_analyzer_model)
    root_context.setContextProperty("advanced_system_monitor_model", advanced_system_monitor_model)
    root_context.setContextProperty("fusion_engine_flags_model", fusion_engine_flags_model)
    root_context.setContextProperty("baseline_plot_model", baseline_plot_model)
    root_context.setContextProperty("baseline_table_model", baseline_table_model)
    root_context.setContextProperty("settings_tab_model", settings_tab_model)
    root_context.setContextProperty("settings_table_model", settings_table_model)
    root_context.setContextProperty("solution_position_model", solution_position_model)
    root_context.setContextProperty("solution_table_model", solution_table_model)
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("status_bar_model", status_bar_model)
    root_context.setContextProperty("logging_bar_model", logging_bar_model)
    root_context.setContextProperty("tracking_sky_plot_model", tracking_sky_plot_model)
    root_context.setContextProperty("update_tab_model", update_tab_model)
    root_context.setContextProperty("data_model", data_model)

    # Unfortunately it is not possible to access singletons directly using the PySide2 API.
    # This approach stores the globals somwhere that can be grabbed and manipulated.
    component = QQmlComponent(engine)
    component.setData(
        b'import QtQuick 2.0\nimport "Constants"\nItem{ property var globals: Globals }',  # type: ignore
        QUrl("qrc:/grabGlobals.qml"),
    )
    globals_main = component.create()
    globals_main = globals_main.property("globals")  # type: ignore

    handle_cli_arguments(args_main, globals_main)

    server_thread = threading.Thread(
        target=receive_messages,
        args=(
            app,
            backend_main,
            messages_main,
        ),
        daemon=True,
    )

    server_thread.start()
    app.exec_()

    endpoint_main.shutdown()
    server_thread.join()

    return 0


if __name__ == "__main__":
    sys.exit(main())
