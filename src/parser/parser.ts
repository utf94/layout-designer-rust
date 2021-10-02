import { ColorPalette } from "./colorPalette";
import { SpacingScale } from "./spacingScale";

export type ColConfig = { [name: string]: string };

export class TailwindConfig {
  private colorPalette: ColorPalette = new ColorPalette();
  private spacingScale: SpacingScale = new SpacingScale();

  private customGridCol: ColConfig = {};
  private customGridRow: ColConfig = {};

  public getColorName(color: string): string {
    return this.colorPalette.getColorName(color);
  }

  public getSpacingName(spacing: number): string {
    return this.spacingScale.getSpacingName(spacing);
  }

  public getGridColName(cellSize: number): string {
    const name = `auto-${cellSize}px`;
    this.customGridCol[name] = `repeat(auto-fill, ${cellSize}px)`;
    return name;
  }

  public getGridRowName(cellSize: number): string {
    const name = `auto-${cellSize}px`;
    this.customGridRow[name] = `repeat(auto-fill, ${cellSize}px)`;
    return name;
  }

  public getTailwindConfig() {
    return {
      theme: {
        extend: {
          spacing: this.spacingScale.getTailwindConfig(),
          colors: this.colorPalette.getTailwindConfig(),
          gridTemplateColumns: this.customGridCol,
          gridTemplateRows: this.customGridRow,
        },
      },
    } as any;
  }
}
