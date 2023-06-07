import "../BaseComponents"
import "../Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: ntripClientTab

    property bool connected: false
    property var floatValidator
    property var intValidator
    property var stringValidator

    RowLayout {
        anchors.fill: parent

        ColumnLayout {
            width: 200
            Layout.alignment: Qt.AlignTop | Qt.AlignLeft

            Repeater {
                id: generalRepeater

                model: ["Url", "Username", "Password", "GGA Period"]

                RowLayout {
                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    TextField {
                        Layout.preferredWidth: 250
                        text: {
                            if (modelData == "Url")
                                return "na.skylark.swiftnav.com:2101";

                            if (modelData == "GGA Period")
                                return "10";

                            return "";
                        }
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                        validator: {
                            if (modelData == "GGA Period")
                                return intValidator;

                            return stringValidator;
                        }
                        readOnly: connected
                    }

                }

            }

            Repeater {
                id: positionRepeater

                model: ["Lat", "Lon", "Alt"]

                RowLayout {
                    visible: staticRadio.checked

                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    TextField {
                        id: textField

                        Layout.preferredWidth: 250
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                        validator: floatValidator
                        text: {
                            if (modelData == "Lat")
                                return "37.77101999622968";

                            if (modelData == "Lon")
                                return "-122.40315159140708";

                            if (modelData == "Alt")
                                return "-5.549358852471994";

                            return "";
                        }
                        readOnly: connected
                    }

                }

            }

        }

        ColumnLayout {
            width: 200
            height: parent.height
            Layout.alignment: Qt.AlignTop | Qt.AlignLeft

            RadioButton {
                checked: true
                text: "Dynamic"
                ToolTip.visible: hovered
                ToolTip.text: "Allow automatically fetching position from device"
                enabled: !connected
            }

            RadioButton {
                id: staticRadio

                text: "Static"
                ToolTip.visible: hovered
                ToolTip.text: "Allow user input position"
                enabled: !connected
            }

            RowLayout {
                ComboBox {
                    id: binaryPath

                    Layout.preferredWidth: 200
                    editable: true
                    selectTextByMouse: true

                    Label {
                        anchors.fill: parent.contentItem
                        anchors.leftMargin: 4
                        verticalAlignment: Text.AlignVCenter
                        text: "Enter conveter binary path"
                        color: Constants.loggingBar.placeholderTextColor
                        visible: !binaryPath.editText
                    }

                }

                SwiftButton {
                    id: localFileSelectionButton

                    Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonWidth
                    onClicked: fileDialog.visible = !fileDialog.visible

                    Label {
                        text: Constants.updateTab.dotDotDotLabel
                        anchors.centerIn: parent
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                    }

                }

                FileDialog {
                    id: fileDialog

                    selectExisting: true
                    folder: shortcuts.home + "/" + Constants.settingsTab.defaultImportExportRelativePathFromHome
                    onAccepted: {
                        binaryPath.editText = Utils.fileUrlToString(fileUrl);
                    }
                }

            }

        }

        ColumnLayout {
            width: 600
            height: parent.height
            Layout.alignment: Qt.AlignTop | Qt.AlignLeft

            Label {
                id: inputErrorLabel

                visible: false
                text: ""
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
                color: "red"
            }

            RowLayout {
                SwiftButton {
                    invertColor: true
                    icon.width: 10
                    icon.height: 10
                    icon.source: Constants.icons.playPath
                    icon.color: Constants.materialGrey
                    ToolTip.visible: hovered
                    ToolTip.text: "Start"
                    enabled: !connected
                    onClicked: {
                        let url = generalRepeater.itemAt(0).children[1].text;
                        if (!url) {
                            inputErrorLabel.text = "URL is not provided!";
                            inputErrorLabel.visible = true;
                            return ;
                        }
                        let username = generalRepeater.itemAt(1).children[1].text;
                        let password = generalRepeater.itemAt(2).children[1].text;
                        let ggaPeriod = generalRepeater.itemAt(3).children[1].text;
                        if (!ggaPeriod) {
                            inputErrorLabel.text = "GGA Period is not provided!";
                            inputErrorLabel.visible = true;
                            return ;
                        }
                        let lat = null;
                        let lon = null;
                        let alt = null;
                        if (staticRadio.checked) {
                            lat = positionRepeater.itemAt(0).children[1].text;
                            lon = positionRepeater.itemAt(1).children[1].text;
                            alt = positionRepeater.itemAt(2).children[1].text;
                            if (!lat || !lon || !alt) {
                                inputErrorLabel.text = "Position missing!";
                                inputErrorLabel.visible = true;
                                return ;
                            }
                        }
                        backend_request_broker.ntrip_connect(url, username, password, ggaPeriod, lat, lon, alt, binaryPath.editText);
                        connected = true;
                        inputErrorLabel.visible = false;
                    }
                }

                SwiftButton {
                    invertColor: true
                    icon.width: 10
                    icon.height: 10
                    icon.source: Constants.icons.pauseButtonUrl
                    icon.color: Constants.materialGrey
                    ToolTip.visible: hovered
                    ToolTip.text: "Stop"
                    enabled: connected
                    onClicked: {
                        backend_request_broker.ntrip_disconnect();
                        connected = false;
                        inputErrorLabel.visible = false;
                    }
                }

            }

        }

    }

    NtripStatusData {
        id: ntripStatusData

        signal ntrip_connected(bool connected)

        function setConnection(connected) {
            ntripClientTab.connected = connected;
        }

        Component.onCompleted: {
            ntripStatusData.ntrip_connected.connect(setConnection);
        }
    }

    floatValidator: DoubleValidator {
    }

    intValidator: IntValidator {
    }

    stringValidator: RegExpValidator {
    }

}
