import "../BaseComponents"
import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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
