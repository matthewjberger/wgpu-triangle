# Rust / Wasm / Wgpu Triangle

This project demonstrates how to setup a [rust](https://www.rust-lang.org/) project
that uses [wgpu](https://wgpu.rs/) to render a spinning triangle, supporting
both webgl and webgpu [wasm](https://webassembly.org/) as well as native.

![Screenshot 2024-06-29 at 10 20 19 PM](https://github.com/matthewjberger/wgpu-triangle/assets/7131091/8c634a8a-73ba-41ac-86cb-83ae0f666826)
![Screenshot 2024-06-29 at 10 21 07 PM](https://github.com/matthewjberger/wgpu-triangle/assets/7131091/b7bfe9cc-8714-4488-8561-31c6ed1652d8)

## Quickstart

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
