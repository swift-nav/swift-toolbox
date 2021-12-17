import "../Constants"
import "../BaseComponents"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedImuTab

    property variant lines: []

    width: parent.width
    height: parent.height

    AdvancedImuPoints {
        id: advancedImuPoints
    }

    ColumnLayout {
        id: advancedImuArea

        width: parent.width
        height: parent.height

        ChartView {
            id: advancedImuChart

            visible: false
            title: Constants.advancedImu.title
            titleColor: Constants.commonChart.titleColor
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: true
            titleFont: Constants.commonChart.titleFont

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
                                font.pointSize: Constants.smallPointSize
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
                min: Constants.advancedImu.xAxisMin
                max: Constants.advancedImu.xAxisMax

            }

            SwiftValueAxis {
                id: advancedImuYAxis

                tickInterval: Constants.advancedImu.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                min: Constants.advancedImu.yAxisMin
                max: Constants.advancedImu.yAxisMax

            }

            Timer {
                id: advancedImuTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    advancedImuChart.visible = true;
                    advanced_imu_model.fill_console_points(advancedImuPoints);
                    if (!advancedImuPoints.points.length)
                        return ;

                    var points = advancedImuPoints.points;
                    textDataRow.visible = true;
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

            visible: false
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.advancedImu.urlBarHeight
            Layout.alignment: Qt.AlignBottom

            Label {
                text: Constants.advancedImu.textDataLabels[0]
                Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuTempText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[1]
                Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuConfText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[2]
                Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccXText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[3]
                Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccYText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

            Label {
                text: Constants.advancedImu.textDataLabels[4]
                Layout.preferredWidth: Constants.advancedImu.textDataLabelWidth
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccZText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pointSize: Constants.mediumPointSize
                }

            }

        }

    }

}
