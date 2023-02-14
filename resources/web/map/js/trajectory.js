var data = {
    type: 'Feature',
    properties: {},
    geometry: {
        type: 'LineString',
        coordinates: []
    }
};

const style = {
    "version": 8,
    "sources": {
        "osm": {
            "type": "raster",
            "tiles": ["https://a.tile.openstreetmap.org/{z}/{x}/{y}.png"],
            "tileSize": 256,
            "attribution": "&copy; OpenStreetMap Contributors",
            "maxzoom": 22
        },
        "route": {
            "type": "geojson",
            data
        }
    },
    "layers": [
        {
            "id": "osm",
            "type": "raster",
            "source": "osm"
        },
        {
            "id": "route",
            "type": "circle",
            "source": "route",
            "paint": {
                'circle-color': 'red',
                'circle-radius': 3,
            }
        }
    ]
};

var map = new maplibregl.Map({
    container: 'map',
    style,
    center: [-122.486052, 37.830348],  // Initial focus coordinate
    zoom: 16
});

// MapLibre GL JS does not handle RTL text by default, so we recommend adding this dependency to fully support RTL rendering.
maplibregl.setRTLTextPlugin('https://api.mapbox.com/mapbox-gl-js/plugins/mapbox-gl-rtl-text/v0.2.1/mapbox-gl-rtl-text.js');

// Add zoom and rotation controls to the map.
map.addControl(new maplibregl.NavigationControl());

map.on('load', function () {
    console.log("loaded");
    var start;
    new QWebChannel(qt.webChannelTransport, (channel) => {
        channel.objects.currPos.recvPos.connect((id, lat, lng) => {
            console.log(`received ${id} ${lat} ${lng}`);
            const pos = [lat, lng];
            if (!start) {
                new maplibregl.Marker().setLngLat(pos).addTo(map);
                start = pos;
            }
            map.panTo(pos);
            data.geometry.coordinates.push(pos);
            map.getSource('route').setData(data);
        })
    });
});
