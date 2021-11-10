import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Controls 2.15

Item {
    Image {
        id: unknownStatusImage

        anchors.verticalCenter: parent.verticalCenter
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: Constants.advancedImu.unknownStatusPath
        antialiasing: true

        ColorOverlay {
            anchors.fill: parent
            source: parent
            color: Constants.advancedImu.unknownStatusColor
            antialiasing: true
        }

    }

    Label {
        id: label

        text: "UNKNOWN"
        anchors.left: unknownStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: unknownStatusImage.verticalCenter
    }

}
