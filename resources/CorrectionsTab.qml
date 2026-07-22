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
import "ObservationTabComponents" as ObservationTabComponents
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

// Monitors incoming corrections: auto-detects which bundle (Generic RTCM,
// Swift NXRTK-MSM5, Swift OSR, Swift SSR) is active and shows only the
// panels relevant to it, based on the SBP messages the device itself
// reports (MsgSsr* / MsgObs+MsgOsr with sender_id=0) - this works regardless
// of whether corrections are relayed through this console's NTRIP client or
// fetched by the receiver on its own. SSR integrity/bounds messages are
// intentionally out of scope for this pass. Raw RTCM3 message-ID detail
// (only available when this console is the NTRIP client) is temporarily
// not surfaced in the UI; the backend (rtcm_monitor.rs) still tracks it.
MainTab {
    id: correctionsTab

    property string bundleOverride: "AUTO"
    property var overrideOptions: [
        { text: "Auto", value: "AUTO" },
        { text: "Generic RTCM", value: "GENERIC" },
        { text: "Swift NXRTK-MSM5", value: "NXRTK_MSM5" },
        { text: "Swift OSR", value: "OSR" },
        { text: "Swift SSR", value: "SSR" }
    ]
    property bool showObservationsPanel: bundleOverride === "OSR" || bundleOverride === "NXRTK_MSM5" || (bundleOverride === "AUTO" && osrObservationTableModel.row_count > 0)
    // ssrSatCorrectionTableModel/ssrTileTableModel are only ever populated by
    // genuine MSG_SSR_* content, unlike ssrStreamTableModel which also
    // carries MSG_OBS/MSG_OSR rows.
    property bool showSsrPanels: bundleOverride === "SSR" || (bundleOverride === "AUTO" && (ssrSatCorrectionTableModel.row_count > 0 || ssrTileTableModel.row_count > 0))

    // Scales a panel to its content: one row of height per decoded message
    // plus one for the table header, clamped to [minRows, maxRows] so an
    // empty/sparse table doesn't collapse to nothing and a huge one doesn't
    // take over the tab (SwiftTableView's own scrollbar handles the rest).
    // `chrome` covers the SwiftGroupBox title/padding and, for
    // ObservationTable, its extra stats/filter rows above the table itself.
    function panelHeight(rowCount, minRows, maxRows, chrome) {
        var rows = Math.max(minRows, Math.min(maxRows, rowCount));
        return Constants.genericTable.cellHeight * (rows + 1) + chrome;
    }

    OsrObservationTableModel {
        id: osrObservationTableModel
    }

    SsrStreamTableModel {
        id: ssrStreamTableModel
    }

    SsrSatCorrectionTableModel {
        id: ssrSatCorrectionTableModel
    }

    SsrTileTableModel {
        id: ssrTileTableModel
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 4

        RowLayout {
            Layout.fillWidth: true

            Label {
                text: "Corrections bundle:"
                padding: 4
            }

            ComboBox {
                id: bundleComboBox

                model: overrideOptions.map(o => o.text)
                onCurrentIndexChanged: correctionsTab.bundleOverride = overrideOptions[currentIndex].value
            }
        }

        // Each panel below sizes itself to its decoded row count (see
        // panelHeight above) and the whole stack scrolls as one, rather
        // than squeezing panels to a fraction of the tab height - that way a
        // sparse table doesn't waste space, a large one doesn't crowd out
        // its neighbors, and panels pushed out of view (e.g. several shown
        // at once, or a small window) stay reachable by scrolling.
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ColumnLayout {
                width: correctionsTab.width
                spacing: 4

                SwiftGroupBox {
                    // Driven by the SBP messages the device itself reports
                    // (any MSG_SSR_*/MSG_OBS/MSG_OSR arrival), so this
                    // always works regardless of how the device got its
                    // corrections.
                    Layout.fillWidth: true
                    Layout.preferredHeight: panelHeight(ssrStreamTableModel.row_count, 2, 12, 40)
                    title: "Message Rates"

                    CorrectionsTabComponents.SsrTable {
                        anchors.fill: parent
                        tableModel: ssrStreamTableModel
                        columnNames: ["Message", "Age (s)", "Rate (s)", "IOD", "Count"]
                        columnWidths: [parent.width / 3, parent.width / 6, parent.width / 6, parent.width / 8, parent.width / 8]
                    }
                }

                SwiftGroupBox {
                    visible: showObservationsPanel
                    Layout.fillWidth: true
                    Layout.preferredHeight: showObservationsPanel ? panelHeight(osrObservationTableModel.row_count, 3, 20, 90) : 0
                    title: "Decoded Observations"

                    ObservationTabComponents.ObservationTable {
                        anchors.fill: parent
                        observationTableModel: osrObservationTableModel
                    }
                }

                SwiftGroupBox {
                    visible: showSsrPanels
                    Layout.fillWidth: true
                    Layout.preferredHeight: showSsrPanels ? panelHeight(ssrSatCorrectionTableModel.row_count, 3, 20, 40) : 0
                    title: "Satellite Corrections"

                    CorrectionsTabComponents.SsrTable {
                        anchors.fill: parent
                        tableModel: ssrSatCorrectionTableModel
                        columnNames: ["Signal", "Radial", "Along", "Cross", "Clock C0", "Code Bias", "Phase Bias", "Age (s)"]
                        columnWidths: [parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8, parent.width / 8]
                    }
                }

                SwiftGroupBox {
                    visible: showSsrPanels
                    Layout.fillWidth: true
                    Layout.preferredHeight: showSsrPanels ? panelHeight(ssrTileTableModel.row_count, 2, 10, 40) : 0
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
}
