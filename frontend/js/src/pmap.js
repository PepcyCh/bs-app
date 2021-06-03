import React from 'react';
import { Map, MapvglLayer, MapvglView } from 'react-bmapgl';

const whiteStyle = [{
    "featureType": "background",
    "elementType": "geometry",
    "stylers": {
        "color": "#fefefeff"
    }
}, {
    "featureType": "poilabel",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "road",
    "elementType": "labels",
    "stylers": {
        "visibility": "on"
    }
}, {
    "featureType": "road",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#f0f1f7ff"
    }
}, {
    "featureType": "road",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "highway",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#e8eaf5ff"
    }
}, {
    "featureType": "highway",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "nationalway",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#e8eaf5ff"
    }
}, {
    "featureType": "nationalway",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "provincialway",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#e8eaf5ff"
    }
}, {
    "featureType": "provincialway",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "railway",
    "elementType": "geometry",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "highwaysign",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "highwaysign",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "nationalwaysign",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "nationalwaysign",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "provincialwaysign",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "provincialwaysign",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "tertiarywaysign",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "tertiarywaysign",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "subwaylabel",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "subwaylabel",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "poilabel",
    "elementType": "labels.text.fill",
    "stylers": {
        "color": "#80868dff"
    }
}, {
    "featureType": "poilabel",
    "elementType": "labels.text.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "districtlabel",
    "elementType": "labels.text.fill",
    "stylers": {
        "color": "#7d82b0ff"
    }
}, {
    "featureType": "districtlabel",
    "elementType": "labels.text.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "poilabel",
    "elementType": "labels",
    "stylers": {
        "visibility": "on"
    }
}, {
    "featureType": "airportlabel",
    "elementType": "labels",
    "stylers": {
        "visibility": "on"
    }
}, {
    "featureType": "airportlabel",
    "elementType": "labels.icon",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "airportlabel",
    "elementType": "labels.text.fill",
    "stylers": {
        "color": "#80868dff"
    }
}, {
    "featureType": "airportlabel",
    "elementType": "labels.text.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "manmade",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "manmade",
    "elementType": "geometry",
    "stylers": {
        "color": "#eef1fbff"
    }
}, {
    "featureType": "water",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "water",
    "elementType": "geometry",
    "stylers": {
        "color": "#a7c9ffff"
    }
}, {
    "featureType": "green",
    "elementType": "geometry",
    "stylers": {
        "visibility": "off",
        "color": "#eef1fbff"
    }
}, {
    "featureType": "subway",
    "elementType": "geometry",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "highway",
    "elementType": "labels",
    "stylers": {
        "visibility": "on"
    }
}, {
    "featureType": "highway",
    "elementType": "labels.text.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "highway",
    "elementType": "labels.text.fill",
    "stylers": {
        "color": "#7d82b0ff"
    }
}, {
    "featureType": "town",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "village",
    "elementType": "labels",
    "stylers": {
        "visibility": "off"
    }
}, {
    "featureType": "highway",
    "elementType": "geometry",
    "stylers": {
        "weight": "3"
    }
}, {
    "featureType": "cityhighway",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#e8eaf5ff"
    }
}, {
    "featureType": "arterial",
    "elementType": "geometry.fill",
    "stylers": {
        "color": "#e8eaf5ff"
    }
}, {
    "featureType": "arterial",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "cityhighway",
    "elementType": "geometry.stroke",
    "stylers": {
        "color": "#ffffff00"
    }
}, {
    "featureType": "road",
    "elementType": "labels.text.fill",
    "stylers": {
        "color": "#889db6ff"
    }
}];

export default class PMap extends React.Component {
    render() {
        const point = this.props.data.map(function(val, ind, arr) {
            return {
                geometry: {
                    type: "Point",
                    coordinates: [val.x, val.y]
                },
                value: val.value
            }
        });
        const color_func = function(item) {
            if (item.value < 80.0) {
                const temp = item.value / 80.0;
                const colorMain = Math.round(temp * 100.0 + 155.0);
                const colorOther = Math.round(temp * 150.0);
                return 'rgb(' + colorOther + ', ' + colorMain + ', ' + colorOther + ')';
            } else {
                const temp = 1.0 - (item.value - 80.0) / 20.0;
                const colorMain = Math.round(temp * 100.0 + 155.0);
                const colorOther = Math.round(temp * 150.0);
                return 'rgb(' + colorMain + ', ' + colorOther + ', ' + colorOther + ')';
            }
        };
        return (
            <Map
                center={{lng: 119.9, lat: 30.1}}
                zoom="11"
                mapStyleV2={{styleJson: whiteStyle}}>
                <MapvglView>
                    <MapvglLayer
                        type="PointLayer"
                        data={point}
                        options={{
                            blend: 'lighter',
                            size: 12,
                            color: color_func
                        }}>
                    </MapvglLayer>
                </MapvglView>
            </Map>
        );
    }
}