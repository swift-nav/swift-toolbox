import "../BaseComponents"
import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias observationConnectionMonitor: observationConnectionMonitor
    property alias deviceMonitor: deviceMonitorAndResetDevice.deviceMonitor

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

    }

}
