import "../BaseComponents"
import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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
