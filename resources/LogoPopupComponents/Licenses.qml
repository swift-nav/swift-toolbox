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
import "../BaseComponents"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

ColumnLayout {
    SwiftComboBox {
        id: licenses

        Layout.preferredHeight: Constants.logoPopup.licenses.dropdownHeight
        Layout.preferredWidth: parent.width / 2
        Layout.alignment: Qt.AlignHCenter
        font.family: Constants.logoPopup.licenses.fontFamily
        model: [Constants.logoPopup.licenses.robotoFontTabLabel, Constants.logoPopup.licenses.fontAwesomeIconsTabLabel]
    }

    StackLayout {
        currentIndex: licenses.currentIndex
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.alignment: Qt.AlignHCenter

        ScrollView {
            ScrollBar.vertical.policy: ScrollBar.AlwaysOn
            clip: true

            TextEdit {
                id: robotoFontTextArea

                text: robotoFileIO.text
                readOnly: true
                activeFocusOnPress: false
                horizontalAlignment: TextEdit.AlignJustify
                selectByKeyboard: true
                selectByMouse: true
                font.family: Constants.logoPopup.licenses.fontFamily
                font.pixelSize: Constants.largePixelSize

                FileIO {
                    id: robotoFileIO

                    source: Constants.logoPopup.licenses.robotoFontLicensePath
                    onError: msg => {
                        console.log("Roboto Font License file read error: " + msg);
                        robotoFontTextArea.text = msg;
                    }
                }
            }
        }

        ScrollView {
            ScrollBar.vertical.policy: ScrollBar.AlwaysOn
            clip: true

            TextEdit {
                id: fontAwesomeTextArea

                text: fontAwesomeFileIO.text
                readOnly: true
                activeFocusOnPress: false
                horizontalAlignment: TextEdit.AlignJustify
                selectByKeyboard: true
                selectByMouse: true
                font.family: Constants.logoPopup.licenses.fontFamily
                font.pixelSize: Constants.largePixelSize

                FileIO {
                    id: fontAwesomeFileIO

                    source: Constants.logoPopup.licenses.fontAwesomeIconsLicensePath
                    onError: msg => {
                        console.log("Font Awesome License file read error: " + msg);
                        fontAwesomeTextArea.text = msg;
                    }
                }
            }
        }
    }
}
