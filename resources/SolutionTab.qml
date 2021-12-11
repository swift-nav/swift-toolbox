import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "SolutionTabComponents" as SolutionTabComponents

MainTab {
    id: solutionTab

    subTabNames: ["Position", "Velocity"]
    curSubTabIndex: Globals.initialMainTabIndex == 1 ? Globals.initialSubTabIndex : 0

    SplitView {
        id: solutionSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal

        SolutionTabComponents.SolutionTable {
            SplitView.minimumWidth: Constants.solutionTable.minimumWidth
        }

        StackLayout {
            id: solutionBarLayout

            SplitView.minimumWidth: Constants.solutionPosition.minimumWidth
            SplitView.fillWidth: true
            SplitView.fillHeight: true
            currentIndex: curSubTabIndex

            SolutionTabComponents.SolutionPositionTab {
            }

            SolutionTabComponents.SolutionVelocityTab {
            }

        }

    }

}
