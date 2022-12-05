import "BaseComponents"
import "Constants"
import Qt.labs.qmlmodels
import QtQuick
import QtQuick.Controls
import SwiftConsole
import "TableComponents"

Item {
    property real mouse_x: 0
    property bool forceLayoutLock: false
    property variant logLevelLabels: []
    property int logLevelIndex: 3
    property bool consolePaused: false
    property int preferredHeight: Constants.logPanel.preferredHeight
    property var columnWidths: [parent.width * Constants.logPanel.defaultColumnWidthRatios[0], parent.width * Constants.logPanel.defaultColumnWidthRatios[1], parent.width * Constants.logPanel.defaultColumnWidthRatios[2]]

    function update() {
        logPanelModel.fill_data(logPanelData);
        let logPanel = Constants.logPanel;
        if (!tableView.model.rows.length) {
            tableView.model.clear();
            tableView.model.rows = [{
                    [logPanel.timestampHeader]: "",
                    [logPanel.levelHeader]: "",
                    [logPanel.msgHeader]: ""
                }];
        }
        if (!logPanelData.entries.length)
            return;
        if (forceLayoutLock)
            return;
        if (!logLevelLabels.length)
            logLevelLabels = logPanelData.log_level_labels;
        logLevelIndex = logLevelLabels.indexOf(logPanelData.log_level);
        if (consolePaused)
            return;
        let logPanelEntries = logPanelData.entries;
        for (var idx in logPanelEntries) {
            var new_row = {};
            var logPanelEntry = logPanelEntries[idx];
            new_row[logPanel.timestampHeader] = logPanelEntry.timestamp;
            new_row[logPanel.levelHeader] = logPanelEntry.level;
            new_row[logPanel.msgHeader] = logPanelEntry.msg;
            var rows = tableView.model.rows;
            rows.unshift(new_row);
            tableView.model.rows = rows.slice(0, logPanel.maxRows);
        }
        logPanelData.entries = [];
    }

    onVisibleChanged: {
        if (visible)
            update();
    }

    LogPanelData {
        id: logPanelData

        onData_updated: update()
    }

    LogPanelModel {
        id: logPanelModel
    }

    Item {
        anchors.fill: parent
        anchors.topMargin: Constants.genericTable.cellHeight
        anchors.rightMargin: Constants.logPanel.pauseButtonRightMargin
        z: Constants.logPanel.zAboveTable

        SwiftButton {
            invertColor: true
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
                var new_row = {};
                let logPanel = Constants.logPanel;
                new_row[logPanel.timestampHeader] = "";
                new_row[logPanel.levelHeader] = "";
                new_row[logPanel.msgHeader] = "";
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
            invertColor: true
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
                menu.x = tableView.columnWidths[0];
                menu.width = tableView.columnWidths[1];
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
                        backend_request_broker.log_level(modelData);
                    }

                    contentItem: Label {
                        text: modelData
                        color: logLevelIndex == index ? Constants.swiftOrange : Constants.genericTable.textColor
                        font.pixelSize: Constants.mediumPixelSize
                    }
                }
            }
        }

        delegate: Rectangle {
            id: header

            implicitWidth: tableView.columnWidths[index]
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
                font.pixelSize: Constants.largePixelSize

                Button {
                    id: button

                    visible: index == 1
                    enabled: !menu.visible
                    icon.source: Constants.icons.dropIndicatorPath
                    anchors.right: headerText.right
                    anchors.rightMargin: Constants.logPanel.logLevelRightMargin
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
                enabled: index != 2
                visible: index != 2
                onPressed: {
                    mouse_x = mouseX;
                    forceLayoutLock = true;
                }
                onPositionChanged: {
                    if (pressed) {
                        var delta_x = (mouseX - mouse_x);
                        var next_idx = (index + 1) % 3;
                        var min_width = tableView.width / 10;
                        if (tableView.columnWidths[index] + delta_x > min_width && tableView.columnWidths[next_idx] - delta_x > min_width) {
                            tableView.columnWidths[index] += delta_x;
                            tableView.columnWidths[next_idx] -= delta_x;
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
        columnWidths: parent.columnWidths
        delegateBorderWidth: Constants.logPanel.delegateBorderWidth
        horizontalScrollBar.visible: false

        model: TableModel {
            id: tableModel

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
}
