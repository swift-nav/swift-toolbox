import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property string tcp_ip: "TCP/IP"
    property string serial_usb: "Serial/USB"
    property string file: "File"
    property var sources: [tcp_ip, serial_usb, file]
    property variant available_baudrates: []
    property variant available_devices: []
    property variant available_flows: []

    width: parent.width
    height: parent.height

    BottomNavbarData {
        id: bottomNavbarData
    }

    RowLayout {
        id: bottomNavBarRowLayout

        width: parent.width
        height: parent.height

        ComboBox {
            id: bottomNavBarSourceSelection

            Layout.preferredWidth: Constants.bottomNavBar.connectionDropdownWidth
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
            Layout.alignment: Qt.AlignLeft
            Layout.preferredWidth: Constants.bottomNavBar.serialSelectionDropdownWidth
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: available_devices
            onActivated: {
            }
        }

        Button {
            id: serialDeviceRefresh

            visible: false
            Layout.alignment: Qt.AlignLeft
            Layout.preferredWidth: Constants.bottomNavBar.serialDeviceRefreshWidth
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            text: "F5"
            onClicked: {
                data_model.serial_refresh();
            }
        }

        ComboBox {
            id: serialDeviceBaudRate

            visible: false
            Layout.alignment: Qt.AlignLeft
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: available_baudrates
            onActivated: {
            }
        }

        ComboBox {
            id: serialDeviceFlowControl

            visible: false
            Layout.alignment: Qt.AlignLeft
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            model: available_flows
            onActivated: {
            }
        }

        Row {
            id: tcpUrlBarPortBar

            Layout.alignment: Qt.AlignLeft
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
            Layout.alignment: Qt.AlignLeft
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
            ToolTip.text: !checked ? "Pause" : "Unpause"
            checkable: true
            onClicked: data_model.pause(checked)
        }

        Button {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: Constants.bottomNavBar.navBarMargin
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            checkable: true
            text: !checked ? "Connect" : "Disconnect"
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
                        data_model.connect_serial(serialDevice.currentText, serialDeviceBaudRate.currentText, serialDeviceFlowControl.currentText);
                    }
                }
            }
        }

        ComboBox {
            id: refreshRateDrop

            visible: true
            Layout.preferredWidth: Constants.bottomNavBar.plotRefreshRateDropdownWidth
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            ToolTip.visible: hovered
            ToolTip.text: "Refresh Rate (Hz)"
            model: Constants.bottomNavBar.all_refresh_rates
            currentIndex: Constants.bottomNavBar.default_refresh_rate_index
            onActivated: {
                Constants.currentRefreshRate = 1000 / Constants.bottomNavBar.all_refresh_rates[currentIndex];
            }
        }

        Timer {
            interval: Constants.defaultTimerIntervalRate
            running: true
            repeat: true
            onTriggered: {
                bottom_navbar_model.fill_data(bottomNavbarData);
                if (!bottomNavbarData.available_baudrates.length)
                    return ;

                if (available_baudrates.length == 0) {
                    available_baudrates = bottomNavbarData.available_baudrates;
                    serialDeviceBaudRate.currentIndex = 1;
                }
                if (available_flows.length == 0)
                    available_flows = bottomNavbarData.available_flows;

                available_devices = bottomNavbarData.available_ports;
            }
        }

    }

}
