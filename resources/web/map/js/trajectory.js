import mapboxGlStyleSwitcher from 'https://cdn.skypack.dev/mapbox-gl-style-switcher';

const lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];

mapboxgl.accessToken = "pk.eyJ1Ijoic3dpZnQtYWRyaWFuIiwiYSI6ImNsZTN1MW82bDA2OGgzdXFvOWFuZTJlMnEifQ.9nR8m0C-B_ISNR4r4cMExw";
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
            this._btn.className = focusCurrent ? "mapboxgl-ctrl-icon mapboxgl-ctrl-unfocus-toggle" : "mapboxgl-ctrl-icon mapboxgl-ctrl-focus-toggle";
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


var data = [];

function setupData() {
    data = [];
    for (let i = 0; i < lines.length; i++) {
        data.push({
            type: 'FeatureCollection',
            features: []
        });
    }
}

setupData();

function setupLayers() {
    for (let i = 0; i < lines.length; i++) {
        if (map.getSource(`route${i}`) != null) continue;
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
            type: 'fill',
            source: `route${i}`,
            paint: {
                'fill-color': lines[i],
                'fill-opacity': 0.2
            }
        });
    }
}

function syncLayers() {
    for (let i = 0; i < lines.length; i++) {
        map.getSource(`route${i}`).setData(data[i]);
    }
}

const LNG_KM = 111.320, LAT_KM = 110.574;

/**
 * Helper method to create elliptical geojson data
 * @param center {[lng: number, lat: number]}
 * @param rX horizontal radius in kilometers of ellipse
 * @param rY vertical radius in kilometers of ellipse
 * @param points optional number of points to render ellipse, higher for smoothness
 * @return {{geometry: {coordinates: [][], type: string}, type: string}}
 */
function createGeoJsonEllipse(center, rX, rY, points = 32) {
    let coords = {latitude: center[1], longitude: center[0]};
    let ret = [];
    let dX = rX / (LNG_KM * Math.cos(coords.latitude * Math.PI / 180));
    let dY = rY / LAT_KM;

    let theta, x, y;
    for (let i = 0; i < points; i++) {
        theta = (i / points) * (2 * Math.PI);
        x = dX * Math.cos(theta);
        y = dY * Math.sin(theta);

        ret.push([coords.longitude + x, coords.latitude + y]);
    }
    ret.push(ret[0]);

    return {
        type: "Feature",
        geometry: {
            type: "Polygon",
            coordinates: [ret]
        }
    };
}

new QWebChannel(qt.webChannelTransport, (channel) => {

    let chn = channel.objects.currPos;

    chn.clearPos.connect(() => {
        setupData();
        if (map) syncLayers();
        if (startMarker) {
            startMarker.remove();
            startMarker = null;
        }
    });

    chn.recvPos.connect((id, lat, lng, hAcc) => {
        const pos = [lat, lng],
            rX = hAcc / 1000;
        data[id].features.push(createGeoJsonEllipse(pos, rX, rX));
        if (!map) return;
        if (!startMarker) {
            startMarker = new mapboxgl.Marker().setLngLat(pos).addTo(map);
            map.panTo(pos);
        } else if (focusCurrent) map.panTo(pos);
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
