import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Layouts 1.15

Item {
    Image {
        id: warningStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedImu.warningStatusPath
        antialiasing: true

        ColorOverlay {
            anchors.fill: parent
            source: parent
            color: Constants.advancedImu.warningStatusColor
            antialiasing: true
        }

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
