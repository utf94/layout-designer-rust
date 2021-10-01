import tailwind from "../tailwind.config";

import TailWindMap from "../constants";

export const convertUnit = (
  remArray: any[],
  value: any,
  conversionFactor = 16,
  cssPropKey: string,
  stripLeadingZeros = false
) => {
  let converted = value;
  if (value.endsWith("rem")) {
    converted = `${roundToNearestRem(remArray, value.split("rem")[0])}rem`;
  } else if (value.endsWith("px")) {
    converted = convertPxToRem(remArray, value, conversionFactor, cssPropKey);
  }
  if (stripLeadingZeros) {
    converted = converted.replace(/^[0.]+/, ".");
  }
  return converted;
};

const convertPxToRem = (
  remArray: any[],
  value: string,
  conversionFactor = 16,
  cssPropKey: string
) => {
  const numericVal = parseFloat(value.split("px")[0]);
  const min = Math.min(...remArray);
  const max = Math.max(...remArray);

  if (
    numericVal &&
    numericVal <= conversionFactor * max &&
    numericVal >= conversionFactor * min
  ) {
    let rem = numericVal / conversionFactor;
    const closest = roundToNearestRem(remArray, rem);
    if (closest === 0) {
      return 0;
    } else {
      return `${closest}rem`;
    }
  } else {
    var cal = (numericVal / conversionFactor).toFixed(2);
    var rem: number = +cal;
    var classPrefix: string;

    switch (cssPropKey) {
      case "border-radius":
        classPrefix = ".rounded-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "letter-spacing":
        classPrefix = ".tracking-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "height":
        classPrefix = ".h-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "width":
        classPrefix = ".w-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "margin-left":
        classPrefix = ".ml-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "margin-right":
        classPrefix = ".mr-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "margin-top":
        classPrefix = ".mt-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      case "margin-bottom":
        classPrefix = ".mb-";
        updateTailwind(cssPropKey, rem, numericVal, classPrefix);
        return `${rem}rem`;
      default:
        return `${rem}rem`;
    }
  }
};

const updateTailwind = (
  cssPropKey: string,
  rem: number,
  numericVal: number,
  classPrefix: string
) => {
  if (rem !== 0) {
    if (TailWindMap[cssPropKey]) {
      if (TailWindMap[cssPropKey][`${rem}rem`] === undefined) {
        TailWindMap[cssPropKey][`${rem}rem`] = `${classPrefix}${numericVal}`;
      }
    }
  }

  tailwind.theme.extend[cssPropKey.replace(/-./g, (x) => x.toUpperCase()[1])]
    ? null
    : (tailwind.theme.extend[
        cssPropKey.replace(/-./g, (x) => x.toUpperCase()[1])
      ] = {});

  if (
    tailwind.theme.extend[cssPropKey.replace(/-./g, (x) => x.toUpperCase()[1])][
      numericVal
    ] === undefined
  ) {
    tailwind.theme.extend[cssPropKey.replace(/-./g, (x) => x.toUpperCase()[1])][
      numericVal
    ] = `${rem}rem`;

    console.log(tailwind.theme);
    // const directoryPath = path.join("./tailwind.config.js");
    // fs.writeFile(
    //   directoryPath,
    //   "module.exports = " + JSON.stringify(tailwind, null, "\t"),
    //   function (err: any) {
    //     if (err) {
    //       return console.error(err);
    //     }
    //     console.log("Tailwind updated!");
    //   }
    // );
  }
};

const roundToNearestRem = (remArray: any[], num: number) => {
  return remArray.reduce((prev: number, curr: number) => {
    return Math.abs(curr - num) < Math.abs(prev - num) ? curr : prev;
  });
};
