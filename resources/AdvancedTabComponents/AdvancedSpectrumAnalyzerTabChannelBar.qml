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
import "../Constants"
import "../BaseComponents"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

RowLayout {
    id: channelSelectionRowLayout

    property alias dropdownIdx: channelDropdown.currentIndex
    property real parentWidth: parent ? parent.width : width

    Item {
        Layout.preferredWidth: parent.parentWidth / 6
    }

    Label {
        text: Constants.advancedSpectrumAnalyzer.dropdownLabel
        font.bold: true
        color: Constants.commonChart.titleColor
        antialiasing: Globals.useAntiAliasing
    }

    SwiftComboBox {
        id: channelDropdown

        Layout.preferredHeight: Constants.advancedSpectrumAnalyzer.dropdownHeight
        Layout.preferredWidth: Constants.advancedSpectrumAnalyzer.dropdownWidth
        topInset: 0
        bottomInset: 0
        model: Constants.advancedSpectrumAnalyzer.dropdownModel
        onActivated: {
            backend_request_broker.advanced_spectrum_analyzer_channel(currentIndex);
        }
    }

    Item {
        Layout.fillWidth: true
    }

    Label {
        text: Constants.advancedSpectrumAnalyzer.dropdownRowSuggestionText
        font.italic: true
        antialiasing: Globals.useAntiAliasing
    }

    Item {
        Layout.preferredWidth: parent.parentWidth / 6
    }
}
