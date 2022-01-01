import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property alias selectedRowIdx: tableView._currentSelectedIndex
    property var rowOffsets: ({
    })
    property bool showExpert: false
    property bool lastShowExpert: false
    property alias table: settingsTableEntries.entries
    property real mouse_x: 0

    function isHeader(entry) {
        return !entry.hasOwnProperty("name");
    }

    function groupHasNonExpertSetting(entries, entryIdx) {
        for (var idx = entryIdx + 1; idx < entries.length; idx++) {
            if (isHeader(entries[idx]))
                return false;

            if (!entries[idx].expert)
                return true;

        }
        return false;
    }

    function row(entry) {
        return {
            [Constants.settingsTable.tableLeftColumnHeader]: entry.name,
            [Constants.settingsTable.tableRightColumnHeader]: entry.valueOnDevice || "---"
        };
    }

    function headerRow(entry) {
        return {
            [Constants.settingsTable.tableLeftColumnHeader]: entry.group,
            [Constants.settingsTable.tableRightColumnHeader]: ""
        };
    }

    function isHeaderRow(row) {
        var item = tableView.model.getRow(row);
        return (item[Constants.settingsTable.tableRightColumnHeader] == "");
    }

    SettingsTableEntries {
        id: settingsTableEntries
    }

    ColumnLayout {
        property variant columnWidths: [parent.width * 0.45, parent.width * 0.55]

        width: parent.width
        height: parent.height
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
                    font.pointSize: Constants.largePointSize
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

            onVisibleChanged: {
                tableView.model.clear();
            }
            columnWidths: parent.columnWidths
            Layout.fillWidth: true
            Layout.fillHeight: true
            stayFocused: true

            model: TableModel {
                id: tableModel

                rows: [{
                    "Name": "",
                    "Value": ""
                }]

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
                    font.pointSize: isHeaderRow(row) ? Constants.xlPointSize : Constants.largePointSize
                    font.bold: isHeaderRow(row)
                    text: model.display
                    elide: Text.ElideRight
                    padding: Constants.genericTable.padding
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

        Timer {
            interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                settings_table_model.fill_console_points(settingsTableEntries);
                var entries = settingsTableEntries.entries;
                if (!entries.length) {
                    selectedRowIdx = -1;
                    tableView.model.clear();
                    tableView.forceLayout();
                    return ;
                }
                if (lastShowExpert != showExpert) {
                    tableView.model.clear();
                    rowOffsets = {
                    };
                    lastShowExpert = showExpert;
                }
                var offset = 0;
                entries.forEach((entry, idx, entries) => {
                    var new_row;
                    if (!isHeader(entry)) {
                        if (showExpert || entry.expert === false) {
                            new_row = row(entry);
                        } else {
                            offset++;
                            return ;
                        }
                    } else {
                        if (showExpert || groupHasNonExpertSetting(entries, idx)) {
                            new_row = headerRow(entry);
                        } else {
                            offset++;
                            return ;
                        }
                    }
                    rowOffsets[idx - offset] = idx;
                    tableView.model.setRow(idx - offset, new_row);
                });
                tableView.forceLayout();
            }
        }

    }

}
