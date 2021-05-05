import "../Constants"
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
                    text: "| |"
                    ToolTip.visible: hovered
                    ToolTip.text: "Pause"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionClearButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    text: " X "
                    ToolTip.visible: hovered
                    ToolTip.text: "Clear"
                    onPressed: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionZoomAllButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    text: "[ ]"
                    ToolTip.visible: hovered
                    ToolTip.text: "Zoom All"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[3].checked, solutionButtonGroup.buttons[2].pressed, solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].checked])
                }

                Button {
                    id: solutionCenterButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
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
                Layout.preferredHeight: parent.height - Constants.commonChart.heightOffset
                Layout.alignment: Qt.AlignBottom
                Layout.bottomMargin: Constants.commonChart.margin
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

                            model: labels

                            Row {
                                Text {
                                    id: marker

                                    text: "+ "
                                    font.pointSize: Constants.commonLegend.markerPointSize
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: Constants.commonLegend.labelPointSize
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    id: solutionPositionXAxis

                    titleText: Constants.solutionPosition.xAxisTitleText
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
                    id: solutionPositionYAxis

                    titleText: Constants.solutionPosition.yAxisTitleText
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
                    interval: Globals.currentRefreshRate
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
                                cur_scatter.markerSize = Constants.commonChart.currentSolutionMarkerSize;
                                var scatter = solutionPositionChart.createSeries(ChartView.SeriesTypeScatter, labels[idx] + "scatter", solutionPositionXAxis, solutionPositionYAxis);
                                scatter.color = colors[idx];
                                scatter.markerSize = Constants.commonChart.solutionMarkerSize;
                                var line = solutionPositionChart.createSeries(ChartView.SeriesTypeLine, labels[idx], solutionPositionXAxis, solutionPositionYAxis);
                                line.color = colors[idx];
                                line.width = Constants.commonChart.solutionLineWidth;
                                line.useOpenGL = true;
                                scatter.useOpenGL = true;
                                cur_scatter.useOpenGL = true;
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
