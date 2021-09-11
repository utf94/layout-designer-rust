import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

const path = require("path");

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    target: "esnext",
    polyfillDynamicImport: false,
    lib: {
      entry: path.resolve(__dirname, "src/index.tsx"),
      name: "MyLib",
    },
  },
});
