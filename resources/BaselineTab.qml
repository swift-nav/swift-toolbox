import "BaselineTabComponents" as BaselineTabComponents
import "Constants"
import QtQuick
import QtQuick.Controls

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

        Rectangle {
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
