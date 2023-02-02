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
        url: "http://localhost:63342/swift-toolbox/resources/web/map/index.html"
        onCertificateError: error.ignoreCertificateError()
    }

}