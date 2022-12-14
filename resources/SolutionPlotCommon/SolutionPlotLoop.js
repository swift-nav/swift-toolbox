/**
 * Copyright (c) 2022 Swift Navigation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
    if (use_line)
        line = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.solutionLineWidth, "line", "line", "lightgrey", /*series=*/QtCharts.ChartView.SeriesTypeLine);
    for (let idx in labels) {
        let cur_scatter = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.currentSolutionMarkerSize, "cur-scatter", labels[idx], colors[idx]);
        if (use_ref && idx == 0) {
            cur_scatter.append(0, 0);
            cur_scatter.pointsVisible = true;
            continue;
        }
        let scatter = createSeries(chart, Globals, xaxis, yaxis, Constants.commonChart.solutionMarkerSize, "scatter", labels[idx], colors[idx]);
        
        scatters.push(scatter);
        cur_scatters.push(cur_scatter);
    }
    return [scatters, cur_scatters, line]
}

function createSeries(chart, Globals, xaxis, yaxis, markerSize, postFix, label, color, series=QtCharts.ChartView.SeriesTypeScatter) {
    var scatter = chart.createSeries(series,  label + postFix, xaxis, yaxis);
    scatter.color = color;
    scatter.width = markerSize;
    scatter.borderColor = "transparent";
    scatter.markerSize = markerSize;
    scatter.useOpenGL = Globals.useOpenGL;
    return scatter;
}
