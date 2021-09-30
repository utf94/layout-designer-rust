import { convertCss } from "./parser/parser";
import { cssToTailWind } from "./parser/wip";

// {
//   let out = cssToTailWind("background-color", "#f00");
//   console.log(out);
// }

{
  let p = "width";
  let v = "14px";
  let s: string[] = [];
  let e: string[] = [];
  convertCss(p, v, s, e, {
    autoConvertColor: true,
    autoConvertSpacing: true,
    remConversion: 16,
  });

  console.log(s);
}
