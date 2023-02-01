import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtWebEngine
import SwiftConsole


Item {
    id: solutionMapTab

    WebEngineView {
        anchors.fill: parent
        url: "https://www.google.com"
    }

}