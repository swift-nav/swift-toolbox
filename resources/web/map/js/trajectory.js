const layerURL = "https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}{r}.png?api_key=ba52a661-85af-472c-a071-bb62fa316c3f"


let mapLoc = [33, -121]
let map = L.map('map', {
    zoomControl: false,
    attribution: false
}).setView(mapLoc, 13);

L.tileLayer(layerURL, {
    maxZoom: 20
}).addTo(map);

const complete = (start_tow, end_tow) => (result) => {
    clearPath();
    let latlngs = [], slng = [], lastPosMode = -1;
    for (let i = 1; i < result.data.length; i++) {
        const datum = result.data[i], tow = datum[1], lat = datum[9], lon = datum[10];
        if (tow && lat && lon) {
            const latlng = new L.LatLng(lat, lon);
            if (start_tow && tow < start_tow) continue
            if (end_tow && tow > end_tow) break // assume sorted
            latlngs.push(latlng);
            slng.push(latlng);
            if (datum[2]) {
                const pmode = parseInt(datum[2]);

                if (pmode !== lastPosMode) {
                    lastPosMode = pmode;
                    const path = L.polyline(slng, {
                        dashArray: "20,5",
                        dashSpeed: -10,
                        color: color[lastPosMode]
                    });
                    map.addLayer(path);

                    slng = [latlng]
                }
            }
        }
    }
    if (slng) {
        const path = L.polyline(slng, {
            dashArray: "20,5",
            dashSpeed: -10,
            color: color[lastPosMode]
        });
        map.addLayer(path);
    }

    map.fitBounds(L.latLngBounds(latlngs));

    const start = L.marker(latlngs[0], {title: "Start"}).bindPopup("Start");
    start.on('mouseover', () => start.openPopup());
    start.on('mouseout', () => start.closePopup());
    map.addLayer(start);

    const end = L.marker(latlngs[latlngs.length - 1], {title: "End"}).bindPopup("End");
    end.on('mouseover', () => end.openPopup());
    end.on('mouseout', () => end.closePopup());
    map.addLayer(end);
};

const clearPath = () => {
    for (let i in map._layers) {
        if (map._layers[i]._marker !== undefined) {
            try {
                map.removeLayer(map._layers[i]);
            } catch (e) {
            }
        }
        if (map._layers[i]._path !== undefined || map._layers[i]._popup !== undefined) {
            try {
                map.removeLayer(map._layers[i]);
            } catch (e) {
            }
        }
    }
}