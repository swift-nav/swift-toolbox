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
    id: baselineTable

    property real mouse_x: 0

    function syncColumnWidthsWithSplitView() {
        var oldcols = columnWidths.slice();
        columnWidths[0] = Math.max(columnWidths[0], Constants.baselineTable.defaultColumnWidth);
        let column_width_sum = columnWidths[0] + columnWidths[1];
        if (column_width_sum != tableView.width) {
            let final_column_diff = tableView.width - column_width_sum;
            columnWidths[1] += final_column_diff;
        }
        if (columnWidths != oldcols)
            tableView.forceLayout();
    }

    implicitWidth: Constants.solutionTable.minimumWidth

    BaselineTableEntries {
        id: baselineTableEntries

        function update() {
            baseline_table_model.fill_console_points(baselineTableEntries);
            if (!baselineTableEntries.entries.length)
                return;
            var entries = baselineTableEntries.entries;
            let baselineTable = Constants.baselineTable;
            for (var idx in entries) {
                var new_row = {};
                var entry = entries[idx];
                new_row[baselineTable.tableLeftColumnHeader] = entry[0];
                new_row[baselineTable.tableRightColumnHeader] = entry[1];
                tableView.model.setRow(idx, new_row);
            }
            tableView.forceLayout();
        }
    }

    ColumnLayout {
        id: baselineTableRowLayout

        property variant columnWidths: [parent.width * 0.4, parent.width * 0.6]

        spacing: Constants.baselineTable.tableHeaderTableDataTableSpacing
        width: parent.width
        height: parent.height

        HorizontalHeaderView {
            id: horizontalHeader

            interactive: false
            syncView: tableView
            Layout.fillWidth: true

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
                    enabled: index == 0
                    visible: index == 0
                    onPressed: {
                        mouse_x = mouseX;
                    }
                    onPositionChanged: {
                        if (pressed) {
                            var delta_x = (mouseX - mouse_x);
                            var next_idx = (index + 1) % 2;
                            var min_width = tableView.width / 4;
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

            Layout.fillWidth: true
            Layout.fillHeight: true
            columnWidths: parent.columnWidths

            model: TableModel {
                id: tableModel

                rows: [{
                        "Item": "",
                        "Value": ""
                    }]

                TableModelColumn {
                    display: "Item"
                }

                TableModelColumn {
                    display: "Value"
                }
            }
        }
    }
}
