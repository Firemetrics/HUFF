#!/bin/sh

cd hff-rs/wasm
wasm-pack build --release --target web --out-name hff-wasm --out-dir ../../chrome-extension/src/static

cd ../lib
cargo build --release

