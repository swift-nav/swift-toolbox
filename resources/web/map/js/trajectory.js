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
            if (focus) this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-unfocus-toggle";
            else this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-focus-toggle";
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

const lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];

var data = [];

setupData();

function setupData() {
    data = [];
    for (let i = 0; i < lines.length; i++) {
        data.push({
            type: 'FeatureCollection',
            features: []
        });
    }
}

function setupLayers() {
    for (let i = 0; i < lines.length; i++) {
        if (map.getSource(`route${i}`) != null) {
            continue;
        }
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
}

function syncLayers() {
    for (let i = 0; i < lines.length; i++) {
        map.getSource(`route${i}`).setData(data[i]);
    }
}

// https://stackoverflow.com/a/39006388
function createGeoJsonCircle(center, radiusKm, points = 64) {
    let coords = {latitude: center[1], longitude: center[0]};

    const lngKm = 111.320, latKm = 110.574;

    let ret = [];
    let distanceX = radiusKm / (lngKm * Math.cos(coords.latitude * Math.PI / 180));
    let distanceY = radiusKm / latKm;

    let theta, x, y;
    for (let i = 0; i < points; i++) {
        theta = (i / points) * (2 * Math.PI);
        x = distanceX * Math.cos(theta);
        y = distanceY * Math.sin(theta);

        ret.push([coords.longitude + x, coords.latitude + y]);
    }
    ret.push(ret[0]);

    return {
        "type": "geojson",
        "data": {
            "type": "FeatureCollection",
            "features": [{
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [ret]
                }
            }]
        }
    };
}

var start;

new QWebChannel(qt.webChannelTransport, (channel) => {

    // Signals defined in SolutionMap (SolutionMapTab.qml), with WebChannel id "currPos"
    let chn = channel.objects.currPos;

    chn.clearPos.connect(() => {
        setupData();
        if (map) syncLayers();
        if (start) {
            start.remove();
            start = null;
        }
    });

    chn.recvPos.connect((id, lat, lng) => {
        const pos = [lat, lng];

        data[id].features.push({
            type: 'Feature',
            geometry: {
                type: 'Point',
                coordinates: pos
            }
        });
        if (!map) return;
        if (!start) {
            start = new mapboxgl.Marker().setLngLat(pos).addTo(map);
            map.panTo(pos);
        } else if (focus) map.panTo(pos);

        let src = map.getSource(`route${id}`);
        if (src != null) src.setData(data[id]);
    })
});

map.on('style.load', () => {
    setupLayers();
    syncLayers();
})

map.on('load', () => {
    console.log("loaded");
    setupLayers();
});
