import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtWebEngine
import SwiftConsole
import QtWebChannel

Item {
    id: solutionMapTab

    SolutionMap {
        id: solutionMap
        WebChannel.id: "currPos"
        signal recvPos(int id, double lat, double z)
        signal clearPos
    }

    WebChannel {
        id: solutionMapChannel
        registeredObjects: [solutionMap]
    }

    WebEngineView {
        anchors.fill: parent
        webChannel: solutionMapChannel
        url: Constants.solutionMap.pageURL
        onCertificateError: error.ignoreCertificateError()
        onJavaScriptConsoleMessage: (level, message, lineNumber, sourceID) => console.log("[MAP LOG] " + message)
    }
}
