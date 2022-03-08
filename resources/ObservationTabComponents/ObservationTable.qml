import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    id: observationTable

    property var observationTableModel: null
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property variant avgWidth: parent.width / 8
    property variant columnWidths: [parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 16, 3 * parent.width / 16]
    property variant columnNames: ["PRN", "Pseudorange [m]", "Carrier Phase [cycles]", "C/N0 [dB-Hz]", "Meas. Doppler [Hz]", "Comp. Doppler [Hz]", "Lock", "Flags"]
    property variant columnAlignments: [Text.AlignLeft, Text.AlignRight, Text.AlignRight, Text.AlignRight, Text.AlignRight, Text.AlignRight, Text.AlignRight, Text.AlignLeft]
    property real mouse_x: 0

    function update() {
        observationTableModel.update();
    }

    spacing: 0
    onWidthChanged: {
        innerTable.forceLayout();
    }
    onHeightChanged: {
        innerTable.forceLayout();
    }

    RowLayout {
        id: innerStats

        property int textPadding: 3

        spacing: 3

        Label {
            id: weekLabel

            text: "Week:"
            padding: parent.textPadding
            ToolTip.text: "GPS Week Number (since 1980)"
        }

        Label {
            id: weekValue

            text: observationTableModel ? observationTableModel.week : ""
            padding: parent.textPadding
        }

        Label {
            id: towLabel

            text: "TOW [s]:"
            padding: parent.textPadding
            ToolTip.text: "GPS milliseconds in week"
        }

        Label {
            id: towValue

            text: observationTableModel ? observationTableModel.padFloat(observationTableModel.tow, 2) : ""
            padding: parent.textPadding
        }

        Label {
            id: totalLabel

            text: "Total:"
            padding: parent.textPadding
            ToolTip.text: "Total observation count"
        }

        Label {
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

    HorizontalHeaderView {
        id: horizontalHeader

        interactive: false
        syncView: innerTable
        z: Constants.genericTable.headerZOffset

        delegate: Rectangle {
            implicitWidth: columnWidths[index]
            implicitHeight: Constants.genericTable.cellHeight
            border.color: Constants.genericTable.borderColor

            Label {
                width: parent.width
                anchors.centerIn: parent
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                text: columnNames[index]
                elide: Text.ElideRight
                clip: true
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

            MouseArea {
                width: Constants.genericTable.mouseAreaResizeWidth
                height: parent.height
                anchors.right: parent.right
                cursorShape: Qt.SizeHorCursor
                enabled: index !== (columnNames.length - 1)
                visible: index !== (columnNames.length - 1)
                onPressed: {
                    mouse_x = mouseX;
                }
                onPositionChanged: {
                    if (pressed) {
                        var delta_x = (mouseX - mouse_x);
                        var next_idx = (index + 1) % 8;
                        var min_width = observationTable.width / 12;
                        if (columnWidths[index] + delta_x > min_width && columnWidths[next_idx] - delta_x > min_width) {
                            columnWidths[index] += delta_x;
                            columnWidths[next_idx] -= delta_x;
                        }
                        innerTable.forceLayout();
                    }
                }
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

    SwiftTableView {
        id: innerTable

        Layout.fillHeight: true
        Layout.fillWidth: true
        columnWidths: parent.columnWidths
        columnAlignments: parent.columnAlignments
        model: observationTableModel
    }

    Rectangle {
        height: 1
        width: innerTable.width - 1
        color: "transparent"
        border.color: Constants.genericTable.borderColor
    }

}
