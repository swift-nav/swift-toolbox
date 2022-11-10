import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

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

    BaselinePlotPoints {
        id: baselinePlotPoints
    }

    ColumnLayout {
        id: baselinePlotArea

        anchors.fill: parent
        visible: true
        spacing: Constants.baselinePlot.navBarSpacing

        ButtonGroup {
            id: baselineButtonGroup

            exclusive: false
        }

        RowLayout {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: Constants.baselinePlot.navBarMargin

            SwiftButton {
                id: baselinePauseButton

                ButtonGroup.group: baselineButtonGroup
                Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Pause"
                checkable: true
                onClicked: backend_request_broker.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])

                icon {
                    source: Constants.icons.pauseButtonUrl
                    color: !baselinePauseButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            SwiftButton {
                id: baselineClearButton

                onPressed: backend_request_broker.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])
                ButtonGroup.group: baselineButtonGroup
                Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Clear"

                icon {
                    source: Constants.icons.clearButtonUrl
                    color: !baselineClearButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }

                Image {
                    id: baselineClearImage

                    anchors.centerIn: parent
                    width: Constants.baselinePlot.buttonSvgHeight
                    height: Constants.baselinePlot.buttonSvgHeight
                    source: Constants.icons.clearButtonUrl
                    visible: false
                }
            }

            SwiftButton {
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
                ToolTip.visible: hovered
                ToolTip.text: "Zoom All"
                checkable: true
                checked: true

                icon {
                    source: Constants.icons.zoomAllButtonUrl
                    color: !baselineZoomAllButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            SwiftButton {
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
                ToolTip.visible: hovered
                ToolTip.text: "Center On Solution"
                checkable: true

                icon {
                    source: Constants.icons.centerOnButtonUrl
                    color: !baselineCenterButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            SwiftButton {
                id: baselineResetFiltersButton

                onPressed: backend_request_broker.baseline_plot([baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].pressed, baselineButtonGroup.buttons[0].pressed])
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
                            Label {
                                id: marker

                                text: "● "
                                font.pixelSize: (Constants.mediumPixelSize + Constants.commonLegend.markerPixelSizeOffset)
                                font.bold: true
                                color: Constants.baselinePlot.colors[index]
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                            Label {
                                id: label

                                text: modelData
                                font.pixelSize: Constants.mediumPixelSize
                                font.bold: true
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }
                        }
                    }
                }
            }

            SwiftValueAxis {
                id: baselinePlotXAxis

                titleText: Constants.baselinePlot.xAxisTitleText
            }

            SwiftValueAxis {
                id: baselinePlotYAxis

                titleText: Constants.baselinePlot.yAxisTitleText
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

            LineSeries {
                name: "emptySeries"
                axisY: baselinePlotYAxis
                axisX: baselinePlotXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL

                XYPoint {
                    x: 0
                    y: 0
                }

                XYPoint {
                    x: 1
                    y: 1
                }
            }

            Timer {
                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!baselinePlot.visible)
                        return;
                    baseline_plot_model.fill_console_points(baselinePlotPoints);
                    if (!baselinePlotPoints.points.length)
                        return;
                    baselinePlotArea.visible = true;
                    let _lines = null;
                    let bp = Constants.baselinePlot;
                    if (!scatters.length || !cur_scatters.length)
                        [scatters, cur_scatters, _lines] = SolutionPlotLoop.setupScatterSeries(baselinePlotChart, Constants, Globals, baselinePlotXAxis, baselinePlotYAxis, bp.legendLabels, bp.colors);
                    baselinePlotPoints.fill_series([scatters, cur_scatters]);
                    let point = SolutionPlotLoop.getCurSolution(baselinePlotPoints.cur_points);
                    if (point)
                        cur_solution = point;
                    if (center_solution)
                        baselinePlotChart.centerToSolution();
                    let hasData = false;
                    let bpPoints = baselinePlotPoints.points;
                    for (let idx in bpPoints) {
                        if (bpPoints[idx].length > 0) {
                            hasData = true;
                            break;
                        }
                    }
                    let new_n_min = bp.axesDefaultMin;
                    let new_n_max = bp.axesDefaultMax;
                    let new_e_min = bp.axesDefaultMin;
                    let new_e_max = bp.axesDefaultMax;
                    baselineZoomAllButton.enabled = hasData;
                    baselineCenterButton.enabled = hasData;
                    if (hasData) {
                        new_n_min = baselinePlotPoints.n_min;
                        new_n_max = baselinePlotPoints.n_max;
                        new_e_min = baselinePlotPoints.e_min;
                        new_e_max = baselinePlotPoints.e_max;
                    } else {
                        zoom_all = true;
                        center_solution = false;
                        baselineZoomAllButton.checked = true;
                        baselineCenterButton.checked = false;
                        baselinePlotChart.resetChartZoom();
                    }
                    if (orig_n_min != new_n_min || orig_n_max != new_n_max || orig_e_min != new_e_min || orig_e_max != new_e_max) {
                        orig_n_min = new_n_min;
                        orig_n_max = new_n_max;
                        orig_e_min = new_e_min;
                        orig_e_max = new_e_max;
                        if (zoom_all)
                            baselinePlotChart.resetChartZoom();
                    }
                }
            }
        }
    }
}
