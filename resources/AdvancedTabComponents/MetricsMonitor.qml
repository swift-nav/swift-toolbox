import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property real mouse_x: 0
    property variant entries: []
    property bool csacReceived: false

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Constants.systemMonitor.obsTextMargins
        visible: csacReceived

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.systemMonitor.textHeight

            Label {
                text: "Metrics"
            }

        }

        Item {
            id: contentItem
            property variant columnWidths: [width / 2, width / 2]

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            HorizontalHeaderView {
                id: horizontalHeader

                interactive: false
                syncView: tableView
                z: Constants.genericTable.headerZOffset

                delegate: Rectangle {
                    implicitWidth: contentItem.columnWidths[index]
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
                                let oldcols = contentItem.columnWidths.slice();
                                var delta_x = (mouseX - mouse_x);
                                contentItem.columnWidths[index] += delta_x;
                                contentItem.columnWidths[(index + 1) % 2] -= delta_x;
                                tableView.forceLayout();
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

                anchors.top: horizontalHeader.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                columnWidths: parent.columnWidths

                model: TableModel {
                    id: tableModel

                    rows: [Constants.systemMonitor.defaultMetricsList]

                    TableModelColumn {
                        display: Constants.systemMonitor.metricColumnHeaders[0]
                    }

                    TableModelColumn {
                        display: Constants.systemMonitor.metricColumnHeaders[1]
                    }

                }

            }

            Timer {
                interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    for (var idx in entries) {
                        var new_row = {
                        };
                        new_row[Constants.systemMonitor.metricColumnHeaders[0]] = entries[idx][0];
                        new_row[Constants.systemMonitor.metricColumnHeaders[1]] = entries[idx][1];
                        tableView.model.setRow(idx, new_row);
                    }
                }
            }

        }

    }

}
