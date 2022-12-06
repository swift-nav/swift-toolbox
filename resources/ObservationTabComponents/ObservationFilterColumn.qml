import "../BaseComponents"
import "../Constants"
import "../TableComponents"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

ColumnLayout {
    id: obsFilterColumn

    property variant codes: []

    visible: codes.length > 0
    Layout.alignment: Qt.AlignTop

    Repeater {
        model: codes

        SmallCheckBox {
            indicatorWidth: 15
            indicatorHeight: 15
            spacing: 2
            padding: 2
            verticalPadding: 0.2
            checked: !observationTableModel.code_filters.includes(modelData)
            onCheckedChanged: {
                observationTableModel.filter_prn(modelData, !checked);
                observationTableModel.update();
            }
            text: modelData + ": " + observationTableModel.codes.filter(x => {
                    return x === modelData;
                }).length
        }
    }
}
