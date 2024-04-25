set windows-shell := ["powershell.exe"]

export RUST_LOG := "info"
export RUST_BACKTRACE := "1"

@just:
    just --list

build:
    cargo build -r

check:
    cargo check --all --tests
    cargo fmt --all -- --check

docs $project="mui":
    cargo doc --open -p {{project}}

format:
    cargo fmt --all

fix:
    cargo clippy --all --tests --fix

lint:
    cargo clippy --all --tests -- -D warnings

run $project="app":
    cargo run -r -p {{project}}

run-webgl $project="app":
    trunk serve --features webgl --open --config apps/{{project}}/Trunk.toml

run-webgpu $project="app":
    trunk serve --features webgpu --open --config apps/{{project}}/Trunk.toml

udeps:
    cargo machete

test:
    cargo test --all -- --nocapture

@versions:
    rustc --version
    cargo fmt -- --version
    cargo clippy -- --version
