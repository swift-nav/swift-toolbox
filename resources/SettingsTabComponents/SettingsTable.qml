import "../Constants"
import Qt.labs.qmlmodels
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property variant columnWidths: [Constants.settingsTable.maximumWidth * 0.4, Constants.settingsTable.maximumWidth * 0.6]
    property int selectedRowIdx: -1
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

    width: columnWidths[0] + columnWidths[1]
    height: parent.height

    SettingsTableEntries {
        id: settingsTableEntries
    }

    ColumnLayout {
        width: parent.width
        height: parent.height

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true

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

            TextEdit {
                id: textEdit

                visible: false
            }

            Shortcut {
                sequence: StandardKey.Copy
                onActivated: {
                    if (selectedRowIdx != -1) {
                        textEdit.text = JSON.stringify(tableView.model.getRow(selectedRowIdx));
                        textEdit.selectAll();
                        textEdit.copy();
                        selectedRowIdx = -1;
                    }
                }
            }

            TableView {
                id: tableView

                columnSpacing: -1
                rowSpacing: -1
                columnWidthProvider: function(column) {
                    return columnWidths[column];
                }
                reuseItems: true
                boundsBehavior: Flickable.StopAtBounds
                anchors.top: horizontalHeader.bottom
                width: parent.width
                height: parent.height - horizontalHeader.height

                ScrollBar.horizontal: ScrollBar {
                }

                ScrollBar.vertical: ScrollBar {
                }

                model: TableModel {
                    id: tableModel

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
                    border.color: Constants.genericTable.borderColor
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
                            if (selectedRowIdx == row)
                                selectedRowIdx = -1;
                            else
                                selectedRowIdx = row;
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
