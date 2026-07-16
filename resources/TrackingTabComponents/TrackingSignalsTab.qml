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
import "../"
import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: trackingSignalsTab

    property alias all_series: trackingSignalsPoints.all_series
    property alias enabled_series: trackingSignalsPoints.enabled_series
    property alias check_labels: trackingSignalsPoints.check_labels
    property alias num_labels: trackingSignalsPoints.num_labels
    property variant check_visibility: []

    TrackingSignalsPoints {
        id: trackingSignalsPoints

        onData_updated: if (visible)
            update()
    }

    onVisibleChanged: if (visible)
        update()

    function update() {
        let commonChart = Constants.commonChart;
        if (all_series.length < num_labels) {
            for (var i = all_series.length; i < num_labels; i++) {
                var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, trackingSignalsPoints.getLabel(i), trackingSignalsXAxis);
                series.axisYRight = trackingSignalsYAxis;
                series.width = commonChart.lineWidth;
                series.useOpenGL = Globals.useOpenGL;
                // Color will be set in Python with fill_all_series call.
                trackingSignalsPoints.addSeries(series);
            }
        }
        trackingSignalsPoints.fill_all_series();
        trackingSignalsChart.visible = true;
        trackingSignalsXAxis.min = trackingSignalsPoints.xaxis_min;
        trackingSignalsXAxis.max = trackingSignalsPoints.xaxis_max;
    }

    ColumnLayout {
        id: trackingSignalsArea

        anchors.fill: parent
        spacing: 0

        ChartView {
            id: trackingSignalsChart

            Layout.preferredHeight: parent.height
            Layout.fillHeight: true
            Layout.fillWidth: true
            visible: all_series.length > 0
            title: Constants.trackingSignals.title
            titleFont: Constants.commonChart.titleFont
            titleColor: Constants.commonChart.titleColor
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: Globals.useAntiAliasing

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            ChartLegend {
                x: Constants.trackingSignals.legendLeftMargin
                y: Constants.trackingSignals.legendTopMargin
                maximumHeight: parent.height - Constants.trackingSignals.legendTopMargin - Constants.trackingSignals.legendBottomMargin
                cellTextSample: Constants.trackingSignals.legendCellTextSample
                model: enabled_series
            }

            SwiftValueAxis {
                id: trackingSignalsXAxis

                titleText: Constants.trackingSignals.xAxisTitleText
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.xAxisTickInterval
                labelFormat: "%d"
            }

            SwiftValueAxis {
                id: trackingSignalsYAxis

                titleText: Constants.trackingSignals.yAxisTitleText
                max: Constants.trackingSignals.yAxisMax
                min: Constants.trackingSignals.snrThreshold
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.yAxisTickInterval
                labelFormat: "%d"
                titleFont: Constants.trackingSignals.yAxisTitleFont
            }
        }

        RowLayout {
            id: trackingSignalsCheckboxes

            readonly property var familyMeta: ({
                    "GPS L1": {
                        "c": "GPS",
                        "b": "L1",
                        "d": "L1"
                    },
                    "GPS L2": {
                        "c": "GPS",
                        "b": "L2",
                        "d": "L2"
                    },
                    "GPS L5": {
                        "c": "GPS",
                        "b": "L5",
                        "d": "L5"
                    },
                    "GLO G1": {
                        "c": "GLO",
                        "b": "L1",
                        "d": "G1"
                    },
                    "GLO G2": {
                        "c": "GLO",
                        "b": "L2",
                        "d": "G2"
                    },
                    "GAL E1": {
                        "c": "GAL",
                        "b": "L1",
                        "d": "E1"
                    },
                    "GAL E5b": {
                        "c": "GAL",
                        "b": "L2",
                        "d": "E5b"
                    },
                    "GAL E5a": {
                        "c": "GAL",
                        "b": "L5",
                        "d": "E5a"
                    },
                    "GAL E5ab/E6": {
                        "c": "GAL",
                        "b": "Others",
                        "d": "E5ab/E6"
                    },
                    "BDS B1/B1C": {
                        "c": "BDS",
                        "b": "L1",
                        "d": "B1/B1C"
                    },
                    "BDS B2I": {
                        "c": "BDS",
                        "b": "L2",
                        "d": "B2I"
                    },
                    "BDS B2a": {
                        "c": "BDS",
                        "b": "L5",
                        "d": "B2a"
                    },
                    "BDS B2b/B3": {
                        "c": "BDS",
                        "b": "Others",
                        "d": "B2b/B3"
                    },
                    "SBAS L1": {
                        "c": "SBAS",
                        "b": "L1",
                        "d": "L1"
                    },
                    "SBAS L5": {
                        "c": "SBAS",
                        "b": "L5",
                        "d": "L5"
                    },
                    "QZS L1": {
                        "c": "QZSS",
                        "b": "L1",
                        "d": "L1"
                    },
                    "QZS L2": {
                        "c": "QZSS",
                        "b": "L2",
                        "d": "L2"
                    },
                    "QZS L5": {
                        "c": "QZSS",
                        "b": "L5",
                        "d": "L5"
                    }
                })
            readonly property var constellations: ["GPS", "GLO", "GAL", "BDS", "QZSS", "SBAS"]
            readonly property var bands: ["L1", "L2", "L5", "Others"]

            property var familyChecked: ({})

            function familiesMatching(predKey, predValue) {
                var out = [];
                for (var i = 0; i < check_labels.length; i++) {
                    var k = check_labels[i];
                    var meta = familyMeta[k];
                    if (meta && meta[predKey] === predValue)
                        out.push(k);
                }
                return out;
            }

            function computeState(families) {
                if (families.length === 0)
                    return Qt.Unchecked;
                var checked = 0;
                for (var i = 0; i < families.length; i++) {
                    if (familyChecked[families[i]] !== false)
                        checked++;
                }
                return checked === families.length ? Qt.Checked : checked === 0 ? Qt.Unchecked : Qt.PartiallyChecked;
            }

            function setFamilies(families, checked) {
                var newState = {};
                for (var k in familyChecked)
                    newState[k] = familyChecked[k];
                for (var i = 0; i < families.length; i++)
                    newState[families[i]] = checked;
                familyChecked = newState;
                sendVisibility();
            }

            function sendVisibility() {
                var labels_not_visible = [];
                for (var i = 0; i < check_labels.length; i++) {
                    var f = check_labels[i];
                    if (familyChecked[f] === false)
                        labels_not_visible.push(f);
                }
                check_visibility = labels_not_visible;
                backend_request_broker.tracking_signals_check_visibility(labels_not_visible);
            }

            Connections {
                target: trackingSignalsTab
                function onCheck_labelsChanged() {
                    var init = {};
                    for (var i = 0; i < check_labels.length; i++) {
                        var f = check_labels[i];
                        init[f] = trackingSignalsCheckboxes.familyChecked[f] !== false;
                    }
                    trackingSignalsCheckboxes.familyChecked = init;
                }
            }

            Component.onCompleted: {
                var init = {};
                for (var i = 0; i < check_labels.length; i++)
                    init[check_labels[i]] = true;
                familyChecked = init;
            }

            spacing: 8
            Layout.margins: 0
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredHeight: 28

            SmallCheckBox {
                text: "All"
                tristate: true
                checkState: trackingSignalsCheckboxes.computeState(check_labels)
                onClicked: trackingSignalsCheckboxes.setFamilies(check_labels.slice(), checkState === Qt.Checked)
                nextCheckState: function () {
                    return checkState === Qt.Checked ? Qt.Unchecked : Qt.Checked;
                }
            }

            Repeater {
                model: trackingSignalsCheckboxes.bands

                SmallCheckBox {
                    property string band: modelData
                    text: band
                    tristate: true
                    checkState: trackingSignalsCheckboxes.computeState(trackingSignalsCheckboxes.familiesMatching("b", band))
                    onClicked: {
                        var fam = trackingSignalsCheckboxes.familiesMatching("b", band);
                        trackingSignalsCheckboxes.setFamilies(fam, checkState === Qt.Checked);
                    }
                    nextCheckState: function () {
                        return checkState === Qt.Checked ? Qt.Unchecked : Qt.Checked;
                    }
                }
            }

            Repeater {
                model: trackingSignalsCheckboxes.constellations

                Item {
                    id: conItem
                    property string con: modelData
                    implicitWidth: conRow.implicitWidth
                    implicitHeight: conRow.implicitHeight

                    RowLayout {
                        id: conRow
                        spacing: 0
                        anchors.fill: parent

                        SmallCheckBox {
                            text: conItem.con
                            tristate: true
                            checkState: trackingSignalsCheckboxes.computeState(trackingSignalsCheckboxes.familiesMatching("c", conItem.con))
                            onClicked: {
                                var fam = trackingSignalsCheckboxes.familiesMatching("c", conItem.con);
                                trackingSignalsCheckboxes.setFamilies(fam, checkState === Qt.Checked);
                            }
                            nextCheckState: function () {
                                return checkState === Qt.Checked ? Qt.Unchecked : Qt.Checked;
                            }
                        }

                        Button {
                            text: "▾"
                            flat: true
                            padding: 0
                            implicitWidth: 16
                            implicitHeight: 18
                            font.pixelSize: 10
                            onClicked: conPopup.open()
                        }
                    }

                    Popup {
                        id: conPopup
                        y: conRow.height + 2
                        padding: 6

                        ColumnLayout {
                            spacing: 2

                            Repeater {
                                model: trackingSignalsCheckboxes.familiesMatching("c", conItem.con)

                                SmallCheckBox {
                                    property string family: modelData
                                    text: trackingSignalsCheckboxes.familyMeta[family] ? trackingSignalsCheckboxes.familyMeta[family].d : family
                                    checkState: trackingSignalsCheckboxes.familyChecked[family] === false ? Qt.Unchecked : Qt.Checked
                                    onClicked: trackingSignalsCheckboxes.setFamilies([family], checkState === Qt.Checked)
                                    nextCheckState: function () {
                                        return checkState === Qt.Checked ? Qt.Unchecked : Qt.Checked;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
