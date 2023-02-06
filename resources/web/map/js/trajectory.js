// const layerURL = "https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}{r}.png?api_key=ba52a661-85af-472c-a071-bb62fa316c3f"
//
//
// let mapLoc = [33, -121]
// let map = L.map('map', {
//     zoomControl: false,
//     attribution: false
// }).setView(mapLoc, 13);
//
// L.tileLayer(layerURL, {
//     maxZoom: 20
// }).addTo(map);
//
// const complete = (start_tow, end_tow) => (result) => {
//     clearPath();
//     let latlngs = [], slng = [], lastPosMode = -1;
//     for (let i = 1; i < result.data.length; i++) {
//         const datum = result.data[i], tow = datum[1], lat = datum[9], lon = datum[10];
//         if (tow && lat && lon) {
//             const latlng = new L.LatLng(lat, lon);
//             if (start_tow && tow < start_tow) continue
//             if (end_tow && tow > end_tow) break // assume sorted
//             latlngs.push(latlng);
//             slng.push(latlng);
//             if (datum[2]) {
//                 const pmode = parseInt(datum[2]);
//
//                 if (pmode !== lastPosMode) {
//                     lastPosMode = pmode;
//                     const path = L.polyline(slng, {
//                         dashArray: "20,5",
//                         dashSpeed: -10,
//                         color: color[lastPosMode]
//                     });
//                     map.addLayer(path);
//
//                     slng = [latlng]
//                 }
//             }
//         }
//     }
//     if (slng) {
//         const path = L.polyline(slng, {
//             dashArray: "20,5",
//             dashSpeed: -10,
//             color: color[lastPosMode]
//         });
//         map.addLayer(path);
//     }
//
//     map.fitBounds(L.latLngBounds(latlngs));
//
//     const start = L.marker(latlngs[0], {title: "Start"}).bindPopup("Start");
//     start.on('mouseover', () => start.openPopup());
//     start.on('mouseout', () => start.closePopup());
//     map.addLayer(start);
//
//     const end = L.marker(latlngs[latlngs.length - 1], {title: "End"}).bindPopup("End");
//     end.on('mouseover', () => end.openPopup());
//     end.on('mouseout', () => end.closePopup());
//     map.addLayer(end);
// };
//
// const clearPath = () => {
//     for (let i in map._layers) {
//         if (map._layers[i]._marker !== undefined) {
//             try {
//                 map.removeLayer(map._layers[i]);
//             } catch (e) {
//             }
//         }
//         if (map._layers[i]._path !== undefined || map._layers[i]._popup !== undefined) {
//             try {
//                 map.removeLayer(map._layers[i]);
//             } catch (e) {
//             }
//         }
//     }
// }

var map = new maplibregl.Map({
    container: 'map',
    style: {
        "version": 8,
        "sources": {
            "osm": {
                "type": "raster",
                "tiles": ["https://a.tile.openstreetmap.org/{z}/{x}/{y}.png"],
                "tileSize": 256,
                "attribution": "&copy; OpenStreetMap Contributors",
                "maxzoom": 22
            }
        },
        "layers": [
            {
                "id": "osm",
                "type": "raster",
                "source": "osm" // This must match the source key above
            }
        ]
    },
    center: [-122.486052, 37.830348],  // Initial focus coordinate
    zoom: 16
});

// MapLibre GL JS does not handle RTL text by default, so we recommend adding this dependency to fully support RTL rendering.
maplibregl.setRTLTextPlugin('https://api.mapbox.com/mapbox-gl-js/plugins/mapbox-gl-rtl-text/v0.2.1/mapbox-gl-rtl-text.js');

// Add zoom and rotation controls to the map.
map.addControl(new maplibregl.NavigationControl());

map.on('load', function () {
    var data = {
        type: 'Feature',
        properties: {},
        geometry: {
            type: 'LineString',
            coordinates: []
        }
    };
    map.addSource('route', {type: 'geojson', data});
    map.addLayer({
        id: 'route',
        type: 'line',
        source: 'route',
        layout: {
            'line-join': 'round',
            'line-cap': 'round'
        },
        paint: {
            'line-color': '#ff8600',
            'line-width': 10
        }
    });
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
