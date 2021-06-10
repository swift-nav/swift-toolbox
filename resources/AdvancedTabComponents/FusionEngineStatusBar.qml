import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: fusionEngineStatusBar

    property variant lines: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    FusionEngineStatusBarData {
        id: fusionEngineStatusBarData
    }

    ColumnLayout {
        id: fusionEngineStatusBarArea

        width: parent.width
        height: parent.height

        RowLayout {
            id: insStatusRow

            visible: false
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            Layout.alignment: Qt.AlignBottom

            Text {
                text: Constants.advancedIns.insStatusLabels[0]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: gnssposText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Text {
                text: Constants.advancedIns.insStatusLabels[1]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: gnssvelText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Text {
                text: Constants.advancedIns.insStatusLabels[2]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: wheelticksText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Text {
                text: Constants.advancedIns.insStatusLabels[3]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: speedText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Text {
                text: Constants.advancedIns.insStatusLabels[4]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: nhcText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Text {
                text: Constants.advancedIns.insStatusLabels[5]
                font.pointSize: Constants.mediumPointSize
            }

            Text {
                id: zerovelText

                Layout.preferredWidth: Constants.advancedIns.insStatusEmojiWidth
            }

            Item {
                Layout.fillWidth: true
            }

            Timer {
                interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    fusion_engine_status_bar_model.fill_console_points(fusionEngineStatusBarData);
                    if (!fusionEngineStatusBarData.gnsspos)
                        return ;

                    insStatusRow.visible = true;
                    gnssposText.text = fusionEngineStatusBarData.gnsspos;
                    gnssposText.color = Utils.insStatusColor(fusionEngineStatusBarData.gnsspos);
                    gnssvelText.text = fusionEngineStatusBarData.gnssvel;
                    gnssvelText.color = Utils.insStatusColor(fusionEngineStatusBarData.gnssvel);
                    wheelticksText.text = fusionEngineStatusBarData.wheelticks;
                    wheelticksText.color = Utils.insStatusColor(fusionEngineStatusBarData.wheelticks);
                    speedText.text = fusionEngineStatusBarData.speed;
                    speedText.color = Utils.insStatusColor(fusionEngineStatusBarData.speed);
                    nhcText.text = fusionEngineStatusBarData.nhc;
                    nhcText.color = Utils.insStatusColor(fusionEngineStatusBarData.nhc);
                    zerovelText.text = fusionEngineStatusBarData.zerovel;
                    zerovelText.color = Utils.insStatusColor(fusionEngineStatusBarData.zerovel);
                }
            }

        }

    }

}
