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
