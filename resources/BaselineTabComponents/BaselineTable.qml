import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: baselineTable

    property variant columnWidths: [Constants.baselineTable.defaultColumnWidth, Constants.baselineTable.defaultColumnWidth]
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

    width: parent.width
    height: parent.height

    BaselineTableEntries {
        id: baselineTableEntries
    }

    ColumnLayout {
        id: baselineTableRowLayout

        spacing: Constants.baselineTable.tableHeaderTableDataTableSpacing
        width: parent.width
        height: parent.height

        HorizontalHeaderView {
            id: horizontalHeader

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.genericTable.cellHeight
            interactive: false
            syncView: tableView

            delegate: Rectangle {
                implicitWidth: columnWidths[index]
                implicitHeight: Constants.genericTable.cellHeight
                border.color: Constants.genericTable.borderColor
                clip: true

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
                            columnWidths[index] += delta_x;
                            syncColumnWidthsWithSplitView();
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

            onWidthChanged: syncColumnWidthsWithSplitView()
            Layout.fillWidth: true
            Layout.fillHeight: true
            columnWidths: parent.parent.columnWidths

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

        Timer {
            interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                if (!baselineTab.visible)
                    return ;

                baseline_table_model.fill_console_points(baselineTableEntries);
                if (!baselineTableEntries.entries.length)
                    return ;

                var entries = baselineTableEntries.entries;
                for (var idx in entries) {
                    var new_row = {
                    };
                    new_row[Constants.baselineTable.tableLeftColumnHeader] = entries[idx][0];
                    new_row[Constants.baselineTable.tableRightColumnHeader] = entries[idx][1];
                    tableView.model.setRow(idx, new_row);
                }
                tableView.forceLayout();
            }
        }

    }

}
