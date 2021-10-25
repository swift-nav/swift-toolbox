import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Layouts 1.15

Item {
    Image {
        id: unknownStatusImage

        anchors.centerIn: parent
        width: Constants.advancedImu.insStatusImageWidth
        height: Constants.advancedImu.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedImu.unknownStatusPath
        antialiasing: true

        ColorOverlay {
            anchors.fill: parent
            source: parent
            color: Constants.advancedImu.unknownStatusColor
            antialiasing: true
        }

    }

    Text {
        id: label

        text: "UNKNOWN"
        anchors.left: unknownStatusImage.right
        anchors.leftMargin: Constants.fusionStatusFlags.labelMargin
        anchors.verticalCenter: unknownStatusImage.verticalCenter
        Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
        font.pointSize: Constants.mediumPointSize
    }

}
