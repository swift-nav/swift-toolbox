const lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"];
const LNG_KM = 111.320, LAT_KM = 110.574;

const map = L.map('map').setView([37.830348, -122.486052], 16);

L.tileLayer(`https://tiles.stadiamaps.com/tiles/alidade_smooth/{z}/{x}/{y}{r}.png?api_key=@STADIA_TOKEN@`, {
    maxZoom: 20,
    attribution: '&copy; <a href="https://stadiamaps.com/" target="_blank">Stadia Maps</a> ' +
        '&copy; <a href="https://openmaptiles.org/" target="_blank">OpenMapTiles</a> ' +
        '&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a> contributors',
}).addTo(map);

var focusCurrent = false;
var startMarker = null;
var currentMarker = null;
var protLayer = null;

const FocusToggle = L.Control.extend({
    options: { position: "topright" },
    onAdd: function () {
        this._btn = document.createElement("button");
        this._btn.className = "leaflet-ctrl-focus-toggle";
        this._btn.type = "button";
        this._btn.onclick = () => {
            focusCurrent = !focusCurrent;
            this._btn.className = focusCurrent ? "leaflet-ctrl-unfocus-toggle" : "leaflet-ctrl-focus-toggle";
        };
        const container = L.DomUtil.create("div", "leaflet-bar leaflet-control");
        container.appendChild(this._btn);
        L.DomEvent.disableClickPropagation(container);
        return container;
    },
});

new FocusToggle().addTo(map);

const routeGroups = lines.map(() => L.layerGroup().addTo(map));
const crumbLine = L.polyline([], { color: "#888", weight: 1 }).addTo(map);

/**
 * Helper method to create a Leaflet polygon ring approximating an ellipse.
 * @param center {[lat: number, lng: number]}
 * @param rX horizontal radius in kilometers of ellipse
 * @param rY vertical radius in kilometers of ellipse
 * @return {[lat: number, lng: number][]}
 */
function createEllipsePoints(center, rX, rY) {
    const lat = center[0], lng = center[1];
    const dX = rX / (LNG_KM * Math.cos(lat * Math.PI / 180));
    const dY = rY / LAT_KM;

    const points = 16;
    const ring = [];
    for (let i = 0; i < points; i++) {
        const theta = (i / points) * (2 * Math.PI);
        const x = dX * Math.cos(theta);
        const y = dY * Math.sin(theta);
        ring.push([lat + y, lng + x]);
    }
    return ring;
}

new QWebChannel(qt.webChannelTransport, (channel) => {

    let chn = channel.objects.currPos;

    chn.clearPos.connect(() => {
        routeGroups.forEach((group) => group.clearLayers());
        crumbLine.setLatLngs([]);
        if (protLayer) {
            protLayer.remove();
            protLayer = null;
        }
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
        const pos = [lat, lon], rX = hAcc / 1000;
        L.polygon(createEllipsePoints(pos, rX, rX), {
            weight: 0,
            color: lines[id],
            fillColor: lines[id],
            fillOpacity: 0.3,
        }).addTo(routeGroups[id]);
        crumbLine.addLatLng(pos);

        if (!currentMarker) currentMarker = L.marker(pos).addTo(map);
        else currentMarker.setLatLng(pos);

        if (!startMarker) {
            startMarker = L.marker(pos).addTo(map);
            map.panTo(pos);
        } else if (focusCurrent) map.panTo(pos);
    });

    chn.protPos.connect((lat, lon, hpl) => {
        const pos = [lat, lon], rX = hpl / 100_000; // hpl in cm, convert to km
        const ring = createEllipsePoints(pos, rX, rX);
        if (!protLayer) {
            protLayer = L.polygon(ring, { weight: 0, color: "#00FF00", fillColor: "#00FF00", fillOpacity: 0.5 }).addTo(map);
        } else {
            protLayer.setLatLngs(ring);
        }
    });
});
