import "../Constants"
import QtGraphicalEffects 1.15
import QtQuick 2.5

Item {
    Image {
        anchors.centerIn: parent
        width: Constants.advancedIns.insStatusImageWidth
        height: Constants.advancedIns.insStatusImageWidth
        smooth: true
        source: "../" + Constants.advancedIns.okStatusPath
        antialiasing: true

        ColorOverlay {
            anchors.fill: parent
            source: parent
            color: Constants.advancedIns.okStatusColor
            antialiasing: true
        }

    }

}
