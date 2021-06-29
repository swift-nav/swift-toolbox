// Baseline Plot Loop functions.
.pragma library

.import QtCharts 2.15 as QtCharts


function getCurSolution(curPoints) {
    let point = null
    for (let idx in curPoints) {
        if (curPoints[idx].length)
            point = curPoints[idx][0];
    }
    return point
}

function setupScatterSeries(chart, Constants, Globals, xaxis, yaxis) {
    let scatters = []
    let cur_scatters = []
    for (let idx in Constants.baselinePlot.legendLabels) {
        let cur_scatter = createScatter(chart, Constants, Globals, idx, xaxis, yaxis, Constants.commonChart.currentSolutionMarkerSize, "cur-scatter");
        if (idx == 0) {
            cur_scatter.append(0, 0);
            cur_scatter.pointsVisible = true;
            continue;
        }
        let scatter = createScatter(chart, Constants, Globals, idx, xaxis, yaxis, Constants.commonChart.solutionMarkerSize, "scatter");
        scatters.push(scatter);
        cur_scatters.push(cur_scatter);
    }
    return [scatters, cur_scatters]
}

function createScatter(chart, Constants, Globals, idx, xaxis, yaxis, markerSize, postFix) {
    var scatter = chart.createSeries(QtCharts.ChartView.SeriesTypeScatter, Constants.baselinePlot.legendLabels[idx] + postFix, xaxis, yaxis);
    scatter.color = Constants.baselinePlot.colors[idx];
    scatter.markerSize = markerSize;
    scatter.useOpenGL = Globals.useOpenGL;
    return scatter;
}
