import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.14
import QtQuick.Controls 1.4
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    // property variant columnWidths: [Constants.baselineTable.defaultColumnWidth, Constants.baselineTable.defaultColumnWidth]

    id: baselineTable

    width: parent.width
    height: parent.height
    visible: false

    BaselineTableEntries {
        id: baselineTableEntries
    }

    Rectangle {
        id: baselineTableInner

        border.color: Constants.baselineTable.borderColor
        border.width: Constants.baselineTable.borderWidth
        width: parent.width
        height: parent.height

        ListModel {
            id: tableModel

            dynamicRoles: false
        }

        TableView {
            id: tableViewInner

            width: parent.width
            height: parent.height
            anchors.margins: Constants.baselineTable.surroundingMargin
            clip: true
            model: tableModel

            TableViewColumn {
                id: nonresizableColumn

                role: Constants.baselineTable.leftColumnHeader
                title: Constants.baselineTable.leftColumnHeader
                width: parent.width / 2
                horizontalAlignment: Text.AlignHCenter
            }

            TableViewColumn {
                id: resizableColumn

                role: Constants.baselineTable.rightColumnHeader
                title: Constants.baselineTable.rightColumnHeader
                width: parent.width / 2 - Constants.baselineTable.surroundingMargin
                horizontalAlignment: Text.AlignHCenter
            }

            itemDelegate: Item {
                Row {
                    id: row

                    width: parent.width

                    Rectangle {
                        width: parent.width
                        implicitHeight: Constants.baselineTable.cellHeight
                        border.width: Constants.baselineTable.borderWidth

                        Text {
                            width: parent.width
                            text: styleData.value
                            horizontalAlignment: Text.AlignLeft
                            leftPadding: Constants.baselineTable.leftPadding
                            font.pointSize: Constants.mediumPointSize
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

                baselineTable.visible = true;
                var entries = baselineTableEntries.entries;
                for (var idx in entries) {
                    var new_row = {
                    };
                    new_row[Constants.baselineTable.leftColumnHeader] = entries[idx][0];
                    new_row[Constants.baselineTable.rightColumnHeader] = entries[idx][1];
                    tableModel.set(idx, new_row);
                }
            }
        }

    }

}
