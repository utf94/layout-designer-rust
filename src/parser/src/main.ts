import { ColorPalette } from "./colorPalette";
import { SpacingScale } from "./spacingScale";

{
  const colorPalette = new ColorPalette();
  {
    let tailwind = colorPalette.getColorName("#faffff");
    console.log(tailwind);
  }
  // {
  //   let tailwind = colorPalette.getColorName("#F00");
  //   console.log(tailwind);
  // }

  const spacingScale = new SpacingScale();
  console.log(spacingScale.getSpacingName(10));

  const config = {
    color: colorPalette.getTailwindConfig(),
    spacing: spacingScale.getTailwindConfig(),
  };

  console.log(config);
}
