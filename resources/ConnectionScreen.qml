import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
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
        width: parent.width
        height: parent.height
        anchors.centerIn: parent

        Image {
            width: parent.width
            height: parent.height
            source: Constants.icons.splashScreenPath
        }

        Dialog {
            id: dialog

            visible: stack.connectionScreenVisible()
            implicitHeight: 3 * parent.height / 7
            implicitWidth: parent.width / 2
            anchors.centerIn: parent
            title: "Connect to device..."
            closePolicy: Popup.NoAutoClose

            ColumnLayout {
                anchors.fill: parent

                RowLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop

                    RadioButton {
                        id: tcpRadio

                        checked: true
                        text: tcp_ip
                    }

                    RadioButton {
                        id: serialRadio

                        text: serial_usb
                    }

                    RadioButton {
                        id: fileRadio

                        text: file
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
                    }

                    ComboBox {
                        id: serialDeviceFlowControl

                        visible: serialRadio.checked
                        Layout.preferredHeight: Constants.connection.dropdownHeight
                        Layout.preferredWidth: Constants.connection.serialDeviceFlowControlDropdownWidth
                        model: available_flows

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

                        Layout.preferredWidth: parent.width / 4
                        checkable: true
                        checked: Globals.conn_state == Constants.connection.connected
                        enabled: Globals.conn_state == Constants.connection.disconnected || Globals.conn_state == Constants.connection.connected
                        ToolTip.visible: hovered
                        ToolTip.text: !checked ? "Connect" : "Disconnect"
                        text: !checked ? "Connect" : "Disconnect"
                        onClicked: {
                            if (!checked) {
                                data_model.disconnect();
                            } else {
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
                    let connected = connectionData.conn_state == Constants.connection.connected;
                    if (Globals.conn_state == Constants.connection.disconnected && stack.connectionScreenVisible() && connected) {
                        stack.mainView();
                        Globals.connected_at_least_once = true;
                    }
                    Globals.conn_state = connectionData.conn_state;
                }
            }

        }

    }

}
