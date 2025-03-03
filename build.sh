#!/bin/sh

cd hff-rs
wasm-pack build --release --target web --out-name hff-wasm --out-dir ../chrome-extension/src/static -- --features wasm
cargo build --release --features cli
