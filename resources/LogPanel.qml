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
    property variant logLevelLabels: []
    property int logLevelIndex: 3
    property bool consolePaused: false

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

        Item {
            anchors.fill: parent
            anchors.topMargin: Constants.genericTable.cellHeight
            anchors.rightMargin: Constants.logPanel.pauseButtonRightMargin
            z: Constants.logPanel.zAboveTable

            RoundButton {
                id: baselinePauseButton

                visible: !consolePaused
                width: Constants.logPanel.pauseButtonWidth
                height: Constants.logPanel.pauseButtonWidth
                radius: Constants.logPanel.pauseButtonWidth / 3
                padding: Constants.logPanel.pauseButtonPadding
                icon.width: Constants.logPanel.pauseButtonWidth / 3
                icon.height: Constants.logPanel.pauseButtonWidth / 3
                icon.source: Constants.icons.pauseButtonUrl
                icon.color: Constants.materialGrey
                anchors.right: parent.right
                anchors.top: parent.top
                ToolTip.visible: hovered
                ToolTip.text: Constants.logPanel.pauseButtonTooltip
                onClicked: {
                    consolePaused = true;
                }
            }

            RoundButton {
                id: baselinePlayButton

                visible: consolePaused
                width: Constants.logPanel.pauseButtonWidth
                height: Constants.logPanel.pauseButtonWidth
                radius: Constants.logPanel.pauseButtonWidth / 3
                padding: Constants.logPanel.pauseButtonPadding
                icon.width: Constants.logPanel.pauseButtonWidth / 3
                icon.height: Constants.logPanel.pauseButtonWidth / 3
                icon.source: Constants.icons.playPath
                icon.color: Constants.swiftOrange
                anchors.right: parent.right
                anchors.top: parent.top
                ToolTip.visible: hovered
                ToolTip.text: Constants.logPanel.playButtonTooltip
                onClicked: {
                    consolePaused = false;
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
                id: header

                implicitWidth: columnWidths[index]
                implicitHeight: Constants.genericTable.cellHeight
                border.color: Constants.genericTable.borderColor

                Label {
                    id: headerText

                    width: parent.width
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                    anchors.fill: parent
                    text: tableView.model.columns[index].display
                    elide: Text.ElideRight
                    clip: true
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize

                    Button {
                        visible: index == 1
                        icon.source: Constants.icons.dropIndicatorPath
                        anchors.right: headerText.right
                        icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
                        height: parent.height
                        width: Constants.logPanel.dropdownButtonWidth
                        icon.width: Constants.logPanel.dropdownButtonWidth
                        icon.height: parent.height
                        padding: Constants.logPanel.dropdownButtonPadding
                        onClicked: {
                            if (!menu.visible) {
                                menu.open();
                                menu.y = (parent.y - menu.height);
                                menu.x = width - columnWidths[1];
                                menu.width = columnWidths[1];
                            }
                        }

                        Menu {
                            id: menu

                            y: Constants.logPanel.logLevelMenuHeight
                            onXChanged: {
                                this.close();
                            }

                            Repeater {
                                model: logLevelLabels

                                MenuItem {
                                    id: menuItem

                                    onTriggered: {
                                        logLevelIndex = index;
                                        data_model.log_level(modelData);
                                    }

                                    contentItem: Text {
                                        text: modelData
                                        color: logLevelIndex == index ? Constants.swiftOrange : Constants.genericTable.textColor
                                        font.pointSize: Constants.mediumPointSize
                                        font.family: Constants.genericTable.fontFamily
                                    }

                                }

                            }

                        }

                        background: Item {
                        }

                    }

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
                            var delta_x = (mouseX - mouse_x);
                            var next_idx = (index + 1) % 3;
                            var min_width = tableView.width / 10;
                            if (columnWidths[index] + delta_x > min_width && columnWidths[next_idx] - delta_x > min_width) {
                                columnWidths[index] += delta_x;
                                columnWidths[next_idx] -= delta_x;
                            }
                            tableView.forceLayout();
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
            onWidthChanged: {
                tableView.forceLayout();
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

                if (!logLevelLabels.length)
                    logLevelLabels = logPanelData.log_level_labels;

                logLevelIndex = logLevelLabels.indexOf(logPanelData.log_level);
                for (var idx in logPanelData.entries) {
                    var new_row = {
                    };
                    new_row[Constants.logPanel.timestampHeader] = logPanelData.entries[idx].timestamp;
                    new_row[Constants.logPanel.levelHeader] = logPanelData.entries[idx].level;
                    new_row[Constants.logPanel.msgHeader] = logPanelData.entries[idx].msg;
                    logEntries.unshift(new_row);
                }
                logEntries = logEntries.slice(0, Constants.logPanel.maxRows);
                if (consolePaused)
                    return ;

                for (var idx in logEntries) {
                    tableView.model.setRow(idx, logEntries[idx]);
                }
                if (logPanelData.entries.length && selectedRow != -1)
                    selectedRow += logPanelData.entries.length;

                logPanelData.entries = [];
            }
        }

    }

}
