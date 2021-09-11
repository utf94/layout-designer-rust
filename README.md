# Requirements:

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# Tree:

- `./rust` - Rust codebase
- `./src` - Ts codebase
  - mostly meant for glue code, and stuff that is easier to write and maintain in TS (or simply did not get ported to rust yet)
- `./solid-ui` - Custom HTML element wrapper for `solid-ui`

# Build

#### With `make`

```sh
make build
```

#### Without `make`

```sh
wasm-pack build ./rust --release --target web --out-name web --out-dir ./dist
npx spack
cp ./rust/dist/web_bg.wasm ./dist/web_bg.wasm
```
