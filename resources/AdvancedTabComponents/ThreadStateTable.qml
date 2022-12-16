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
import "../TableComponents"
import Qt.labs.qmlmodels
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property real mouse_x: 0
    property variant entries: []

    HorizontalHeaderView {
        id: horizontalHeader

        interactive: false
        syncView: tableView
        z: Constants.genericTable.headerZOffset

        delegate: Rectangle {
            implicitWidth: tableView.columnWidths[index]
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
                font.pixelSize: Constants.largePixelSize
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
                        if (tableView.columnWidths[index] + delta_x > min_width && tableView.columnWidths[next_idx] - delta_x > min_width) {
                            tableView.columnWidths[index] += delta_x;
                            tableView.columnWidths[next_idx] -= delta_x;
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
        columnWidths: [parent.width / 3, parent.width / 3, parent.width / 3]

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
                return;
            for (var idx in entries) {
                var new_row = {};
                new_row[Constants.systemMonitor.columnHeaders[0]] = entries[idx][0];
                new_row[Constants.systemMonitor.columnHeaders[1]] = entries[idx][1];
                new_row[Constants.systemMonitor.columnHeaders[2]] = entries[idx][2];
                tableView.model.setRow(idx, new_row);
            }
        }
    }
}
