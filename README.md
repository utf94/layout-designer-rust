# Requirements:

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# Tree:

- `./rust` - Rust codebase
- `./src` - Ts codebase
  - mostly meant for glue code, and stuff that is easier to write and mantain in TS (or simply did not get ported to rust yet)
- `./solid-ui` - Custom HTML element wrapper for `solid-ui`
