import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15

Item {
    Image {
        id: warningStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: Constants.advancedImu.warningStatusPath
        antialiasing: Globals.useAntiAliasing

//        ColorOverlay {
//            anchors.fill: parent
//            source: parent
//            color: Constants.advancedImu.warningStatusColor
//            antialiasing: Globals.useAntiAliasing
//        }

    }

    Label {
        id: label

        text: "WARNING"
        anchors.left: warningStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: warningStatusImage.verticalCenter
    }

}
