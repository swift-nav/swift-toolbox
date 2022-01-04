import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.3 as Dialogs
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
    property variant previous_hosts: []
    property variant previous_ports: []
    property variant previous_files: []
    property variant previous_serial_configs: []
    property variant last_used_serial_device: null
    property string connMessage: ""
    property bool warningTimerRecentlyUsed: false

    function restore_previous_serial_settings(device_name) {
        const config = previous_serial_configs.find((element) => {
            return element[0] === device_name;
        });
        if (config) {
            serialDeviceBaudRate.currentIndex = available_baudrates.indexOf(config[1]);
            serialDeviceFlowControl.currentIndex = available_flows.indexOf(config[2]);
        }
    }

    ConnectionData {
        id: connectionData
    }

    Rectangle {
        id: dialogRect

        anchors.fill: parent
        Keys.onReturnPressed: {
            connectButton.clicked();
        }

        Image {
            width: parent.width
            height: parent.height
            source: Constants.icons.splashScreenPath
        }

        Rectangle {
            anchors.left: parent.left
            height: parent.height
            width: 1
            color: "white"
        }

        Dialog {
            id: dialog

            visible: stack.connectionScreenVisible()
            implicitHeight: 3 * parent.height / 7
            implicitWidth: parent.width / 2
            anchors.centerIn: parent
            title: "Connect to device..."
            closePolicy: Popup.NoAutoClose
            onVisibleChanged: {
                if (visible)
                    dialogRect.forceActiveFocus();

            }

            ColumnLayout {
                anchors.fill: parent

                RowLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop

                    RadioButton {
                        id: tcpRadio

                        checked: true
                        text: tcp_ip
                        onToggled: dialogRect.forceActiveFocus()
                    }

                    RadioButton {
                        id: serialRadio

                        text: serial_usb
                        onToggled: dialogRect.forceActiveFocus()
                    }

                    RadioButton {
                        id: fileRadio

                        text: file
                        onToggled: dialogRect.forceActiveFocus()
                    }

                    Item {
                        Layout.fillWidth: true
                    }

                }

                RowLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop

                    ComboBox {
                        id: serialDevice

                        visible: serialRadio.checked
                        Layout.preferredHeight: Constants.connection.dropdownHeight
                        Layout.fillWidth: true
                        model: available_devices
                        onActivated: {
                            restore_previous_serial_settings(available_devices[currentIndex]);
                        }
                        Keys.onReturnPressed: {
                            connectButton.clicked();
                        }
                    }

                    Button {
                        id: serialDeviceRefresh

                        visible: serialRadio.checked
                        Layout.preferredHeight: Constants.connection.buttonHeight
                        Layout.preferredWidth: Constants.connection.serialDeviceRefreshWidth
                        icon.source: Constants.icons.refreshPath
                        icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
                        onClicked: {
                            data_model.serial_refresh();
                        }
                    }

                    ComboBox {
                        id: serialDeviceBaudRate

                        visible: serialRadio.checked
                        Layout.preferredHeight: Constants.connection.dropdownHeight
                        Layout.preferredWidth: Constants.connection.serialDeviceBaudRateDropdownWidth
                        model: available_baudrates
                        Keys.onReturnPressed: {
                            connectButton.clicked();
                        }
                    }

                    ComboBox {
                        id: serialDeviceFlowControl

                        visible: serialRadio.checked
                        Layout.preferredHeight: Constants.connection.dropdownHeight
                        Layout.preferredWidth: Constants.connection.serialDeviceFlowControlDropdownWidth
                        model: available_flows
                        Keys.onReturnPressed: {
                            connectButton.clicked();
                        }

                        states: State {
                            when: serialDeviceFlowControl.down

                            PropertyChanges {
                                target: serialDeviceFlowControl
                                width: Constants.connection.serialDeviceFlowControlDropdownWidth * 1.1
                            }

                        }

                    }

                    Item {
                        id: serialDeviceFill

                        visible: serialRadio.checked
                        Layout.fillWidth: true
                    }

                    ComboBox {
                        id: tcpUrlBar

                        visible: tcpRadio.checked
                        Layout.fillWidth: true
                        model: previous_hosts
                        editable: true
                        selectTextByMouse: true
                        onAccepted: {
                            connectButton.clicked();
                        }

                        Label {
                            anchors.fill: parent.contentItem
                            anchors.leftMargin: 4
                            verticalAlignment: Text.AlignVCenter
                            text: "Host"
                            color: Constants.connection.placeholderTextColor
                            visible: (!tcpUrlBar.editText)
                        }

                    }

                    ComboBox {
                        id: tcpPortBar

                        visible: tcpRadio.checked
                        Layout.preferredWidth: parent.width / 4
                        model: previous_ports
                        editable: true
                        selectTextByMouse: true
                        onAccepted: {
                            connectButton.clicked();
                        }

                        Label {
                            anchors.fill: parent.contentItem
                            anchors.leftMargin: 4
                            verticalAlignment: Text.AlignVCenter
                            text: "Port"
                            color: Constants.connection.placeholderTextColor
                            visible: !tcpPortBar.editText
                        }

                    }

                    ComboBox {
                        id: fileUrlBar

                        visible: fileRadio.checked
                        Layout.alignment: Qt.AlignLeft
                        Layout.fillWidth: true
                        model: previous_files
                        editable: true
                        selectTextByMouse: true
                        onAccepted: {
                            connectButton.clicked();
                        }

                        Label {
                            anchors.fill: parent.contentItem
                            anchors.leftMargin: 4
                            verticalAlignment: Text.AlignVCenter
                            text: "path/to/file"
                            color: Constants.connection.placeholderTextColor
                            visible: !fileUrlBar.editText
                        }

                    }

                }

                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                RowLayout {
                    Layout.fillWidth: true

                    Item {
                        Layout.fillWidth: true
                    }

                    Button {
                        id: connectButton

                        property string tooltipText: "Connect"

                        Layout.preferredWidth: parent.width / 4
                        checkable: true
                        state: Constants.connection.disconnected
                        ToolTip.visible: hovered
                        ToolTip.text: tooltipText
                        onClicked: {
                            if (connectButton.state == Constants.connection.connected || connectButton.state == Constants.connection.connecting) {
                                connectButton.state = Constants.connection.disconnecting;
                                data_model.disconnect();
                            } else if (connectButton.state == Constants.connection.disconnected) {
                                connectButton.state = Constants.connection.connecting;
                                if (tcpRadio.checked) {
                                    if (tcpUrlBar.editText && tcpPortBar.editText)
                                        data_model.connect_tcp(tcpUrlBar.editText, tcpPortBar.editText);
                                    else
                                        data_model.connect();
                                } else if (fileRadio.checked) {
                                    if (fileUrlBar.editText)
                                        data_model.connect_file(fileUrlBar.editText);

                                } else {
                                    data_model.connect_serial(serialDevice.currentText, serialDeviceBaudRate.currentText, serialDeviceFlowControl.currentText);
                                }
                            }
                        }
                        states: [
                            State {
                                name: Constants.connection.connecting

                                PropertyChanges {
                                    target: connectButton
                                    enabled: true
                                    checked: true
                                    text: "Connecting"
                                    tooltipText: "Disconnect"
                                }

                            },
                            State {
                                name: Constants.connection.connected

                                PropertyChanges {
                                    target: connectButton
                                    enabled: true
                                    checked: true
                                    text: "Disconnect"
                                    tooltipText: "Disconnect"
                                }

                            },
                            State {
                                name: Constants.connection.disconnecting

                                PropertyChanges {
                                    target: connectButton
                                    enabled: false
                                    checked: false
                                    text: "Disconnecting"
                                    tooltipText: "Disconnecting"
                                }

                            },
                            State {
                                name: Constants.connection.disconnected

                                PropertyChanges {
                                    target: connectButton
                                    enabled: true
                                    checked: false
                                    text: "Connect"
                                    tooltipText: "Connect"
                                }

                            }
                        ]
                    }

                }

            }

            Timer {
                interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
                running: true
                repeat: true
                onTriggered: {
                    connection_model.fill_data(connectionData);
                    if (!connectionData.available_baudrates.length)
                        return ;

                    if (!available_baudrates.length || !available_flows.length) {
                        Globals.consoleVersion = connectionData.console_version;
                        available_baudrates = connectionData.available_baudrates;
                        serialDeviceBaudRate.currentIndex = 1;
                        available_flows = connectionData.available_flows;
                    }
                    available_devices = connectionData.available_ports;
                    previous_hosts = connectionData.previous_hosts;
                    previous_ports = connectionData.previous_ports;
                    previous_files = connectionData.previous_files;
                    previous_serial_configs = connectionData.previous_serial_configs;
                    if (!last_used_serial_device && connectionData.last_used_serial_device) {
                        // Set the default selected to the last used
                        last_used_serial_device = connectionData.last_used_serial_device;
                        serialDevice.currentIndex = available_devices.indexOf(last_used_serial_device);
                        if (serialDevice.currentIndex != -1)
                            restore_previous_serial_settings(available_devices[serialDevice.currentIndex]);

                    }
                    if (connectionData.connection_message !== "") {
                        connMessage = connectionData.connection_message;
                        warningTimer.startTimer();
                    }
                    connectButton.state = connectionData.conn_state.toLowerCase();
                    if (!Globals.connected_at_least_once && connectionData.conn_state == Constants.connection.connected.toUpperCase()) {
                        stack.mainView();
                        Globals.connected_at_least_once = true;
                    }
                    Globals.conn_state = connectionData.conn_state;
                }
            }

        }

        Timer {
            id: warningTimer

            function startTimer() {
                if (!warningTimerRecentlyUsed) {
                    warningTimerRecentlyUsed = true;
                    connectionMessage.visible = true;
                    warningTimer.start();
                }
            }

            interval: Constants.connection.warningTimerLockedInterval
            repeat: false
            onTriggered: {
                warningTimerRecentlyUsed = false;
            }
        }

        Dialogs.MessageDialog {
            id: connectionMessage

            title: "Connection Message"
            text: connMessage
            icon: Dialogs.StandardIcon.Warning
            standardButtons: Dialogs.StandardButton.Cancel
        }

    }

}
