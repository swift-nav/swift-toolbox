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
        //     SplitView.fillWidth: true
        // }

        id: baselineSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal
        width: parent.width
        height: parent.height

        Item {
            SplitView.minimumWidth: 200
            SplitView.fillHeight: true

            BaselineTabComponents.BaselineTable {
            }

        }

        Item {
            // BaselineTabComponents.BaselineTable {
            // }

            SplitView.fillWidth: true
            SplitView.fillHeight: true
        }
        // BaselineTabComponents.BaselinePlot {

    }

}
