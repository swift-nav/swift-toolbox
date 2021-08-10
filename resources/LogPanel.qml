import "./Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import SwiftConsole 1.0

Item {
    property var logEntries: []
    property variant columnWidths: [parent.width * Constants.logPanel.defaultColumnWidthRatios[0], parent.width * Constants.logPanel.defaultColumnWidthRatios[1], parent.width * Constants.logPanel.defaultColumnWidthRatios[2]]
    property real mouse_x: 0
    property int selectedRow: -1
    property bool forceLayoutLock: false

    function syncColumnWidths() {
        let column_width_sum = columnWidths[0] + columnWidths[1] + columnWidths[2];
        if (column_width_sum != tableView.width) {
            let final_column_diff = tableView.width - column_width_sum;
            columnWidths[2] += final_column_diff;
        }
        tableView.forceLayout();
    }

    width: parent.width
    height: parent.height

    LogPanelData {
        id: logPanelData
    }

    Rectangle {
        anchors.fill: parent

        TextEdit {
            id: textEdit

            visible: false
        }

        Shortcut {
            sequence: StandardKey.Copy
            onActivated: {
                if (selectedRow != -1) {
                    textEdit.text = JSON.stringify(tableView.model.getRow(selectedRow));
                    textEdit.selectAll();
                    textEdit.copy();
                    selectedRow = -1;
                }
            }
        }

        HorizontalHeaderView {
            id: horizontalHeader

            interactive: false
            syncView: tableView
            anchors.top: parent.top
            z: Constants.genericTable.headerZOffset

            delegate: Rectangle {
                implicitWidth: columnWidths[index]
                implicitHeight: Constants.genericTable.cellHeight
                border.color: Constants.genericTable.borderColor

                Text {
                    width: parent.width
                    anchors.centerIn: parent
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                    text: tableView.model.columns[index].display
                    elide: Text.ElideRight
                    clip: true
                    font.family: Constants.genericTable.fontFamily
                }

                MouseArea {
                    width: Constants.genericTable.mouseAreaResizeWidth
                    height: parent.height
                    anchors.right: parent.right
                    cursorShape: Qt.SizeHorCursor
                    onPressed: {
                        mouse_x = mouseX;
                        forceLayoutLock = true;
                    }
                    onPositionChanged: {
                        if (pressed) {
                            if (index == 2)
                                return ;

                            var delta_x = (mouseX - mouse_x);
                            columnWidths[index] += delta_x;
                            syncColumnWidths();
                        }
                    }
                    onReleased: {
                        forceLayoutLock = false;
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

                Component.onCompleted: {
                    let row_init = {
                    };
                    row_init[Constants.logPanel.timestampHeader] = "";
                    row_init[Constants.logPanel.levelHeader] = "";
                    row_init[Constants.logPanel.msgHeader] = "";
                    tableView.model.setRow(0, row_init);
                }
                rows: []

                TableModelColumn {
                    display: Constants.logPanel.timestampHeader
                }

                TableModelColumn {
                    display: Constants.logPanel.levelHeader
                }

                TableModelColumn {
                    display: Constants.logPanel.msgHeader
                }

            }

            delegate: Rectangle {
                implicitHeight: Constants.logPanel.cellHeight
                implicitWidth: tableView.columnWidthProvider(column)
                color: row == selectedRow ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

                Text {
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

        Timer {
            interval: Globals.currentRefreshRate
            running: true
            repeat: true
            onTriggered: {
                log_panel_model.fill_data(logPanelData);
                if (!logPanelData.entries.length)
                    return ;

                if (forceLayoutLock)
                    return ;

                for (var idx in logPanelData.entries) {
                    var new_row = {
                    };
                    new_row[Constants.logPanel.timestampHeader] = logPanelData.entries[idx].timestamp;
                    new_row[Constants.logPanel.levelHeader] = logPanelData.entries[idx].level;
                    new_row[Constants.logPanel.msgHeader] = logPanelData.entries[idx].msg;
                    logEntries.unshift(new_row);
                }
                logEntries = logEntries.slice(0, Constants.logPanel.maxRows);
                for (var idx in logEntries) {
                    tableView.model.setRow(idx, logEntries[idx]);
                }
                if (logPanelData.entries.length && selectedRow != -1)
                    selectedRow += logPanelData.entries.length;

                logPanelData.entries = [];
                tableView.forceLayout();
            }
        }

    }

}
