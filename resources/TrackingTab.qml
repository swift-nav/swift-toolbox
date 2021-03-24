import QtQuick 2.5
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15

import "TrackingTabComponents" as TrackingTabComponents

Item{
    id: trackingTab
    width: parent.width
    height: parent.height
    TabBar {
        id: trackingBar
        z: 100
        Repeater {
            model: ["Signals", "Sky Plot"]
            TabButton {
                text: modelData
                width: implicitWidth
            }
        }
    }
    Rectangle {
        id: trackingTabBackground
        width: parent.width
        height: parent.height
        anchors.top: trackingBar.bottom
        anchors.bottom: trackingTab.bottom
        StackLayout {
            id: trackingbarlayout
            currentIndex: trackingBar.currentIndex
            TrackingTabComponents.TrackingSignalsTab{}
            Item {
                id: trackingskyplotTab
            }

        } 
        Component.onCompleted: {
        }
    }
      
}