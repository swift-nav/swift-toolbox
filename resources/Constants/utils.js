// Util functions.
.pragma library

function hzToMilliseconds(hz) {
    return 1000 / hz ;
}

function fileUrlToString(url) {
    return url.toString().replace("file:///", "")
}

function spanBetweenValues(v1, v2){
    return Math.abs(v1 - v2)
}

// Utility function for printing properites of an object.
function listObject(object){
    for (var prop in object) {
        console.log(prop + "=>" + object[prop]);
    }
}

// Read text from file and store it into the "text" property of the "ele" object.
function readTextFile(path, ele){
    var req = new XMLHttpRequest;
    req.onreadystatechange = function () {
        if(req.readyState === 4){
            ele.text += req.responseText;
        }
    }
    req.open("GET", path);
    req.send();
}
