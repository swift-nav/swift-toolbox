import QtQuick 2.5
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

    TrackingSignalsPoints {
        id: tracking_signals_points
    }
    
    
    ColumnLayout {
        id: tabzone
        anchors.fill: parent
        spacing: 2
        width: parent.width
        height: parent.height

        
        Rectangle {
            id: tabs
            height: parent.height - console_log.height
            width: parent.width
            TabBar {
                id: bar
                z: 100
                width: parent.width
                
                Repeater {
                    model: ["Tracking", "Solution", "Baseline", "Observations", "Settings", "Update", "Advanced"]

                    TabButton {
                        text: modelData
                        width: implicitWidth
                    }
                }
            }
            
            StackLayout {
                width: parent.width
                height: parent.height - bar.height
                anchors.top: bar.bottom
                currentIndex: bar.currentIndex
                TrackingTab {}
                Item {
                    id: solutionTab
                    RowLayout{
                        anchors.fill: parent
                        spacing: 2
                        ListView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            Layout.alignment: Qt.AlignLeft
                            

                            Component {
                                id: contactsDelegate
                                Rectangle {
                                    id: wrapper
                                    width: 180
                                    height: contactInfo.height
                                    color: ListView.isCurrentItem ? "black" : "red"
                                    Text {
                                        id: contactInfo
                                        text: name + ": " + number
                                        color: wrapper.ListView.isCurrentItem ? "red" : "black"
                                    }
                                }
                            }

                            model: ContactModel {}
                            delegate: contactsDelegate
                            focus: true
                        }

                        ChartView {

                            
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            Layout.alignment: Qt.AlignLeft
                            titleFont.pointSize: 8
                            antialiasing: true
                            width: parent.width/2
                            legend.font.pointSize: 7
                            legend.alignment: Qt.AlignTop
                            legend.showToolTips: true

                            ValueAxis {
                                id: x_axis
                                labelsFont.pointSize: 7
                                titleText: "GPS Time of Week"
                            }
                            ValueAxis {
                                id: y_axis
                                min: -1.0
                                max: 1.0
                                labelsFont.pointSize: 7
                            }

                            LineSeries {
                                id: hseries
                                name: "Horizontal [m/s]"
                                axisX: x_axis
                                axisY: y_axis
                                //useOpenGL: true
                            }
                            LineSeries {
                                id: vseries
                                name: "Vertical [m/s]"
                                axisX: x_axis
                                axisY: y_axis
                                //useOpenGL: true
                            }

                            Timer {
                                interval: 1000/5 // 5 Hz refresh
                                running: true
                                repeat: true
                                onTriggered: {
                                    if (!solutionTab.visible) {
                                        return;
                                    }
                                    data_model.fill_console_points(console_points);
                                    if (!console_points.valid) {
                                        return;
                                    }
                                    var hpoints = console_points.hpoints;
                                    var last = hpoints[hpoints.length - 1];
                                    x_axis.min = last.x - 10;
                                    x_axis.max = last.x;
                                    y_axis.min = console_points.min_;
                                    y_axis.max = console_points.max_;
                                    console_points.fill_hseries(hseries);
                                    console_points.fill_vseries(vseries);
                                }
                            }
                        }
                    }
                    
                }    
                Item {
                    id: baselineTab
                }
                Item {
                    id: observationsTab
                }
                Item {
                    id: settingsTab
                }
                Item {
                    id: updateTab
                }
                Item {
                    id: advancedTab
                }
            }

        }
        Rectangle {
            id: console_log
            width: parent.width
            height: 100
            Layout.alignment: Qt.AlignBottom

            RowLayout {
                Button {
                    text: "Connect"
                    onClicked: data_model.connect()
                }
                Button {
                    text: "File In"
                    onClicked: data_model.readfile()
                }
            }
        }
    }

    Component.onCompleted: {
        visible = true;
    }
}
