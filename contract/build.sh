#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release & cp -f ./target/wasm32-unknown-unknown/release/*.wasm ./dist