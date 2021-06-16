import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedInsTab

    property variant lines: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    AdvancedInsPoints {
        id: advancedInsPoints
    }

    ColumnLayout {
        id: advancedInsArea

        width: parent.width
        height: parent.height

        ChartView {
            id: advancedInsChart

            visible: false
            title: Constants.advancedIns.title
            titleColor: Constants.advancedIns.titleColor
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.advancedIns.titlePointSize
                bold: true
            }

            Rectangle {
                id: lineLegend

                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: advancedInsChart.bottom
                anchors.left: advancedInsChart.left
                anchors.bottomMargin: Constants.advancedIns.legendBottomMargin
                anchors.leftMargin: Constants.advancedIns.legendLeftMargin

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.advancedIns.legendLabels

                        Row {
                            Component.onCompleted: {
                                for (var idx in Constants.advancedIns.lineColors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = Constants.advancedIns.lineColors[idx];

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
                                font.pointSize: Constants.smallPointSize
                                font.bold: true
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            ValueAxis {
                id: advancedInsXAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedIns.xAxisTickCount
                tickType: ValueAxis.TicksDynamic
                min: Constants.advancedIns.xAxisMin
                max: Constants.advancedIns.xAxisMax

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: advancedInsYAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedIns.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                min: Constants.advancedIns.yAxisMin
                max: Constants.advancedIns.yAxisMax

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: advancedInsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    advancedInsChart.visible = true;
                    advanced_ins_model.fill_console_points(advancedInsPoints);
                    if (!advancedInsPoints.points.length)
                        return ;

                    var points = advancedInsPoints.points;
                    textDataRow.visible = true;
                    if (!lines.length) {
                        for (var idx in advancedInsPoints.points) {
                            var line = advancedInsChart.createSeries(ChartView.SeriesTypeLine, idx, advancedInsXAxis);
                            line.color = Constants.advancedIns.lineColors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = advancedInsYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push(line);
                        }
                    }
                    imuTempText.text = `${advancedInsPoints.fields_data[0].toFixed(2)} C`;
                    imuConfText.text = `0x${advancedInsPoints.fields_data[1].toString(16).padStart(2, "0")}`;
                    rmsAccXText.text = `${advancedInsPoints.fields_data[2].toFixed(2)} g`;
                    rmsAccYText.text = `${advancedInsPoints.fields_data[3].toFixed(2)} g`;
                    rmsAccZText.text = `${advancedInsPoints.fields_data[4].toFixed(2)} g`;
                    advancedInsPoints.fill_series(lines);
                }
            }

        }

        RowLayout {
            id: textDataRow

            visible: false
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            Layout.alignment: Qt.AlignBottom

            Text {
                text: Constants.advancedIns.textDataLabels[0]
                Layout.preferredWidth: Constants.advancedIns.textDataLabelWidth
                font.pointSize: Constants.mediumPointSize
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedIns.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedIns.textDataBarBorderWidth

                Text {
                    id: imuTempText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedIns.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Text {
                text: Constants.advancedIns.textDataLabels[1]
                Layout.preferredWidth: Constants.advancedIns.textDataLabelWidth
                font.pointSize: Constants.mediumPointSize
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedIns.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedIns.textDataBarBorderWidth

                Text {
                    id: imuConfText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedIns.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Text {
                text: Constants.advancedIns.textDataLabels[2]
                Layout.preferredWidth: Constants.advancedIns.textDataLabelWidth
                font.pointSize: Constants.mediumPointSize
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedIns.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedIns.textDataBarBorderWidth

                Text {
                    id: rmsAccXText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedIns.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Text {
                text: Constants.advancedIns.textDataLabels[3]
                Layout.preferredWidth: Constants.advancedIns.textDataLabelWidth
                font.pointSize: Constants.mediumPointSize
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedIns.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedIns.textDataBarBorderWidth

                Text {
                    id: rmsAccYText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedIns.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Text {
                text: Constants.advancedIns.textDataLabels[4]
                Layout.preferredWidth: Constants.advancedIns.textDataLabelWidth
                font.pointSize: Constants.mediumPointSize
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedIns.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedIns.textDataBarBorderWidth

                Text {
                    id: rmsAccZText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedIns.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

        }

        FusionStatusFlags {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBar.urlBarHeight
            Layout.alignment: Qt.AlignBottom
        }

    }

}
