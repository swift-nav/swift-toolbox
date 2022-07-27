import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls

Item {
    SwiftImage {
        id: warningStatusImage

        anchors.verticalCenter: parent.verticalCenter
        sourceSize: Qt.size(Constants.advancedImu.insStatusImageWidth, Constants.advancedImu.insStatusImageWidth)
        smooth: true
        source: Constants.advancedImu.warningStatusPath
        antialiasing: Globals.useAntiAliasing
        color: Constants.advancedImu.warningStatusColor
    }

    Label {
        id: label

        text: "WARNING"
        anchors.left: warningStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: warningStatusImage.verticalCenter
    }
}
