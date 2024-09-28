# Rust / Wasm / Wgpu Triangle

This project demonstrates how to setup a [rust](https://www.rust-lang.org/) project
that uses [wgpu](https://wgpu.rs/) to render a spinning triangle, supporting
both webgl and webgpu [wasm](https://webassembly.org/) as well as native.

![Screenshot 2024-09-28 at 10 25 44 AM](https://github.com/user-attachments/assets/8e097121-1213-4036-89f3-ec708ea8582b)

![Screenshot 2024-09-28 at 10 26 09 AM](https://github.com/user-attachments/assets/fced00ff-c157-441b-adc6-9cca9a0441df)

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
