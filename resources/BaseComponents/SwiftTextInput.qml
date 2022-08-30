import "../Constants"
import QtQuick
import QtQuick.Controls

Rectangle {
    property alias text: textInput.text
    property string placeholderText: ""
    property color placeholderColor: Constants.updateTab.placeholderTextColor
    property int rightMargin: 5
    property int leftMargin: 5
    property int topMargin: 2
    property int bottomMargin: 0
    property int borderWidth: Constants.genericTable.borderWidth
    property bool readOnly: false
    property var labelHorizontalAlignment: Text.AlignRight
    property var fontFamily: Constants.genericTable.fontFamily
    property font labelFont: Qt.font({
            "family": Constants.genericTable.fontFamily,
            "pixelSize": Constants.largePixelSize
        })

    signal textEdited
    signal editingFinished

    clip: true
    border.width: borderWidth
    Component.onCompleted: {
        textInput.textEdited.connect(textEdited);
        textInput.editingFinished.connect(editingFinished);
    }

    TextInput {
        id: textInput

        text: ""
        cursorVisible: true
        selectByMouse: true
        readOnly: parent.readOnly
        font: labelFont
        anchors.fill: parent
        anchors.leftMargin: leftMargin
        anchors.rightMargin: rightMargin
        anchors.topMargin: topMargin
        anchors.bottomMargin: bottomMargin
        anchors.verticalCenter: parent.verticalCenter

        Label {
            id: placeholder

            text: placeholderText
            color: placeholderColor
            visible: !textInput.text
        }
    }
}
