# Rust / Wasm / Wgpu Triangle

This project demonstrates how to setup a [rust](https://www.rust-lang.org/) project
that uses [wgpu](https://wgpu.rs/) to render a spinning triangle, supporting
both webgl and webgpu [wasm](https://webassembly.org/) as well as native.

```
# native
cargo run -r 

# webgl
trunk serve --features webgl

# webgl
trunk serve --features webgpu
```

## Prerequisites (web)

* [trunk](https://trunkrs.dev/)