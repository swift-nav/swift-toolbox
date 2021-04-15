import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15

Item {
    width: parent.width
    height: parent.height
    property string tcp_ip: "TCP/IP"
    property string serial_usb: "Serial/USB"
    property string file: "File"
    property var sources: [tcp_ip, serial_usb, file]
    property var source_defaults: {"TCP/IP": "Host:Port", "Serial/USB": "", "File": "path/to/file"}
    RowLayout {
        width: parent.width
        height: parent.height
        id: bottomNavBarRowLayout
        ComboBox {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: Constants.bottomNavBar.navBarMargin
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            id: bottomNavBarSourceSelection
            
            model: sources
            
            onActivated: {
                if (find(tcp_ip) === currentIndex || find(file) === currentIndex){
                    urlBar.visible = true
                } else {
                    urlBar.visible = false
                }
                if(find(serial_usb) === currentIndex) {
                    serialDevice.visible = true
                    serialDeviceRefresh.visible = true
                    serialDeviceBaudRate.visible = true
                    serialDeviceFlowControl.visible = true
                } else {
                    serialDevice.visible = false
                    serialDeviceRefresh.visible = false
                    serialDeviceBaudRate.visible = false
                    serialDeviceFlowControl.visible = false
                }
                
            }

        }
        ComboBox {
            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            id: serialDevice

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
            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            id: serialDeviceBaudRate
            currentIndex: 1

            model: [57600, 115200, 230400, 460800, 921600, 1000000]
            
            onActivated: {
                
            }
        }
        ComboBox {
            visible: false
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            id: serialDeviceFlowControl

            model: ["None", "Hardware RTS/CTS"]
            
            onActivated: {
                
            }
        }
        

        Rectangle {
            id: urlBar

            height: 25
            Layout.preferredWidth: parent.width / 2
            border.width: 1

            TextInput {
                id: urlBarText
                anchors.fill: parent
                anchors.margins: 4
                onTextChanged: {
                }
                
                Text {
                    id: placeholderText
                    text: ""
                    color: Constants.bottomNavBar.placeholderTextColor
                    visible: !urlBarText.text
                }
            }

        }
        Button {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: Constants.bottomNavBar.navBarMargin
            Layout.bottomMargin: Constants.bottomNavBar.navBarMargin
            checkable: true
            text: "Connect"
            onClicked: data_model.connect(checked)
        }
        Timer {
            interval: 1000 / 5 // 5 Hz refresh
            running: true
            repeat: true
            onTriggered: {
                if (placeholderText.text != source_defaults[bottomNavBarSourceSelection.currentText]){
                    placeholderText.text = source_defaults[bottomNavBarSourceSelection.currentText];
                }
                
            }
        }

    }

}
