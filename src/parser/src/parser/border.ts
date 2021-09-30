import { convertUnit } from './unit-convert';

const applyUNIT = (value: string) => {

    if (/^[+-]?([0-9]*[.])?[0-9]+$/.test(value)) {
      return value += 'px';
    }
    return value;
  }

export const convertBorderRadius = (borderRadius: string, settings: { autoConvertColor?: boolean; autoConvertSpacing?: boolean; remConversion: any; }) => {
  return convertUnit(borderRadiusArray, applyUNIT(borderRadius), settings.remConversion, 'border-radius');
};

const borderRadiusArray = [0, 0.125, 0.25, 0.375, 0.5];
