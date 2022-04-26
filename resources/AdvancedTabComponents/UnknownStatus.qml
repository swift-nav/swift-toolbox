import "../Constants"
import QtQuick
import QtQuick.Controls

Item {
    Image {
        id: unknownStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: Constants.advancedImu.unknownStatusPath
        antialiasing: Globals.useAntiAliasing

//        ColorOverlay {
//            anchors.fill: parent
//            source: parent
//            color: Constants.advancedImu.unknownStatusColor
//            antialiasing: Globals.useAntiAliasing
//        }

    }

    Label {
        id: label

        text: "UNKNOWN"
        anchors.left: unknownStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: unknownStatusImage.verticalCenter
    }

}
