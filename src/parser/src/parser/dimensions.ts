import { convertUnit } from './unit-convert';

const applyUNIT = (value: string) => {

    if (/^[+-]?([0-9]*[.])?[0-9]+$/.test(value)) {
      return value += 'px';
    }
    return value;
  }

  var num = new Number(10); 
  
export const convertDimensions = (property:string, dimension: string, settings: { autoConvertColor?: boolean; autoConvertSpacing?: boolean; remConversion: any; }) => {
    if (dimension === '0' || dimension === '1px') {
        return dimension;
    }
    return convertUnit(dimensionArray, applyUNIT(dimension), settings.remConversion, property);
};

const dimensionArray = [
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
