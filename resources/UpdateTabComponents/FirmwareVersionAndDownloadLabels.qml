import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    RowLayout {
        anchors.fill: parent
        width: parent.width
        height: parent.height

        Rectangle {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true

            Text {
                text: Constants.updateTab.firmwareVersionTitle
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

        }

        Rectangle {
            Layout.preferredWidth: parent.width / 2
            Layout.fillHeight: true

            Text {
                text: Constants.updateTab.firmwareDownloadTitle
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

        }

    }

}
