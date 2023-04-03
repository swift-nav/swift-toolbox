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
import "../BaseComponents"
import Qt.labs.qmlmodels
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Rectangle {
    property alias selectedRowIdx: tableView._currentSelectedIndex
    property var rowOffsets: ({})
    property bool showExpert: false
    property bool lastShowExpert: false
    property alias table: settingsTableEntries.entries
    property bool settingsHealthy: false
    property real mouse_x: 0
    property int curRowLength: 0
    property int lastEntriesLen: 0

    function clearTableModel() {
        tableView.model.clear();
    }

    function loaderVisibility(value) {
        refreshLoader.visible = value;
    }

    function isHeader(entry) {
        return !entry.hasOwnProperty("name");
    }

    function groupHasNonExpertSetting(entries, entryIdx) {
        for (var idx = entryIdx + 1; idx < entries.length; idx++) {
            let entry = entries[idx];
            if (isHeader(entry))
                return false;
            if (!entry.expert)
                return true;
        }
        return false;
    }

    function row(entry) {
        let settingsTable = Constants.settingsTable;
        return {
            [settingsTable.tableLeftColumnHeader]: entry.name.replace(/_/g, " "),
            [settingsTable.tableRightColumnHeader]: entry.valueOnDevice || "---"
        };
    }

    function headerRow(entry) {
        let settingsTable = Constants.settingsTable;
        return {
            [settingsTable.tableLeftColumnHeader]: entry.group.replace(/_/g, " "),
            [settingsTable.tableRightColumnHeader]: ""
        };
    }

    function isHeaderRow(row) {
        var item = tableView.model.getRow(row);
        return (item[Constants.settingsTable.tableRightColumnHeader] == "");
    }

    onVisibleChanged: {
        if (!visible)
            selectedRowIdx = -1;
    }
    Keys.onUpPressed: {
        let cellDecrease = 1;
        let new_row = selectedRowIdx - 1;
        if (new_row > -1 && isHeaderRow(new_row)) {
            cellDecrease += 1;
            new_row -= 1;
        }
        selectedRowIdx = (new_row <= -1) ? (tableView.rows - 1) : new_row;
        if (selectedRowIdx == tableView.rows - 1) {
            tableView.verticalScrollBar.position = 1 - tableView.verticalScrollBar.size;
        } else {
            let proposed_position = tableView.verticalScrollBar.position - cellDecrease / tableView.rows;
            let min_position = 0;
            tableView.verticalScrollBar.position = Math.max(proposed_position, min_position);
        }
    }
    Keys.onDownPressed: {
        let cellIncrease = 1;
        let new_row = selectedRowIdx + 1;
        if (new_row < tableView.rows && isHeaderRow(new_row)) {
            cellIncrease += 1;
            new_row += 1;
        }
        selectedRowIdx = (new_row >= (tableView.rows)) ? 1 : new_row;
        if (selectedRowIdx == 1) {
            tableView.verticalScrollBar.position = 0;
        } else {
            let proposed_position = tableView.verticalScrollBar.position + cellIncrease / tableView.rows;
            let max_position = 1 - tableView.verticalScrollBar.size;
            tableView.verticalScrollBar.position = Math.min(proposed_position, max_position);
        }
    }

    SettingsTableEntries {
        id: settingsTableEntries

        function update() {
            settings_table_model.fill_table_entries(settingsTableEntries);
            var entries = settingsTableEntries.entries;
            if (!entries.length) {
                settingsHealthy = false;
                return;
            }
            refreshLoader.visible = false;
            settingsHealthy = true;
            if (lastShowExpert != showExpert) {
                tableView._currentSelectedIndex = -1;
                tableView.model.clear();
                rowOffsets = {};
                lastShowExpert = showExpert;
            }
            if (entries.length != lastEntriesLen) {
                lastEntriesLen = entries.length;
                tableView.model.clear();
            }
            var offset = 0;
            entries.forEach((entry, idx, entries) => {
                    var new_row;
                    if (!isHeader(entry)) {
                        if (!showExpert && entry.expert === true) {
                            offset++;
                            return;
                        }
                        new_row = row(entry);
                    } else {
                        if (!showExpert && !groupHasNonExpertSetting(entries, idx)) {
                            offset++;
                            return;
                        }
                        new_row = headerRow(entry);
                    }
                    rowOffsets[idx - offset] = idx;
                    tableView.model.setRow(idx - offset, new_row);
                });
            tableView.forceLayout();
        }
    }

    ColumnLayout {
        property variant columnWidths: [parent.width * 0.45, parent.width * 0.55]

        anchors.fill: parent
        spacing: Constants.settingsTable.layoutSpacing

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

        SwiftLoader {
            id: refreshLoader
            visible: true
        }

        SwiftTableView {
            id: tableView

            columnWidths: parent.columnWidths
            Layout.fillWidth: true
            Layout.fillHeight: true
            stayFocused: true
            horizontalScrollBar.visible: false

            model: TableModel {
                id: tableModel

                rows: []

                TableModelColumn {
                    display: "Name"
                }

                TableModelColumn {
                    display: "Value"
                }
            }

            delegate: Rectangle {
                implicitHeight: Constants.genericTable.cellHeight
                implicitWidth: tableView.columnWidthProvider(column)
                border.color: tableView.delegateBorderColor
                border.width: tableView.delegateBorderWidth
                color: isHeaderRow(row) ? Constants.genericTable.borderColor : selectedRowIdx == row ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

                Label {
                    width: parent.width
                    height: parent.height
                    horizontalAlignment: Text.AlignLeft
                    verticalAlignment: Text.AlignVCenter
                    clip: true
                    font.family: Constants.genericTable.fontFamily
                    font.pixelSize: isHeaderRow(row) ? Constants.xlPixelSize : Constants.largePixelSize
                    font.bold: isHeaderRow(row)
                    text: model.display
                    elide: Text.ElideRight
                    leftPadding: Constants.genericTable.leftPadding
                    rightPadding: Constants.genericTable.rightPadding
                }

                MouseArea {
                    width: parent.width
                    height: parent.height
                    anchors.centerIn: parent
                    onPressed: {
                        tableView.focus = true;
                        if (tableView._currentSelectedIndex == row) {
                            Globals.currentSelectedTable = null;
                            tableView._currentSelectedIndex = -1;
                        } else {
                            Globals.currentSelectedTable = tableView;
                            tableView._currentSelectedIndex = row;
                            Globals.copyClipboard = JSON.stringify(tableView.model.getRow(tableView._currentSelectedIndex));
                        }
                    }
                }
            }
        }
    }
}
