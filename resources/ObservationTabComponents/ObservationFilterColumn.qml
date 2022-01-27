import "../BaseComponents"
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

        SmallCheckBox {
            indicator.width: 15
            indicator.height: 15
            spacing: 2
            padding: 2
            verticalPadding: 0.2
            checked: !observationTableModel.code_filters.includes(modelData)
            onCheckedChanged: {
                observationTableModel.filter_prn(modelData, !checked);
                observationTableModel.update();
            }
            text: modelData + ": " + observationTableModel.codes.filter((x) => {
                return x === modelData;
            }).length
        }

    }

}
