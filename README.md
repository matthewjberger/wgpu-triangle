# Rust / Wasm / Wgpu Triangle

This project demonstrates how to setup a [rust](https://www.rust-lang.org/) project
that uses [wgpu](https://wgpu.rs/) to render a spinning triangle, supporting
both webgl and webgpu [wasm](https://webassembly.org/) as well as native.

```
# native
cargo run -r 

# webgpu
trunk serve --features webgpu

# webgl
trunk serve --features webgl
```

## Prerequisites (web)

* [trunk](https://trunkrs.dev/)