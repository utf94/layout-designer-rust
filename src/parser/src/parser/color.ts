import { Colors } from '../constants';
// import NearestColor from 'nearest-color';
var NearestColor = require('nearest-color').from(Colors);

export const convertColor = (color: any, settings: { autoConvertColor: any; autoConvertSpacing?: boolean; remConversion?: number; }) => {
    var original_color = color;
    color = isColor(color);
    if (!settings.autoConvertColor || !color) {
        return original_color;
    }
    try {
        return NearestColor.from(Colors)(color);
    } catch (e) {
        // console.error('error converting color', e);
        return color;
    }
};

// Only checks names and Hexes - Need to improve
export const isColor = (strColor: string) => {
    // var s = new Option().style;
    var s: any = {};
    s.color = strColor;
    var test1 = s.color == strColor;
    var test2 = /^#[0-9A-F]{6}$/i.test(strColor);
    var colorWithOpacity = /^#[0-9A-F]{8}$/i.test(strColor);

    if(colorWithOpacity){
        strColor = strColor.substring(0, strColor.length - 2);
        test2 = true;
    }

    if (test1 == true || test2 == true) {
        return strColor;
    } else {
        return false;
    }
};

// function rgba2hex(r: number, g: number, b: number, a: number) {
//     if (r > 255 || g > 255 || b > 255 || a > 255)
//         throw "Invalid color component";

//     r = Math.round(r * 255);
//     g = Math.round(g * 255);
//     b = Math.round(b * 255);
//     a = Math.round(a * 255);

//     return ("0" + parseInt(r.toString(), 10).toString(16)).slice(-2) +
//         ("0" + parseInt(g.toString(), 10).toString(16)).slice(-2) +
//         ("0" + parseInt(b.toString(), 10).toString(16)).slice(-2) +
//         ("0" + parseInt(a.toString(), 10).toString(16)).slice(-2);
// }

// function rgbaToHex(color: color) {
//     // debugger;
//     if (color !== undefined) {
//         if (color.a === undefined) {
//             return '#' + rgb2hex(color.r, color.g, color.b);
//         }
//         return '#' + rgba2hex(color.r, color.g, color.b, color.a);
//     }
//     return ''
// }

// function rgb2hex(r: number, g: number, b: number) {
//     if (r > 255 || g > 255 || b > 255)
//         throw "Invalid color component";
//     return ((r << 16) | (g << 8) | b).toString(16);
// }
