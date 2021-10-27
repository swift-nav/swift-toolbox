import "../Constants"
import QtCharts 2.3
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: trackingSignalsTab

    property variant lines: []
    property variant labels: []
    property variant colors: []
    property variant check_labels: []
    property variant check_visibility: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    Rectangle {
        id: trackingSignalsArea

        width: parent.width
        height: parent.height

        ChartView {
            id: trackingSignalsChart

            visible: false
            title: Constants.trackingSignals.title
            titleColor: Constants.trackingSignals.titleColor
            width: parent.width
            height: parent.height - trackingSignalsCheckboxes.height
            anchors.top: parent.top
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

                        model: labels

                        Row {
                            Component.onCompleted: {
                                for (var idx in colors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];

                                }
                            }

                            Rectangle {
                                id: marker

                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Label {
                                id: label

                                text: modelData
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
                        return ;

                    tracking_signals_model.fill_console_points(trackingSignalsPoints);
                    if (!trackingSignalsPoints.points.length)
                        return ;

                    var points = trackingSignalsPoints.points;
                    colors = trackingSignalsPoints.colors;
                    labels = trackingSignalsPoints.labels;
                    trackingSignalsChart.visible = true;
                    check_labels = trackingSignalsPoints.check_labels;
                    for (var idx in labels) {
                        if (idx < lines.length) {
                            if (labels[idx] != lines[idx][1]) {
                                trackingSignalsChart.removeSeries(lines[idx][0]);
                                var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                                line.color = colors[idx];
                                line.width = Constants.commonChart.lineWidth;
                                line.axisYRight = trackingSignalsYAxis;
                                line.useOpenGL = Globals.useOpenGL;
                                lines[idx] = [line, labels[idx]];
                            }
                        } else {
                            var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                            line.color = colors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = trackingSignalsYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push([line, labels[idx]]);
                        }
                    }
                    trackingSignalsPoints.fill_series(lines);
                    var last = points[0][points[0].length - 1];
                    trackingSignalsXAxis.min = last.x + trackingSignalsPoints.xmin_offset;
                    trackingSignalsXAxis.max = last.x;
                }
            }

        }

        GridLayout {
            id: trackingSignalsCheckboxes

            columns: Math.floor(parent.width / Constants.trackingSignals.checkBoxPreferredWidth)
            rows: Math.ceil(check_labels.length / trackingSignalsCheckboxes.columns)
            anchors.horizontalCenter: trackingSignalsChart.horizontalCenter
            anchors.top: trackingSignalsChart.bottom

            Repeater {
                id: trackingSignalsCheckbox

                model: check_labels

                Column {
                    Layout.rowSpan: index === 0 ? trackingSignalsCheckboxes.rows : 1
                    CheckBox {
                        checked: true
                        text: modelData
                        verticalPadding: Constants.trackingSignals.checkBoxVerticalPadding
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

}
