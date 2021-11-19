import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

TabBar {
    id: tabBar

    property var subTabNames: ["Hello World", "Foo Bar", "Quux Quuux"]
    property alias curSubTabIndex: tabBar.currentIndex

    Repeater {
        model: subTabNames

        TabButton {
            text: modelData
            width: implicitWidth
        }

    }

}
