import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: baselinePlot

    property variant cur_scatters: []
    property variant scatters: []
    property variant colors: []
    property real mouse_x: 0
    property real mouse_y: 0
    property real orig_n_max: 0
    property real orig_n_min: 0
    property real orig_e_max: 0
    property real orig_e_min: 0
    property real x_axis_half: 0
    property real y_axis_half: 0
    property variant cur_solution: null
    property bool zoom_all: true
    property bool center_solution: false

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    BaselinePlotPoints {
        id: baselinePlotPoints
    }

    Rectangle {
        id: baselinePlotArea

        width: parent.width
        height: parent.height
        visible: false

        ColumnLayout {
            id: baselinePlotAreaRowLayout

            anchors.fill: parent
            spacing: Constants.baselinePlot.navBarSpacing

            ButtonGroup {
                id: baselineButtonGroup

                exclusive: false
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: Constants.baselinePlot.navBarMargin

                Button {
                    id: baselinePauseButton

                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "| |"
                    ToolTip.visible: hovered
                    ToolTip.text: "Pause"
                    checkable: true
                    onClicked: data_model.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])
                }

                Button {
                    id: baselineClearButton

                    onPressed: data_model.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: " X "
                    ToolTip.visible: hovered
                    ToolTip.text: "Clear"
                }

                Button {
                    id: baselineZoomAllButton

                    onClicked: {
                        if (checked) {
                            zoom_all = true;
                            baselineCenterButton.checked = false;
                            center_solution = false;
                            baselinePlotChart.resetChartZoom();
                        } else {
                            zoom_all = false;
                        }
                    }
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "[ ]"
                    ToolTip.visible: hovered
                    ToolTip.text: "Zoom All"
                    checkable: true
                    checked: true
                }

                Button {
                    id: baselineCenterButton

                    onClicked: {
                        if (checked) {
                            baselineZoomAllButton.checked = false;
                            y_axis_half = Utils.spanBetweenValues(baselinePlotXAxis.max, baselinePlotXAxis.min) / 2;
                            x_axis_half = Utils.spanBetweenValues(baselinePlotYAxis.max, baselinePlotYAxis.min) / 2;
                            center_solution = true;
                            zoom_all = false;
                        } else {
                            center_solution = false;
                        }
                    }
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "(><)"
                    ToolTip.visible: hovered
                    ToolTip.text: "Center On Solution"
                    checkable: true
                }

                Button {
                    id: baselineResetFiltersButton

                    onPressed: data_model.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.resetFiltersButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "Reset Filters"
                    ToolTip.visible: hovered
                    ToolTip.text: "Reset Filters"
                }

            }

            ChartView {
                id: baselinePlotChart

                function resetChartZoom() {
                    baselinePlotChart.zoomReset();
                    baselinePlotXAxis.max = orig_e_max;
                    baselinePlotXAxis.min = orig_e_min;
                    baselinePlotYAxis.max = orig_n_max;
                    baselinePlotYAxis.min = orig_n_min;
                }

                function centerToSolution() {
                    baselinePlotChart.zoomReset();
                    if (cur_scatters.length) {
                        baselinePlotXAxis.max = cur_solution.x + x_axis_half;
                        baselinePlotXAxis.min = cur_solution.x - x_axis_half;
                        baselinePlotYAxis.max = cur_solution.y + y_axis_half;
                        baselinePlotYAxis.min = cur_solution.y - y_axis_half;
                    }
                }

                function chartZoomByDirection(delta) {
                    if (delta > 0)
                        baselinePlotChart.zoom(Constants.commonChart.zoomInMult);
                    else
                        baselinePlotChart.zoom(Constants.commonChart.zoomOutMult);
                }

                function stopZoomFeatures() {
                    baselineCenterButton.checked = false;
                    center_solution = false;
                    baselineZoomAllButton.checked = false;
                    zoom_all = false;
                }

                Layout.preferredWidth: parent.width
                Layout.preferredHeight: parent.height - Constants.commonChart.heightOffset
                Layout.alignment: Qt.AlignBottom
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
                    anchors.top: baselinePlotChart.top
                    anchors.right: baselinePlotChart.right
                    anchors.topMargin: Constants.commonLegend.topMargin
                    anchors.rightMargin: Constants.commonLegend.rightMargin
                    implicitHeight: lineLegendRepeater.height
                    width: lineLegendRepeater.width

                    Column {
                        id: lineLegendRepeater

                        padding: Constants.commonLegend.padding
                        anchors.bottom: lineLegend.bottom

                        Repeater {
                            id: lineLegendRepeaterRows

                            model: Constants.baselinePlot.legendLabels

                            Row {
                                Text {
                                    id: marker

                                    text: "+ "
                                    font.pointSize: (Constants.mediumPointSize + Constants.commonLegend.markerPointSizeOffset)
                                    font.bold: true
                                    color: Constants.baselinePlot.colors[index]
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: Constants.mediumPointSize
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    id: baselinePlotXAxis

                    titleText: Constants.baselinePlot.xAxisTitleText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.mediumPointSize
                        bold: true
                    }

                }

                ValueAxis {
                    id: baselinePlotYAxis

                    titleText: Constants.baselinePlot.yAxisTitleText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.mediumPointSize
                        bold: true
                    }

                }

                MouseArea {
                    anchors.fill: baselinePlotChart
                    onDoubleClicked: {
                        baselinePlotChart.stopZoomFeatures();
                        baselineZoomAllButton.checked = true;
                        baselinePlotChart.resetChartZoom();
                    }
                    onWheel: {
                        baselinePlotChart.stopZoomFeatures();
                        baselinePlotChart.chartZoomByDirection(wheel.angleDelta.y);
                    }
                    onPositionChanged: {
                        if (pressed) {
                            baselinePlotChart.stopZoomFeatures();
                            var current = baselinePlotChart.plotArea;
                            var x_unit = Utils.spanBetweenValues(baselinePlotXAxis.max, baselinePlotXAxis.min) / current.width;
                            var y_unit = Utils.spanBetweenValues(baselinePlotYAxis.max, baselinePlotYAxis.min) / current.height;
                            var delta_x = (mouse_x - mouseX) * x_unit;
                            var delta_y = (mouse_y - mouseY) * y_unit;
                            baselinePlotXAxis.max += delta_x;
                            baselinePlotXAxis.min += delta_x;
                            baselinePlotYAxis.max -= delta_y;
                            baselinePlotYAxis.min -= delta_y;
                            mouse_x = mouseX;
                            mouse_y = mouseY;
                        }
                    }
                    onPressed: {
                        mouse_x = mouseX;
                        mouse_y = mouseY;
                    }
                }

                Timer {
                    interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                    running: true
                    repeat: true
                    onTriggered: {
                        if (!baselineTab.visible)
                            return ;

                        baseline_plot_model.fill_console_points(baselinePlotPoints);
                        if (!baselinePlotPoints.points.length)
                            return ;

                        baselinePlotArea.visible = true;
                        if (!scatters.length || !cur_scatters.length)
                            [scatters, cur_scatters, _lines] = SolutionPlotLoop.setupScatterSeries(baselinePlotChart, Constants, Globals, baselinePlotXAxis, baselinePlotYAxis, Constants.baselinePlot.legendLabels, Constants.baselinePlot.colors);

                        baselinePlotPoints.fill_series([scatters, cur_scatters]);
                        let point = SolutionPlotLoop.getCurSolution(baselinePlotPoints.cur_points);
                        if (point)
                            cur_solution = point;

                        if (center_solution)
                            baselinePlotChart.centerToSolution();

                        if (orig_n_min != baselinePlotPoints.n_min || orig_n_max != baselinePlotPoints.n_max || orig_e_min != baselinePlotPoints.e_min || orig_e_max != baselinePlotPoints.e_max) {
                            orig_n_min = baselinePlotPoints.n_min;
                            orig_n_max = baselinePlotPoints.n_max;
                            orig_e_min = baselinePlotPoints.e_min;
                            orig_e_max = baselinePlotPoints.e_max;
                            if (zoom_all)
                                baselinePlotChart.resetChartZoom();

                        }
                    }
                }

            }

        }

    }

}
