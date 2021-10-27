import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    id: observationTable

    property alias remote: observationTableModel.remote
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property font tableFont: Qt.font({
        "family": Constants.genericTable.fontFamily,
        "pointSize": Constants.largePointSize
    })
    property variant avgWidth: parent.width / 8
    property variant columnWidths: [parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8]
    property variant columnNames: ["PRN", "Pseudorange (m)", "Carrier Phase (cycles)", "C/N0 (dB-Hz)", "Meas. Doppler (Hz)", "Comp. Doppler (Hz)", "Lock", "Flags"]
    property real mouse_x: 0

    function update() {
        observationTableModel.update();
    }

    spacing: 0

    ObservationTableModel {
        id: observationTableModel
    }

    RowLayout {
        id: innerStats

        property int textPadding: 3

        spacing: 3

        Text {
            id: weekLabel

            text: "Week:"
            padding: parent.textPadding
            ToolTip.text: "GPS Week Number (since 1980)"
        }

        Text {
            id: weekValue

            text: observationTableModel ? observationTableModel.week : ""
            padding: parent.textPadding
        }

        Text {
            id: towLabel

            text: "TOW:"
            padding: parent.textPadding
            ToolTip.text: "GPS milliseconds in week"
        }

        Text {
            id: towValue

            text: observationTableModel ? observationTableModel.padFloat(observationTableModel.tow, 2) : ""
            padding: parent.textPadding
        }

        Text {
            id: totalLabel

            text: "Total:"
            padding: parent.textPadding
            ToolTip.text: "Total observation count"
        }

        Text {
            id: totalValue

            text: observationTableModel ? observationTableModel.row_count : ""
            padding: parent.textPadding
        }

    }

    RowLayout {
        spacing: 3

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.gps_codes : 0
        }

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.glo_codes : 0
        }

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.bds_codes : 0
        }

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.gal_codes : 0
        }

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.qzs_codes : 0
        }

        ObservationFilterColumn {
            codes: observationTableModel ? observationTableModel.sbas_codes : 0
        }

    }

    Item {
        id: headerItem

        Layout.fillWidth: true
        implicitHeight: header.implicitHeight
        clip: true

        Row {
            id: header

            Layout.fillWidth: true
            x: -innerTable.contentX
            z: 1
            spacing: innerTable.columnSpacing

            Repeater {
                id: headerRepeater

                model: observationTableModel ? observationTableModel.columnCount() : 0

                Rectangle {
                    implicitWidth: columnWidths[index]
                    implicitHeight: Constants.genericTable.cellHeight
                    border.color: Constants.genericTable.borderColor

                    Text {
                        width: parent.width
                        anchors.centerIn: parent
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: columnNames[index]
                        elide: Text.ElideRight
                        clip: true
                        font.family: Constants.genericTable.fontFamily
                    }

                    gradient: Gradient {
                        GradientStop {
                            position: 0
                            color: Constants.genericTable.cellColor
                        }

                        GradientStop {
                            position: 1
                            color: Constants.genericTable.gradientColor
                        }

                    }

                }

            }

        }

    }

    Rectangle {
        id: tableRect

        height: 2000 // This is the only way I've been able to make it grow to take up the whole space
        Layout.fillHeight: true
        width: observationTable.width
        Layout.fillWidth: true
        color: "white"
        border.color: "#dcdcdc"
        border.width: 1

        TableView {
            id: innerTable

            anchors.top: tableRect.top
            Layout.fillHeight: true
            Layout.fillWidth: true
            width: observationTable.width
            height: tableRect.height
            rowSpacing: -1
            clip: true
            boundsBehavior: Flickable.StopAtBounds
            columnWidthProvider: function(column) {
                return columnWidths[column];
            }
            onWidthChanged: {
                forceLayout();
            }
            model: observationTableModel

            delegate: TableCellDelegate {
                implicitHeight: Constants.genericTable.cellHeight
                font: tableFont
            }

            ScrollBar.horizontal: ScrollBar {
            }

            ScrollBar.vertical: ScrollBar {
            }

        }

    }

    Rectangle {
        height: 1
        width: innerTable.width - 1
        color: "transparent"
        border.color: Constants.genericTable.borderColor
    }

}
