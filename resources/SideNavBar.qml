/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "SideNavBarComponents"
import SwiftConsole

Item {
    id: top

    property alias currentIndex: navButtons.currentIndex
    property alias checkedButton: navButtonGroup.checkedButton
    property string currentTabName: top.currentIndex < 0 ? "" : tabModel[top.currentIndex].tooltip
    property bool solidConnection: false
    property real dataRate: 0
    property bool enabled: true
    property var tabModel: [{
            "name": "Tracking",
            "tooltip": "Tracking",
            "source": Constants.icons.trackingPath
        }, {
            "name": "Solution",
            "tooltip": "Solution",
            "source": Constants.icons.solutionPath
        }, {
            "name": "Baseline",
            "tooltip": "Baseline",
            "source": Constants.icons.baselinePath
        }, {
            "name": "Observations",
            "tooltip": "Observations",
            "source": Constants.icons.observationsPath
        }, {
            "name": "Settings",
            "tooltip": "Settings",
            "source": Constants.icons.settingsPath
        }, {
            "name": "Update",
            "tooltip": "Update",
            "source": Constants.icons.updatePath
        }, {
            "name": "Advanced",
            "tooltip": "Advanced",
            "source": Constants.icons.advancedPath
        }]

    function clickButton(index) {
        navButtons.itemAtIndex(index).toggle();
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
                    if (checkedButton === null)
                        return;
                    for (var idx = 0; idx < buttons.length && buttons[idx] != checkedButton; idx++)
                        ;
                    navButtons.currentIndex = idx;
                    backend_request_broker.switch_tab(tabModel[idx]["name"]);
                }
            }

            ListView {
                id: navButtons

                anchors.fill: parent
                model: tabModel
                currentIndex: -1
                enabled: top.enabled
                highlightMoveDuration: 100
                highlightResizeDuration: 0
                highlightFollowsCurrentItem: true
                onCurrentIndexChanged: {
                    if (navButtons.currentIndex >= 0)
                        navButtonGroup.checkedButton = navButtonGroup.buttons[navButtons.currentIndex];
                }

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

        Rectangle {
            Layout.alignment: Qt.AlignBottom
            Layout.fillWidth: true
            height: 1
            color: Qt.darker("white", connectButton.enabled ? 1 : 1.4)
        }

        SideNavButton {
            id: connectButton

            Layout.alignment: Qt.AlignBottom
            Layout.fillWidth: true
            implicitHeight: Constants.sideNavBar.tabBarHeight
            text: "Connection"
            icon.source: Constants.icons.connectionPath
            ToolTip.text: "Connection Dialog"
            checkable: false
            enabled: stack.mainViewVisible()
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
            implicitHeight: Constants.sideNavBar.tabBarHeight
            enabled: top.enabled
            color: Constants.sideNavBar.backgroundColor
            state: solidConnection ? (dataRate > 0 ? "good" : "ok") : "bad"
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
                    text: dataRate.toFixed(2) + "  KB/s"
                    font.pixelSize: Constants.smallPixelSize
                    font.bold: true
                    color: Qt.darker("white", enabled ? 1 : 1.4)
                }

                Rectangle {
                    id: connectionStatusCircle

                    property int diameter: 15

                    anchors.horizontalCenter: parent.horizontalCenter
                    width: diameter
                    height: diameter
                    radius: diameter / 2

                    Behavior on color  {
                        ColorAnimation {
                        }
                    }
                }
            }
        }
    }
}
