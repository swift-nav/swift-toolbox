/**
 * Copyright (c) 2022 Swift Navigation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
