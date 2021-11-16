import "SideNavBarComponents"
import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias currentIndex: navButtons.currentIndex
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
            icon.width: 32
            icon.height: 32
            checkable: false
            ToolTip.delay: 1000
            ToolTip.timeout: 5000
            ToolTip.visible: hovered
            ToolTip.text: "About this application"
            onClicked: {
                logoPopup.open();
            }

            background: Item {
            }

        }

        Rectangle {
            color: Constants.swiftLightGrey
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredHeight: Constants.sideNavBar.separatorHeight
            Layout.fillWidth: true
            Layout.rightMargin: Constants.sideNavBar.separatorMargin
            Layout.leftMargin: Constants.sideNavBar.separatorMargin
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: Constants.swiftGrey

            ButtonGroup {
                id: navButtonGroup
                buttons: navButtons.children
                onCheckedButtonChanged: {
                    for (var idx = 0; idx < buttons.length && buttons[idx] != checkedButton; idx++);
                    navButtons.currentIndex = idx;
                }

            }

            ListView {
                id: navButtons
                anchors.fill: parent

                model: tabModel
                currentIndex: Globals.initialMainTabIndex

                highlightMoveDuration: 200
                highlightResizeDuration: 0
                highlightFollowsCurrentItem: true
                highlight: Item {
                    // TODO: This is an odd z order which depends on the Z order of some things in the buttons, refactor.
                    z: 6
                    Rectangle {
                        height: 2
                        width: parent.width
                        y: parent.height - height
                        color: Constants.swiftOrange
                    }

                }

                delegate: SideNavButton {
                    buttonGroup: navButtonGroup
                }

            }
        }

        TabButton {
            id: connectButton

            Layout.alignment: Qt.AlignBottom
            Layout.preferredWidth: Constants.sideNavBar.tabBarWidth
            border: false
            icon.source: Constants.icons.lightningBoltPath
            icon.color: !enabled ? Constants.materialGrey : Constants.swiftOrange
            backgroundColor: hovered ? Qt.darker("white", 1.1) : "white"
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
