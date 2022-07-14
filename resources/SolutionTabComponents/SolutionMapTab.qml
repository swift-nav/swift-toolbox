import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts 2.15
import QtGraphicalEffects 1.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

import QtLocation 5.15
import QtPositioning 5.15

Item {
    id: solutionMapTab
    visible: true

    property variant cur_solution: null

    Plugin {
        id: mapPlugin
        name: "osm"
    }

    property Component markerProvider: MapQuickItem {
        anchorPoint.x: rect.width / 2
        anchorPoint.y: rect.height / 2
        sourceItem: Rectangle{
            id: rect
            width: 5 
            height: 5
            color: "blue"
            radius: width*0.5
        }
    }

    function addMarker(coordinate){
        var marker = markerProvider.createObject()
        marker.coordinate = coordinate
        positionMap.addMapItem(marker)
    }

    Map {
        id: positionMap
        anchors.fill: parent
        plugin: mapPlugin
        center: QtPositioning.coordinate(59.91, 10.75) // Oslo
        zoomLevel: 20

    }

    SolutionPositionPoints2 {
        id: solutionPositionPoints2
    }

    Timer {
        interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
        running: true
        repeat: true
        onTriggered: {
            if (!solutionMapTab.visible)
                return;

            solution_position_model2.fill_console_points(solutionPositionPoints2);

            if (!solutionPositionPoints2.points.length)
                return;

            let point = SolutionPlotLoop.getCurSolution(solutionPositionPoints2.cur_points);
            cur_solution = point;

            if (!cur_solution)
               return;

            positionMap.center.latitude = cur_solution.y;
            positionMap.center.longitude = cur_solution.x;

            let coordinate = QtPositioning.coordinate(cur_solution.y, cur_solution.x);
            addMarker(coordinate);
        }
    }
}
