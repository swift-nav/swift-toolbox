/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
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
