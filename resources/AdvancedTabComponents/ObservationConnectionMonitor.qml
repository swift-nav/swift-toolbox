/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "../BaseComponents"
import "../Constants"
import Qt.labs.qmlmodels
import QtCharts
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
    }
}
