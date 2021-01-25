import QtQuick 2.12
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15

import SwiftConsole 1.0

ApplicationWindow {

    width: 640
    height: 480

    font.pointSize: 8

    ConsolePoints {
        id: console_points
    }

    ColumnLayout {

        anchors.fill: parent
        anchors.margins: 2
        spacing: 2

        ChartView {

            Layout.fillHeight: true
            Layout.fillWidth: true

            legend.font.pointSize: 7
            titleFont.pointSize: 8

            title: "Velocity"
            antialiasing: true

            LineSeries {
                id: velocity_graph
                name: "m/s"
                axisX: ValueAxis {
                    id: x_axis
                    labelsFont.pointSize: 7
                }
                axisY: ValueAxis {
                    id: y_axis
                    min: -1.0
                    max: 1.0
                    labelsFont.pointSize: 7
                }
                //useOpenGL: true
            }

            Timer {
                interval: 1000/5 // 5 Hz refresh
                running: true
                repeat: true
                onTriggered: {
                    data_model.fill_console_points(console_points);
                    if (!console_points.valid) {
                        return;
                    }
                    var points = console_points.points;
                    var last = points[points.length - 1];
                    x_axis.min = last.x - 10;
                    x_axis.max = last.x;
                    y_axis.min = console_points.min;
                    y_axis.max = console_points.max;
                    console_points.fill_series(velocity_graph);
                }
            }
        }

        Button {
            text: "Connect"
            onClicked: data_model.connect()
        }
    }

    Component.onCompleted: {
        visible = true;
    }
}
