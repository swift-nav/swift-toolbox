import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15

Item {
    width: parent.width
    height: parent.height

    
    // property var tcp_ip: {"a": "Host:Port", "b": "TCP/IP"}
    // property var serial_usb: {"a": "", "b": "Serial/USB"}
    // property var file: {"a": "path/to/file", "b": "File"}
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
            Layout.leftMargin: Constants.navBarMargin
            Layout.bottomMargin: 10
            id: bottomNavBarSourceSelection
            
            model: sources

            //[sources[0]["a"], sources[1]["a"]]
            
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
            height: 25
            visible: false
            // Layout.preferredWidth: 100
            Layout.bottomMargin: 10
            id: serialDevice

            model: ["usb0", "usb1"]
            
            onActivated: {
                
            }
        }
        Button {
            id: serialDeviceRefresh
            visible: false
            Layout.preferredWidth: 30
            Layout.bottomMargin: 10
            text: "F5"
        }
        ComboBox {
            height: 25
            visible: false
            // Layout.preferredWidth: 100
            Layout.bottomMargin: 10
            id: serialDeviceBaudRate
            currentIndex: 1

            model: [57600, 115200, 230400, 460800, 921600, 1000000]
            
            onActivated: {
                
            }
        }
        ComboBox {
            height: 25
            visible: false
            // Layout.preferredWidth: 120
            Layout.bottomMargin: 10
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
                    color: "#CDC9C9"
                    visible: !urlBarText.text
                }
            }

        }
        Button {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: 10
            Layout.bottomMargin: 10
            checkable: true
            text: "Connect"
            onClicked: data_model.connect()
        }
        Timer {
            interval: 1000 / 5 // 5 Hz refresh
            running: true
            repeat: true
            onTriggered: {
                if (urlBar.children[0].text != source_defaults[bottomNavBarSourceSelection.currentText]){
                    urlBar.children[0].text = source_defaults[bottomNavBarSourceSelection.currentText];
                }
                
            }
        }

    }

}
