import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    ComboBox {
        id: licenses

        Layout.preferredHeight: Constants.logoPopup.licenses.dropdownHeight
        Layout.preferredWidth: parent.width / 2
        Layout.alignment: Qt.AlignHCenter
        model: [Constants.logoPopup.licenses.robotoFontTabLabel, Constants.logoPopup.licenses.fontAwesomeIconsTabLabel]
    }

    StackLayout {
        currentIndex: licenses.currentIndex
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.alignment: Qt.AlignHCenter

        ScrollView {
            ScrollBar.vertical.policy: ScrollBar.AlwaysOn

            TextArea {
                id: robotoFontTextArea

                readOnly: true
                activeFocusOnPress: false
                horizontalAlignment: TextEdit.AlignJustify
                selectByKeyboard: true
                selectByMouse: true
            }

        }

        ScrollView {
            ScrollBar.vertical.policy: ScrollBar.AlwaysOn

            TextArea {
                id: fontAwesomeTextArea

                readOnly: true
                activeFocusOnPress: false
                horizontalAlignment: TextEdit.AlignJustify
                selectByKeyboard: true
                selectByMouse: true
            }

        }

    }

    Timer {
        interval: 1
        running: true
        repeat: false
        onTriggered: {
            Utils.readTextFile(Constants.logoPopup.licenses.robotoFontLicensePath, robotoFontTextArea);
            Utils.readTextFile(Constants.logoPopup.licenses.fontAwesomeIconsLicensePath, fontAwesomeTextArea);
        }
    }

}
