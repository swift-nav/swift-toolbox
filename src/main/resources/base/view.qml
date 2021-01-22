import QtQuick 2.12
import QtQuick.Controls 2.12
import QtCharts 2.2

import SwiftConsole 1.0

ApplicationWindow {

    width: 640
    height: 480

    ConsolePoints {
        id: console_points
    }

    ChartView {
        title: "Velocity"
        anchors.fill: parent
        antialiasing: true

        LineSeries {
            id: velocity_graph
            name: "m/s"
            axisX: ValueAxis {
                id: x_axis
            }
            axisY: ValueAxis {
                id: y_axis
                min: -1.0
                max: 1.0
            }
            useOpenGL: true
        }

        Timer {
            interval: 1000/50 // 50 Hz refresh
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

    Component.onCompleted: {
        visible = true;
    }
}
