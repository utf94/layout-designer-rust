export type SpacingConfig = { [key: number]: string };

/**
 *
 */
export class SpacingScale {
  private customSpacings: SpacingConfig = {};

  /**
   * Get name of the color from tailwind color palette
   */
  getSpacingName(spacing: number): string {
    let out = DEFAULT_SPACING_SCALE[spacing];

    if (out !== undefined) {
      return out;
    } else {
      const remSpacing = spacing / 16.0;
      const name = remSpacing * 4.0;

      this.customSpacings[name] = remSpacing + "rem";
      return name.toString();
    }
  }

  /**
   * Get tailwind config that defines all custom colors
   */
  getTailwindConfig(): SpacingConfig {
    return this.customSpacings;
  }
}

export function getTailwindSpacingName(spacing: number): string | null {
  let out = DEFAULT_SPACING_SCALE[spacing];

  if (out !== undefined) {
    return out;
  } else {
    return null;
  }
}

/**
 * List of all default colors
 *
 * Keys are in px
 */
export const DEFAULT_SPACING_SCALE: SpacingConfig = {
  0: "0",
  1: "px",
  2: "0.5",
  4: "1",
  6: "1.5",
  8: "2",
  10: "2.5",
  12: "3",
  14: "3.5",
  16: "4",
  20: "5",
  24: "6",
  28: "7",
  32: "8",
  36: "9",
  40: "10",
  44: "11",
  48: "12",
  56: "14",
  64: "16",
  80: "20",
  96: "24",
  112: "28",
  128: "32",
  144: "36",
  160: "40",
  176: "44",
  192: "48",
  208: "52",
  224: "56",
  240: "60",
  256: "64",
  288: "72",
  320: "80",
  384: "96",
};
