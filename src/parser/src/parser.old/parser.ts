import TailWindMap from "../constants";
import { convertColor, isColor } from "./color";
import { convertDimensions } from "./dimensions";

export const convertCss = (
  property: string,
  value: string,
  tailWindStyles: string[],
  errors: string[],
  settings: {
    autoConvertColor: boolean;
    autoConvertSpacing: boolean;
    remConversion: number;
  }
) => {
  let processedProperty = processProperty(property, value);
  let processedValue = processValue(
    processedProperty,
    value,
    tailWindStyles,
    errors,
    settings
  );

  if (
    TailWindMap[processedProperty] &&
    TailWindMap[processedProperty][processedValue]
  ) {
    tailWindStyles.push(
      `${TailWindMap[processedProperty][processedValue].substring(1)}`
    );
  } else {
    errors.push(`${property}: ${value};`);
  }
};

const processProperty = (property: string, value: string) => {
  switch (property) {
    case "background":
      if (isColor(value)) {
        return "background-color";
      }
      return property;
    default:
      return property;
  }
};

const processValue = (
  property: string,
  value: string,
  tailWindStyles: string[],
  errors: string[],
  settings: {
    autoConvertColor: boolean;
    autoConvertSpacing: boolean;
    remConversion: number;
  }
) => {
  if (
    ["0em", "0ex", "0ch", "0rem", "0vw", "0vh", "0%", "0px"].indexOf(value) !==
    -1
  ) {
    return 0;
  }
  switch (property) {
    case "background-color":
      return convertColor(value, settings);
    case "height":
    case "width":
      return convertDimensions(property, value, settings);
    default:
      return value;
  }
};
