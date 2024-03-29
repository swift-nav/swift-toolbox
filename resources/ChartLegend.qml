/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: lineLegend

    property int maximumHeight: 100
    property int openedHeight: gridView.count < maxCellsPerColumn ? gridView.cellHeight * (gridView.count + 1) : maximumHeight
    property int openCloseSpeed: Constants.commonLegend.shadeSpeed
    property int maxCellsPerColumn: Math.floor((maximumHeight - gridView.cellHeight) / gridView.cellHeight)
    property string cellTextSample: cellTextSampleDefault
    readonly property string cellTextSampleDefault: "12345"
    property alias model: gridView.model

    visible: gridView.count > 0
    radius: Constants.commonLegend.borderRadius
    height: openedHeight
    // Size to two cols if there are cells for 2+ cols.
    width: gridView.cellWidth * (gridView.count <= maxCellsPerColumn ? 1 : 2) + 1
    state: "opened"
    Component.onCompleted: {
        if (cellTextSample == cellTextSampleDefault)
            console.log("warning: default cellTextSample used in ChartLegend");
    }
    states: [
        State {
            name: "opened"

            PropertyChanges {
                target: lineLegend
                height: lineLegend.openedHeight
            }

            PropertyChanges {
                target: gridView
                visible: true
            }
        },
        State {
            name: "closed"

            PropertyChanges {
                target: lineLegend
                height: legendHideBar.height + 2
            }

            PropertyChanges {
                target: gridView
                visible: false
            }
        }
    ]
    transitions: [
        Transition {
            to: "closed"

            // reversible property should be what we want here instead of duplicating this,
            // but it doesn't seem to work right in this situation.
            SequentialAnimation {
                SmoothedAnimation {
                    property: "height"
                    duration: lineLegend.openCloseSpeed
                }

                PropertyAction {
                    property: "visible"
                }
            }
        },
        Transition {
            to: "opened"

            SequentialAnimation {
                PropertyAction {
                    property: "visible"
                }

                SmoothedAnimation {
                    property: "height"
                    duration: lineLegend.openCloseSpeed
                }
            }
        }
    ]

    Rectangle {
        // This rectangle ensures that the border of the legend is painted nicely.
        anchors.fill: parent
        z: 2
        color: "transparent"
        radius: parent.radius
        border.color: Constants.commonLegend.borderColor
        border.width: Constants.commonLegend.borderWidth
    }

    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.NoButton
        onWheel: wheelEvent => {
            if (wheelEvent.angleDelta.y > 0)
                legendScrollBar.decrease();
            else
                legendScrollBar.increase();
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            id: legendHideBar

            Layout.fillWidth: true
            color: Constants.commonLegend.shadeColor
            height: Constants.commonLegend.shadeHeight
            radius: lineLegend.radius

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    lineLegend.state = lineLegend.state == "opened" ? "closed" : "opened";
                }
                cursorShape: pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
                //drag.target: lineLegend
                hoverEnabled: true
            }
        }

        GridView {
            id: gridView

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            flow: GridView.FlowTopToBottom
            cellWidth: Constants.commonLegend.markerWidth + legendTextMetrics.width + 4
            cellHeight: legendTextMetrics.height + 2
            boundsBehavior: Flickable.StopAtBounds

            TextMetrics {
                id: legendTextMetrics

                font.family: Constants.fontFamily
                font.pixelSize: Constants.xSmallPixelSize
                text: cellTextSample
            }

            ScrollBar.horizontal: ScrollBar {
                id: legendScrollBar
            }

            delegate: Row {
                padding: 1
                leftPadding: 4
                rightPadding: leftPadding

                Rectangle {
                    id: marker

                    color: modelData.color
                    width: Constants.commonLegend.markerWidth
                    height: Constants.commonLegend.markerHeight
                    anchors.verticalCenter: parent.verticalCenter
                }

                Label {
                    id: label

                    text: modelData.name
                    font: legendTextMetrics.font
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                }
            }
        }
    }
}
