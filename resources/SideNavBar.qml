import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property alias curIndex: tab.currentIndex
    property var tabModel: [{
        "name": "Tracking",
        "tooltip": "Tracking",
        "source": Constants.sideNavBar.trackingPath
    }, {
        "name": "Solution",
        "tooltip": "Solution",
        "source": Constants.sideNavBar.solutionPath
    }, {
        "name": "Baseline",
        "tooltip": "Baseline",
        "source": Constants.sideNavBar.baselinePath
    }, {
        "name": "Obs ",
        "tooltip": "Observations",
        "source": Constants.sideNavBar.observationsPath
    }, {
        "name": "Settings",
        "tooltip": "Settings",
        "source": Constants.sideNavBar.settingsPath
    }, {
        "name": "Update",
        "tooltip": "Update",
        "source": Constants.sideNavBar.updatePath
    }, {
        "name": "Advanced",
        "tooltip": "Advanced",
        "source": Constants.sideNavBar.advancedPath
    }]

    color: Constants.sideNavBar.backgroundColor

    ConnectionData {
        id: connectionData
    }

    ColumnLayout {
        anchors.fill: parent

        Button {
            id: logo

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.sideNavBar.tabBarHeight
            padding: Constants.sideNavBar.buttonPadding
            icon.source: Constants.icons.swiftLogoPath
            icon.color: "transparent"
            ToolTip.visible: hovered
            ToolTip.text: "About this application"
            onClicked: {
                logoPopup.open();
            }

            background: Item {
            }

        }

        Rectangle {
            color: Constants.materialGrey
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredHeight: Constants.sideNavBar.separatorHeight
            Layout.fillWidth: true
            Layout.rightMargin: Constants.sideNavBar.separatorMargin
            Layout.leftMargin: Constants.sideNavBar.separatorMargin
        }

        TabBar {
            id: tab

            Layout.fillWidth: true
            Layout.fillHeight: true
            z: Constants.commonChart.zAboveCharts
            height: parent.height
            contentHeight: Constants.sideNavBar.tabBarHeight
            contentWidth: Constants.sideNavBar.tabBarWidth
            currentIndex: Globals.initialMainTabIndex + 1
            Component.onCompleted: {
                logo.checkable = false;
            }

            TabButton {
                enabled: false
                height: 0
            }

            Repeater {
                id: repeater

                model: tabModel

                TabButton {
                    text: modelData.name
                    width: Constants.sideNavBar.tabBarWidth
                    anchors.horizontalCenter: parent.horizontalCenter
                    icon.source: modelData.source
                    icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
                    display: AbstractButton.TextUnderIcon
                    font.pointSize: Constants.smallPointSize
                    padding: Constants.sideNavBar.buttonPadding
                    rightInset: Constants.sideNavBar.buttonInset
                    leftInset: Constants.sideNavBar.buttonInset
                    enabled: Globals.connected_at_least_once
                    ToolTip.visible: hovered
                    ToolTip.text: modelData.tooltip
                    onClicked: {
                        if (stack.connectionScreenVisible())
                            stack.mainView();

                    }
                }

            }

            contentItem: ListView {
                model: tab.contentModel
                currentIndex: tab.currentIndex
                spacing: Constants.sideNavBar.tabBarSpacing
                orientation: ListView.Vertical
            }

        }

        TabButton {
            id: connectButton

            Layout.alignment: Qt.AlignBottom
            Layout.preferredWidth: Constants.sideNavBar.tabBarWidth
            icon.source: Constants.icons.lightningBoltPath
            icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
            checkable: false
            padding: Constants.sideNavBar.buttonPadding
            rightInset: Constants.sideNavBar.buttonInset
            leftInset: Constants.sideNavBar.buttonInset
            ToolTip.visible: hovered
            ToolTip.text: "Connection Dialog"
            enabled: Globals.connected_at_least_once
            onClicked: {
                if (stack.connectionScreenVisible())
                    stack.mainView();
                else if (stack.mainViewVisible())
                    stack.connectionScreen();
            }
        }

        Timer {
            interval: Utils.hzToMilliseconds(Constants.staticTimerSlowIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                connectButton.checked = Globals.conn_state == Constants.connection.connected;
            }
        }

    }

}
