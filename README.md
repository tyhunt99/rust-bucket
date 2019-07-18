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
