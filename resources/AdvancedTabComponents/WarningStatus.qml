import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5

Item {
    Image {
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

}
