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

    implicitHeight: rowLayout.implicitHeight
    implicitWidth: rowLayout.implicitWidth

    // Top grey line separating the bar from the window title area
    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        height: 1
        color: "#C2C2C2"
        z: 2
    }

    // Bottom grey line separating the bar from the main tab area
    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        height: 1
        color: "#C2C2C2"
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
            color: Constants.swiftOrange
            font.family: "Roboto Condensed"
            font.capitalization: Font.AllUppercase
            font.letterSpacing: 1
            font.pointSize: 20
            font.bold: true // Could also try playing with font.weight
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
            color: "#C2C2C2"
            Layout.alignment: Qt.AlignVCenter
        }

        Label {
            leftPadding: rhsItemSpacing
            rightPadding: rhsItemSpacing
            text: "Console"
            color: Constants.swiftLightGrey
            font.family: "Roboto Condensed"
            font.capitalization: Font.AllUppercase
            font.letterSpacing: 2
            font.pointSize: 20
        }

        Rectangle {
            Layout.fillHeight: true
            Layout.topMargin: 7
            Layout.bottomMargin: 7
            width: 1
            color: "#C2C2C2"
            Layout.alignment: Qt.AlignVCenter
        }

        Item {
            implicitWidth: children[0].implicitWidth + rhsItemSpacing * 4 / 3
            Layout.fillHeight: true

            RoundButton {
                anchors.centerIn: parent
                flat: true
                icon.source: Constants.infoPath
                icon.width: 20
                icon.height: 20
                icon.color: Constants.swiftLightGrey
                padding: rhsItemSpacing / 3
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
            color: "#C2C2C2"
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
