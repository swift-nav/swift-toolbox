// Util functions.
.pragma library

function hzToMilliseconds(hz) {
    return 1000 / hz ;
}

function fileUrlToString(url) {
    var filepath = url.toString().replace("file:///", "");
    if (Qt.platform.os !== "windows")
        return "/" + filepath
    return filepath
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

// Dump all properties in a QML item (or any javascript object).
function listProperty(item)
{
    for (var p in item)
        if (typeof item[p] != "function")
            console.log(p + ": " + item[p]);
}
