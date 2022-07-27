import "Constants"
import QtQuick
import QtQuick.Layouts
import SwiftConsole

Item {
    id: mainTabs

    property alias currentIndex: stackLayout.currentIndex
    property var subTabNames: mainTabs.currentIndex < 0 ? [] : stackLayout.children[stackLayout.currentIndex].subTabNames
    property int curSubTabIndex: -1

    StackLayout {
        id: stackLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.mainTabs.horizontalMargins
        anchors.rightMargin: Constants.mainTabs.horizontalMargins
        anchors.topMargin: Constants.mainTabs.verticalMargins
        anchors.bottomMargin: Constants.mainTabs.verticalMargins

        TrackingTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        SolutionTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        BaselineTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        ObservationTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        SettingsTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        UpdateTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        AdvancedTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }
    }
}
