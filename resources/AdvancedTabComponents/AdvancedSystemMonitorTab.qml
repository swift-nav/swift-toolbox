/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
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

        function update() {
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
}
