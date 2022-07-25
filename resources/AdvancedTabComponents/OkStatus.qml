import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls

Item {
    SwiftImage {
        id: okStatusImage

        anchors.verticalCenter: parent.verticalCenter
        sourceSize: Qt.size(Constants.advancedImu.insStatusImageWidth, Constants.advancedImu.insStatusImageWidth)
        smooth: true
        source: Constants.advancedImu.okStatusPath
        antialiasing: Globals.useAntiAliasing
        color: Constants.advancedImu.okStatusColor
    }

    Label {
        id: label

        text: "OK"
        anchors.left: okStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: okStatusImage.verticalCenter
    }

}
