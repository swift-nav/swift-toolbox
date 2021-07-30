import "../Constants"
import "../Constants/utils.js" as Utils
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    Dialog {
        id: dialog

        visible: true
        width: Constants.width / 2
        height: parent.height - Constants.licensesPopup.dialogPopupHeightPadding
        anchors.centerIn: parent
        standardButtons: Dialog.Ok | Dialog.Cancel

        TabBar {
            id: licenseBar

            contentHeight: Constants.licensesPopup.tabBarHeight
            width: parent.width

            TabButton {
                width: implicitWidth
                text: Constants.licensesPopup.robotoFontTabLabel
            }

            TabButton {
                width: implicitWidth
                text: Constants.licensesPopup.fontAwesomeIconsTabLabel
            }

        }

        StackLayout {
            currentIndex: licenseBar.currentIndex
            width: parent.width
            height: parent.height
            anchors.top: licenseBar.bottom

            Flickable {

                TextArea.flickable: TextArea {
                    id: robotoFontTextArea
                }

                ScrollBar.vertical: ScrollBar {
                }

            }

            Flickable {

                TextArea.flickable: TextArea {
                    id: fontAwesomeTextArea
                }

                ScrollBar.vertical: ScrollBar {
                }

            }

        }

        Timer {
            id: readStarter

            interval: 1
            running: true
            repeat: false
            onTriggered: {
                Utils.readTextFile(Constants.licensesPopup.robotoFontLicensePath, robotoFontTextArea);
                Utils.readTextFile(Constants.licensesPopup.fontAwesomeIconsLicensePath, fontAwesomeTextArea);
            }
        }

    }

}
