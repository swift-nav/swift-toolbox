import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtQml.Models 2.15
import QtQuick 2.14
import QtQuick.Controls 1.4
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionTable

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
            id: solutionTableElement

            spacing: Constants.solutionTable.tableHeaderTableDataTableSpacing
            width: parent.width
            height: parent.height

            ListModel {
                id: myModel
            }

            TableView {
                id: solutionTableElementHeaders

                Layout.minimumHeight: parent.height
                Layout.fillWidth: true
                Layout.leftMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.rightMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.bottomMargin: Constants.solutionTable.tableSurroundingMargin
                Layout.topMargin: Constants.solutionTable.tableSurroundingMargin
                clip: true
                model: myModel

                TableViewColumn {
                    id: nonresizableColumn

                    role: Constants.solutionTable.tableLeftColumnHeader
                    title: Constants.solutionTable.tableLeftColumnHeader
                    width: Constants.solutionTable.defaultColumnWidth
                    horizontalAlignment: Text.AlignHCenter
                }

                TableViewColumn {
                    id: resizableColumn

                    role: Constants.solutionTable.tableRightColumnHeader
                    title: Constants.solutionTable.tableRightColumnHeader
                    width: parent.width - nonresizableColumn.width
                    horizontalAlignment: Text.AlignHCenter
                }

                itemDelegate: Item {
                    Row {
                        id: row

                        width: parent.width

                        Rectangle {
                            width: parent.width
                            implicitHeight: Constants.solutionTable.tableCellHeight
                            border.width: Constants.solutionTable.tableBorderWidth

                            Text {
                                width: parent.width
                                text: styleData.value
                                horizontalAlignment: Text.AlignLeft
                                leftPadding: Constants.solutionTable.tableLeftPadding
                            }

                        }

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
            interval: Constants.currentRefreshRate
            running: true
            repeat: true
            onTriggered: {
                if (!solutionTab.visible)
                    return ;

                solution_table_model.fill_console_points(solutionTableEntries);
                if (!solutionTableEntries.entries.length)
                    return ;

                var entries = solutionTableEntries.entries;
                for (var idx in entries) {
                    var new_row = {
                    };
                    new_row[Constants.solutionTable.tableLeftColumnHeader] = entries[idx][0];
                    new_row[Constants.solutionTable.tableRightColumnHeader] = entries[idx][1];
                    myModel.set(idx, new_row);
                }
            }
        }

    }

}
