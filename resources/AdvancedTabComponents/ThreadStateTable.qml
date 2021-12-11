import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property variant columnWidths: [parent.width / 3, parent.width / 3, parent.width / 3]
    property real mouse_x: 0
    property variant entries: []

    HorizontalHeaderView {
        id: horizontalHeader

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
                        var next_idx = (index + 1) % 3;
                        var min_width = tableView.width / 6;
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

        anchors.top: horizontalHeader.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        columnWidths: parent.columnWidths

        model: TableModel {
            id: tableModel

            rows: [Constants.systemMonitor.defaultThreadsList]

            TableModelColumn {
                display: Constants.systemMonitor.columnHeaders[0]
            }

            TableModelColumn {
                display: Constants.systemMonitor.columnHeaders[1]
            }

            TableModelColumn {
                display: Constants.systemMonitor.columnHeaders[2]
            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!advancedTab.visible)
                return ;

            for (var idx in entries) {
                var new_row = {
                };
                new_row[Constants.systemMonitor.columnHeaders[0]] = entries[idx][0];
                new_row[Constants.systemMonitor.columnHeaders[1]] = entries[idx][1];
                new_row[Constants.systemMonitor.columnHeaders[2]] = entries[idx][2];
                tableView.model.setRow(idx, new_row);
            }
        }
    }

}
