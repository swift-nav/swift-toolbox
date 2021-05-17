import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    anchors.fill: parent
    border.width: Constants.statusBar.borderWidth
    border.color: Constants.statusBar.borderColor

    StatusBarData {
        id: statusBarData
    }

    RowLayout {
        id: statusBarRowLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.statusBar.margin
        anchors.rightMargin: Constants.statusBar.margin
        spacing: Constants.statusBar.spacing

        Row {
            id: statusBarRowPort

            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "Port: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarPort

                font.pointSize: Constants.largePointSize
            }

        }

        Row {
            id: statusBarRowPos

            Layout.minimumWidth: statusBarRowLayout.width * Constants.statusBar.smallKeyWidthRatio
            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "Pos: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarPos

                font.pointSize: Constants.largePointSize
            }

        }

        Row {
            id: statusBarRowRTK

            Layout.minimumWidth: statusBarRowLayout.width * Constants.statusBar.smallKeyWidthRatio
            Layout.alignment: Qt.AlignLeft
            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "RTK: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarRTK

                font.pointSize: Constants.largePointSize
            }

        }

        Row {
            id: statusBarRowSats

            Layout.minimumWidth: statusBarRowLayout.width * Constants.statusBar.smallKeyWidthRatio
            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "Sats: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarSats

                font.pointSize: Constants.largePointSize
            }

        }

        Row {
            id: statusBarRowCorrAge

            Layout.minimumWidth: statusBarRowLayout.width * Constants.statusBar.smallKeyWidthRatio
            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "Corr Age: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarCorrAge

                font.pointSize: Constants.largePointSize
            }

        }

        Row {
            id: statusBarRowINS

            Layout.minimumWidth: statusBarRowLayout.width * Constants.statusBar.smallKeyWidthRatio
            spacing: Constants.statusBar.innerKeyValSpacing

            Text {
                text: "INS: "
                color: Constants.statusBar.keyTextColor
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Text {
                id: statusBarINS

                font.pointSize: Constants.largePointSize
            }

        }

        RowLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignRight

            Text {
                id: statusBarDataRate

                Layout.alignment: Qt.AlignRight
                font.pointSize: Constants.largePointSize
                font.bold: true
            }

            Image {
                id: statusBarGoodConnectionImage

                visible: false
                width: Constants.statusBar.arrowsSideLength
                height: Constants.statusBar.arrowsSideLength
                Layout.alignment: Qt.AlignRight
                source: Constants.statusBar.arrowsBluePath
            }

            Image {
                id: statusBarBadConnectionImage

                visible: true
                width: Constants.statusBar.arrowsSideLength
                height: Constants.statusBar.arrowsSideLength
                Layout.alignment: Qt.AlignRight
                source: Constants.statusBar.arrowsGreyPath
            }

        }

        Timer {
            // if (!statusBarData.available_baudrates.length)
            //     return ;
            // if (!available_baudrates.length || !available_flows.length || available_refresh_rates.length) {
            //     available_baudrates = statusBarData.available_baudrates;
            //     serialDeviceBaudRate.currentIndex = 1;
            //     available_flows = statusBarData.available_flows;
            //     available_refresh_rates = statusBarData.available_refresh_rates;
            //     refreshRateDrop.currentIndex = available_refresh_rates.indexOf(Globals.currentRefreshRate);
            // }
            // available_devices = statusBarData.available_ports;
            // previous_hosts = statusBarData.previous_hosts;
            // previous_ports = statusBarData.previous_ports;
            // previous_files = statusBarData.previous_files;
            // connectButton.checked = statusBarData.connected;

            interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                status_bar_model.fill_data(statusBarData);
                statusBarPort.text = statusBarData.port;
                statusBarPos.text = statusBarData.pos;
                statusBarRTK.text = statusBarData.rtk;
                statusBarSats.text = statusBarData.sats;
                statusBarCorrAge.text = statusBarData.corr_age;
                statusBarINS.text = statusBarData.ins;
                statusBarDataRate.text = statusBarData.data_rate;
                if (statusBarData.solid_connection) {
                    statusBarGoodConnectionImage.visible = true;
                    statusBarBadConnectionImage.visible = false;
                } else {
                    statusBarGoodConnectionImage.visible = false;
                    statusBarBadConnectionImage.visible = true;
                }
            }
        }

    }

}
