import "../Constants"
import QtCharts

CategoryAxis {
    labelsVisible: true
    labelsPosition: CategoryAxis.AxisLabelsPositionOnValue
    gridVisible: true
    lineVisible: true
    minorGridVisible: true
    minorGridLineColor: Constants.commonChart.minorGridLineColor
    gridLineColor: Constants.commonChart.gridLineColor
    labelsColor: Constants.commonChart.labelsColor
    titleFont: Constants.commonChart.axisTitleFont
    labelsFont: Constants.commonChart.axisLabelsFont
}
