#!/bin/bash
set -e

# Install Rust and required components
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    rustup target add wasm32-unknown-unknown
fi

# Install wasm-bindgen-cli only if it's not already installed
if ! cargo install -f wasm-bindgen-cli; then
    echo "wasm-bindgen-cli is already installed"
fi

# Build the project
cargo build --release --features production --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen --out-dir ./dist/out/ --target web ./target/wasm32-unknown-unknown/release/rcs.wasm

# Copy any additional files if needed
# cp -r static/* dist/