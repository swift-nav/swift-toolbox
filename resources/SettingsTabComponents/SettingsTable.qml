import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property variant columnWidths: [Constants.settingsTable.maximumWidth * 0.4, Constants.settingsTable.maximumWidth * 0.6]
    property alias selectedRowIdx: tableView.selectedRow
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
        // Item {
        //     Layout.fillHeight: true
        //     Layout.fillWidth: true

        width: parent.width
        height: parent.height
        spacing: Constants.settingsTable.layoutSpacing

        HorizontalHeaderView {
            // clip: true
            // clip: true
            // z: 100

            id: horizontalHeader

            interactive: false
            syncView: tableView
            Layout.fillWidth: true
            Layout.minimumHeight: Constants.genericTable.cellHeight
            z: 100

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
                    // clip: true
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

        SwiftTableView {
            id: tableView

            Layout.fillWidth: true
            Layout.fillHeight: true
            columnWidths: parent.parent.columnWidths

            model: TableModel {
                id: tableModel

                TableModelColumn {
                    display: "Name"
                }

                TableModelColumn {
                    display: "Value"
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
