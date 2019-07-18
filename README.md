# rust-bucket
An end to end rust application experiment


## Development setup

Clone or download this repository.

### Install [cargo-web]

This is an optional tool that simplifies deploying web applications:

```bash
cargo install cargo-web
```

> Add `--force` option to ensure you install the latest version.

### Build

```bash
cargo web build

# without cargo-web, only the wasm32-unknown-unknown target is supported
cargo build --target wasm32-unknown-unknown
```

### Running It
To start an example enter its directory and start it with [cargo-web]:

```bash
cargo web start
```

To run an optimised build instead of a debug build use:

```bash
cargo web start --release
```

This will use the `wasm32-unknown-unknown` target by default, which is Rust's native WebAssembly target.
The Emscripten-based `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten` targets are also supported
if you tell the `cargo-web` to build for them using the `--target` parameter.
