import "BaselineTabComponents" as BaselineTabComponents
import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    id: baselineTab

    width: parent.width
    height: parent.height

    SplitView {
        id: baselineSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal
        width: parent.width
        height: parent.height

        Item {
            SplitView.minimumWidth: Constants.baselineTable.width
            SplitView.fillHeight: true

            BaselineTabComponents.BaselineTable {
            }

        }

        BaselineTabComponents.BaselinePlot {
            SplitView.fillWidth: true
            SplitView.fillHeight: true
        }

    }

}
