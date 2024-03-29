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
import "BaseComponents"
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Rectangle {
    id: tabInfoBar

    property string tabName: "Tracking"
    property var subTabNames: ["Hello World", "Foo Bar", "Quux Quuux"]
    property alias curSubTabIndex: tabBar.currentIndex
    property int rhsItemSpacing: 15

    signal aboutClicked
    signal entered
    signal exited

    function open() {
        state = "opened";
    }

    function close() {
        state = "closed";
    }

    implicitHeight: rowLayout.implicitHeight
    implicitWidth: rowLayout.implicitWidth
    state: "opened"
    // When the tab name changes, make sure this item is shown.
    onTabNameChanged: {
        open();
    }

    // This captures any clicks outside of the buttons, and toggles
    // the state from opened to closed or vice versa.
    MouseArea {
        enabled: false
        anchors.fill: parent
        z: -1
        onClicked: parent.state = parent.state == "opened" ? "closed" : "opened"
    }

    MouseArea {
        enabled: false
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.NoButton
        onEntered: parent.entered()
        onExited: parent.exited()
    }

    // Top grey line separating the bar from the window title area
    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        height: 1
        color: Constants.spacerColor
        z: 2
    }

    // Bottom grey line separating the bar from the main tab area
    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        height: 1
        color: Constants.spacerColor
    }

    RowLayout {
        id: rowLayout

        anchors.fill: parent
        spacing: 0

        Label {
            id: tabLabel

            Layout.fillWidth: false
            padding: 8
            leftPadding: 13
            rightPadding: 13
            text: tabName
            color: Constants.tabInfoBar.tabLabelColor
            font: Constants.tabInfoBar.tabLabelFont
        }

        SwiftTabBar {
            id: tabBar

            visible: tabBarRepeater.count > 0
            Layout.fillWidth: false
            spacing: 1

            Repeater {
                id: tabBarRepeater

                onModelChanged: {
                    if (count > 0) {
                        let button = tabBar.itemAt(tabBar.currentIndex);
                        if (!button.checked)
                            button.toggle();
                    }
                }
                model: subTabNames

                SwiftTabButton {
                    width: implicitWidth
                    topPadding: 17.5
                    bottomPadding: 17.5
                    rightInset: -1
                    leftInset: -1
                    text: modelData
                }
            }
        }

        // Spacer item
        Item {
            implicitHeight: tabBar.implicitHeight
            Layout.fillWidth: true
        }

        Rectangle {
            width: 10
        }

        Item {
            implicitWidth: children[0].implicitWidth + rhsItemSpacing
            Layout.fillHeight: true

            Image {
                anchors.top: parent.top
                anchors.topMargin: 7
                anchors.bottom: parent.bottom
                anchors.bottomMargin: 7
                source: Constants.icons.swiftLogoWidePath
                fillMode: Image.PreserveAspectFit
                asynchronous: true
            }
        }

        Rectangle {
            Layout.fillHeight: true
            Layout.topMargin: 7
            Layout.bottomMargin: 7
            width: 1
            color: Constants.spacerColor
            Layout.alignment: Qt.AlignVCenter
        }

        Label {
            leftPadding: rhsItemSpacing
            rightPadding: rhsItemSpacing
            text: Constants.tabInfoBar.appName
            color: Constants.tabInfoBar.appNameColor
            font: Constants.tabInfoBar.appNameFont
        }

        Rectangle {
            Layout.fillHeight: true
            Layout.topMargin: 7
            Layout.bottomMargin: 7
            width: 1
            color: Constants.spacerColor
            Layout.alignment: Qt.AlignVCenter
        }

        Item {
            implicitWidth: children[0].implicitWidth + rhsItemSpacing * 4 / 3
            Layout.fillHeight: true

            SwiftRoundButton {
                anchors.centerIn: parent
                flat: true
                icon.source: Constants.tabInfoBar.infoButtonIconPath
                icon.width: 20
                icon.height: 20
                icon.color: Constants.tabInfoBar.infoButtonIconColor
                padding: rhsItemSpacing / 3
                onClicked: tabInfoBar.aboutClicked()
            }
        }

        Item {
            id: closeRect

            Layout.fillHeight: true
            Layout.rightMargin: 5
            implicitWidth: 20
            clip: true
            visible: false
            enabled: false

            MouseArea {
                id: closeMouseArea

                anchors.fill: parent
                hoverEnabled: true
                onEntered: closeArrowAnimation.start()
                onExited: {
                    if (closeArrowAnimation.running) {
                        closeArrowAnimation.stop();
                        closeArrow.y = closeArrowAnimation.startingPropertyValue;
                    }
                }
                onClicked: tabInfoBar.state = tabInfoBar.state == "opened" ? "closed" : "opened"
            }

            PositionLoopAnimation {
                id: closeArrowAnimation

                target: closeArrow
                property: "y"
                startingPropertyValue: 0
                totalDuration: 700
            }

            Text {
                id: closeArrow

                anchors.horizontalCenter: parent.horizontalCenter
                y: (parent.height - height) / 2
                text: "▲"
                color: Constants.swiftLightGrey
                onYChanged: {
                    if (!closeArrowAnimation.running)
                        closeArrowAnimation.startingPropertyValue = y;
                }
            }
        }
    }

    // Add in single-line separators between the items.
    Repeater {
        model: tabInfoBar.tabName.length > 0 ? tabBar.count + 1 : 0

        Rectangle {
            property var tabButton: tabBar.itemAt(index)

            height: rowLayout.height
            width: 1
            color: Constants.spacerColor
            x: tabBar.count > 0 ? tabBar.x + (tabButton ? tabButton.x - 1 : tabBar.width) : tabLabel.x + tabLabel.width
        }
    }

    gradient: Gradient {
        GradientStop {
            position: 0
            color: "white"
        }

        GradientStop {
            position: 1
            color: Constants.swiftControlBackground
        }
    }
}
