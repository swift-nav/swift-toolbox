import "BaseComponents"
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
    property string previous_connection_type: ""
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
    property string connectedConstant: Constants.connection.connected.toUpperCase()
    property string connectingConstant: Constants.connection.connecting.toUpperCase()
    property string disconnectedConstant: Constants.connection.disconnected.toUpperCase()
    property string disconnectingConstant: Constants.connection.disconnecting.toUpperCase()

    function restore_previous_serial_settings(device_name) {
        const config = previous_serial_configs.find((element) => {
            return element[0] === device_name;
        });
        if (config) {
            serialDeviceBaudRate.currentIndex = available_baudrates.indexOf(config[1]);
            serialDeviceFlowControl.currentIndex = available_flows.indexOf(config[2]);
        }
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
            implicitHeight: 3 * Globals.height / 7
            implicitWidth: Globals.width / 2
            anchors.centerIn: parent
            title: "Connect to Device"
            onVisibleChanged: {
                if (visible)
                    dialogRect.forceActiveFocus();

                if (typeof (data_model) !== "undefined")
                    data_model.connection_dialog_status(visible);

            }
            onClosed: {
                stack.mainView();
            }

            ColumnLayout {
                anchors.fill: parent

                RowLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop

                    RadioButton {
                        id: serialRadio

                        checked: (previous_connection_type == "Serial" || previous_connection_type == "File" && !Globals.showFileConnection)
                        text: serial_usb
                        onToggled: dialogRect.forceActiveFocus()
                    }

                    RadioButton {
                        id: tcpRadio

                        checked: previous_connection_type == "Tcp"
                        text: tcp_ip
                        onToggled: dialogRect.forceActiveFocus()
                    }

                    RadioButton {
                        id: fileRadio

                        checked: previous_connection_type == "File" && Globals.showFileConnection
                        text: file
                        onToggled: dialogRect.forceActiveFocus()
                        visible: Globals.showFileConnection
                    }

                    Item {
                        Layout.fillWidth: true
                    }

                }

                Label {
                    id: connectionMessage

                    visible: false
                    text: connMessage
                    Layout.leftMargin: Constants.connection.labelLeftMargin
                    color: "red"
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    ToolTip {
                        id: tooltip

                        visible: connectButton.state == Constants.connection.connected && mouseArea.containsMouse
                        text: "Disconnect before connecting to a new device."
                    }

                    MouseArea {
                        id: mouseArea

                        anchors.fill: parent
                        hoverEnabled: true
                    }

                    GridLayout {
                        anchors.fill: parent
                        rowSpacing: Constants.connection.labelRowSpacing
                        rows: 2
                        visible: serialRadio.checked
                        flow: GridLayout.TopToBottom
                        enabled: connectButton.state !== Constants.connection.connected

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.serialLabel
                        }

                        ComboBox {
                            id: serialDevice

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

                        Label {
                        }

                        Button {
                            id: serialDeviceRefresh

                            Layout.preferredHeight: Constants.connection.buttonHeight
                            Layout.preferredWidth: Constants.connection.serialDeviceRefreshWidth
                            icon.source: Constants.icons.refreshPath
                            icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
                            onClicked: {
                                data_model.serial_refresh();
                            }
                        }

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.baudrateLabel
                        }

                        ComboBox {
                            id: serialDeviceBaudRate

                            Layout.preferredHeight: Constants.connection.dropdownHeight
                            Layout.preferredWidth: Constants.connection.serialDeviceBaudRateDropdownWidth
                            model: available_baudrates
                            Keys.onReturnPressed: {
                                connectButton.clicked();
                            }
                        }

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.flowLabel
                        }

                        ComboBox {
                            id: serialDeviceFlowControl

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

                            Layout.fillWidth: true
                        }

                    }

                    GridLayout {
                        anchors.fill: parent
                        rowSpacing: Constants.connection.labelRowSpacing
                        rows: 2
                        visible: tcpRadio.checked
                        flow: GridLayout.TopToBottom
                        enabled: connectButton.state !== Constants.connection.connected

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.hostLabel
                        }

                        ComboBox {
                            id: tcpUrlBar

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

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.portLabel
                        }

                        ComboBox {
                            id: tcpPortBar

                            Layout.preferredWidth: parent.width / 4
                            model: previous_ports
                            editable: true
                            selectTextByMouse: true
                            onAccepted: {
                                connectButton.clicked();
                            }
                            onEditTextChanged: {
                                // This will perform the same validation but live.
                                this.editText = Math.max(0, Math.min(this.editText, 65535));
                            }

                            Label {
                                anchors.fill: parent.contentItem
                                anchors.leftMargin: 4
                                verticalAlignment: Text.AlignVCenter
                                text: "Port"
                                color: Constants.connection.placeholderTextColor
                                visible: !tcpPortBar.editText
                            }

                            validator: IntValidator {
                                bottom: 0
                                top: 65535
                            }

                        }

                    }

                    GridLayout {
                        anchors.fill: parent
                        rowSpacing: Constants.connection.labelRowSpacing
                        rows: 2
                        visible: fileRadio.checked
                        flow: GridLayout.TopToBottom
                        enabled: connectButton.state !== Constants.connection.connected

                        Label {
                            Layout.leftMargin: Constants.connection.labelLeftMargin
                            text: Constants.connection.fileLabel
                        }

                        ComboBox {
                            id: fileUrlBar

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

                }

                RowLayout {
                    Layout.fillWidth: true

                    Item {
                        Layout.fillWidth: true
                    }

                    SwiftButton {
                        id: closeButton

                        text: "Cancel"
                        Layout.preferredWidth: parent.width / 4
                        checkable: false
                        onClicked: {
                            dialog.close();
                        }
                    }

                    SwiftButton {
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

                                PropertyChanges {
                                    target: dialog
                                    title: "Connecting..."
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

                                PropertyChanges {
                                    target: dialog
                                    title: "Connected to Device"
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

                                PropertyChanges {
                                    target: dialog
                                    title: "Disconnecting..."
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

                                PropertyChanges {
                                    target: dialog
                                    title: "Connect to Device"
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
                    previous_connection_type = connectionData.previous_connection_type;
                    if (!last_used_serial_device && connectionData.last_used_serial_device) {
                        // Set the default selected to the last used
                        last_used_serial_device = connectionData.last_used_serial_device;
                        serialDevice.currentIndex = available_devices.indexOf(last_used_serial_device);
                        if (serialDevice.currentIndex != -1)
                            restore_previous_serial_settings(available_devices[serialDevice.currentIndex]);

                    }
                    if (connectionData.connection_message !== "") {
                        connMessage = connectionData.connection_message;
                        connectionMessage.visible = true;
                    }
                    connectButton.state = connectionData.conn_state.toLowerCase();
                    if ([disconnectedConstant, connectingConstant].includes(Globals.conn_state) && connectionData.conn_state == connectedConstant) {
                        connectionMessage.visible = false;
                        connMessage = "";
                        dialog.close();
                    }
                    Globals.conn_state = connectionData.conn_state;
                }
            }

        }

    }

}
