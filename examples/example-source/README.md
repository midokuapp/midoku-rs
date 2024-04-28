# `example-source`

This is an example source for Midoku. It demonstrates how use the helper
functions provided by the `midoku-*` packages to create a source.

## Building

To build this module, you need the following `profile` configuration in your
workspace's `Cargo.toml`:

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "s"
strip = true
lto = true
codegen-units = 1
```

Then, run the following command:

```sh
cargo component build --release --package example-source --target wasm32-unknown-unknown
```

Make sure to target `wasm32-unknown-unknown` since we do not want unwanted
WASI dependencies in our modules (`wasm32-wasi` is targeted by default).
