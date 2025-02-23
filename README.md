# wasip2-playground

Just a minimal example to setup a guest and host
for [wasip2](https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md)
using Rust.
This uses [Taskfiles](https://taskfile.dev/usage/) for build command execution.

## wit file

The [math.wit](math.wit) is manual written and corresponds the data structures and functions exported by the guest.
It is used by both guest and host to auto-generate bindings.

## Running it

We have to compils both host and guest with the rust compiler and then executes the host `wasm-runtime`.
This is already setup via the Taskfile.

### Default compilation

```bash
```bash
rustup target add wasm32-wasip2
task run
```

We can also observe the `wit`-file included in the guest wasm component module (requires
the [wasm-tools cli](https://github.com/bytecodealliance/wasm-tools)) by executing `task print-wit`.
The result is a bit surprising as we get a lot of WASI imports for our simple math component:

```wit
package root:component;

world root {
  import wasi:cli/environment@0.2.0;
  import wasi:cli/exit@0.2.0;
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;

  record arguments {
    x: s32,
    y: s32,
  }

  export sum: func(args: arguments) -> s32;
  export minus: func(args: arguments) -> s32;
  export mul: func(args: arguments) -> s32;
  export div: func(args: arguments) -> result<f32, string>;
}
```

The origin for these includes is [rust panic formatting](https://github.com/rust-lang/rust/issues/133235).

### Without panic formatting

We can use the `nightly` rust toolchain to ommit panic formatting for the guest component:

```bash
rustup toolchain install nightly
rustup target add wasm32-wasip2 --toolchain nighly
rustup component add rust-src --toolchain nightly
task run MINIMAL_WASM=true
```

We can also print the `wit`-file included into the guest component for this setup with
`task print-wit MINIMAL_WASM=true`
and confirm that it has no more WASI-includes:

```wit
package root:component;

world root {
  record arguments {
    x: s32,
    y: s32,
  }

  export sum: func(args: arguments) -> s32;
  export minus: func(args: arguments) -> s32;
  export mul: func(args: arguments) -> s32;
  export div: func(args: arguments) -> result<f32, string>;
}
```
