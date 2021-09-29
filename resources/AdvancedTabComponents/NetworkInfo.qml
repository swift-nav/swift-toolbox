import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    
    property variant entries: []

    property var columnWidths: [parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5, parent.width / 5]//[40,40,40,40,40] 
    property real mouse_x: 0
    property int selectedRow: -1

    // anchors.fill: parent
    

    HorizontalHeaderView {
        id: horizontalHeader

        Layout.fillWidth: true
        Layout.preferredHeight: Constants.genericTable.cellHeight

        interactive: false
        syncView: table
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
                text: table.model.columns[index].display
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
                }
                onPositionChanged: {
                    if (pressed) {
                        var delta_x = (mouseX - mouse_x);
                        var next_idx = (index + 1) % 5;
                        var min_width = table.width / 10;
                        if (columnWidths[index] + delta_x > min_width && columnWidths[next_idx] - delta_x > min_width) {
                            columnWidths[index] += delta_x;
                            columnWidths[next_idx] -= delta_x;
                        }
                        table.forceLayout();
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

    TableView {
        id: table

        // visible: true
        columnSpacing: -1
        rowSpacing: -1
        columnWidthProvider: function(column) {
            return columnWidths[column];
        }
        reuseItems: true
        boundsBehavior: Flickable.StopAtBounds

        Layout.fillWidth: true
        Layout.fillHeight: true


        Rectangle {
            // anchors.fill: parent
            border.width: 1
            border.color: "black"
            color: "transparent"
        }

        // anchors.fill: parent
        // anchors.top: horizontalHeader.bottom        
        // width: parent.width
        // height: parent.height - horizontalHeader.height
        // onWidthChanged: {
        //     // if (!advancedTab.visible)
        //     //     return ;

        //     let col_width = parent.width / 5;
        //     columnWidths = new Array(5).fill(col_width);
        //     table.forceLayout();
        // }

        ScrollBar.horizontal: ScrollBar {
        }

        ScrollBar.vertical: ScrollBar {
        }

        model: TableModel {
            // id: tableModel

            rows: [Constants.networking.defaultList]

            TableModelColumn {
                display: Constants.networking.columnHeaders[0]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[1]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[2]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[3]
            }

            TableModelColumn {
                display: Constants.networking.columnHeaders[4]
            }

        }

        delegate: Rectangle {
            implicitHeight: Constants.genericTable.cellHeight
            implicitWidth: table.columnWidthProvider(column)
            border.color: Constants.genericTable.borderColor
            color: row == selectedRow ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

            Component.onCompleted: {
                print(this.width)
                print(this.height)
            }

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
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!entries.length) {
                // columnWidths = [40,40,40,40,40];
                // table.forceLayout()
                // let col_width = parent.width / 5;
                // columnWidths = new Array(5).fill(col_width);
                // table.forceLayout();
            }
            // print(columnWidths)
            if (!advancedTab.visible)
                return ;
            
                
            // print(table.x)
            // print(table.y)
            // print(table.height)
            // print(table.width)
            // print(table.model.rows)
            for (var idx in entries) {
                var new_row = {
                };
                new_row[Constants.networking.columnHeaders[0]] = String(entries[idx][0]);
                new_row[Constants.networking.columnHeaders[1]] = String(entries[idx][1]);
                new_row[Constants.networking.columnHeaders[2]] = String(entries[idx][2]);
                new_row[Constants.networking.columnHeaders[3]] = String(entries[idx][3]);
                new_row[Constants.networking.columnHeaders[4]] = String(entries[idx][4]);

                table.model.setRow(idx, new_row);
            }
            
            // print(Utils.listObject(table))
            // print("---------------")
            // ;
            // let col_width = parent.width / 5;
            // columnWidths = new Array(5).fill(col_width);
            // table.forceLayout();

        }
    }

}
