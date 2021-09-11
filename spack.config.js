const { config } = require("@swc/core/spack");

module.exports = config({
  entry: {
    web: __dirname + "/src/index.tsx",
  },
  output: {
    path: __dirname + "/dist",
  },
  module: {},
});
