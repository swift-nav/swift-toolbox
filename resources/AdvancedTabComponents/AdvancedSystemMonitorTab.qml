import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedSystemMonitorTab

    AdvancedSystemMonitorData {
        id: advancedSystemMonitorData
    }

    RowLayout {
        id: gridLayout

        anchors.fill: parent

        AdvancedSystemMonitorTabLeftPane {
            id: leftPane

            Layout.minimumWidth: parent.width / 2
            Layout.fillHeight: true
        }

        ThreadStateTable {
            id: threadStateTable

            Layout.fillHeight: true
            Layout.fillWidth: true
        }
    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: advancedSystemMonitorTab.visible
        repeat: true
        onTriggered: {
            if (!advancedTab.visible)
                return;
            advanced_system_monitor_model.fill_console_points(advancedSystemMonitorData);
            if (!advancedSystemMonitorData.threads_table.length)
                return;
            threadStateTable.entries = advancedSystemMonitorData.threads_table;
            leftPane.observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[0][0]] = advancedSystemMonitorData.obs_period[0][1];
            leftPane.observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[1][0]] = advancedSystemMonitorData.obs_period[1][1];
            leftPane.observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[2][0]] = advancedSystemMonitorData.obs_period[2][1];
            leftPane.observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[3][0]] = advancedSystemMonitorData.obs_period[3][1];
            leftPane.observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[0][0]] = advancedSystemMonitorData.obs_latency[0][1];
            leftPane.observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[1][0]] = advancedSystemMonitorData.obs_latency[1][1];
            leftPane.observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[2][0]] = advancedSystemMonitorData.obs_latency[2][1];
            leftPane.observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[3][0]] = advancedSystemMonitorData.obs_latency[3][1];
            leftPane.observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[3][0]] = advancedSystemMonitorData.obs_latency[3][1];
            leftPane.deviceMonitor.zynqTemp = advancedSystemMonitorData.zynq_temp;
            leftPane.deviceMonitor.feTemp = advancedSystemMonitorData.fe_temp;
        }
    }
}
