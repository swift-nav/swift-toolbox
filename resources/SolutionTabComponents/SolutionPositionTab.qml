import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionPositionTab

    property variant available_units: []
    property variant cur_scatters: []
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

            ButtonGroup {
                id: solutionButtonGroup

                exclusive: false
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: 10

                Button {
                    id: solutionPauseButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * 0.1
                    text: "| |"
                    ToolTip.visible: hovered
                    ToolTip.text: "Pause"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionClearButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * 0.1
                    text: " X "
                    ToolTip.visible: hovered
                    ToolTip.text: "Clear"
                    onPressed: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionZoomAllButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * 0.1
                    text: "[ ]"
                    ToolTip.visible: hovered
                    ToolTip.text: "Zoom All"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionCenterButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * 0.1
                    text: "(><)"
                    ToolTip.visible: hovered
                    ToolTip.text: "Center On Solution"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Text {
                    text: "Display Units: "
                }

                ComboBox {
                    id: solutionPositionSelectedUnit

                    model: available_units
                    onCurrentIndexChanged: {
                        if (!available_units)
                            return ;

                        data_model.solution_position_unit(available_units[currentIndex]);
                    }
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

                        padding: 5
                        spacing: -1
                        anchors.bottom: lineLegend.bottom

                        Repeater {
                            id: lineLegendRepeaterRows

                            model: labels

                            Row {
                                Text {
                                    id: marker

                                    text: "+ "
                                    font.pointSize: 14
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: -1
                                }

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: 10
                                    font.bold: true
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
                    gridLineColor: "#CDC9C9"
                    labelsColor: "#000000"

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
                    gridLineColor: "#CDC9C9"
                    labelsColor: "#000000"

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

                        for (var idx in colors) {
                            if (lineLegendRepeaterRows.itemAt(idx))
                                lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];

                        }
                        if (labels != solutionPositionPoints.labels)
                            labels = solutionPositionPoints.labels;

                        if (available_units != solutionPositionPoints.available_units)
                            available_units = solutionPositionPoints.available_units;

                        if (!lines.length || !scatters.length || !cur_scatters.length) {
                            for (var idx in labels) {
                                var cur_scatter = solutionPositionChart.createSeries(ChartView.SeriesTypeScatter, labels[idx] + "cur-scatter", solutionPositionXAxis, solutionPositionYAxis);
                                cur_scatter.color = colors[idx];
                                cur_scatter.markerSize = 15;
                                var scatter = solutionPositionChart.createSeries(ChartView.SeriesTypeScatter, labels[idx] + "scatter", solutionPositionXAxis, solutionPositionYAxis);
                                scatter.color = colors[idx];
                                scatter.markerSize = 5;
                                var line = solutionPositionChart.createSeries(ChartView.SeriesTypeLine, labels[idx], solutionPositionXAxis, solutionPositionYAxis);
                                line.color = colors[idx];
                                line.width = 0.2;
                                line.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                scatter.useOpenGL = true; // [CPP-93] Invesigate usage of `useOpenGL` in plots
                                lines.push(line);
                                scatters.push(scatter);
                                cur_scatters.push(cur_scatter);
                            }
                        }
                        var combined = [lines, scatters, cur_scatters];
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
