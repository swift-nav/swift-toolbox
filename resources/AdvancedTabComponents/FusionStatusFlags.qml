import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: fusionStatusFlags

    property variant lines: []
    property string last_gnsspos: "UNKNOWN"
    property string last_gnssvel: "UNKNOWN"
    property string last_wheelticks: "UNKNOWN"
    property string last_speed: "UNKNOWN"
    property string last_nhc: "UNKNOWN"
    property string last_zerovel: "UNKNOWN"

    Component.onCompleted: {
    }

    FusionStatusFlagsData {
        id: fusionStatusFlagsData
    }

    GroupBox {
        anchors.centerIn: parent

        GridLayout {
            id: fusionStatusFlagsArea

            columns: 2

            Label {
                text: Constants.advancedImu.insStatusLabels[0]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: gnssposUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: gnssposWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: gnssposOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[1]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: gnssvelUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: gnssvelWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: gnssvelOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[2]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: wheelticksUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: wheelticksWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: wheelticksOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[3]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: speedUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: speedWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: speedOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[4]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: nhcUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: nhcWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: nhcOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[5]
                font.pointSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: zerovelUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: zerovelWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: zerovelOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
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

                    fusion_engine_flags_model.fill_console_points(fusionStatusFlagsData);
                    if (!fusionStatusFlagsData.gnsspos)
                        return ;

                    fusionStatusFlagsArea.visible = true;
                    var gnsspos = fusionStatusFlagsData.gnsspos;
                    if (gnsspos != last_gnsspos) {
                        if (gnsspos == "UNKNOWN") {
                            gnssposUnknown.visible = true;
                            gnssposWarning.visible = false;
                            gnssposOk.visible = false;
                        } else if (gnsspos == "WARNING") {
                            gnssposUnknown.visible = false;
                            gnssposWarning.visible = true;
                            gnssposOk.visible = false;
                        } else {
                            gnssposUnknown.visible = false;
                            gnssposWarning.visible = false;
                            gnssposOk.visible = true;
                        }
                        last_gnsspos = gnsspos;
                    }
                    var gnssvel = fusionStatusFlagsData.gnssvel;
                    if (gnssvel != last_gnssvel) {
                        if (gnssvel == "UNKNOWN") {
                            gnssvelUnknown.visible = true;
                            gnssvelWarning.visible = false;
                            gnssvelOk.visible = false;
                        } else if (gnssvel == "WARNING") {
                            gnssvelUnknown.visible = false;
                            gnssvelWarning.visible = true;
                            gnssvelOk.visible = false;
                        } else {
                            gnssvelUnknown.visible = false;
                            gnssvelWarning.visible = false;
                            gnssvelOk.visible = true;
                        }
                        last_gnssvel = gnssvel;
                    }
                    var wheelticks = fusionStatusFlagsData.wheelticks;
                    if (wheelticks != last_wheelticks) {
                        if (wheelticks == "UNKNOWN") {
                            wheelticksUnknown.visible = true;
                            wheelticksWarning.visible = false;
                            wheelticksOk.visible = false;
                        } else if (wheelticks == "WARNING") {
                            wheelticksUnknown.visible = false;
                            wheelticksWarning.visible = true;
                            wheelticksOk.visible = false;
                        } else {
                            wheelticksUnknown.visible = false;
                            wheelticksWarning.visible = false;
                            wheelticksOk.visible = true;
                        }
                        last_wheelticks = wheelticks;
                    }
                    var speed = fusionStatusFlagsData.speed;
                    if (speed != last_speed) {
                        if (speed == "UNKNOWN") {
                            speedUnknown.visible = true;
                            speedWarning.visible = false;
                            speedOk.visible = false;
                        } else if (speed == "WARNING") {
                            speedUnknown.visible = false;
                            speedWarning.visible = true;
                            speedOk.visible = false;
                        } else {
                            speedUnknown.visible = false;
                            speedWarning.visible = false;
                            speedOk.visible = true;
                        }
                        last_speed = speed;
                    }
                    var nhc = fusionStatusFlagsData.nhc;
                    if (nhc != last_nhc) {
                        if (nhc == "UNKNOWN") {
                            nhcUnknown.visible = true;
                            nhcWarning.visible = false;
                            nhcOk.visible = false;
                        } else if (nhc == "WARNING") {
                            nhcUnknown.visible = false;
                            nhcWarning.visible = true;
                            nhcOk.visible = false;
                        } else {
                            nhcUnknown.visible = false;
                            nhcWarning.visible = false;
                            nhcOk.visible = true;
                        }
                        last_nhc = nhc;
                    }
                    var zerovel = fusionStatusFlagsData.zerovel;
                    if (zerovel != last_zerovel) {
                        if (zerovel == "UNKNOWN") {
                            zerovelUnknown.visible = true;
                            zerovelWarning.visible = false;
                            zerovelOk.visible = false;
                        } else if (zerovel == "WARNING") {
                            zerovelUnknown.visible = false;
                            zerovelWarning.visible = true;
                            zerovelOk.visible = false;
                        } else {
                            zerovelUnknown.visible = false;
                            zerovelWarning.visible = false;
                            zerovelOk.visible = true;
                        }
                        last_zerovel = zerovel;
                    }
                }
            }

        }

        label: Label {
            text: Constants.fusionStatusFlags.title
            font.pointSize: Constants.fusionStatusFlags.titleFontSize
        }

    }

}
