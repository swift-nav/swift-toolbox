import "./Constants"
import QtQuick 2.14
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    LogPanelData {
        id: logPanelData
    }

    Text {
        id: innerText

        text: ""
        padding: 5
    }

    Timer {
        interval: Constants.currentRefreshRate
        running: true
        repeat: true
        onTriggered: {
            if (innerText.text.length > 32000) {
                innerText.text = "Overflowed";
                logPanelData.entries = [];
                return ;
            }
            log_panel_model.fill_data(logPanelData);
            let newText = '';
            for (const entry of logPanelData.entries) {
                newText = entry + '\n' + newText;
            }
            logPanelData.entries = [];
            innerText.text = newText + innerText.text;
        }
    }

}
