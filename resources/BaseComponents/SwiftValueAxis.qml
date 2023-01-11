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
import "../Constants"
import QtCharts

ValueAxis {
    gridVisible: true
    lineVisible: true
    minorGridVisible: true
    minorGridLineColor: Constants.commonChart.minorGridLineColor
    gridLineColor: Constants.commonChart.gridLineColor
    labelsColor: Constants.commonChart.labelsColor
    titleFont: Constants.commonChart.axisTitleFont
    labelsFont: Constants.commonChart.axisLabelsFont

    // A function that can be placed in onRangeChanged signal
    // slot to get that good tick spacing. It requires the 
    // tickType to be `ValueAxis.TicksDynamic` otherwise it
    // does nothing but spin yer CPU
    //
    // adapted from 
    // https://stackoverflow.com/questions/8506881/nice-label-algorithm-for-charts-with-minimum-ticks

    function getGoodTicks (new_min, new_max) {
        const ticksRange = (new_max - new_min)/(tickCount - 1);
        const exponent = Math.floor(Math.log10(ticksRange));
        const fraction = ticksRange/Math.pow(10, exponent);     
        var niceFraction = 10;
        if (fraction < 1.5) {
            niceFraction = 1;
        } else if (fraction < 3) {
            niceFraction = 2;
        } else if (fraction < 7) {
            niceFraction = 5;
        }

        const tickSpacing = niceFraction * Math.pow(10, exponent);

        tickAnchor = Math.ceil(new_min / tickSpacing) * tickSpacing;
        tickInterval = tickSpacing;
    }
}
