import "../Constants"
import QtCharts 2.15

ValueAxis {
    // A function that returns the best tick interval for the
    // plot to have ticks that land on round numbers
    // --
    // adapted from
    // https://stackoverflow.com/questions/8506881/nice-label-algorithm-for-charts-with-minimum-ticks
    function getGoodTickInterval() : real {
        const ticksRange = (max - min) / (tickCount - 1);
        const exponent = Math.floor(Math.log10(ticksRange));
        const fraction = ticksRange / Math.pow(10, exponent);
        var niceFraction = 10;
        if (fraction < 1.5)
            niceFraction = 1;
        else if (fraction < 3)
            niceFraction = 2;
        else if (fraction < 7)
            niceFraction = 5;
        return niceFraction * Math.pow(10, exponent);
    }

    // A function that can be used to get that good tick spacing.
    // --
    // WARNING! if the tickType is dynamic, and you drastically change
    // the size of the plot via zooming or plotting a new series, it is
    // NOT ENOUGH to have the getGoodTicks function on the onRangeChanged
    // slot of the ValueAxis to avoid your plot trying to render thousands
    // of tick lines. I think there may be a way around that, but instead
    // what I chose to do is manually call the freezeTicks function below before
    // the big change in range, and then calling the setGoodTicks() function
    // after.
    function setGoodTicks(tickSpacing: real) {
        tickAnchor = Math.ceil(min / tickSpacing) * tickSpacing;
        tickInterval = tickSpacing;
        tickType = ValueAxis.TicksDynamic;
    }

    // Kind of a throwaway function, but it is used whenever you
    // need to make sure you won't render too many ticks.
    function freezeTicks() {
        tickType = ValueAxis.TicksFixed;
    }

    gridVisible: true
    lineVisible: true
    minorGridVisible: true
    minorGridLineColor: Constants.commonChart.minorGridLineColor
    gridLineColor: Constants.commonChart.gridLineColor
    labelsColor: Constants.commonChart.labelsColor
    titleFont: Constants.commonChart.axisTitleFont
    labelsFont: Constants.commonChart.axisLabelsFont
}
