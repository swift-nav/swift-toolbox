import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "SolutionTabComponents" as SolutionTabComponents

MainTab {
    id: solutionTab

    SplitView {
        id: solutionSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal

        SolutionTabComponents.SolutionTable {
            width: Constants.solutionTable.width
        }

        Rectangle {
            id: solutionPlots

            Layout.minimumWidth: 200
            Layout.fillWidth: true

            TabBar {
                id: solutionBar

                currentIndex: Globals.initialMainTabIndex == 1 ? Globals.initialSubTabIndex : 0
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
