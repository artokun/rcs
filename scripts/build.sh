#!/bin/bash
set -e

# Install Rust and required components
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    rustup target add wasm32-unknown-unknown
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
else
    echo "wasm-bindgen-cli is already installed"
fi

# Build the project
cargo build --release --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen --out-dir ./dist/out/ --target web ./target/wasm32-unknown-unknown/release/rcs.wasm
echo "Deleting old assets..."
rm -rf dist/assets

echo "Copying new assets..."
cp -r assets dist/