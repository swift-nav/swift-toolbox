import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    readonly property string emptyString: "--"

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

                Label {
                    text: Constants.statusBar.posLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarPos

                    text: Constants.statusBar.defaultValue
                }

            }

            Row {
                id: statusBarRowRTK

                Layout.alignment: Qt.AlignLeft
                spacing: Constants.statusBar.innerKeyValSpacing

                Label {
                    text: Constants.statusBar.rtkLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarRTK

                    text: Constants.statusBar.defaultValue
                }

            }

            Row {
                id: statusBarRowINS

                spacing: Constants.statusBar.innerKeyValSpacing

                Label {
                    text: Constants.statusBar.insLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarINS

                    text: Constants.statusBar.defaultValue
                }

            }

            Row {
                id: statusBarRowSats

                spacing: Constants.statusBar.innerKeyValSpacing

                Label {
                    text: Constants.statusBar.satsLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarSats

                    text: Constants.statusBar.defaultValue
                }

            }

            Row {
                id: statusBarRowCorrAge

                spacing: Constants.statusBar.innerKeyValSpacing

                Label {
                    text: Constants.statusBar.corrAgeLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarCorrAge

                    text: Constants.statusBar.defaultValue
                }

            }

            Row {
                id: statusBarRowAntenna

                spacing: Constants.statusBar.innerKeyValSpacing

                Label {
                    text: Constants.statusBar.antennaLabel
                    color: Constants.statusBar.keyTextColor
                    font.bold: true
                }

                Label {
                    id: statusBarAntenna

                    text: Constants.statusBar.defaultValue
                }

            }

        }

        Rectangle {
            Layout.fillWidth: true
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
                if (!statusBarData.solid_connection)
                    statusBarSats.text = emptyString;
                else
                    statusBarSats.text = statusBarData.sats;
                if (statusBarData.corr_age == 0)
                    statusBarCorrAge.text = emptyString;
                else
                    statusBarCorrAge.text = Utils.padFloat(statusBarData.corr_age, 1, 1) + " s";
                statusBarINS.text = statusBarData.ins;
                parent.dataRate = statusBarData.data_rate;
                parent.solidConnection = statusBarData.solid_connection;
                statusBarAntenna.text = statusBarData.antenna_status;
                parent.title = (parent.sbpRecording ? "[L] " : "     ") + statusBarData.title;
            }
        }
    }

}
