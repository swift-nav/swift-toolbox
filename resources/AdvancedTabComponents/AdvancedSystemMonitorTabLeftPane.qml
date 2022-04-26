import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property alias observationConnectionMonitor: observationConnectionMonitor
    property alias deviceMonitor: deviceMonitorAndResetDevice.deviceMonitor
    property alias metricsMonitor: metricsMonitor

    ColumnLayout {
        id: gridLayout

        anchors.fill: parent

        DeviceMonitorAndResetDevice {
            id: deviceMonitorAndResetDevice

            Layout.fillWidth: true
            Layout.preferredHeight: parent.height * 0.25
        }

        ObservationConnectionMonitor {
            id: observationConnectionMonitor

            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        MetricsMonitor {
            id: metricsMonitor

            visible: false
            enabled: false
        }

    }

}
