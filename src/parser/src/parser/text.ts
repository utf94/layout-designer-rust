import { convertUnit } from './unit-convert';

const applyUNIT = (value: string) => {

    if (/^[+-]?([0-9]*[.])?[0-9]+$/.test(value)) {
      return value += 'px';
    }
    return value;
  }
  
export const convertFontWeight = (fontWeight: string) => {
    switch (fontWeight) {
        case 'normal':
            return 400;
        case 'bold':
            return 700;
        default:
            return fontWeight;
    }
};

export const convertFontSize = (fontSize: string, settings: { autoConvertColor?: boolean; autoConvertSpacing?: boolean; remConversion: any; }) => {
    return convertUnit(fontSizeArray, applyUNIT(fontSize), settings.remConversion, 'font-size', true);
};

export const convertLetterSpacing = (letterSpacing: string, settings: { autoConvertColor?: boolean; autoConvertSpacing?: boolean; remConversion: any; }) => {
    return convertUnit(letterSpacingArray, applyUNIT(letterSpacing), settings.remConversion, 'letter-spacing', true);
};

export const convertLineHeight = (lineHeight: string, settings: { autoConvertColor?: boolean; autoConvertSpacing?: boolean; remConversion: any; }) => {
    var a = convertUnit(
        lineHeightArray,
        lineHeight,
        settings.remConversion,
        'line-height',
        true
    );
    // debugger;
    return a;
};

const fontSizeArray = [
    0.75,
    0.875,
    1,
    1.125,
    1.25,
    1.5,
    1.875,
    2.25,
    2.5,
    3,
    4,
];

const lineHeightArray = [0.75, 1, 1.25, 1.5, 1.75, 2, 2.25, 2.5];

const letterSpacingArray = [0.75, 1, 1.25, 1.5, 1.75, 2, 2.25, 2.5];
