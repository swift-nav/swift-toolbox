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

    

    anchors.fill: parent
    width: parent.width
    height: parent.height


    SettingsTableEntries {
        id: settingsTableEntries
    }

    ColumnLayout {
        
        anchors.fill: parent
        width: parent.width
        height: parent.height
        spacing: Constants.settingsTable.layoutSpacing

        HorizontalHeaderView {

            id: horizontalHeader

            interactive: false
            syncView: tableView

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
                // MouseArea {
                //     width: Constants.genericTable.mouseAreaResizeWidth
                //     height: parent.height
                //     anchors.right: parent.right
                //     cursorShape: Qt.SizeHorCursor
                //     onPressed: {
                //         mouse_x = mouseX;
                //     }
                //     onPositionChanged: {
                //         if (pressed) {
                //             var delta_x = (mouseX - mouse_x);
                //             columnWidths[index] += delta_x;
                //             syncColumnWidthsWithSplitView();
                //         }
                //     }
                // }

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

            // function syncColumnWidthsWithSplitView() {
            //     var oldcols = columnWidths.slice();
            //     columnWidths[0] = Math.max(columnWidths[0], Constants.baselineTable.defaultColumnWidth);
            //     let column_width_sum = columnWidths[0] + columnWidths[1];
            //     if (column_width_sum != tableView.width) {
            //         let final_column_diff = tableView.width - column_width_sum;
            //         columnWidths[1] += final_column_diff;
            //     }
            //     if (columnWidths != oldcols)
            //         tableView.forceLayout();

            // }
            onWidthChanged: {
                print(this.x, this.y, this.width, this.height)

                print(parent.width)
                columnWidths = [parent.width * 0.4, parent.width * 0.6]
            }

            columnWidths: [parent.width * 0.4, parent.width * 0.6]

            Layout.fillWidth: true
            Layout.fillHeight: true

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
                color: {
                    var item = tableView.model.getRow(row);
                    if (item[Constants.settingsTable.tableRightColumnHeader] == "")
                        return Constants.genericTable.borderColor;

                    if (selectedRowIdx == row)
                        return Constants.genericTable.cellHighlightedColor;

                    return Constants.genericTable.cellColor;
                }

                Label {
                    width: parent.width
                    horizontalAlignment: Text.AlignLeft
                    verticalAlignment: Text.AlignVCenter
                    clip: true
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
                    font.bold: {
                        var item = tableView.model.getRow(row);
                        return item[Constants.settingsTable.tableRightColumnHeader] == "";
                    }
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
                            Globals.copyClipboard = JSON.stringify(tableView.model.getRow(_currentSelectedIndex));
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
                // print(tableView.columnWidths)
                settings_table_model.fill_console_points(settingsTableEntries);
                var entries = settingsTableEntries.entries;
                if (!entries.length) {
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
