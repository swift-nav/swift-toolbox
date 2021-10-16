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

        RowLayout {
            Row {
                id: statusBarRowPos

                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.posLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarPos

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

            Row {
                id: statusBarRowRTK

                Layout.alignment: Qt.AlignLeft
                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.rtkLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarRTK

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

            Row {
                id: statusBarRowINS

                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.insLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarINS

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

            Row {
                id: statusBarRowSats

                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.satsLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarSats

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

            Row {
                id: statusBarRowCorrAge

                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.corrAgeLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarCorrAge

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

            Row {
                id: statusBarRowAntenna

                spacing: Constants.statusBar.innerKeyValSpacing

                Text {
                    text: Constants.statusBar.antennaLabel
                    color: Constants.statusBar.keyTextColor
                    font.pointSize: Constants.largePointSize
                    font.bold: true
                }

                Text {
                    id: statusBarAntenna

                    text: Constants.statusBar.defaultValue
                    font.pointSize: Constants.largePointSize
                }

            }

        }

        Rectangle {
            Layout.fillWidth: true
        }

        RowLayout {
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

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            status_bar_model.fill_data(statusBarData);
            if (statusBarData.title) {
                statusBarPos.text = statusBarData.pos;
                statusBarRTK.text = statusBarData.rtk;
                statusBarSats.text = statusBarData.sats;
                statusBarCorrAge.text = statusBarData.corr_age;
                statusBarINS.text = statusBarData.ins;
                statusBarDataRate.text = statusBarData.data_rate;
                statusBarAntenna.text = statusBarData.antenna_status;
                let recordingPrefix = "🔴";
                if (Qt.platform.os === "windows")
                    recordingPrefix = "[L]";

                parent.title = (parent.sbpRecording ? recordingPrefix : " ") + statusBarData.title;
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
