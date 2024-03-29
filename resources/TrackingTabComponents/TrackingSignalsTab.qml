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

        GridLayout {
            id: trackingSignalsCheckboxes

            property int numChecked: trackingSignalsCbRepeater.count
            property real maxCheckboxImplicitWidth: 0

            flow: GridLayout.TopToBottom
            columns: Math.floor(parent.width / maxCheckboxImplicitWidth)
            rows: Math.ceil(check_labels.length / trackingSignalsCheckboxes.columns)
            rowSpacing: 0
            Layout.margins: 0
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredHeight: 20

            SmallCheckBox {
                id: toggleAllCheckBox

                function setAllOn() {
                    for (var i = 0; i < trackingSignalsCbRepeater.count; i++) {
                        var cb = trackingSignalsCbRepeater.itemAt(i);
                        if (!cb.checked)
                            cb.toggle();
                    }
                }

                onVisibleChanged: {
                    if (visible && checkState != Qt.Checked)
                        setAllOn();
                }
                Layout.margins: 0
                Layout.rowSpan: parent.rows <= 0 ? 1 : parent.rows
                tristate: true
                checkState: (parent.numChecked == trackingSignalsCbRepeater.count ? Qt.Checked : parent.numChecked > 0 ? Qt.PartiallyChecked : Qt.Unchecked)
                text: "All"
                onClicked: {
                    var curCheckState = checkState;
                    for (var i = 0; i < trackingSignalsCbRepeater.count; i++) {
                        var cb = trackingSignalsCbRepeater.itemAt(i);
                        if ((curCheckState == Qt.Checked && !cb.checked) || (curCheckState != Qt.Checked && cb.checked))
                            cb.toggle();
                    }
                }
                Component.onCompleted: {
                    if (implicitWidth > parent.maxCheckboxImplicitWidth)
                        parent.maxCheckboxImplicitWidth = implicitWidth;
                }
                nextCheckState: function () {
                    return (checkState == Qt.Checked) ? Qt.Unchecked : Qt.Checked;
                }
            }

            Repeater {
                id: trackingSignalsCbRepeater

                model: check_labels

                SmallCheckBox {
                    Layout.margins: 0
                    Layout.rowSpan: index === 0 ? trackingSignalsCheckboxes.rows : 1
                    checked: true
                    text: modelData
                    onCheckedChanged: {
                        trackingSignalsCheckboxes.numChecked += checked ? 1 : -1;
                        check_visibility[index] = checked;
                        var labels_not_visible = [];
                        for (var idx in check_visibility) {
                            if (!check_visibility[idx])
                                labels_not_visible.push(check_labels[idx]);
                        }
                        backend_request_broker.tracking_signals_check_visibility(labels_not_visible);
                    }
                    Component.onCompleted: {
                        check_visibility.push(checked);
                        if (implicitWidth > trackingSignalsCheckboxes.maxCheckboxImplicitWidth)
                            trackingSignalsCheckboxes.maxCheckboxImplicitWidth = implicitWidth;
                    }
                }
            }
        }
    }
}
