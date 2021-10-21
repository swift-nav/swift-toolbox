import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    property alias revision: hardwareRevisionText.text

    RowLayout {
        id: hardwareRevision

        anchors.fill: parent
        width: parent.width
        height: parent.height
        spacing: Constants.updateTab.labelTextAreaSpacing

        Rectangle {
            Layout.preferredWidth: Constants.updateTab.hardwareRevisionLabelWidth
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignBottom

            Text {
                text: Constants.updateTab.hardwareRevisionLabel
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            border.width: Constants.advancedImu.textDataBarBorderWidth

            Text {
                id: hardwareRevisionText

                text: ""
                clip: true
                anchors.fill: parent
                color: Constants.updateTab.placeholderTextColor
                anchors.margins: Constants.advancedImu.textDataBarMargin
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

    }

}
