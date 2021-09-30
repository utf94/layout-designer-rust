import { convertUnit } from './unit-convert';

const applyUNIT = (value: string) => {

    if (/^[+-]?([0-9]*[.])?[0-9]+$/.test(value)) {
      return value += 'px';
    }
    return value;
  }

export const convertSpacing = (
    property: string,
    spacing: string,
    tailWindStyles: string[],
    errors: string[],
    settings: { autoConvertColor?: boolean; autoConvertSpacing: any; remConversion: any; }
) => {
    if (!settings.autoConvertSpacing) {
        return spacing;
    }
    if (
        (property === 'padding' || property === 'margin') &&
        spacing === '1px'
    ) {
        return spacing;
    }
    if (
        ['-1px', 'auto'].indexOf(spacing) !== -1 &&
        [
            'margin',
            'margin-left',
            'margin-right',
            'margin-top',
            'margin-bottom',
        ].indexOf(property) !== -1
    ) {
        return spacing;
    }
    const dimensions = spacing.split(' ');
    const remArray = property.startsWith('padding')
        ? paddingArray
        : marginArray;
    if (dimensions.length === 1) {
        // if(property == 'margin-left') {
        //     if(spacing == '-0.5')
        //     {
        //         debugger;
        //     }
        // }
        // debugger;
        // if(!remArray.includes(Number(dimensions[0])))
        // {
        //     remArray.push(Number(dimensions[0]))
        // }
        return convertUnit(remArray, applyUNIT(dimensions[0]), settings.remConversion, property);
    }
    //cannot handle short hands
    return spacing;
};

const paddingArray = [
    0,
    0.25,
    0.5,
    0.75,
    1,
    1.25,
    1.5,
    2,
    2.5,
    3,
    4,
    5,
    6,
    8,
    10,
    12,
    14,
    16,
];
const marginArray = [
    // -0.25,
    // -0.5,
    // -0.75,
    // -1,
    // -1.25,
    // -1.5,
    // -2,
    // -2.5,
    // -3,
    // -4,
    // -5,
    // -6,
    // -8,
    // -10,
    // -12,
    // -14,
    // -16,
    0,
    // 0.25,
    // 0.5,
    // 0.75,
    // 1,
    // 1.25,
    // 1.5,
    // 2,
    // 2.5,
    // 3,
    // 4,
    // 5,
    // 6,
    // 8,
    // 10,
    // 12,
    // 14,
    // 16,
];
