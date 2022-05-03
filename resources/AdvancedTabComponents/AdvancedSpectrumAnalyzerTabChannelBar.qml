import "../Constants"
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

    ComboBox {
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
