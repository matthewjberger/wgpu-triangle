set windows-shell := ["powershell.exe"]
export RUST_LOG := "info,wgpu_core=off"
export RUST_BACKTRACE := "1"

# Displays the list of available commands
@just:
    just --list

# Builds the project in release mode
build:
    cargo build -r

# Builds the project for WebGL
build-webgl $project="app":
    trunk build --features webgl --config apps/{{project}}/Trunk.toml

# Builds the project for WebGPU
build-webgpu $project="app":
    trunk build --features webgl --config apps/{{project}}/Trunk.toml

# Runs cargo check and format check
check:
    cargo check --all --tests
    cargo fmt --all -- --check

# Generates and opens documentation for a specific project
docs $project="engine":
    cargo doc --open -p {{project}}

# Fixes linting issues automatically
fix:
    cargo clippy --all --tests --fix

# Formats the code using cargo fmt
format:
    cargo fmt --all

# Runs linter and displays warnings
lint:
    cargo clippy --all --tests -- -D warnings

# Runs the specified project
run $project="app":
    cargo run -r -p {{project}}

# Runs the project using WebGL
run-webgl $project="app":
    trunk serve --features webgl --open --config apps/{{project}}/Trunk.toml

# Runs the project using WebGPU
run-webgpu $project="app":
    trunk serve --features webgpu --open --config apps/{{project}}/Trunk.toml

# Runs all tests
test:
    cargo test --all -- --nocapture

# Checks for unused dependencies
udeps:
    cargo machete

# Displays version information for Rust tools
@versions:
    rustc --version
    cargo fmt -- --version
    cargo clippy -- --version

# Watches for changes and runs the specified project
watch $project="app":
    cargo watch -x 'run -r -p {{project}}'
