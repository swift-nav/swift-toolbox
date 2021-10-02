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

            Text {
                text: "Device Monitor"
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
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

                    Text {
                        anchors.fill: parent
                        anchors.margins: Constants.systemMonitor.obsTextMargins
                        text: Constants.systemMonitor.zynqTempLabel + ": " + zynqTemp.toFixed(1) + Constants.systemMonitor.tempUnits
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.systemMonitor.textHeight

                    Text {
                        anchors.fill: parent
                        anchors.margins: Constants.systemMonitor.obsTextMargins
                        text: Constants.systemMonitor.feTempLabel + ": " + feTemp.toFixed(1) + Constants.systemMonitor.tempUnits
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
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
