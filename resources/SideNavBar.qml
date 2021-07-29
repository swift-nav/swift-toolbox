import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.3
import SwiftConsole 1.0

Rectangle {
    property alias curIndex: tab.currentIndex
    property var drawer: parent.drawer
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

    TabBar {
        id: tab

        z: Constants.commonChart.zAboveCharts
        height: parent.height
        contentHeight: Constants.sideNavBar.tabBarHeight
        contentWidth: Constants.sideNavBar.tabBarWidth
        currentIndex: Globals.initialMainTabIndex + 1
        spacing: Constants.sideNavBar.tabBarSpacing
        Component.onCompleted: {
            tab.contentItem.orientation = ListView.Vertical;
            hamburger.checkable = false;
        }

        TabButton {
            id: hamburger

            width: Constants.sideNavBar.tabBarWidth
            anchors.horizontalCenter: parent.horizontalCenter
            icon.source: Constants.sideNavBar.hamburgerPath
            display: AbstractButton.IconOnly
            rightInset: Constants.sideNavBar.buttonInset
            leftInset: Constants.sideNavBar.buttonInset
            onClicked: {
                drawer.open();
            }
        }

        Repeater {
            id: repeater

            model: tabModel

            TabButton {
                text: modelData.name
                width: Constants.sideNavBar.tabBarWidth
                anchors.horizontalCenter: parent.horizontalCenter
                icon.source: modelData.source
                icon.color: checked ? Constants.materialRed : Constants.materialGrey
                display: AbstractButton.TextUnderIcon
                font.pointSize: Constants.smallPointSize
                padding: Constants.sideNavBar.buttonPadding
                rightInset: Constants.sideNavBar.buttonInset
                leftInset: Constants.sideNavBar.buttonInset
                ToolTip.visible: hovered
                ToolTip.text: modelData.tooltip
            }

        }

    }

}
