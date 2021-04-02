import QtQuick 2.5
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15
import QtQuick.Controls 1.4

import "SolutionTabComponents" as SolutionTabComponents

Item{
    id: solutionTab
    width: parent.width
    height: parent.height
    SplitView {
        id: solutionSplitView
        anchors.fill: parent
        orientation: Qt.Horizontal
        width: parent.width
        height: parent.height
        Rectangle {
            id: solutionTable
            width: 200
            color: "lightblue"
            Text {
                text: "View 1"
                anchors.centerIn: parent
            }
        }
        Rectangle {
            id: solutionPlots
            Layout.minimumWidth: 200
            Layout.fillWidth: true

            TabBar {
                id: solutionBar
                z: 100
                Repeater {
                    model: ["Position", "Velocity"]
                    TabButton {
                        text: modelData
                        width: implicitWidth
                    }
                }
            }
            Rectangle {
                id: solutionTabBackground
                width: parent.width
                height: parent.height
                anchors.top: solutionBar.bottom
                StackLayout {
                    id: solutionBarLayout
                    width: parent.width
                    height: parent.height
                    currentIndex: solutionBar.currentIndex
                    Item {
                        id: solutionPositionTab
                    }
                    SolutionTabComponents.SolutionVelocityTab{}

                } 
                Component.onCompleted: {
                }
            }

        }

        
    }      
}
