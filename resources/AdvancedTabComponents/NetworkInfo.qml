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
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

ColumnLayout {
    property variant entries: []
    property var columnWidths: [parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5]
    property real mouse_x: 0

    spacing: Constants.networking.layoutSpacing

    HorizontalHeaderView {
        id: horizontalHeader

        interactive: false
        syncView: tableView

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
                return;
            let networking = Constants.networking;
            let colHeader = networking.columnHeaders;
            for (var idx in entries) {
                var new_row = {};
                var entry = entries[idx];
                new_row[colHeader[0]] = entry[0];
                new_row[colHeader[1]] = entry[1];
                new_row[colHeader[2]] = entry[2];
                new_row[colHeader[3]] = entry[3];
                new_row[colHeader[4]] = entry[4];
                tableView.model.setRow(idx, new_row);
            }
        }
    }
}
