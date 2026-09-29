const lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];
const LNG_KM = 111.320, LAT_KM = 110.574;

const map = new maplibregl.Map({
    container: 'map',
    style: 'https://tiles.stadiamaps.com/styles/alidade_smooth.json?api_key=@STADIA_TOKEN@',
    center: [-122.486052, 37.830348],  // Initial focus coordinate
    zoom: 16,
});

var focusCurrent = false;
var startMarker = null;
var currentMarker = null;

// Flag icon for the start position -- pole is centered horizontally so it lines
// up with the marker's default "bottom" anchor (base of the pole = the coordinate).
const START_MARKER_SVG = `<svg width="22" height="30" viewBox="0 0 22 30" xmlns="http://www.w3.org/2000/svg">
    <line x1="11" y1="29" x2="11" y2="1" stroke="#1b5e20" stroke-width="2" stroke-linecap="round"/>
    <path d="M11 2 L21 7 L11 12 Z" fill="#2e7d32" stroke="#1b5e20" stroke-width="1" stroke-linejoin="round"/>
</svg>`;

// "Current location" dot for the live position -- symmetric, so the default
// "center" anchor lines up its center with the coordinate.
const CURRENT_MARKER_SVG = `<svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
    <circle cx="10" cy="10" r="9" fill="#1976d2" fill-opacity="0.25"/>
    <circle cx="10" cy="10" r="5" fill="#1976d2" stroke="#ffffff" stroke-width="2"/>
</svg>`;

function createMarkerElement(svgMarkup) {
    const wrapper = document.createElement('div');
    wrapper.innerHTML = svgMarkup;
    return wrapper.firstElementChild;
}

class FocusToggle {
    onAdd(map) {
        this._map = map;
        this._btn = document.createElement("button");
        this._btn.className = "maplibregl-ctrl-icon maplibregl-ctrl-focus-toggle";
        this._btn.type = "button";
        this._btn.onclick = () => {
            focusCurrent = !focusCurrent;
            this._btn.className = focusCurrent ? "maplibregl-ctrl-icon maplibregl-ctrl-unfocus-toggle" : "maplibregl-ctrl-icon maplibregl-ctrl-focus-toggle";
        };
        this._container = document.createElement("div");
        this._container.className = "maplibregl-ctrl-group maplibregl-ctrl";
        this._container.appendChild(this._btn);
        return this._container;
    }

    onRemove() {
        this._container.parentNode.removeChild(this._container);
        this._map = undefined;
    }
}

map.addControl(new FocusToggle(), "top-right");
map.addControl(new maplibregl.NavigationControl());

var data = [];
var crumbCoords = [];

function setupData() {
    data = [];
    for (let i = 0; i < lines.length; i++) {
        data.push({
            type: 'FeatureCollection',
            features: []
        });
    }
    crumbCoords = [];
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
                'fill-opacity': 0.3,
                'fill-outline-color': '#000000'
            }
        });
    }
    if (map.getSource('prot') == null) {
        map.addSource('prot', {
            type: 'geojson',
            cluster: false,
            data: {
                type: 'FeatureCollection',
                features: []
            }
        })
        map.addLayer({
            id: 'prot',
            type: 'fill',
            source: 'prot',
            paint: {
                'fill-color': "#00FF00",
                'fill-opacity': 0.5
            }
        });
    }
    if (map.getSource('breadcrumb') == null) {
        map.addSource('breadcrumb', {
            type: 'geojson',
            data: {
                type: 'Feature',
                geometry: {
                    type: "LineString",
                    coordinates: []
                }
            }
        })
        map.addLayer({
            id: 'breadcrumb',
            type: 'line',
            source: 'breadcrumb',
            layout: {
                'line-join': 'round',
                'line-cap': 'round'
            },
            paint: {
                'line-color': '#888',
                'line-width': 1
            }
        })
    }
}

function syncCrumbCoords(){
    map.getSource('breadcrumb').setData({
        type: 'Feature',
        geometry: {
            type: 'LineString',
            coordinates: crumbCoords
        }
    })
}

function syncLayers() {
    // sync route datas with stored points
    for (let i = 0; i < lines.length; i++) {
        map.getSource(`route${i}`).setData(data[i]);
    }
    // clear protection, since its only one point and temporary
    map.getSource('prot').setData({
        type: 'FeatureCollection',
        features: []
    });
    syncCrumbCoords();
}

/**
 * Helper method to create elliptical geojson data
 * @param center {[lng: number, lat: number]}
 * @param rX horizontal radius in kilometers of ellipse
 * @param rY vertical radius in kilometers of ellipse
 * @return {{geometry: {coordinates: [][], type: string}, type: string}}
 */
function createGeoJsonEllipse(center, rX, rY) {
    let coords = {latitude: center[1], longitude: center[0]};
    let ret = [];
    let dX = rX / (LNG_KM * Math.cos(coords.latitude * Math.PI / 180));
    let dY = rY / LAT_KM;

    let points = 16;
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
        if (currentMarker) {
            currentMarker.remove();
            currentMarker = null;
        }
    });

    chn.recvPos.connect((id, lon, lat, hAcc) => {
        const pos = [lon, lat], rX = hAcc / 1000;
        data[id].features.push(createGeoJsonEllipse(pos, rX, rX));
        crumbCoords.push(pos);
        if (!map) return;
        if (!currentMarker) currentMarker = new maplibregl.Marker({element: createMarkerElement(CURRENT_MARKER_SVG)}).setLngLat(pos).addTo(map);
        else currentMarker.setLngLat(pos);
        if (!startMarker) {
            startMarker = new maplibregl.Marker({element: createMarkerElement(START_MARKER_SVG), anchor: 'bottom'}).setLngLat(pos).addTo(map);
            map.panTo(pos);
        } else if (focusCurrent) map.panTo(pos);
        let src = map.getSource(`route${id}`);
        if (src) src.setData(data[id]);

        if (map.getSource('breadcrumb')) syncCrumbCoords();
    })

    chn.protPos.connect((lat, lng, hpl) => {
        const pos = [lng, lat], rX = hpl / 100_000; // hpl in cm, convert to km
        if (!map) return;
        let src = map.getSource(`prot`);
        if (src) src.setData({
            type: 'FeatureCollection',
            features: [createGeoJsonEllipse(pos, rX, rX)]
        });
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
