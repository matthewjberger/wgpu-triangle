# Rust / Wasm / Wgpu Triangle

This project demonstrates how to setup a [rust](https://www.rust-lang.org/) project
that uses [wgpu](https://wgpu.rs/) to render a spinning triangle, supporting
both webgl and webgpu [wasm](https://webassembly.org/) as well as native.

```
# native
cargo run -r  -p app

# webgpu
trunk serve --features webgpu --open --config apps/app/Trunk.toml

# webgl
trunk serve --features webgl --open --config apps/app/Trunk.toml
```

## Prerequisites (web)

* [trunk](https://trunkrs.dev/)