import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedImuTab

    property variant lines: []

    AdvancedImuPoints {
        id: advancedImuPoints
    }

    ColumnLayout {
        id: advancedImuArea

        anchors.fill: parent
        visible: true

        ChartView {
            id: advancedImuChart

            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            visible: true
            title: Constants.advancedImu.title
            titleColor: Constants.commonChart.titleColor
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: Globals.useAntiAliasing
            titleFont: Constants.commonChart.titleFont

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            Rectangle {
                id: lineLegend

                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: advancedImuChart.bottom
                anchors.left: advancedImuChart.left
                anchors.bottomMargin: Constants.advancedImu.legendBottomMargin
                anchors.leftMargin: Constants.advancedImu.legendLeftMargin

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.advancedImu.legendLabels

                        Row {
                            Component.onCompleted: {
                                for (var idx in Constants.advancedImu.lineColors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = Constants.advancedImu.lineColors[idx];

                                }
                            }

                            Rectangle {
                                id: marker

                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Label {
                                id: label

                                text: modelData
                                font.pixelSize: Constants.smallPixelSize
                                font.bold: true
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            SwiftValueAxis {
                id: advancedImuXAxis

                tickInterval: Constants.advancedImu.xAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
                min: Constants.advancedImu.xAxisMin
                max: Constants.advancedImu.xAxisMax
                reverse: true
            }

            SwiftValueAxis {
                id: advancedImuYAxis

                tickInterval: Constants.advancedImu.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
                min: Constants.advancedImu.yAxisMin
                max: Constants.advancedImu.yAxisMax
            }

            ScatterSeries {
                name: "emptySeries"
                axisYRight: advancedImuYAxis
                axisX: advancedImuXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL
                markerSize: 0.1

                XYPoint {
                    x: 0
                    y: 0
                }

            }

            Timer {
                id: advancedImuTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedImuTab.visible)
                        return ;

                    advanced_imu_model.fill_console_points(advancedImuPoints);
                    if (!advancedImuPoints.points.length)
                        return ;

                    var points = advancedImuPoints.points;
                    advancedImuArea.visible = true;
                    if (!lines.length) {
                        for (var idx in advancedImuPoints.points) {
                            var line = advancedImuChart.createSeries(ChartView.SeriesTypeLine, idx, advancedImuXAxis);
                            line.color = Constants.advancedImu.lineColors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = advancedImuYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push(line);
                        }
                    }
                    imuTempText.text = `${advancedImuPoints.fields_data[0].toFixed(2)} C`;
                    imuConfText.text = `0x${advancedImuPoints.fields_data[1].toString(16).padStart(2, "0")}`;
                    rmsAccXText.text = `${advancedImuPoints.fields_data[2].toFixed(2)} g`;
                    rmsAccYText.text = `${advancedImuPoints.fields_data[3].toFixed(2)} g`;
                    rmsAccZText.text = `${advancedImuPoints.fields_data[4].toFixed(2)} g`;
                    advancedImuPoints.fill_series(lines);
                }
            }

        }

        RowLayout {
            id: textDataRow

            visible: true
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.advancedImu.urlBarHeight
            Layout.alignment: Qt.AlignBottom

            Label {
                text: Constants.advancedImu.textDataLabels[0]
            }

            Rectangle {
                Layout.preferredWidth: parent.width / 15
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuTempText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 C"
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[1]
            }

            Rectangle {
                Layout.preferredWidth: parent.width / 15
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuConfText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0x00"
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[2]
            }

            Rectangle {
                Layout.preferredWidth: parent.width / 15
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccXText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[3]
            }

            Rectangle {
                Layout.preferredWidth: parent.width / 15
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccYText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[4]
            }

            Rectangle {
                Layout.preferredWidth: parent.width / 15
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccZText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }

            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedMagnetometer.suggestionTextRowHeight

                Label {
                    text: Constants.advancedMagnetometer.suggestionText
                    font.italic: true
                    antialiasing: Globals.useAntiAliasing
                    anchors.horizontalCenter: parent.horizontalCenter
                }

            }

        }

    }

}
