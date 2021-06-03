import Chart from './chart.js';
import PMap from './pmap.js';

export function render_line_chart(node, height, data) {
    const chart = <Chart height={height} data={data} />;
    ReactDOM.render(chart, node);
}

export function render_map(node, data) {
    const map = <PMap data={data} />;
    ReactDOM.render(map, node);
}