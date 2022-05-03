import "../Constants"
import "../BaseComponents"
import QtQuick
import QtQuick.Controls

Item {
    SwiftImage {
        id: unknownStatusImage

        anchors.verticalCenter: parent.verticalCenter
        sourceSize: Qt.size(Constants.advancedImu.insStatusImageWidth, Constants.advancedImu.insStatusImageWidth)
        smooth: true
        source: Constants.advancedImu.unknownStatusPath
        antialiasing: Globals.useAntiAliasing
        color: Constants.advancedImu.unknownStatusColor

    }

    Label {
        id: label

        text: "UNKNOWN"
        anchors.left: unknownStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: unknownStatusImage.verticalCenter
    }

}
