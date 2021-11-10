import "Constants"
import "LogoPopupComponents" as LogoPopupComponents
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    function open() {
        if (!dialog.visible)
            dialog.open();

    }

    Dialog {
        id: dialog

        width: parent.width / 2
        height: parent.height - Constants.logoPopup.heightPadding
        anchors.centerIn: parent
        standardButtons: Dialog.Close

        ColumnLayout {
            anchors.fill: parent

            TabBar {
                id: logoPopupBar

                z: Constants.commonChart.zAboveCharts
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.tabBarHeight

                Repeater {
                    model: ["About", "Licenses"]

                    TabButton {
                        text: modelData
                        width: implicitWidth
                    }

                }

            }

            StackLayout {
                currentIndex: logoPopupBar.currentIndex
                Layout.fillWidth: true
                Layout.fillHeight: true

                LogoPopupComponents.AboutMe {
                }

                LogoPopupComponents.Licenses {
                }

            }

        }

    }

}
