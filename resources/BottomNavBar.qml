import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15

Item {
    property string tcp_ip: "TCP/IP"
    property string serial_usb: "Serial/USB"
    property string file: "File"
    property var sources: [tcp_ip, serial_usb, file]

    width: parent.width
    height: parent.height

    RowLayout {
        id: bottomNavBarRowLayout

        width: parent.width
        height: parent.height

        ComboBox {
            id: bottomNavBarSourceSelection

            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: Constants.bottomNavBar.navBarMargin
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: sources
            onActivated: {
                if (find(tcp_ip) === currentIndex)
                    tcpUrlBarPortBar.visible = true;
                else
                    tcpUrlBarPortBar.visible = false;
                if (find(file) === currentIndex)
                    fileUrlBar.visible = true;
                else
                    fileUrlBar.visible = false;
                if (find(serial_usb) === currentIndex) {
                    serialDevice.visible = true;
                    serialDeviceRefresh.visible = true;
                    serialDeviceBaudRate.visible = true;
                    serialDeviceFlowControl.visible = true;
                } else {
                    serialDevice.visible = false;
                    serialDeviceRefresh.visible = false;
                    serialDeviceBaudRate.visible = false;
                    serialDeviceFlowControl.visible = false;
                }
            }
        }

        ComboBox {
            id: serialDevice

            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: ["usb0", "usb1"]
            onActivated: {
            }
        }

        Button {
            id: serialDeviceRefresh

            visible: false
            Layout.preferredWidth: Constants.bottomNavBar.serialDeviceRefreshWidth
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            text: "F5"
        }

        ComboBox {
            id: serialDeviceBaudRate

            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            currentIndex: 1
            model: [57600, 115200, 230400, 460800, 921600, 1e+06]
            onActivated: {
            }
        }

        ComboBox {
            id: serialDeviceFlowControl

            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: ["None", "Hardware RTS/CTS"]
            onActivated: {
            }
        }

        Row {
            id: tcpUrlBarPortBar

            Layout.preferredWidth: parent.width / 2
            Layout.preferredHeight: Constants.bottomNavBar.urlBarHeight
            spacing: 1

            Rectangle {
                id: tcpUrlBar

                height: parent.height
                width: parent.width / 2
                border.width: Constants.bottomNavBar.urlBarBorder

                TextInput {
                    id: tcpUrlBarText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.bottomNavBar.urlBarTextMargin
                    onTextChanged: {
                    }

                    Text {
                        text: "Host"
                        color: Constants.bottomNavBar.placeholderTextColor
                        visible: !tcpUrlBarText.text
                    }

                }

            }

            Rectangle {
                id: tcpPortBar

                height: parent.height
                width: parent.width / 2
                border.width: Constants.bottomNavBar.urlBarBorder

                TextInput {
                    id: tcpPortBarText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.bottomNavBar.urlBarTextMargin
                    onTextChanged: {
                    }

                    Text {
                        text: "Port"
                        color: Constants.bottomNavBar.placeholderTextColor
                        visible: !tcpPortBarText.text
                    }

                }

            }

        }

        Rectangle {
            id: fileUrlBar

            visible: false
            Layout.preferredWidth: parent.width / 2
            Layout.preferredHeight: Constants.bottomNavBar.urlBarHeight
            border.width: Constants.bottomNavBar.urlBarBorder

            TextInput {
                id: fileUrlBarText

                anchors.fill: parent
                anchors.margins: Constants.bottomNavBar.urlBarTextMargin
                onTextChanged: {
                }
                clip: true

                Text {
                    text: "path/to/file"
                    color: Constants.bottomNavBar.placeholderTextColor
                    visible: !fileUrlBarText.text
                }

            }

        }

        Button {
            id: connectionPauseButton

            Layout.preferredWidth: Constants.bottomNavBar.connectionPauseWidth
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            text: "| |"
            ToolTip.visible: hovered
            ToolTip.text: "Pause"
            checkable: true
            onClicked: data_model.pause(checked)
        }

        Button {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: Constants.bottomNavBar.navBarMargin
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            checkable: true
            text: "Connect"
            onClicked: {
                if (!checked) {
                    data_model.disconnect();
                } else {
                    if (bottomNavBarSourceSelection.currentText === tcp_ip) {
                        if (tcpUrlBarText.text && tcpPortBarText.text)
                            data_model.connect_tcp(tcpUrlBarText.text, tcpPortBarText.text);
                        else
                            data_model.connect();
                    } else if (bottomNavBarSourceSelection.currentText === file) {
                        if (fileUrlBarText.text)
                            data_model.connect_file(fileUrlBarText.text);

                    } else {
                        data_model.connect_file(serialDevice.currentText, serialDeviceBaudRate.currentText, serialDeviceFlowControl.currentIndex == 1);
                    }
                }
            }
        }

    }

}
