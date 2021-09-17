import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedSystemMonitorTab

    AdvancedSystemMonitorData {
        id: advancedSystemMonitorData
    }

    GridLayout {
        id: gridLayout

        rows: Constants.systemMonitor.rows
        columns: Constants.systemMonitor.columns
        rowSpacing: Constants.systemMonitor.rowSpacing
        columnSpacing: Constants.systemMonitor.columnSpacing
        anchors.fill: parent

        ThreadStateTable {
            id: threadStateTable

            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.columnSpan: Constants.systemMonitor.columns
            Layout.rowSpan: Constants.systemMonitor.topRowSpan
            Layout.preferredHeight: Constants.systemMonitor.topRowSpan
            Layout.preferredWidth: Constants.systemMonitor.columns
        }

        ObservationConnectionMonitor {
            id: observationConnectionMonitor

            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.columnSpan: Constants.systemMonitor.observationConnectionMonitorColumnSpan
            Layout.rowSpan: Constants.systemMonitor.bottomRowSpan
            Layout.preferredHeight: Constants.systemMonitor.bottomRowSpan
            Layout.preferredWidth: Constants.systemMonitor.observationConnectionMonitorColumnSpan
        }

        DeviceMonitor {
            id: deviceMonitor

            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.columnSpan: Constants.systemMonitor.deviceMonitorColumnSpan
            Layout.rowSpan: Constants.systemMonitor.bottomRowSpan
            Layout.preferredHeight: Constants.systemMonitor.bottomRowSpan
            Layout.preferredWidth: Constants.systemMonitor.deviceMonitorColumnSpan
        }

        MetricsMonitor {
            id: metricsMonitor

            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.columnSpan: Constants.systemMonitor.metricsMonitorColumnSpan
            Layout.rowSpan: Constants.systemMonitor.bottomRowSpan
            Layout.preferredHeight: Constants.systemMonitor.bottomRowSpan
            Layout.preferredWidth: Constants.systemMonitor.metricsMonitorColumnSpan
        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!advancedTab.visible)
                return ;

            advanced_system_monitor_model.fill_console_points(advancedSystemMonitorData);
            if (!advancedSystemMonitorData.threads_table.length)
                return ;

            threadStateTable.entries = advancedSystemMonitorData.threads_table;
            observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[0][0]] = advancedSystemMonitorData.obs_period[0][1];
            observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[1][0]] = advancedSystemMonitorData.obs_period[1][1];
            observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[2][0]] = advancedSystemMonitorData.obs_period[2][1];
            observationConnectionMonitor.obsPeriod[advancedSystemMonitorData.obs_period[3][0]] = advancedSystemMonitorData.obs_period[3][1];
            observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[0][0]] = advancedSystemMonitorData.obs_latency[0][1];
            observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[1][0]] = advancedSystemMonitorData.obs_latency[1][1];
            observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[2][0]] = advancedSystemMonitorData.obs_latency[2][1];
            observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[3][0]] = advancedSystemMonitorData.obs_latency[3][1];
            observationConnectionMonitor.obsLatency[advancedSystemMonitorData.obs_latency[3][0]] = advancedSystemMonitorData.obs_latency[3][1];
            metricsMonitor.entries = advancedSystemMonitorData.csac_telem_list;
            metricsMonitor.csacReceived = advancedSystemMonitorData.csac_received;
            deviceMonitor.zynqTemp = advancedSystemMonitorData.zynq_temp;
            deviceMonitor.feTemp = advancedSystemMonitorData.fe_temp;
        }
    }

}
