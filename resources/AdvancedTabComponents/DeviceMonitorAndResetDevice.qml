import "../BaseComponents"
import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias deviceMonitor: deviceMonitor

    RowLayout {
        anchors.fill: parent

        DeviceMonitor {
            id: deviceMonitor

            Layout.preferredWidth: parent.width * 0.5
            Layout.fillHeight: true
        }

        SwiftButton {
            id: resetButton

            Layout.preferredWidth: parent.width * 0.5
            Layout.alignment: Qt.AlignHCenter
            ToolTip.visible: hovered
            ToolTip.text: Constants.systemMonitor.resetButtonLabel
            text: Constants.systemMonitor.resetButtonLabel
            icon.source: Constants.icons.connectButtonPath
            icon.width: Constants.systemMonitor.resetButtonIconSideLength
            icon.height: Constants.systemMonitor.resetButtonIconSideLength
            display: AbstractButton.TextUnderIcon
            flat: true
            onClicked: {
                data_model.reset_device();
            }
        }

    }

}
