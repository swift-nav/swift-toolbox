import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

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

            Label {
                text: Constants.updateTab.hardwareRevisionLabel
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            border.width: Constants.advancedImu.textDataBarBorderWidth

            Label {
                id: hardwareRevisionText

                text: ""
                clip: true
                anchors.fill: parent
                color: Constants.updateTab.placeholderTextColor
                anchors.margins: Constants.advancedImu.textDataBarMargin
            }

        }

    }

}
