import "../Constants"
import "../TableComponents"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

ColumnLayout {
    id: observationTable

    property alias remote: observationTableModel.remote
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property font tableFont: Qt.font({
        "family": Constants.genericTable.fontFamily,
        "pointSize": Constants.largePointSize
    })
    property variant avgWidth: parent.width / 8
    property variant columnWidths: [parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 16, 3 * parent.width / 16]
    property variant columnNames: ["PRN", "Pseudorange (m)", "Carrier Phase (cycles)", "C/N0 (dB-Hz)", "Meas. Doppler (Hz)", "Comp. Doppler (Hz)", "Lock", "Flags"]
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

    ObservationTableModel {
        id: observationTableModel
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

            text: "TOW:"
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

    TableView {
        id: innerTable

        Layout.fillHeight: true
        Layout.fillWidth: true
        columnSpacing: -1
        rowSpacing: -1
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        columnWidthProvider: function(column) {
            return columnWidths[column];
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

    Rectangle {
        height: 1
        width: innerTable.width - 1
        color: "transparent"
        border.color: Constants.genericTable.borderColor
    }

}
