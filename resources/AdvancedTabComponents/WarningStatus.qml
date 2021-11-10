import "../Constants"
import QtQuick
import QtQuick.Controls

Item {
    Image {
        id: warningStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedImu.warningStatusPath
        antialiasing: true
    }

    Text {
        id: label

        text: "WARNING"
        anchors.left: warningStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: warningStatusImage.verticalCenter
        font.pointSize: Constants.mediumPointSize
    }

}
