import "../Constants"
import QtQuick
import QtQuick.Controls

Item {
    Image {
        id: okStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedImu.okStatusPath
        antialiasing: true
    }

    Text {
        id: label

        text: "OK"
        anchors.left: okStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: okStatusImage.verticalCenter
        font.pointSize: Constants.mediumPointSize
    }

}
