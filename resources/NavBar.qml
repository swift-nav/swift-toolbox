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

    anchors.fill: parent

    NavBarData {
        id: navBarData
    }

    RowLayout {
        id: navBarRowLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.navBar.navBarMargin
        anchors.rightMargin: Constants.navBar.navBarMargin

        ComboBox {
            id: navBarSourceSelection

            Component.onCompleted: {
                navBarSourceSelection.indicator.width = Constants.navBar.connectionDropdownWidth / 3;
            }
            Layout.preferredWidth: Constants.navBar.connectionDropdownWidth
            Layout.preferredHeight: Constants.navBar.dropdownHeight
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

            states: State {
                when: navBarSourceSelection.down

                PropertyChanges {
                    target: navBarSourceSelection
                    width: Constants.navBar.connectionDropdownWidth * 1.5
                }

            }

        }

        ComboBox {
            id: serialDevice

            Component.onCompleted: {
                serialDevice.indicator.width = Constants.navBar.serialSelectionDropdownWidth / 3;
            }
            visible: false
            Layout.preferredHeight: Constants.navBar.dropdownHeight
            Layout.preferredWidth: Constants.navBar.serialSelectionDropdownWidth
            model: available_devices
            onActivated: {
            }

            states: State {
                when: serialDevice.down

                PropertyChanges {
                    target: serialDevice
                    width: Constants.navBar.serialSelectionDropdownWidth * 1.5
                }

            }

        }

        Button {
            id: serialDeviceRefresh

            visible: false
            Layout.preferredHeight: Constants.navBar.buttonHeight
            Layout.preferredWidth: Constants.navBar.serialDeviceRefreshWidth
            text: "F5"
            onClicked: {
                data_model.serial_refresh();
            }
        }

        ComboBox {
            id: serialDeviceBaudRate

            Component.onCompleted: {
                serialDeviceBaudRate.indicator.width = Constants.navBar.serialDeviceBaudRateDropdownWidth / 3;
            }
            visible: false
            Layout.preferredHeight: Constants.navBar.dropdownHeight
            Layout.preferredWidth: Constants.navBar.serialDeviceBaudRateDropdownWidth
            model: available_baudrates
            onActivated: {
            }

            states: State {
                when: serialDeviceBaudRate.down

                PropertyChanges {
                    target: serialDeviceBaudRate
                    width: Constants.navBar.serialDeviceBaudRateDropdownWidth * 1.5
                }

            }

        }

        ComboBox {
            id: serialDeviceFlowControl

            Component.onCompleted: {
                serialDeviceFlowControl.indicator.width = Constants.navBar.serialDeviceFlowControlDropdownWidth / 3;
            }
            visible: false
            Layout.preferredHeight: Constants.navBar.dropdownHeight
            Layout.preferredWidth: Constants.navBar.serialDeviceFlowControlDropdownWidth
            model: available_flows
            onActivated: {
            }

            states: State {
                when: serialDeviceFlowControl.down

                PropertyChanges {
                    target: serialDeviceFlowControl
                    width: Constants.navBar.serialDeviceFlowControlDropdownWidth * 1.5
                }

            }

        }

        Row {
            id: tcpUrlBarPortBar

            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            spacing: 1

            Rectangle {
                id: tcpUrlBar

                height: parent.height
                width: 3 * parent.width / 4
                border.width: Constants.navBar.urlBarBorder

                TextInput {
                    id: tcpUrlBarText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.navBar.urlBarTextMargin
                    onTextChanged: {
                    }

                    Text {
                        text: "Host"
                        color: Constants.navBar.placeholderTextColor
                        visible: !tcpUrlBarText.text
                    }

                }

            }

            Rectangle {
                id: tcpPortBar

                height: parent.height
                width: parent.width / 4
                border.width: Constants.navBar.urlBarBorder

                TextInput {
                    id: tcpPortBarText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.navBar.urlBarTextMargin
                    onTextChanged: {
                    }

                    Text {
                        text: "Port"
                        color: Constants.navBar.placeholderTextColor
                        visible: !tcpPortBarText.text
                    }

                }

            }

        }

        Rectangle {
            id: fileUrlBar

            visible: false
            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            border.width: Constants.navBar.urlBarBorder

            TextInput {
                id: fileUrlBarText

                anchors.fill: parent
                anchors.margins: Constants.navBar.urlBarTextMargin
                onTextChanged: {
                }
                clip: true

                Text {
                    text: "path/to/file"
                    color: Constants.navBar.placeholderTextColor
                    visible: !fileUrlBarText.text
                }

            }

        }

        Button {
            id: connectionPauseButton

            Layout.preferredWidth: Constants.navBar.connectionPauseWidth
            Layout.preferredHeight: Constants.navBar.buttonHeight
            text: "| |"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "Pause" : "Unpause"
            checkable: true
            onClicked: data_model.pause(checked)
        }

        Button {
            Layout.preferredWidth: Constants.navBar.connectButtonWidth
            Layout.preferredHeight: Constants.navBar.buttonHeight
            checkable: true
            text: !checked ? "Connect" : "Disconnect"
            onClicked: {
                if (!checked) {
                    data_model.disconnect();
                } else {
                    if (navBarSourceSelection.currentText === tcp_ip) {
                        if (tcpUrlBarText.text && tcpPortBarText.text)
                            data_model.connect_tcp(tcpUrlBarText.text, tcpPortBarText.text);
                        else
                            data_model.connect();
                    } else if (navBarSourceSelection.currentText === file) {
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

            Component.onCompleted: {
                refreshRateDrop.indicator.width = Constants.navBar.plotRefreshRateDropdownWidth / 3;
            }
            visible: true
            Layout.preferredWidth: Constants.navBar.plotRefreshRateDropdownWidth
            Layout.preferredHeight: Constants.navBar.dropdownHeight
            ToolTip.visible: hovered
            ToolTip.text: "Refresh Rate (Hz)"
            model: Constants.navBar.all_refresh_rates
            currentIndex: Constants.navBar.default_refresh_rate_index
            onActivated: {
                Globals.currentRefreshRate = 1000 / Constants.navBar.all_refresh_rates[currentIndex];
            }

            states: State {
                when: refreshRateDrop.down

                PropertyChanges {
                    target: refreshRateDrop
                    width: Constants.navBar.plotRefreshRateDropdownWidth * 1.5
                }

            }

        }

        Timer {
            interval: Constants.defaultTimerIntervalRate
            running: true
            repeat: true
            onTriggered: {
                nav_bar_model.fill_data(navBarData);
                if (!navBarData.available_baudrates.length)
                    return ;

                if (available_baudrates.length == 0) {
                    available_baudrates = navBarData.available_baudrates;
                    serialDeviceBaudRate.currentIndex = 1;
                }
                if (available_flows.length == 0)
                    available_flows = navBarData.available_flows;

                available_devices = navBarData.available_ports;
            }
        }

    }

}
