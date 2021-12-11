import "BaseComponents"
import "Constants"
import Qt.labs.qmlmodels 1.0
import QtQuick 2.15
import QtQuick.Controls 2.15
import SwiftConsole 1.0
import "TableComponents"

Item {
    property var logEntries: []
    property variant columnWidths: [parent.width * Constants.logPanel.defaultColumnWidthRatios[0], parent.width * Constants.logPanel.defaultColumnWidthRatios[1], parent.width * Constants.logPanel.defaultColumnWidthRatios[2]]
    property real mouse_x: 0
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

        Item {
            anchors.fill: parent
            anchors.topMargin: Constants.genericTable.cellHeight
            anchors.rightMargin: Constants.logPanel.pauseButtonRightMargin
            z: Constants.logPanel.zAboveTable

            SwiftButton {
                width: Constants.logPanel.pauseButtonWidth
                height: Constants.logPanel.pauseButtonWidth
                padding: Constants.logPanel.pauseButtonPadding
                icon.width: Constants.logPanel.pauseButtonWidth / 3
                icon.height: Constants.logPanel.pauseButtonWidth / 3
                icon.source: Constants.icons.xPath
                icon.color: Constants.materialGrey
                anchors.right: parent.right
                anchors.top: parent.top
                ToolTip.visible: hovered
                ToolTip.text: Constants.logPanel.clearButtonTooltip
                onClicked: {
                    tableView.model.clear();
                    var new_row = {
                    };
                    new_row[Constants.logPanel.timestampHeader] = "";
                    new_row[Constants.logPanel.levelHeader] = "";
                    new_row[Constants.logPanel.msgHeader] = "";
                    logEntries = [new_row];
                    tableView.model.setRow(0, new_row);
                    tableView.forceLayout();
                }
            }

        }

        Item {
            anchors.fill: parent
            anchors.topMargin: Constants.genericTable.cellHeight * 2
            anchors.rightMargin: Constants.logPanel.pauseButtonRightMargin
            z: Constants.logPanel.zAboveTable

            SwiftButton {
                visible: !consolePaused
                width: Constants.logPanel.pauseButtonWidth
                height: Constants.logPanel.pauseButtonWidth
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

            SwiftButton {
                visible: consolePaused
                width: Constants.logPanel.pauseButtonWidth
                height: Constants.logPanel.pauseButtonWidth
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

            Menu {
                id: menu

                onAboutToShow: {
                    menu.x = columnWidths[0];
                    menu.width = columnWidths[1];
                }
                onHeightChanged: {
                    menu.y = horizontalHeader.y - menu.height;
                }

                Repeater {
                    model: logLevelLabels

                    MenuItem {
                        id: menuItem

                        onTriggered: {
                            logLevelIndex = index;
                            data_model.log_level(modelData);
                        }

                        contentItem: Label {
                            text: modelData
                            color: logLevelIndex == index ? Constants.swiftOrange : Constants.genericTable.textColor
                            font.pointSize: Constants.mediumPointSize
                        }

                    }

                }

            }

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
                        id: button

                        visible: index == 1
                        enabled: !menu.visible
                        icon.source: Constants.icons.dropIndicatorPath
                        anchors.right: headerText.right
                        icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
                        height: parent.height
                        width: Constants.logPanel.dropdownButtonWidth
                        icon.width: Constants.logPanel.dropdownButtonWidth
                        icon.height: parent.height
                        padding: Constants.logPanel.dropdownButtonPadding
                        onClicked: {
                            menu.open();
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

        SwiftTableView {
            id: tableView

            anchors.top: horizontalHeader.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            columnWidths: parent.parent.columnWidths
            delegateBorderWidth: Constants.logPanel.delegateBorderWidth

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
                if (logPanelData.entries.length && tableView.selectedRow != -1)
                    tableView.selectedRow += logPanelData.entries.length;

                logPanelData.entries = [];
            }
        }

    }

}
