import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Layouts 1.15

Item {
    Image {
        id: warningStatusImage

        anchors.centerIn: parent
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
        anchors.leftMargin: 5
        anchors.verticalCenter: warningStatusImage.verticalCenter
        Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
        font.pointSize: Constants.mediumPointSize
    }

}
