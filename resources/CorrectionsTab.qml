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
import "BaseComponents"
import "CorrectionsTabComponents" as CorrectionsTabComponents
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

// Prototype SSR corrections panel. Covers orbit/clock, code/phase bias,
// and atmospheric (tile + STEC) corrections; SSR integrity/bounds messages
// are intentionally out of scope for this pass.
MainTab {
    id: correctionsTab

    SsrStreamTableModel {
        id: ssrStreamTableModel
    }

    SsrSatCorrectionTableModel {
        id: ssrSatCorrectionTableModel
    }

    SsrTileTableModel {
        id: ssrTileTableModel
    }

    SplitView {
        id: correctionsView

        anchors.fill: parent
        orientation: Qt.Vertical
        width: parent.width
        height: parent.height
        visible: true

        Rectangle {
            SplitView.preferredHeight: 0.25 * parent.height
            width: parent.width

            SwiftGroupBox {
                anchors.fill: parent
                anchors.topMargin: 4
                title: "SSR Streams"

                CorrectionsTabComponents.SsrTable {
                    anchors.fill: parent
                    tableModel: ssrStreamTableModel
                    columnNames: ["Message", "Age (s)", "Rate (s)", "IOD", "Count"]
                    columnWidths: [parent.width / 3, parent.width / 6, parent.width / 6, parent.width / 8, parent.width / 8]
                }
            }
        }

        Rectangle {
            SplitView.preferredHeight: 0.45 * parent.height
            width: parent.width

            SwiftGroupBox {
                anchors.fill: parent
                anchors.topMargin: 4
                title: "Satellite Corrections"

                CorrectionsTabComponents.SsrTable {
                    anchors.fill: parent
                    tableModel: ssrSatCorrectionTableModel
                    columnNames: ["Signal", "Radial", "Along", "Cross", "Clock C0", "Code Bias", "Phase Bias", "Age (s)"]
                    columnWidths: [parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8]
                }
            }
        }

        Rectangle {
            Layout.fillHeight: true
            width: parent.width

            SwiftGroupBox {
                anchors.fill: parent
                anchors.topMargin: 4
                title: "Atmospheric Tiles"

                CorrectionsTabComponents.SsrTable {
                    anchors.fill: parent
                    tableModel: ssrTileTableModel
                    columnNames: ["Tile Set", "Tile ID", "NW Corner", "Grid Size", "Sats"]
                    columnWidths: [parent.width / 6, parent.width / 6, 2 * parent.width / 6, parent.width / 6, parent.width / 6]
                }
            }
        }
    }
}
