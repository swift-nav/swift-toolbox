import "../Constants"
import QtCharts 2.2
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionVelocityTab

    property variant labels: ["Horizontal", "Vertical"]
    property variant lines: []
    property variant colors: []
    property variant available_units: []
    property variant unit: ""

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    SolutionVelocityPoints {
        id: solutionVelocityPoints
    }

    Rectangle {
        id: solutionVelocityArea

        width: parent.width
        height: parent.height

        ColumnLayout {
            id: solutionVelocityAreaRowLayout

            anchors.fill: parent
            width: parent.width
            height: parent.height

            ComboBox {
                id: solutionVelocitySelectedUnit

                Layout.alignment: Qt.AlignCenter
                width: Constants.solutionVelocity.unitDropdownWidth
                model: available_units
                onCurrentIndexChanged: {
                    if (!available_units)
                        return ;

                    data_model.solution_velocity_unit(available_units[currentIndex]);
                }
            }

            ChartView {
                id: solutionVelocityChart

                Layout.preferredWidth: parent.width
                Layout.preferredHeight: parent.height
                Layout.alignment: Qt.AlignBottom
                Layout.bottomMargin: Constants.solutionVelocity.chartBottomMargin
                Layout.fillHeight: true
                backgroundColor: Constants.commonChart.backgroundColor
                plotAreaColor: Constants.commonChart.areaColor
                legend.visible: false
                antialiasing: true
                Component.onCompleted: {
                }

                Rectangle {
                    id: lineLegend

                    border.color: Constants.commonLegend.borderColor
                    border.width: Constants.commonLegend.borderWidth
                    anchors.bottom: solutionVelocityChart.bottom
                    anchors.left: solutionVelocityChart.left
                    anchors.bottomMargin: Constants.solutionVelocity.legendBottomMargin
                    anchors.leftMargin: Constants.solutionVelocity.legendLeftMargin
                    implicitHeight: lineLegendRepeater.height
                    width: lineLegendRepeater.width

                    Column {
                        id: lineLegendRepeater

                        padding: Constants.commonLegend.padding
                        anchors.bottom: lineLegend.bottom

                        Repeater {
                            id: lineLegendRepeaterRows

                            model: Constants.solutionVelocity.labels

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

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: Constants.solutionVelocity.legendLabelPointSize
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    // useOpenGL: true // [CPP-93] Invesigate usage of `useOpenGL` in plots

                    id: solutionVelocityXAxis

                    labelsAngle: Constants.solutionVelocity.xAxisLabelsAngle
                    titleText: Constants.solutionVelocity.xAxisTitleText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.commonChart.tickPointSize
                        bold: true
                    }

                }

                ValueAxis {
                    // useOpenGL: true // [CPP-93] Invesigate usage of `useOpenGL` in plots

                    id: solutionVelocityYAxis

                    titleText: solutionVelocitySelectedUnit.currentText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.commonChart.tickPointSize
                        bold: true
                    }

                }

                Timer {
                    interval: Constants.currentRefreshRate
                    running: true
                    repeat: true
                    onTriggered: {
                        if (!solutionTab.visible)
                            return ;

                        solution_velocity_model.fill_console_points(solutionVelocityPoints);
                        if (!solutionVelocityPoints.points.length)
                            return ;

                        var points = solutionVelocityPoints.points;
                        if (colors != solutionVelocityPoints.colors) {
                            colors = solutionVelocityPoints.colors;
                            for (var idx in colors) {
                                if (lineLegendRepeaterRows.itemAt(idx))
                                    lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];

                            }
                        }
                        if (available_units != solutionVelocityPoints.available_units)
                            available_units = solutionVelocityPoints.available_units;

                        if (!lines.length) {
                            for (var idx in labels) {
                                var line = solutionVelocityChart.createSeries(ChartView.SeriesTypeLine, Constants.solutionVelocity.labels[idx], solutionVelocityXAxis);
                                line.color = colors[idx];
                                line.width = Constants.commonChart.lineWidth;
                                line.axisYRight = solutionVelocityYAxis;
                                line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                lines.push(line);
                            }
                        }
                        solutionVelocityPoints.fill_series(lines);
                        var last = points[0][points[0].length - 1];
                        solutionVelocityXAxis.min = last.x - Constants.solutionVelocity.xAxisMinOffsetFromMaxSeconds;
                        solutionVelocityXAxis.max = last.x;
                        if (solutionVelocityYAxis.min != solutionVelocityPoints.min_ || solutionVelocityYAxis.max != solutionVelocityPoints.max_) {
                            solutionVelocityYAxis.min = solutionVelocityPoints.min_;
                            solutionVelocityYAxis.max = solutionVelocityPoints.max_;
                        }
                    }
                }

            }

        }

    }

}
