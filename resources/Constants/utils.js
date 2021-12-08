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

// Read text from file and store it into the "text" property of the "ele" object.
function readTextFile(path, elem){
    var req = new XMLHttpRequest();
    req.onreadystatechange = function () {
        if(req.readyState === 4){
            elem.text += req.responseText;
        }
    }
    req.open("GET", path);
    req.send();
}

// Dump all properties in a QML item (or any javascript object).
function listProperty(item)
{
    for (var p in item)
        if (typeof item[p] != "function")
            console.log(p + ": " + item[p]);
}

// Helper function to pad before and after decimal of a float.
function padFloat(num, before, after) {
    let new_num = num.toFixed(after).toString();

    let padstart_len = before;
    if (after > 0) {
        padstart_len += 1 + after // Add one for the decimal.
    }    
    let pad_num = new_num.padStart(padstart_len, "0");
    return pad_num;
}
