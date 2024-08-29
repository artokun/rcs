#!/bin/bash
set -e

# Install Rust and required components
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install -f wasm-bindgen-cli

# Build the project
cargo build --release --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen --out-dir ./dist/out/ --target web ./target/wasm32-unknown-unknown/release/rcs.wasm

# Copy any additional files if needed
# cp -r static/* dist/