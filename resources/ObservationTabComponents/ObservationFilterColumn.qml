import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    id: obsFilterColumn

    property variant codes: []

    visible: codes.length > 0
    Layout.alignment: Qt.AlignTop

    Repeater {
        model: codes

        CheckBox {
            spacing: 2
            padding: 2
            verticalPadding: 0.2
            checked: true
            onCheckedChanged: {
                observationTableModel.filter_prn(modelData, !checked);
                observationTableModel.update();
            }
            text: modelData + ": " + observationTableModel.get_code_count(modelData)
        }

    }

}
