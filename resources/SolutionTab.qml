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
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "SolutionTabComponents" as SolutionTabComponents

MainTab {
    id: solutionTab

    subTabNames: Globals.enableMap ? ["Position", "Velocity", "Map"] : ["Position", "Velocity"]
    curSubTabIndex: 0

    SplitView {
        id: solutionSplitView

        anchors.fill: parent
        orientation: Qt.Horizontal

        SolutionTabComponents.SolutionTable {
            SplitView.minimumWidth: Constants.solutionTable.minimumWidth
        }

        StackLayout {
            id: solutionBarLayout

            SplitView.minimumWidth: Constants.solutionPosition.minimumWidth
            SplitView.fillWidth: true
            SplitView.fillHeight: true
            currentIndex: curSubTabIndex

            SolutionTabComponents.SolutionPositionTab {
            }

            SolutionTabComponents.SolutionVelocityTab {
            }

            Rectangle {
                width: parent.width
                height: parent.height
                Loader {
                    sourceComponent: Globals.enableMap ? solutionMap : null
                }
            }

            property Component solutionMap: SolutionTabComponents.SolutionMapTab {
            }
        }
    }
}
