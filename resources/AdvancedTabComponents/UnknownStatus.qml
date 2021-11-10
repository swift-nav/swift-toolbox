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
        source: "../" + Constants.advancedImu.unknownStatusPath
        antialiasing: true
    }

    Text {
        id: label

        text: "UNKNOWN"
        anchors.left: unknownStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: unknownStatusImage.verticalCenter
        font.pointSize: Constants.mediumPointSize
    }

}
