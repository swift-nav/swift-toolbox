import "../Constants"
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
    property int selectedRow: -1

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

            TextEdit {
                id: textEdit

                visible: false
            }

            Shortcut {
                sequences: [StandardKey.Copy]
                onActivated: {
                    if (selectedRow != -1) {
                        textEdit.text = JSON.stringify(tableView.model.getRow(selectedRow));
                        textEdit.selectAll();
                        textEdit.copy();
                        selectedRow = -1;
                    }
                }
            }

            TableView {
                id: tableView

                onWidthChanged: syncColumnWidthsWithSplitView()
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

                delegate: Rectangle {
                    implicitHeight: Constants.genericTable.cellHeight
                    implicitWidth: tableView.columnWidthProvider(column)
                    border.color: Constants.genericTable.borderColor
                    color: row == selectedRow ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

                    Label {
                        width: parent.width
                        horizontalAlignment: Text.AlignLeft
                        clip: true
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        text: model.display
                        elide: Text.ElideRight
                        padding: Constants.genericTable.padding
                    }

                    MouseArea {
                        width: parent.width
                        height: parent.height
                        anchors.centerIn: parent
                        onPressed: {
                            if (selectedRow == row)
                                selectedRow = -1;
                            else
                                selectedRow = row;
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
