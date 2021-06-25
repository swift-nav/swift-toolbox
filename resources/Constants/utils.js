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
