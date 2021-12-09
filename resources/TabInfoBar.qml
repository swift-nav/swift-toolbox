import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    id: tabInfoBar

    property string tabName: "Tracking"
    property var subTabNames: ["Hello World", "Foo Bar", "Quux Quuux"]
    property alias curSubTabIndex: tabBar.currentIndex
    property int rhsItemSpacing: 15

    signal aboutClicked()
    signal entered()
    signal exited()

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
        anchors.fill: parent
        z: -1
        onClicked: parent.state = parent.state == "opened" ? "closed" : "opened"
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.NoButton
        onEntered: parent.entered();
        onExited: parent.exited();
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
        z: 2
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

        TabBar {
            id: tabBar

            visible: tabBarRepeater.count > 0
            Layout.fillWidth: false
            spacing: 1

            Repeater {
                id: tabBarRepeater

                model: subTabNames

                TabButton {
                    width: implicitWidth
                    topPadding: 16
                    bottomPadding: 16
                    rightInset: -0.5
                    text: modelData
                    border: false
                }

            }

        }

        // Spacer item
        Item {
            implicitHeight: tabBar.implicitHeight
            Layout.fillWidth: true
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

            RoundButton {
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

        Rectangle {
            id: closeRect
            Layout.fillHeight: true
            Layout.rightMargin: 5
            implicitWidth: 20
            color: Constants.swiftControlBackground
            radius: 3
            clip: true

            MouseArea {
                id: closeMouseArea
                anchors.fill: parent
                hoverEnabled: true
                acceptedButtons: Qt.NoButton
                onEntered: closeArrowAnimation.start()
                onExited: {
                    if (closeArrowAnimation.running) {
                        // closeArrowAnimation.stop();
                        // closeArrowAnimation.totalDuration = 200;
                        // closeArrowAnimation.start();
                        closeArrowAnimation.stop();
                        closeArrow.y = closeArrowAnimation.startingY;
                    }
                }
            }

            SequentialAnimation {
                id: closeArrowAnimation
                property Item target: closeArrow
                property string property: "y"
                property real startingY: 0
                property real totalDuration: 700
                NumberAnimation {
                    target: closeArrowAnimation.target
                    property: closeArrowAnimation.property
                    duration: closeArrowAnimation.totalDuration / 2
                    easing.type: Easing.InQuad
                    to: 0
                }
                PropertyAction {
                    target: closeArrowAnimation.target
                    property: closeArrowAnimation.property
                    value: closeRect.height
                }
                NumberAnimation {
                    target: closeArrowAnimation.target
                    property: closeArrowAnimation.property
                    duration: closeArrowAnimation.totalDuration / 2
                    onDurationChanged: console.log("last anim duration changed: " + duration)
                    easing.type: Easing.OutQuad
                    to: closeArrowAnimation.startingY
                }
            }

            Text {
                id: closeArrow
                anchors.horizontalCenter: parent.horizontalCenter
                y: (parent.height - height) / 2
                text: "▲"
                color: Constants.swiftLightGrey
                onYChanged: {
                    if (!closeArrowAnimation.running)
                        closeArrowAnimation.startingY = y
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
