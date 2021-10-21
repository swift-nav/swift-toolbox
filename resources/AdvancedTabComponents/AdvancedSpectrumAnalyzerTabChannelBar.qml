import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias dropdownIdx: channelDropdown.currentIndex

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    RowLayout {
        id: channelSelectionRowLayout

        width: parent.width
        height: parent.height

        Item {
            Layout.preferredWidth: parent.width / 6
        }

        Label {
            text: Constants.advancedSpectrumAnalyzer.dropdownLabel
            font.bold: true
            color: Constants.advancedSpectrumAnalyzer.titleColor
            antialiasing: true
        }

        ComboBox {
            id: channelDropdown

            Layout.preferredHeight: Constants.advancedSpectrumAnalyzer.dropdownHeight
            Layout.preferredWidth: Constants.advancedSpectrumAnalyzer.dropdownWidth
            model: Constants.advancedSpectrumAnalyzer.dropdownModel
            onActivated: {
                data_model.advanced_spectrum_analyzer_channel(currentIndex);
            }
        }

        Item {
            Layout.fillWidth: true
        }

        Label {
            text: Constants.advancedSpectrumAnalyzer.dropdownRowSuggestionText
            font.italic: true
            antialiasing: true
        }

        Item {
            Layout.preferredWidth: parent.width / 6
        }

    }

}
