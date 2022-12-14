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

            SwiftTabBar {
                id: logoPopupBar

                z: Constants.commonChart.zAboveCharts
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.tabBarHeight

                Repeater {
                    model: ["About", "Licenses"]

                    SwiftTabButton {
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

                Button {
                    text: `Check for updates`
                    Layout.preferredWidth: dialog.width / 4
                    Layout.alignment: Qt.AlignLeft
                    onClicked: {
                        backend_request_broker.check_for_update();
                    }
                }

                Button {
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
