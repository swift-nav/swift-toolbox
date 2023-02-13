var focus = false;
document.getElementById("focus-btn").addEventListener('click', _ => {
    focus = document.getElementById("focus-btn").checked;
})

function initMap() {
    var options = {
        zoom: 18,
        center: new google.maps.LatLng(37.7913741, -122.3947172),
        streetViewControl: false,
        fullscreenControl: false,
        controlSize: 24
    };

    // init map
    var map = new google.maps.Map(document.getElementById('map'), options);

    map.controls[google.maps.ControlPosition.TOP_CENTER].push(document.getElementById('focus'));

    var lines = ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"].map((value) => {
        var poly = new google.maps.Polyline({
            strokeColor: value,
            strokeOpacity: 1,
            strokeWeight: 3,
        });
        poly.setMap(map);
        return poly;
    });

    const marker = {
        url: "./MapMarker_Swift.svg", // url
        scaledSize: new google.maps.Size(50, 50)
    };
    var startMarker = null;
    var endMarker = null;
    new QWebChannel(qt.webChannelTransport, (channel) => {
        channel.objects.currPos.recvPos.connect((id, lng, lat) => {
            // console.log(`received ${id} ${lat} ${lng}`);
            const pos = new google.maps.LatLng(lat, lng);
            lines[id].getPath().push(pos);
            if (!startMarker) {
                map.setCenter(pos);
                startMarker = new google.maps.Marker({
                    position: pos,
                    map,
                    icon: marker
                })
            }
            if (!endMarker) {
                endMarker = new google.maps.Marker({
                    position: pos,
                    map,
                    icon: marker
                })
            }
            endMarker.setPosition(pos);
            if (focus) map.setCenter(pos);
        })
    });

    console.log("Loaded");
}

window.initMap = initMap;
