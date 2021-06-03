import React from 'react';
import {
    FlexibleWidthXYPlot,
    XAxis,
    YAxis,
    Hint,
    LineSeries,
    HorizontalGridLines,
    VerticalGridLines,
} from 'react-vis';

export default class Chart extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            value: null
        };
    }

    recordValue = value => {
        this.setState({value});
    }

    render() {
        const { value } = this.state;
        return (
            <FlexibleWidthXYPlot height={this.props.height} xType="time">
                <XAxis />
                <YAxis />
                <HorizontalGridLines />
                <VerticalGridLines />
                <LineSeries data={this.props.data} onNearestX={this.recordValue} />
                {
                    value
                    ? <Hint value={value}>
                        <div className="rv-hint__content">
                            {`time: ${new Date(value.x).toUTCString()}`}
                            <br />
                            {`value: ${value.y}`}
                        </div>
                    </Hint>
                    : null
                }
            </FlexibleWidthXYPlot>
        );
    }
}