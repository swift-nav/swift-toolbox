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

    implicitHeight: rowLayout.implicitHeight
    implicitWidth: rowLayout.implicitWidth
    color: Constants.swiftControlBackground

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
            font.pointSize: 18
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
                    rightInset: -0.5
                    text: modelData
                    width: implicitWidth
                    border: false
                }

            }

        }

        Item {
            implicitHeight: tabBar.implicitHeight
            Layout.fillWidth: true
        }

        Label {
            Layout.fillWidth: false
            text: "Swift"
        }

        Label {
            Layout.fillWidth: false
            padding: 8
            leftPadding: 13
            rightPadding: 13
            text: "Console"
            font.family: "Roboto Condensed"
            font.capitalization: Font.AllUppercase
            // font.letterSpacing: 1
            font.pointSize: 18
        }

        Image {
            Layout.fillWidth: false
            source: Constants.infoPath
            sourceSize.width: 20
            sourceSize.height: 20
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
            x: tabBar.count > 0 ? tabBar.x + (tabButton ? tabButton.x - 1 : tabBar.width) : tabBar.x
        }

    }

}
