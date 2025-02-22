# wasip2-playground

Just a minimal example to setup a guest and host for [wasip2](https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md)
using Rust.
This uses [Taskfiles](https://taskfile.dev/usage/) for build command execution.

## wit file

The [math.wit](math.wit) is manual written and corresponds the data structures and functions exported by the guest.
It is used by both guest and host to auto-generate bindings.

## Using it

First compile the wasm-module in `wasip2-example` then execute the host program in `wasmtime-run`.
This is already setup via the Taskfile:

```bash
task run
```

