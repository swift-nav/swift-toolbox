import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Layouts 1.15

Item {
    Image {
        id: okStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedImu.okStatusPath
        antialiasing: true

        ColorOverlay {
            anchors.fill: parent
            source: parent
            color: Constants.advancedImu.okStatusColor
            antialiasing: true
        }

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
