import "Constants"
import QtCharts 2.2
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property string tcp_ip: "TCP/IP"
    property string serial_usb: "Serial/USB"
    property string file: "File"
    property var sources: [tcp_ip, serial_usb, file]
    property variant available_baudrates: []
    property variant available_devices: []
    property variant available_flows: []
    property variant previous_hosts: []
    property variant previous_ports: []
    property variant previous_files: []
    property variant log_level_labels: []

    anchors.fill: parent
    border.width: Constants.statusBar.borderWidth
    border.color: Constants.statusBar.borderColor

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

            ComboBox {
                id: tcpUrlBar

                height: parent.height
                width: 3 * parent.width / 4
                model: previous_hosts
                editable: true
                selectTextByMouse: true

                Text {
                    text: "Host"
                    color: Constants.navBar.placeholderTextColor
                    visible: (!tcpUrlBar.editText)
                }

            }

            ComboBox {
                id: tcpPortBar

                height: parent.height
                width: parent.width / 4
                model: previous_ports
                editable: true
                selectTextByMouse: true

                Text {
                    text: "Port"
                    color: Constants.navBar.placeholderTextColor
                    visible: !tcpPortBar.editText
                }

            }

        }

        ComboBox {
            id: fileUrlBar

            visible: false
            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            model: previous_files
            editable: true
            selectTextByMouse: true

            Text {
                text: "path/to/file"
                color: Constants.navBar.placeholderTextColor
                visible: !fileUrlBar.editText
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
            id: connectButton

            Layout.preferredWidth: Constants.navBar.connectButtonWidth
            Layout.preferredHeight: Constants.navBar.buttonHeight
            checkable: true
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "Connect" : "Disconnect"
            onClicked: {
                if (!checked) {
                    data_model.disconnect();
                } else {
                    if (navBarSourceSelection.currentText === tcp_ip) {
                        if (tcpUrlBar.editText && tcpPortBar.editText)
                            data_model.connect_tcp(tcpUrlBar.editText, tcpPortBar.editText);
                        else
                            data_model.connect();
                    } else if (navBarSourceSelection.currentText === file) {
                        if (fileUrlBar.editText)
                            data_model.connect_file(fileUrlBar.editText);

                    } else {
                        data_model.connect_serial(serialDevice.currentText, serialDeviceBaudRate.currentText, serialDeviceFlowControl.currentText);
                    }
                }
            }

            Image {
                id: navBarConnect

                anchors.centerIn: parent
                width: Constants.navBar.buttonSvgHeight
                height: Constants.navBar.buttonSvgHeight
                smooth: true
                source: Constants.navBar.connectButtonPath
                visible: false
                antialiasing: true
            }

            ColorOverlay {
                anchors.fill: navBarConnect
                source: navBarConnect
                color: !connectButton.checked ? "dimgrey" : "crimson"
                antialiasing: true
            }

        }

        Button {
            id: folderBarButton

            Layout.preferredWidth: Constants.navBar.folderButtonWidth
            Layout.preferredHeight: Constants.navBar.buttonHeight
            checkable: true
            ToolTip.visible: hovered
            ToolTip.text: "Logging"
            onClicked: {
                if (!checked)
                    loggingBar.visible = false;
                else
                    loggingBar.visible = true;
            }

            Image {
                id: navBarFolder

                anchors.centerIn: parent
                width: Constants.navBar.buttonSvgHeight
                height: Constants.navBar.buttonSvgHeight
                smooth: true
                source: Constants.navBar.folderButtonPath
                visible: false
                antialiasing: true
            }

            ColorOverlay {
                anchors.fill: navBarFolder
                source: navBarFolder
                color: !folderBarButton.checked ? "dimgrey" : "crimson"
                antialiasing: true
            }

        }

        ComboBox {
            id: logLevelButton

            Layout.preferredWidth: Constants.navBar.logLevelButtonWidth
            Layout.preferredHeight: Constants.navBar.buttonHeight
            model: log_level_labels
            ToolTip.visible: hovered
            ToolTip.text: "Log Level"
            onActivated: data_model.log_level(logLevelButton.currentText)
        }

        Timer {
            interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                nav_bar_model.fill_data(navBarData);
                if (!navBarData.available_baudrates.length)
                    return ;

                if (!available_baudrates.length || !available_flows.length || !log_level_labels.length) {
                    available_baudrates = navBarData.available_baudrates;
                    serialDeviceBaudRate.currentIndex = 1;
                    available_flows = navBarData.available_flows;
                    log_level_labels = navBarData.log_level_labels;
                }
                available_devices = navBarData.available_ports;
                previous_hosts = navBarData.previous_hosts;
                previous_ports = navBarData.previous_ports;
                previous_files = navBarData.previous_files;
                connectButton.checked = navBarData.connected;
            }
        }

    }

}
