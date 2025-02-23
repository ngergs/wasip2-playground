# wasip2-playground

Just a minimal example to setup a guest and host for [wasip2](https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md)
using Rust.
This uses [Taskfiles](https://taskfile.dev/usage/) for build command execution.

## wit file

The [math.wit](math.wit) is manual written and corresponds the data structures and functions exported by the guest.
It is used by both guest and host to auto-generate bindings.

## Using it

### Toolchains
We use the rust nightly toolchain to compile `wasip2-example` and the stable one (or whatever your default is) for `wasmtime-run`.
We need the nightly toolchain to ommit panic formatting which would result in a lot of [unnecessary wit imports](https://github.com/rust-lang/rust/issues/133235).

The following components can be used to set evertyhing up if it hasn't been done yet:
```bash
# Setup default toolchain for wasmtime-run
rustup target add wasm32-wasip2
# Setup nightly for wasip2-example
rustup toolchain install nightly
rustup target add wasm32-wasip2 --toolchain nighly
rustup component add rust-src --toolchain nightly
```

### Wasm
First compile the wasm-module in `wasip2-example` then execute the host program in `wasmtime-run`.
This is already setup via the Taskfile:

```bash
task run
```

If you want to see the wit file included in the generated wasm guest module:
```bash
task print-wit
```
