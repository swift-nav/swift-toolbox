import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias drawer: sideDrawer
    property var stackView: parent.stackView

    Drawer {
        id: sideDrawer

        width: parent.width / 5
        height: parent.height
        interactive: true
        dim: false
        dragMargin: Constants.sideNavBar.tabBarWidth / 4

        ListView {
            id: drawerItems

            anchors.fill: parent
            focus: true
            currentIndex: -1

            delegate: ItemDelegate {
                highlighted: ListView.isCurrentItem
                width: drawerItems.width
                text: model.name
                onClicked: {
                    drawerItems.currentIndex = index;
                    stackView.push(model.source);
                    sideDrawer.close();
                }
            }

            model: ListModel {
                ListElement {
                    name: "Connection"
                }

                ListElement {
                    name: "License"
                    source: "MainDrawerComponents/LicensesPopup.qml"
                }

                ListElement {
                    name: "About"
                }

            }

        }

    }

}
