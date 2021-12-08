import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "SideNavBarComponents"
import SwiftConsole 1.0

Item {
    id: top

    property alias currentIndex: navButtons.currentIndex
    property string currentTabName: top.currentIndex < 0 ? "" : tabModel[top.currentIndex].tooltip
    property bool solidConnection: false
    property real dataRate: 0
    property bool enabled: true
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
        "name": "Observations",
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

    function clickButton(index) {
        navButtons.itemAtIndex(index).toggle();
    }

    ConnectionData {
        id: connectionData
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: Constants.sideNavBar.backgroundColor

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
                currentIndex: -1
                enabled: top.enabled
                highlightMoveDuration: 200
                highlightResizeDuration: 0
                highlightFollowsCurrentItem: true

                highlight: Item {
                    // TODO: This is an odd z order which depends on the Z order of some things in the buttons, refactor.
                    z: 11

                    Rectangle {
                        height: 2
                        width: parent.width
                        y: parent.height - height
                        color: Constants.swiftOrange
                    }

                }

                delegate: SideNavButton {
                    text: modelData.name
                    icon.source: modelData.source
                    ToolTip.text: modelData.tooltip
                    buttonGroup: navButtonGroup
                    height: Constants.sideNavBar.tabBarHeight
                }

            }

        }

        SideNavButton {
            id: connectButton

            Layout.alignment: Qt.AlignBottom
            Layout.fillWidth: true
            height: Constants.sideNavBar.tabBarHeight
            text: "Connection"
            icon.source: Constants.icons.lightningBoltPath
            ToolTip.text: "Connection Dialog"
            checkable: false
            enabled: Globals.connected_at_least_once
            silenceButtonGroupWarning: true
            onClicked: {
                if (stack.connectionScreenVisible())
                    stack.mainView();
                else if (stack.mainViewVisible())
                    stack.connectionScreen();
            }
        }

        Rectangle {
            id: connectionStatusIndicator

            Layout.alignment: Qt.AlignBottom
            Layout.fillWidth: true
            height: Constants.sideNavBar.tabBarHeight
            enabled: top.enabled
            color: Constants.sideNavBar.backgroundColor
            state: (solidConnection && dataRate > 0) ? "good" : solidConnection ? "ok" : "bad"
            states: [
                State {
                    name: "good"

                    PropertyChanges {
                        target: connectionStatusCircle
                        color: Constants.sideNavBar.statusGoodColor
                    }

                },
                State {
                    name: "ok"

                    PropertyChanges {
                        target: connectionStatusCircle
                        color: Constants.sideNavBar.statusOkColor
                    }

                },
                State {
                    name: "bad"

                    PropertyChanges {
                        target: connectionStatusCircle
                        color: Constants.sideNavBar.statusBadColor
                    }

                }
            ]

            Column {
                anchors.centerIn: parent
                spacing: 2

                Label {
                    anchors.horizontalCenter: parent.horizontalCenter
                    bottomPadding: 0
                    bottomInset: 0
                    text: dataRate.toFixed(2) + " KB/s"
                    font.pointSize: Constants.smallPointSize
                    font.letterSpacing: -1
                    color: Qt.darker("white", enabled ? 1 : 1.4)
                }

                Rectangle {
                    id: connectionStatusCircle

                    property int diameter: 15

                    anchors.horizontalCenter: parent.horizontalCenter
                    width: diameter
                    height: diameter
                    radius: diameter / 2

                    Behavior on color {
                        ColorAnimation {
                        }

                    }

                }

            }

        }

    }

}
