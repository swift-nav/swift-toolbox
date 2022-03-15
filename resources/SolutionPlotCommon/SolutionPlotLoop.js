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

function setupScatterSeries(chart, Constants, Globals, xaxis, yaxis, labels, colors, use_ref=true, use_line=false) {
    let scatters = []
    let cur_scatters = []
    let line = null
    for (let idx in labels) {
        let cur_scatter = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.currentSolutionMarkerSize, "cur-scatter", labels[idx], colors[idx]);
        if (use_ref && idx == 0) {
            cur_scatter.append(0, 0);
            cur_scatter.pointsVisible = true;
            continue;
        }
        let scatter = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.solutionMarkerSize, "scatter", labels[idx], colors[idx]);
        if (use_line) {
            line = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.solutionLineWidth, "line", "line", "grey", /*series=*/QtCharts.ChartView.SeriesTypeLine);
            line.width = Constants.commonChart.solutionLineWidth;
        }
        scatters.push(scatter);
        cur_scatters.push(cur_scatter);
    }
    return [scatters, cur_scatters, line]
}

function createSeries(chart, Globals, xaxis, yaxis, markerSize, postFix, label, color, series=QtCharts.ChartView.SeriesTypeScatter) {
    var scatter = chart.createSeries(series,  label + postFix, xaxis, yaxis);
    scatter.color = color;
    scatter.borderColor = "transparent";
    scatter.markerSize = markerSize;
    scatter.useOpenGL = Globals.useOpenGL;
    return scatter;
}
