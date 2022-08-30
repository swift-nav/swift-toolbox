import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

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
                backend_request_broker.reset_device();
            }
        }
    }
}
