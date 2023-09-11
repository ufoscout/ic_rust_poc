#!/bin/sh

set -e

cargo build --target wasm32-unknown-unknown --release
ic-wasm target/wasm32-unknown-unknown/release/canister_a.wasm -o target/canister_a.wasm shrink
gzip -k target/canister_a.wasm --force

cargo test -- --nocapture
