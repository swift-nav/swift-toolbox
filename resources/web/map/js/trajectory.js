import mapboxGlStyleSwitcher from 'https://cdn.skypack.dev/mapbox-gl-style-switcher';

mapboxgl.accessToken = "pk.eyJ1Ijoic3dpZnQtYWRyaWFuIiwiYSI6ImNsZTN1MW82bDA2OGgzdXFvOWFuZTJlMnEifQ.9nR8m0C-B_ISNR4r4cMExw";
var map = new mapboxgl.Map({
    container: 'map',
    style: "mapbox://styles/mapbox/dark-v11",
    center: [-122.486052, 37.830348],  // Initial focus coordinate
    zoom: 16
});

var focus = false;

class FocusToggle {
    onAdd(map) {
        this._map = map;
        this._btn = document.createElement("button");
        this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-focus-toggle";
        this._btn.type = "button";
        this._btn.onclick = () => {
            focus = !focus;
            if (focus) {
                this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-unfocus-toggle";
            } else {
                this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-focus-toggle";
            }
        };
        this._container = document.createElement("div");
        this._container.className = "mapboxgl-ctrl-group mapboxgl-ctrl";
        this._container.appendChild(this._btn);
        return this._container;
    }

    onRemove() {
        this._container.parentNode.removeChild(this._container);
        this._map = undefined;
    }
}

map.addControl(new mapboxGlStyleSwitcher.MapboxStyleSwitcherControl());
map.addControl(new FocusToggle(), "top-right");
map.addControl(new mapboxgl.NavigationControl());

map.on('load', function () {
    console.log("loaded");
    var start;
    var data = [];
    var lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];
    for (let i = 0; i < lines.length; i++) {
        data.push({
            type: 'FeatureCollection',
            features: []
        });
        map.addSource(`route${i}`, {
            type: 'geojson',
            cluster: false,
            data: {
                type: 'FeatureCollection',
                features: data[i]
            }
        });
        map.addLayer({
            id: `route${i}`,
            type: 'circle',
            source: `route${i}`,
            paint: {
                'circle-color': lines[i],
                'circle-radius': 3,
            }
        });
    }

    new QWebChannel(qt.webChannelTransport, (channel) => {
        channel.objects.currPos.recvPos.connect((id, lat, lng) => {
            console.log(`received ${id} ${lat} ${lng}`);
            const pos = [lat, lng];
            if (!start) {
                new mapboxgl.Marker().setLngLat(pos).addTo(map);
                start = pos;
                map.panTo(pos);
            } else if (focus) map.panTo(pos);
            data[id].features.push({
                type: 'Feature',
                geometry: {
                    type: 'Point',
                    coordinates: pos
                }
            });
            map.getSource(`route${id}`).setData(data[id]);
        })
    });
});
