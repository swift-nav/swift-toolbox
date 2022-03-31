import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    ComboBox {
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
                    onError: (msg) => {
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
                    onError: (msg) => {
                        console.log("Font Awesome License file read error: " + msg);
                        fontAwesomeTextArea.text = msg;
                    }
                }

            }

        }

    }

}
