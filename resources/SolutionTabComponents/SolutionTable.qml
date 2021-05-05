import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.14
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionTable

    property variant columnWidths: [Constants.solutionTable.defaultColumnWidth, Constants.solutionTable.defaultColumnWidth]

    width: parent.width
    height: parent.height

    SolutionTableEntries {
        id: solutionTableEntries
    }

    Rectangle {
        id: solutionTableInner

        border.color: Constants.solutionTable.tableBorderColor
        border.width: Constants.solutionTable.tableBorderWidth
        width: parent.width
        height: parent.height

        ColumnLayout {
            id: solutionTableRowLayout

            spacing: Constants.solutionTable.tableHeaderTableDataTableSpacing
            width: parent.width
            height: parent.height

            TableView {
                id: solutionTableElementHeaders

                interactive: false
                Layout.minimumHeight: Constants.solutionTable.tableCellHeight
                Layout.fillWidth: true
                Layout.leftMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.rightMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.bottomMargin: Constants.solutionTable.tableInnerMargin
                Layout.topMargin: Constants.solutionTable.tableSurroundingMargin
                columnSpacing: Constants.solutionTable.tableCellSpacing
                rowSpacing: Constants.solutionTable.tableCellSpacing
                clip: true
                columnWidthProvider: function(column) {
                    return columnWidths[column];
                }

                model: TableModel {
                    rows: []

                    TableModelColumn {
                        display: Constants.solutionTable.tableLeftColumnHeader
                    }

                    TableModelColumn {
                        display: Constants.solutionTable.tableRightColumnHeader
                    }

                }

                delegate: Rectangle {
                    id: textDelegate

                    implicitHeight: Constants.solutionTable.tableCellHeight
                    border.width: Constants.solutionTable.tableBorderWidth

                    Text {
                        id: rowText

                        text: display
                        anchors.centerIn: parent
                        leftPadding: Constants.solutionTable.tableLeftPadding
                    }

                }

            }

            TableView {
                id: solutionTableElement

                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.leftMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.rightMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.bottomMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.topMargin: Constants.solutionTable.tableInnerMargin
                columnSpacing: Constants.solutionTable.tableCellSpacing
                rowSpacing: Constants.solutionTable.tableCellSpacing
                interactive: true
                clip: true
                columnWidthProvider: function(column) {
                    return columnWidths[column];
                }

                model: TableModel {
                    rows: []

                    TableModelColumn {
                        display: Constants.solutionTable.tableLeftColumnHeader
                    }

                    TableModelColumn {
                        display: Constants.solutionTable.tableRightColumnHeader
                    }

                }

                delegate: Rectangle {
                    id: textDelegateEle

                    implicitHeight: Constants.solutionTable.tableCellHeight
                    border.width: Constants.solutionTable.tableBorderWidth

                    Text {
                        id: rowTextEle

                        text: display
                        leftPadding: Constants.solutionTable.tableLeftPadding
                    }

                }

            }

            Rectangle {
                id: solutionRTKNote

                Layout.minimumHeight: Constants.solutionTable.rtkNoteHeight
                Layout.fillWidth: true
                width: parent.width
                Layout.margins: Constants.solutionTable.rtkNoteMargins
                Layout.alignment: Qt.AlignLeft | Qt.AlignBottom
                border.width: Constants.solutionTable.rtkNoteBorderWidth

                Text {
                    wrapMode: Text.Wrap
                    anchors.fill: parent
                    text: Constants.solutionTable.rtkNoteText
                }

            }

        }

        Timer {
            interval: Utils.hzToMilliseconds(Constants.currentRefreshRate)
            running: true
            repeat: true
            onTriggered: {
                if (!solutionTab.visible)
                    return ;

                solution_table_model.fill_console_points(solutionTableEntries);
                if (!solutionTableEntries.entries.length)
                    return ;

                if (solutionTableElementHeaders.model.rows.length == 0) {
                    var new_row = {
                    };
                    new_row[Constants.solutionTable.tableLeftColumnHeader] = Constants.solutionTable.tableLeftColumnHeader;
                    new_row[Constants.solutionTable.tableRightColumnHeader] = Constants.solutionTable.tableRightColumnHeader;
                    solutionTableElementHeaders.model.rows = [new_row];
                }
                var entries = solutionTableEntries.entries;
                var table_update = [];
                for (var idx in entries) {
                    var new_row = {
                    };
                    new_row[Constants.solutionTable.tableLeftColumnHeader] = entries[idx][0];
                    new_row[Constants.solutionTable.tableRightColumnHeader] = entries[idx][1];
                    table_update.push(new_row);
                }
                solutionTableElement.model.rows = table_update;
                var new_width = solutionTableArea.width - Constants.solutionTable.defaultColumnWidth;
                if (columnWidths[1] != new_width) {
                    columnWidths[1] = solutionTableArea.width - Constants.solutionTable.defaultColumnWidth;
                    solutionTableElement.forceLayout();
                    solutionTableElementHeaders.forceLayout();
                }
            }
        }

    }

}
