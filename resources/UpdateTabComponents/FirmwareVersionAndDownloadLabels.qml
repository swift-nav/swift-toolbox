import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    RowLayout {
        anchors.fill: parent
        width: parent.width
        height: parent.height

        SwiftTextbox {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true
            labelHorizontalAlignment: Text.AlignLeft
            text: Constants.updateTab.firmwareVersionTitle
        }

        SwiftTextbox {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true
            labelHorizontalAlignment: Text.AlignLeft
            text: Constants.updateTab.firmwareDownloadTitle
        }

    }

}
