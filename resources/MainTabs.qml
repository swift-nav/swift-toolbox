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
import QtQuick.Layouts
import SwiftConsole

Item {
    id: mainTabs

    property alias currentIndex: stackLayout.currentIndex
    property var subTabNames: mainTabs.currentIndex < 0 ? [] : stackLayout.children[stackLayout.currentIndex].subTabNames
    property int curSubTabIndex: -1

    StackLayout {
        id: stackLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.mainTabs.horizontalMargins
        anchors.rightMargin: Constants.mainTabs.horizontalMargins
        anchors.topMargin: Constants.mainTabs.verticalMargins
        anchors.bottomMargin: Constants.mainTabs.verticalMargins

        TrackingTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        SolutionTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        BaselineTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        ObservationTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        SettingsTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        UpdateTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }

        AdvancedTab {
            curSubTabIndex: mainTabs.curSubTabIndex
        }
    }
}
