// Util functions.
.pragma library

function hzToMilliseconds(hz) {
    return 1000 / hz ;
}

function fileUrlToString(url) {
    return url.toString().replace("file:///", "")
}
  
function insStatusColor(status) {
    if (status === "⚫") {
        return "green" ;
    } else if (status === "⬛") {
        return "grey" ;
    } else {
        return "goldenrod" ;
    }
}
