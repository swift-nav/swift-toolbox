import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    property var obsPeriod: Constants.systemMonitor.defaultObs
    property var obsLatency: Constants.systemMonitor.defaultObs

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Constants.systemMonitor.obsTextMargins

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.systemMonitor.textHeight

            Label {
                text: "Observation Connection Monitor"
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.genericTable.borderWidth
            border.color: Constants.genericTable.borderColor

            RowLayout {
                anchors.fill: parent

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.margins: Constants.systemMonitor.obsTextMargins

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: Constants.systemMonitor.textHeight

                        Label {
                            text: Constants.systemMonitor.obsLatencyLabel
                        }

                    }

                    Rectangle {
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        border.width: Constants.genericTable.borderWidth
                        border.color: Constants.genericTable.borderColor

                        ColumnLayout {
                            anchors.centerIn: parent

                            Label {
                                text: Constants.systemMonitor.currLabel + ": " + obsLatency.Curr + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.avgLabel + ": " + obsLatency.Avg + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.minLabel + ": " + obsLatency.Min + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.maxLabel + ": " + obsLatency.Max + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                        }

                    }

                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.margins: Constants.systemMonitor.obsTextMargins

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: Constants.systemMonitor.textHeight

                        Label {
                            text: Constants.systemMonitor.obsPeriodLabel
                        }

                    }

                    Rectangle {
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        border.width: Constants.genericTable.borderWidth
                        border.color: Constants.genericTable.borderColor

                        ColumnLayout {
                            anchors.centerIn: parent

                            Label {
                                text: Constants.systemMonitor.currLabel + ": " + obsPeriod.Curr + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.avgLabel + ": " + obsPeriod.Avg + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.minLabel + ": " + obsPeriod.Min + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                            Label {
                                text: Constants.systemMonitor.maxLabel + ": " + obsPeriod.Max + Constants.systemMonitor.obsUnits
                                horizontalAlignment: Text.AlignRight
                            }

                        }

                    }

                }

            }

        }

        Button {
            id: resetButton

            Layout.maximumWidth: Constants.systemMonitor.resetButtonWidth
            Layout.maximumHeight: Constants.systemMonitor.resetButtonHeight
            ToolTip.visible: hovered
            ToolTip.text: Constants.systemMonitor.resetButtonLabel
            text: Constants.systemMonitor.resetButtonLabel
            icon.source: "../" + Constants.icons.connectButtonPath
            icon.width: Constants.systemMonitor.resetButtonIconSideLength
            icon.height: Constants.systemMonitor.resetButtonIconSideLength
            display: AbstractButton.TextUnderIcon
            onClicked: {
                data_model.reset_device();
            }
        }

    }

}
