import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts 2.15
import QtGraphicalEffects 1.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionPositionTab

    property variant available_units: []
    property variant cur_scatters: []
    property variant scatters: []
    property variant lines: []
    property real mouse_x: 0
    property real mouse_y: 0
    property real orig_lat_max: 0
    property real orig_lat_min: 0
    property real orig_lon_max: 0
    property real orig_lon_min: 0
    property real x_axis_half: 0
    property real y_axis_half: 0
    property variant cur_solution: null
    property bool zoom_all: true
    property bool center_solution: false

    SolutionPositionPoints {
        id: solutionPositionPoints
    }

    ColumnLayout {
        id: solutionPositionArea

        anchors.fill: parent
        visible: false
        spacing: Constants.solutionPosition.navBarSpacing

        ButtonGroup {
            id: solutionButtonGroup

            exclusive: false
        }

        RowLayout {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: Constants.solutionPosition.navBarMargin

            Button {
                id: solutionPauseButton

                ButtonGroup.group: solutionButtonGroup
                Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Pause"
                checkable: true
                onClicked: data_model.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])

                Image {
                    id: solutionPauseImage

                    anchors.centerIn: parent
                    width: Constants.solutionPosition.buttonSvgHeight
                    height: Constants.solutionPosition.buttonSvgHeight
                    source: Constants.icons.pauseButtonUrl
                    visible: false
                }

                ColorOverlay {
                    anchors.fill: solutionPauseImage
                    source: solutionPauseImage
                    color: !solutionPauseButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }

            }

            Button {
                id: solutionClearButton

                ButtonGroup.group: solutionButtonGroup
                Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Clear"
                onPressed: data_model.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])

                Image {
                    id: solutionClearImage

                    anchors.centerIn: parent
                    width: Constants.solutionPosition.buttonSvgHeight
                    height: Constants.solutionPosition.buttonSvgHeight
                    source: Constants.icons.clearButtonUrl
                    visible: false
                }

                ColorOverlay {
                    anchors.fill: solutionClearImage
                    source: solutionClearImage
                    color: !solutionClearButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }

            }

            Button {
                id: solutionZoomAllButton

                onClicked: {
                    if (checked) {
                        zoom_all = true;
                        solutionCenterButton.checked = false;
                        center_solution = false;
                        solutionPositionChart.resetChartZoom();
                    } else {
                        zoom_all = false;
                    }
                }
                Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Zoom All"
                checkable: true
                checked: true

                Image {
                    id: solutionZoomAllImage

                    anchors.centerIn: parent
                    width: Constants.solutionPosition.buttonSvgHeight
                    height: Constants.solutionPosition.buttonSvgHeight
                    source: Constants.icons.zoomAllButtonUrl
                    visible: false
                }

                ColorOverlay {
                    anchors.fill: solutionZoomAllImage
                    source: solutionZoomAllImage
                    color: !solutionZoomAllButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }

            }

            Button {
                id: solutionCenterButton

                onClicked: {
                    if (checked) {
                        solutionZoomAllButton.checked = false;
                        y_axis_half = Utils.spanBetweenValues(solutionPositionXAxis.max, solutionPositionXAxis.min) / 2;
                        x_axis_half = Utils.spanBetweenValues(solutionPositionYAxis.max, solutionPositionYAxis.min) / 2;
                        center_solution = true;
                        zoom_all = false;
                    } else {
                        center_solution = false;
                    }
                }
                Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Center On Solution"
                checkable: true

                Image {
                    id: centerButtonImage

                    anchors.centerIn: parent
                    width: Constants.solutionPosition.buttonSvgHeight
                    height: Constants.solutionPosition.buttonSvgHeight
                    source: Constants.icons.centerOnButtonUrl
                    visible: false
                }

                ColorOverlay {
                    anchors.fill: centerButtonImage
                    source: centerButtonImage
                    color: !solutionCenterButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }

            }

            Label {
                text: "Display Units: "
            }

            ComboBox {
                id: solutionPositionSelectedUnit

                model: available_units
                Layout.preferredWidth: Constants.commonChart.unitDropdownWidth
                onCurrentIndexChanged: {
                    if (!available_units)
                        return ;

                    data_model.solution_position_unit(available_units[currentIndex]);
                    zoom_all = true;
                    solutionZoomAllButton.checked = true;
                    solutionCenterButton.checked = false;
                    center_solution = false;
                    solutionPositionChart.resetChartZoom();
                }

                states: State {
                    when: solutionPositionSelectedUnit.down

                    PropertyChanges {
                        target: solutionPositionSelectedUnit
                        width: Constants.commonChart.unitDropdownWidth * 1.5
                    }

                }

            }

        }

        ChartView {
            id: solutionPositionChart

            function resetChartZoom() {
                solutionPositionChart.zoomReset();
                solutionPositionXAxis.max = orig_lon_max;
                solutionPositionXAxis.min = orig_lon_min;
                solutionPositionYAxis.max = orig_lat_max;
                solutionPositionYAxis.min = orig_lat_min;
            }

            function centerToSolution() {
                solutionPositionChart.zoomReset();
                if (cur_scatters.length) {
                    solutionPositionXAxis.max = cur_solution.x + x_axis_half;
                    solutionPositionXAxis.min = cur_solution.x - x_axis_half;
                    solutionPositionYAxis.max = cur_solution.y + y_axis_half;
                    solutionPositionYAxis.min = cur_solution.y - y_axis_half;
                }
            }

            function chartZoomByDirection(delta) {
                if (delta > 0)
                    solutionPositionChart.zoom(Constants.commonChart.zoomInMult);
                else
                    solutionPositionChart.zoom(Constants.commonChart.zoomOutMult);
            }

            function stopZoomFeatures() {
                solutionCenterButton.checked = false;
                center_solution = false;
                solutionZoomAllButton.checked = false;
                zoom_all = false;
            }

            Layout.preferredWidth: parent.width
            Layout.preferredHeight: parent.height - Constants.commonChart.heightOffset
            Layout.alignment: Qt.AlignBottom
            Layout.fillHeight: true
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: true

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
                anchors.top: solutionPositionChart.top
                anchors.right: solutionPositionChart.right
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

                        model: Constants.solutionPosition.legendLabels

                        Row {
                            spacing: Constants.solutionPosition.legendLabelSpacing

                            Label {
                                id: marker

                                text: "+"
                                font.pointSize: (Constants.mediumPointSize + Constants.commonLegend.markerPointSizeOffset)
                                font.bold: true
                                color: Constants.solutionPosition.colors[index]
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                            Label {
                                id: label

                                text: modelData
                                font.pointSize: Constants.mediumPointSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            SwiftValueAxis {
                id: solutionPositionXAxis

                titleText: Constants.solutionPosition.xAxisTitleText + " (" + available_units[solutionPositionSelectedUnit.currentIndex] + ")"
            }

            SwiftValueAxis {
                id: solutionPositionYAxis

                titleText: Constants.solutionPosition.yAxisTitleText + " (" + available_units[solutionPositionSelectedUnit.currentIndex] + ")"
            }

            MouseArea {
                anchors.fill: solutionPositionChart
                onDoubleClicked: {
                    solutionPositionChart.stopZoomFeatures();
                    solutionZoomAllButton.checked = true;
                    solutionPositionChart.resetChartZoom();
                }
                onWheel: {
                    solutionPositionChart.stopZoomFeatures();
                    solutionPositionChart.chartZoomByDirection(wheel.angleDelta.y);
                }
                onPositionChanged: {
                    if (pressed) {
                        solutionPositionChart.stopZoomFeatures();
                        var current = solutionPositionChart.plotArea;
                        var x_unit = Utils.spanBetweenValues(solutionPositionXAxis.max, solutionPositionXAxis.min) / current.width;
                        var y_unit = Utils.spanBetweenValues(solutionPositionYAxis.max, solutionPositionYAxis.min) / current.height;
                        var delta_x = (mouse_x - mouseX) * x_unit;
                        var delta_y = (mouse_y - mouseY) * y_unit;
                        solutionPositionXAxis.max += delta_x;
                        solutionPositionXAxis.min += delta_x;
                        solutionPositionYAxis.max -= delta_y;
                        solutionPositionYAxis.min -= delta_y;
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
                    if (!solutionPositionTab.visible)
                        return ;

                    solution_position_model.fill_console_points(solutionPositionPoints);
                    if (!solutionPositionPoints.points.length)
                        return ;

                    solutionPositionArea.visible = true;
                    if (available_units != solutionPositionPoints.available_units)
                        available_units = solutionPositionPoints.available_units;

                    if (!lines.length || !scatters.length || !cur_scatters.length)
                        [scatters, cur_scatters, lines] = SolutionPlotLoop.setupScatterSeries(solutionPositionChart, Constants, Globals, solutionPositionXAxis, solutionPositionYAxis, Constants.solutionPosition.legendLabels, Constants.solutionPosition.colors, false, true);

                    var combined = [lines, scatters, cur_scatters];
                    solutionPositionPoints.fill_series(combined);
                    let point = SolutionPlotLoop.getCurSolution(solutionPositionPoints.cur_points);
                    if (point)
                        cur_solution = point;

                    if (center_solution)
                        solutionPositionChart.centerToSolution();

                    let hasData = false;
                    for (let idx in solutionPositionPoints.points) {
                        if (solutionPositionPoints.points[idx].length) {
                            hasData = true;
                            break;
                        }
                    }
                    let new_lat_min = Constants.solutionPosition.axesDefaultMin;
                    let new_lat_max = Constants.solutionPosition.axesDefaultMax;
                    let new_lon_min = Constants.solutionPosition.axesDefaultMin;
                    let new_lon_max = Constants.solutionPosition.axesDefaultMax;
                    solutionZoomAllButton.enabled = hasData;
                    solutionCenterButton.enabled = hasData;
                    if (hasData) {
                        new_lat_min = solutionPositionPoints.lat_min_;
                        new_lat_max = solutionPositionPoints.lat_max_;
                        new_lon_min = solutionPositionPoints.lon_min_;
                        new_lon_max = solutionPositionPoints.lon_max_;
                    } else {
                        zoom_all = true;
                        center_solution = false;
                        solutionZoomAllButton.checked = true;
                        solutionCenterButton.checked = false;
                        solutionPositionChart.resetChartZoom();
                    }
                    if (orig_lat_min != new_lat_min || orig_lat_max != new_lat_max || orig_lon_min != new_lon_min || orig_lon_max != new_lon_max) {
                        orig_lat_min = new_lat_min;
                        orig_lat_max = new_lat_max;
                        orig_lon_min = new_lon_min;
                        orig_lon_max = new_lon_max;
                        if (zoom_all)
                            solutionPositionChart.resetChartZoom();

                    }
                }
            }

        }

    }

}
