import "BaseComponents"
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

            RowLayout {
                spacing: 20
                Layout.topMargin: 12
                Layout.alignment: Qt.AlignCenter

                SwiftButton {
                    text: `Check for updates`
                    Layout.preferredWidth: dialog.width / 4
                    Layout.alignment: Qt.AlignLeft
                    onClicked: {
                        backend_request_broker.check_for_update();
                    }
                }

                SwiftButton {
                    id: closeButton

                    text: "Close"
                    Layout.preferredWidth: dialog.width / 4
                    Layout.alignment: Qt.AlignRight
                    checkable: false
                    onClicked: {
                        dialog.close();
                    }
                }
            }
        }
    }
}
