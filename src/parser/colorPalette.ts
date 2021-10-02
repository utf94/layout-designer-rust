export type ColorConfig = { [name: string]: string };

/**
 *
 */
export class ColorPalette {
  private customColors: ColorConfig = {};

  /**
   * Get name of the color from tailwind color palette
   */
  getColorName(color: string): string {
    color = color.toLowerCase();

    let out = DEFAULT_COLOR_PALETTE[color];

    if (out !== undefined) {
      return out;
    } else {
      this.customColors[color.substr(1)] = color;
      return color;
    }
  }

  /**
   * Get tailwind config that defines all custom colors
   */
  getTailwindConfig(): ColorConfig {
    return this.customColors;
  }
}

export function getTailwindColorName(color: string): string | null {
  let out = DEFAULT_COLOR_PALETTE[color.toLowerCase()];

  if (out !== undefined) {
    return out;
  } else {
    return null;
  }
}

export type ColorMap = { [value: string]: string };

/**
 * List of all default colors
 */
export const DEFAULT_COLOR_PALETTE: ColorMap = {
  transparent: "transparent",

  "#000000": "black",
  "#ffffff": "white",
  // gray
  "#f9fafb": "gray-50",
  "#f3f4f6": "gray-100",
  "#e5e7eb": "gray-200",
  "#d1d5db": "gray-300",
  "#9ca3af": "gray-400",
  "#6b7280": "gray-500",
  "#4b5563": "gray-600",
  "#374151": "gray-700",
  "#1f2937": "gray-800",
  "#111827": "gray-900",
  // red
  "#fef2f2": "red-50",
  "#fee2e2": "red-100",
  "#fecaca": "red-200",
  "#fca5a5": "red-300",
  "#f87171": "red-400",
  "#ef4444": "red-500",
  "#dc2626": "red-600",
  "#b91c1c": "red-700",
  "#991b1b": "red-800",
  "#7f1d1d": "red-900",
  // yellow
  "#fefce8": "yellow-50",
  "#fef9c3": "yellow-100",
  "#fef08a": "yellow-200",
  "#fde047": "yellow-300",
  "#facc15": "yellow-400",
  "#eab308": "yellow-500",
  "#ca8a04": "yellow-600",
  "#a16207": "yellow-700",
  "#854d0e": "yellow-800",
  "#713f12": "yellow-900",
  // green
  "#f0fdf4": "green-50",
  "#dcfce7": "green-100",
  "#bbf7d0": "green-200",
  "#86efac": "green-300",
  "#4ade80": "green-400",
  "#22c55e": "green-500",
  "#16a34a": "green-600",
  "#15803d": "green-700",
  "#166534": "green-800",
  "#14532d": "green-900",
  // blue
  "#eff6ff": "blue-50",
  "#dbeafe": "blue-100",
  "#bfdbfe": "blue-200",
  "#93c5fd": "blue-300",
  "#60a5fa": "blue-400",
  "#3b82f6": "blue-500",
  "#2563eb": "blue-600",
  "#1d4ed8": "blue-700",
  "#1e40af": "blue-800",
  "#1e3a8a": "blue-900",
  // indigo
  "#eef2ff": "indigo-50",
  "#e0e7ff": "indigo-100",
  "#c7d2fe": "indigo-200",
  "#a5b4fc": "indigo-300",
  "#818cf8": "indigo-400",
  "#6366f1": "indigo-500",
  "#4f46e5": "indigo-600",
  "#4338ca": "indigo-700",
  "#3730a3": "indigo-800",
  "#312e81": "indigo-900",
  // purple
  "#faf5ff": "purple-50",
  "#f3e8ff": "purple-100",
  "#e9d5ff": "purple-200",
  "#d8b4fe": "purple-300",
  "#c084fc": "purple-400",
  "#a855f7": "purple-500",
  "#9333ea": "purple-600",
  "#7e22ce": "purple-700",
  "#6b21a8": "purple-800",
  "#581c87": "purple-900",
  // pink
  "#fdf2f8": "pink-50",
  "#fce7f3": "pink-100",
  "#fbcfe8": "pink-200",
  "#f9a8d4": "pink-300",
  "#f472b6": "pink-400",
  "#ec4899": "pink-500",
  "#db2777": "pink-600",
  "#be185d": "pink-700",
  "#9d174d": "pink-800",
  "#831843": "pink-900",
};
