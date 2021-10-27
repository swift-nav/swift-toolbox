import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    property real zynqTemp: 0
    property real feTemp: 0

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Constants.systemMonitor.obsTextMargins

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.systemMonitor.textHeight

            Label {
                text: "Device Monitor"
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.genericTable.borderWidth
            border.color: Constants.genericTable.borderColor

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Constants.systemMonitor.obsTextMargins

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.systemMonitor.textHeight
                    Layout.alignment: Qt.AlignRight

                    Label {
                        anchors.fill: parent
                        anchors.margins: Constants.systemMonitor.obsTextMargins
                        text: Constants.systemMonitor.zynqTempLabel + ": " + zynqTemp.toFixed(1) + Constants.systemMonitor.tempUnits
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.systemMonitor.textHeight

                    Label {
                        anchors.fill: parent
                        anchors.margins: Constants.systemMonitor.obsTextMargins
                        text: Constants.systemMonitor.feTempLabel + ": " + feTemp.toFixed(1) + Constants.systemMonitor.tempUnits
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

            }

        }

    }

}
