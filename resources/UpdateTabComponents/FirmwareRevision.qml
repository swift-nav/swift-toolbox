import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    property alias revision: hardwareRevisionText.placeholderText

    RowLayout {
        id: hardwareRevision

        anchors.fill: parent

        SwiftTextbox {
            Layout.preferredWidth: Constants.updateTab.hardwareRevisionLabelWidth
            Layout.fillHeight: true
            labelHorizontalAlignment: Text.AlignLeft
            text: Constants.updateTab.hardwareRevisionLabel
        }

        SwiftTextInput {
            id: hardwareRevisionText

            Layout.fillWidth: true
            Layout.fillHeight: true
            readOnly: true
        }

    }

}
