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
            spacing: 0

            ComboBox {
                id: solutionVelocitySelectedUnit

                Layout.alignment: Qt.AlignCenter
                width: 100
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
                Layout.preferredHeight: parent.height - 100
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
                    anchors.bottom: solutionVelocityChart.bottom
                    anchors.left: solutionVelocityChart.left
                    anchors.bottomMargin: 120
                    anchors.leftMargin: 80
                    implicitHeight: lineLegendRepeater.height
                    width: lineLegendRepeater.width

                    Column {
                        id: lineLegendRepeater

                        spacing: -1
                        padding: 10
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
                                    font.pointSize: 9
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: -1
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    // useOpenGL: true // [CPP-93] Invesigate usage of `useOpenGL` in plots

                    id: solutionVelocityXAxis

                    labelsAngle: 45
                    titleText: "GPS Time of Week"
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
                    // useOpenGL: true // [CPP-93] Invesigate usage of `useOpenGL` in plots

                    id: solutionVelocityYAxis

                    titleText: solutionVelocitySelectedUnit.currentText
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
                                var line = solutionVelocityChart.createSeries(ChartView.SeriesTypeLine, labels[idx], solutionVelocityXAxis);
                                line.color = colors[idx];
                                line.width = 1;
                                line.axisYRight = solutionVelocityYAxis;
                                line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                lines.push(line);
                            }
                        }
                        solutionVelocityPoints.fill_series(lines);
                        var last = points[0][points[0].length - 1];
                        solutionVelocityXAxis.min = last.x - 20;
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
