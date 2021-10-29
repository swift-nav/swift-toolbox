import "Constants"
import "LogoPopupComponents" as LogoPopupComponents
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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
