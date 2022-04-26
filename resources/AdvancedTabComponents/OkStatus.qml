import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15

Item {
    Image {
        id: okStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: Constants.advancedImu.okStatusPath
        antialiasing: Globals.useAntiAliasing

//        ColorOverlay {
//            anchors.fill: parent
//            source: parent
//            color: Constants.advancedImu.okStatusColor
//            antialiasing: Globals.useAntiAliasing
//        }

    }

    Label {
        id: label

        text: "OK"
        anchors.left: okStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: okStatusImage.verticalCenter
    }

}
