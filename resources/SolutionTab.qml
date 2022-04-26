import "Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "SolutionTabComponents" as SolutionTabComponents

MainTab {
    id: solutionTab

    subTabNames: ["Position", "Velocity"]
    curSubTabIndex: 0

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
