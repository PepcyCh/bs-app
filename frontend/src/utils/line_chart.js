const { FlexibleWidthXYPlot, XAxis, YAxis, Hint, LineSeries, HorizontalGridLines, VerticalGridLines } = reactVis;

class Chart extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            value: null
        };
    }

    recordValue = value => {
        this.setState({
            value
        });
    };

    render() {
        const {
            value
        } = this.state;
        return React.createElement(FlexibleWidthXYPlot, {
            xType: "time",
            height: this.props.height
        }, React.createElement(XAxis, null), React.createElement(YAxis, null), React.createElement(HorizontalGridLines, null), React.createElement(VerticalGridLines, null), React.createElement(LineSeries, {
            data: this.props.data,
            onNearestX: this.recordValue
        }), value ? React.createElement(Hint, {
            value: value
        }, React.createElement("div", {
            className: "rv-hint__content"
        }, `time: ${new Date(value.x).toUTCString()}`, React.createElement("br", null), `value: ${value.y}`)) : null);
    }

}

export function render_line_chart(node, height, data) {
    const chart = React.createElement(Chart, {
        height: height,
        data: data
    });
    ReactDOM.render(chart, node);
}
