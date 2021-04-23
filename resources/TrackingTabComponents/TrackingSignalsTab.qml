import "../Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
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

            title: Constants.trackingSignals.title
            titleColor: Constants.trackingSignals.titleColor
            width: parent.width
            height: parent.height - trackingSignalsCheckboxes.height
            anchors.top: parent.top
            backgroundColor: Constants.plotBackgroundColor
            plotAreaColor: Constants.plotAreaColor
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

                border.color: Constants.legendBorderColor
                border.width: Constants.legendBorderWidth
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

                                width: Constants.trackingSignals.legendMarkerWidth
                                height: Constants.trackingSignals.legendMarkerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Text {
                                id: label

                                text: modelData
                                font.pointSize: Constants.trackingSignals.legendLabelPointSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.trackingSignals.legendVerticalCenterOffset
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
                minorGridLineColor: Constants.plotMinorGridLineColor
                gridLineColor: Constants.plotGridLineColor
                labelsColor: Constants.plotLabelsColor

                labelsFont {
                    pointSize: Constants.plotTickPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: trackingSignalsYAxis

                titleText: Constants.trackingSignals.yAxisTitleText
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.plotMinorGridLineColor
                gridLineColor: Constants.plotGridLineColor
                labelsColor: Constants.plotLabelsColor

                labelsFont {
                    pointSize: Constants.plotTickPointSize
                    bold: true
                }

            }

            Timer {
                id: trackingSignalsTimer

                interval: Constants.currentRefreshRate
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
                    if (check_labels != trackingSignalsPoints.check_labels)
                        check_labels = trackingSignalsPoints.check_labels;

                    for (var idx in labels) {
                        if (idx < lines.length) {
                            if (labels[idx] != lines[idx][1]) {
                                trackingSignalsChart.removeSeries(lines[idx][0]);
                                var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                                line.color = colors[idx];
                                line.width = Constants.trackingSignals.chartLineWidth;
                                line.axisYRight = trackingSignalsYAxis;
                                // line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                lines[idx] = [line, labels[idx]];
                            }
                            
                        } else {
                            var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                            line.color = colors[idx];
                            line.width = Constants.trackingSignals.chartLineWidth;
                            line.axisYRight = trackingSignalsYAxis;
                            // line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                            
                            lines.push([line, labels[idx]]);
                        }
                        
                    }
                    trackingSignalsPoints.fill_series(lines);
                    var last = points[0][points[0].length - 1];
                    trackingSignalsXAxis.min = last.x - Constants.trackingSignals.xAxisMinOffsetFromMaxSeconds;
                    trackingSignalsXAxis.max = last.x;
                    if (trackingSignalsYAxis.min != trackingSignalsPoints.min_) {
                        trackingSignalsYAxis.min = trackingSignalsPoints.min_;
                        trackingSignalsYAxis.max = trackingSignalsPoints.max_;
                    }
                }
            }

        }

        GridLayout {
            id: trackingSignalsCheckboxes

            columns: Constants.trackingSignals.checkBoxColumns
            anchors.horizontalCenter: trackingSignalsChart.horizontalCenter
            anchors.top: trackingSignalsChart.bottom

            Repeater {
                id: trackingSignalsCheckbox

                model: check_labels

                Column {
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
