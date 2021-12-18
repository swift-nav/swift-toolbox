import "../Constants"
import QtQuick 2.15

Rectangle {
    property string text: ""
    property string placeholderText: ""
    property int pointSize: Constants.largePointSize
    property string fontFamily: Constants.fontFamily
    property color borderColor: Constants.commonTextBox.borderColor
    property int borderWidth: Constants.commonTextBox.borderWidth
    property int leftMargin: Constants.commonTextBox.leftMargin
    property int rightMargin: Constants.commonTextBox.rightMargin
    property int topMargin: Constants.commonTextBox.topMargin
    property int bottomMargin: Constants.commonTextBox.bottomMargin
    property bool cursorVisible: true
    property bool selectByMouse: true

    signal textEdited()
    signal editingFinished()

    border.width: borderWidth
    border.color: borderColor
    clip: true
    Component.onCompleted: {
        textInput.textEdited.connect(textEdited);
        textInput.editingFinished.connect(editingFinished);
    }

    TextInput {
        id: textInput

        text: text
        cursorVisible: cursorVisible
        selectByMouse: selectByMouse
        font.pointSize: pointSize
        font.family: fontFamily
        anchors.fill: parent
        anchors.topMargin: topMargin
        anchors.bottomMargin: bottomMargin
        anchors.leftMargin: leftMargin
        anchors.rightMargin: rightMargin
    }

}
