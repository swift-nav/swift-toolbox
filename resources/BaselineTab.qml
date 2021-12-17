import "BaselineTabComponents" as BaselineTabComponents
import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15

MainTab {
    id: baselineTab

    SplitView {
        id: baselineSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal

        BaselineTabComponents.BaselineTable {
            SplitView.minimumWidth: Constants.baselineTable.minimumWidth
            SplitView.fillHeight: true
        }

        BaselineTabComponents.BaselinePlot {
            SplitView.minimumWidth: Constants.baselinePlot.minimumWidth
            SplitView.fillWidth: true
            SplitView.fillHeight: true
        }

    }

}
