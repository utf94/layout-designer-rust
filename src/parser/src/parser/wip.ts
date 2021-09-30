// const handleSpacingShorthand = () => {
//     if (['margin', 'padding'].indexOf(property) !== -1) {
//         if (dimensions.length === 2) {
//             convertCss(
//                 `${property}-top`,
//                 dimensions[0],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-bottom`,
//                 dimensions[0],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-left`,
//                 dimensions[1],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-right`,
//                 dimensions[1],
//                 tailWindStyles,
//                 errors
//             );
//         }
//         if (dimensions.length === 3) {
//             convertCss(
//                 `${property}-top`,
//                 dimensions[0],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-left`,
//                 dimensions[1],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-right`,
//                 dimensions[1],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-bottom`,
//                 dimensions[2],
//                 tailWindStyles,
//                 errors
//             );
//         }
//         if (dimensions.length === 4) {
//             convertCss(
//                 `${property}-top`,
//                 dimensions[0],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-right`,
//                 dimensions[1],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-bottom`,
//                 dimensions[2],
//                 tailWindStyles,
//                 errors
//             );
//             convertCss(
//                 `${property}-left`,
//                 dimensions[3],
//                 tailWindStyles,
//                 errors
//             );
//         }
//     }
// };
//
const fs = require("fs");
const path = require("path");
var tailwind = require("../../tailwind.config");

export const cssToTailWind = (
  key: string,
  value: string
): string | undefined => {
  switch (key) {
    case "background-color":
      if (tailwind.theme.extend.colors) {
        if (tailwind.theme.extend.colors[value.replace("#", "")] == undefined) {
          tailwind.theme.extend.colors[value.replace("#", "")] = value;

          const directoryPath = path.join("./tailwind.config.js");

          fs.writeFile(
            directoryPath,
            "module.exports = " + JSON.stringify(tailwind, null, "\t"),
            function (err: any) {
              if (err) {
                // console.log(directoryPath)
                return console.error(err);
              }
              // console.log("Tailwind updated!");
            }
          );
        }
      }

      if (value.startsWith("--use-theme:")) {
        value = value.replace("--use-theme:", "");
        return `bg-${value}`;
      } else if (value.startsWith("#")) {
        return `bg-${value.replace("#", "")}`;
      }
      break;

    case "border-radius":
      if (value.startsWith("--use-theme:")) {
        value = value.replace("--use-theme:", "");
        return `rounded-${value}`;
      }
      break;

    case "color":
      if (value.startsWith("--use-theme:")) {
        value = value.replace("--use-theme:", "");
        return `text-${value}`;
      }
      break;

    default:
      break;
  }
};
