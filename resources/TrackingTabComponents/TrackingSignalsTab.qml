import "../BaseComponents"
import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: trackingSignalsTab

    // property variant lines: []
    // property variant labels: []
    // property variant colors: []
    property alias all_series: trackingSignalsPoints.all_series
    property alias check_labels: trackingSignalsPoints.check_labels
    property variant check_visibility: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    ColumnLayout {
        id: trackingSignalsArea

        width: parent.width
        height: parent.height
        spacing: 0

        ChartView {
            id: trackingSignalsChart

            Layout.bottomMargin: -(Constants.margins * 2)
            Layout.fillHeight: true
            Layout.fillWidth: true
            visible: false
            title: Constants.trackingSignals.title
            titleColor: Constants.trackingSignals.titleColor
            width: parent.width
            height: parent.height - trackingSignalsCheckboxes.height
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.trackingSignals.titlePointSize
                bold: true
            }

            Rectangle {
                id: lineLegend

                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: trackingSignalsChart.bottom
                anchors.left: trackingSignalsChart.left
                anchors.bottomMargin: Constants.trackingSignals.legendBottomMargin
                anchors.leftMargin: Constants.trackingSignals.legendLeftMargin
                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width

                Column {
                    id: lineLegendRepeater

                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: all_series

                        Row {
                            Component.onCompleted: {
                                for (var idx in colors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];

                                }
                            }

                            Rectangle {
                                id: marker

                                color: model.color
                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Label {
                                id: label

                                text: model.name
                                font.pointSize: Constants.smallPointSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            ValueAxis {
                id: trackingSignalsXAxis

                titleText: Constants.trackingSignals.xAxisTitleText
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.xAxisTickInterval
                labelFormat: "%d"

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: trackingSignalsYAxis

                titleText: Constants.trackingSignals.yAxisTitleText
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                max: Constants.trackingSignals.yAxisMax
                min: Constants.trackingSignals.snrThreshold
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.yAxisTickInterval
                labelFormat: "%d"

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: trackingSignalsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!trackingTab.visible)
                        return;

                    if (trackingSignalsPoints.all_series.length < trackingSignalsPoints.num_labels) {
                        for (var i = trackingSignalsPoints.all_series.length;
                             i < trackingSignalsPoints.num_labels; i++) {
                            var series = trackingSignalsChart.createSeries(
                                ChartView.SeriesTypeLine, trackingSignalsPoints.getLabel(i),
                                trackingSignalsXAxis)
                            series.axisYRight = trackingSignalsYAxis
                            series.width = Constants.commonChart.lineWidth
                            // Color and useOpenGL will be set in Python with fill_all_series call.
                            // series.color = sourceSeries.color
                            // series.useOpenGL = sourceSeries.useOpenGL
                            trackingSignalsPoints.addSeries(series)
                        }
                    }
                    trackingSignalsPoints.fill_all_series(Constants.commonChart.lineWidth,
                        Globals.useOpenGL);
                    // for (var series_idx in missing_series_indices) {
                    //     console.log("Creating new series " + sourceSeries.name)
                    //     var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine,
                    //         sourceSeries.name, trackingSignalsXAxis)
                    //     series.axisYRight = trackingSignalsYAxis
                    //     series.color = sourceSeries.color
                    //     series.width = Constants.commonChart.lineWidth
                    //     series.useOpenGL = sourceSeries.useOpenGL
                    // }
                    // if (series_to_create.length > 0) {
                    //     // Fill again to give all the newly created series' their data..
                    //     // Could probably just skip this step, since a short time later, another 
                    //     trackingSignalsPoints.fill_all_series(Constants.commonChart.lineWidth,
                    //         trackingSignalsXAxis, trackingSignalsYAxis, Globals.useOpenGL);
                    // }
                    if (trackingSignalsPoints.all_series.length) {
                        trackingSignalsChart.visible = true;
                        trackingSignalsXAxis.min = trackingSignalsPoints.xaxis_min;  // last.x + trackingSignalsPoints.xmin_offset;
                        trackingSignalsXAxis.max = trackingSignalsPoints.xaxis_max;  // last.x;
                    }

                    // var all_series = trackingSignalsPoints.all_series;
                    // colors = trackingSignalsPoints.colors;
                    // labels = trackingSignalsPoints.labels;
                    // for (var idx in labels) {
                    //     if (idx < lines.length) {
                    //         if (labels[idx] != lines[idx][1]) {
                    //             series = all_series[idx]
                    //             var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                    //             line.color = colors[idx];
                    //             line.width = Constants.commonChart.lineWidth;
                    //             line.axisYRight = trackingSignalsYAxis;
                    //             line.useOpenGL = Globals.useOpenGL;
                    //             // lines[idx] = [line, labels[idx]];
                    //         }
                    //     } else {
                    //         var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                    //         line.color = colors[idx];
                    //         line.width = Constants.commonChart.lineWidth;
                    //         line.axisYRight = trackingSignalsYAxis;
                    //         line.useOpenGL = Globals.useOpenGL;
                    //         // lines.push([line, labels[idx]]);
                    //     }
                    // }
                    // trackingSignalsPoints.fill_series(lines);
                    // var last = points[0][points[0].length - 1];
                }
            }

        }

        GridLayout {
            id: trackingSignalsCheckboxes

            flow: GridLayout.TopToBottom
            columns: Math.floor(parent.width / Constants.trackingSignals.checkBoxPreferredWidth)
            rows: Math.ceil(check_labels.length / trackingSignalsCheckboxes.columns)
            rowSpacing: 0
            Layout.margins: 0
            Layout.alignment: Qt.AlignHCenter

            Repeater {
                id: trackingSignalsCheckbox

                model: check_labels

                SmallCheckBox {
                    Layout.margins: 0
                    Layout.rowSpan: index === 0 ? trackingSignalsCheckboxes.rows : 1
                    checked: true
                    text: modelData
                    onClicked: {
                        check_visibility[index] = checked;
                        if (index == 0) {
                            lineLegend.visible = !lineLegend.visible;
                            return ;
                        }
                        var labels_not_visible = [];
                        for (var idx in check_visibility) {
                            if (!check_visibility[idx])
                                labels_not_visible.push(check_labels[idx]);

                        }
                        data_model.tracking_signals_check_visibility(labels_not_visible);
                    }
                    Component.onCompleted: {
                        check_visibility.push(checked);
                    }
                }

            }

        }

    }

}
