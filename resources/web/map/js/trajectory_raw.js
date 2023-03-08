import mapboxGlStyleSwitcher from 'https://cdn.skypack.dev/mapbox-gl-style-switcher';

const lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];

mapboxgl.accessToken = "%ACCESS_TOKEN%";
var map = new mapboxgl.Map({
    container: 'map',
    style: "mapbox://styles/mapbox/light-v11",
    center: [-122.486052, 37.830348],  // Initial focus coordinate
    zoom: 16
});

var focusCurrent = false;
var startMarker = null;

class FocusToggle {
    onAdd(map) {
        this._map = map;
        this._btn = document.createElement("button");
        this._btn.className = "mapboxgl-ctrl-icon mapboxgl-ctrl-focus-toggle";
        this._btn.type = "button";
        this._btn.onclick = () => {
            focusCurrent = !focusCurrent;
            if (focusCurrent) {
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

const createDatas = () => Array(lines.length).fill({
    type: 'FeatureCollection',
    features: []
});

var data = createDatas();

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

new QWebChannel(qt.webChannelTransport, (channel) => {

    let chn = channel.objects.currPos;

    chn.clearPos.connect(() => {
        data = createDatas();
        if (map) syncLayers();
        if (startMarker) {
            startMarker.remove();
            startMarker = null;
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
        if (!map) return
        if (!startMarker) {
            startMarker = new mapboxgl.Marker().setLngLat(pos).addTo(map);
            map.panTo(pos);
        } else if (focusCurrent) map.panTo(pos);
        if (map.getSource(`route${id}`) != null) map.getSource(`route${id}`).setData(data[id]);
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
