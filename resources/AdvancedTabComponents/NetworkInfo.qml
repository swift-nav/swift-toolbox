import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    property variant entries: []
    property var columnWidths: [parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5]
    property real mouse_x: 0

    spacing: Constants.networking.layoutSpacing

    HorizontalHeaderView {
        id: horizontalHeader

        Layout.fillWidth: true
        Layout.preferredHeight: Constants.genericTable.cellHeight
        interactive: false
        syncView: tableView
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
                text: tableView.model.columns[index].display
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
                        var next_idx = (index + 1) % 5;
                        var min_width = tableView.width / 10;
                        if (columnWidths[index] + delta_x > min_width && columnWidths[next_idx] - delta_x > min_width) {
                            columnWidths[index] += delta_x;
                            columnWidths[next_idx] -= delta_x;
                        }
                        tableView.forceLayout();
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
        id: tableView

        Layout.fillWidth: true
        Layout.fillHeight: true
        columnWidths: parent.columnWidths

        model: TableModel {
            rows: [Constants.networking.defaultList]

            TableModelColumn {
                display: Constants.networking.columnHeaders[0]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[1]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[2]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[3]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[4]
            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!advancedTab.visible || !entries.length)
                return ;

            for (var idx in entries) {
                var new_row = {
                };
                new_row[Constants.networking.columnHeaders[0]] = entries[idx][0];
                new_row[Constants.networking.columnHeaders[1]] = entries[idx][1];
                new_row[Constants.networking.columnHeaders[2]] = entries[idx][2];
                new_row[Constants.networking.columnHeaders[3]] = entries[idx][3];
                new_row[Constants.networking.columnHeaders[4]] = entries[idx][4];
                tableView.model.setRow(idx, new_row);
            }
        }
    }

}
