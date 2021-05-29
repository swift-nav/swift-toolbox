import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.15
import "SolutionTabComponents" as SolutionTabComponents

Item {
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
            id: solutionTableArea

            width: 200

            SolutionTabComponents.SolutionTable {
            }

        }

        Rectangle {
            id: solutionPlots

            Layout.minimumWidth: 200
            Layout.fillWidth: true

            TabBar {
                id: solutionBar

                currentIndex: Globals.initialSubTabIndex
                z: Constants.commonChart.zAboveCharts
                contentHeight: Constants.tabBarHeight

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
                Component.onCompleted: {
                }

                StackLayout {
                    id: solutionBarLayout

                    width: parent.width
                    height: parent.height
                    currentIndex: solutionBar.currentIndex

                    SolutionTabComponents.SolutionPositionTab {
                    }

                    SolutionTabComponents.SolutionVelocityTab {
                    }

                }

            }

        }

    }

}
