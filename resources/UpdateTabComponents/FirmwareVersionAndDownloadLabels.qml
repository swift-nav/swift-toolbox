import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    RowLayout {
        anchors.fill: parent
        width: parent.width
        height: parent.height

        Rectangle {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true

            Label {
                text: Constants.updateTab.firmwareVersionTitle
            }

        }

        Rectangle {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true

            Label {
                text: Constants.updateTab.firmwareDownloadTitle
            }

        }

    }

}
