import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionPositionTab

    property variant scatters: []
    property variant lines: []
    property variant labels: []
    property variant colors: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    SolutionPositionPoints {
        id: solutionPositionPoints
    }
    Rectangle {
        id: solutionPositionArea

        width: parent.width
        height: parent.height

        ColumnLayout {
            id: solutionPositionAreaRowLayout

            anchors.fill: parent
            width: parent.width
            height: parent.height
            spacing: 0

            ComboBox {
                id: solutionPositionSelectedUnit

                Layout.alignment: Qt.AlignCenter
                width: 100
                model: ["Placeholder"]
                onCurrentIndexChanged: {
                    // if (!available_units)
                    //     return ;

                    // data_model.solution_velocity_unit(available_units[currentIndex]);
                }
            }

            ChartView {
                id: solutionPositionChart
                Layout.preferredWidth: parent.width
                Layout.preferredHeight: parent.height - 50
                Layout.alignment: Qt.AlignBottom
                Layout.bottomMargin: 20
                Layout.fillHeight: true
                backgroundColor: "#CDC9C9"
                plotAreaColor: "#FFFFFF"
                legend.visible: false
                antialiasing: true
                Component.onCompleted: {
                }

                Rectangle {
                    id: lineLegend
                    border.color: "#000000"
                    border.width: 1
                    anchors.top: solutionPositionChart.top
                    anchors.right: solutionPositionChart.right
                    anchors.topMargin: 85
                    anchors.rightMargin: 60
                    implicitHeight: lineLegendRepeater.height
                    width: lineLegendRepeater.width

                    Column {
                        id: lineLegendRepeater

                        spacing: -1
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

                                    width: 20
                                    height: 3
                                    color: "#000000"
                                    anchors.verticalCenter: parent.verticalCenter
                                }

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: 6
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: -1
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    id: solutionPositionXAxis

                    titleText: "Longitude"
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: "#CDC9C9"
                    visible: true

                    labelsFont {
                        pointSize: 10
                        bold: true
                    }

                }

                ValueAxis {
                    id: solutionPositionYAxis

                    titleText: "Latitude"
                    min: 0
                    max: 1
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: "#CDC9C9"
                    visible: true

                    labelsFont {
                        pointSize: 10
                        bold: true
                    }

                }

                Timer {
                    interval: 1000 / 5 // 5 Hz refresh
                    running: true
                    repeat: true
                    onTriggered: {
                        if (!solutionTab.visible)
                            return ;
                        solution_position_model.fill_console_points(solutionPositionPoints);
                        
                        
                        if (!solutionPositionPoints.points.length)
                            return ;
                        var points = solutionPositionPoints.points;
                        labels = solutionPositionPoints.labels;
                        if (colors != solutionPositionPoints.colors)
                            colors = solutionPositionPoints.colors;
                        if (labels != solutionPositionPoints.labels)
                            labels = solutionPositionPoints.labels;

                        if (!lines.length || !scatters.length){
                            for (var idx in labels) {
                                var scatter = solutionPositionChart.createSeries(ChartView.SeriesTypeScatter, labels[idx]+"scatter", solutionPositionXAxis);
                                scatter.color = colors[idx];
                                scatter.markerSize = 5.0;
                                scatter.axisY = solutionPositionYAxis;
                                var line = solutionPositionChart.createSeries(ChartView.SeriesTypeLine, labels[idx], solutionPositionXAxis);
                                line.color = colors[idx];
                                line.width = 0.2;
                                line.axisY = solutionPositionYAxis;
                                // line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                // scatter.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                lines.push(line);
                                scatters.push(scatter);
                            }
                        }
                        
                        var combined = [lines, scatters];
                        solutionPositionPoints.fill_series(combined);
                        if (solutionPositionYAxis.min != solutionPositionPoints.lat_min_ || solutionPositionYAxis.max != solutionPositionPoints.lat_max_) {
                            solutionPositionYAxis.min = solutionPositionPoints.lat_min_;
                            solutionPositionYAxis.max = solutionPositionPoints.lat_max_;
                        }
                        if (solutionPositionXAxis.min != solutionPositionPoints.lon_min_ || solutionPositionXAxis.max != solutionPositionPoints.lon_max_) {
                            solutionPositionXAxis.min = solutionPositionPoints.lon_min_;
                            solutionPositionXAxis.max = solutionPositionPoints.lon_max_;
                        }
                    }
                }

            }
        }
    }

}
