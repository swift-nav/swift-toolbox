import "BaselineTabComponents" as BaselineTabComponents
import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

MainTab {
    id: baselineTab

    SplitView {
        id: baselineSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal
        width: parent.width
        height: parent.height

        Rectangle {
            SplitView.minimumWidth: Constants.baselineTable.minimumWidth
            SplitView.fillHeight: true

            BaselineTabComponents.BaselineTable {
            }

        }

        BaselineTabComponents.BaselinePlot {
            SplitView.minimumWidth: Constants.baselinePlot.minimumWidth
            SplitView.fillWidth: true
            SplitView.fillHeight: true
        }

    }

}
