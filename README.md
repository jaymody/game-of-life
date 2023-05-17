Game of life built in Rust + WebAssembly.

Resources:
- [Wasm By Example](https://wasmbyexample.dev/home.en-us.html)
- [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)

### Usage
This project requires:

- `rust`: Language of choice to compile to wasm ([installation instructions](https://doc.rust-lang.org/book/ch01-01-installation.html)).
- `wasm-pack`: Compiles rust code a wasm package we can use in javascript ([installation instructions](https://rustwasm.github.io/wasm-pack/installer/)).
- `vite`: To bundle and serve our website (`npm install -g vite`).

First, we compile the rust code to a wasm package via `wasm-pack`:

```shell
wasm-pack build --target web ./
```

The package is outputted to `pkg/`, which can be imported by `index.js`. Then we serve our `index.html` and `index.js` files with `vite`:

```shell
vite ./
```

