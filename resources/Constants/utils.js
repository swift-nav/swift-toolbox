// Util functions.
.pragma library

function hzToMilliseconds(hz) {
    return 1000 / hz ;
}

function fileUrlToString(url) {
    return url.toString().replace("file://", "")
}
  
