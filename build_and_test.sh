#!/bin/sh

set -e

cargo build --target wasm32-unknown-unknown --release


ic-wasm target/wasm32-unknown-unknown/release/canister_b.wasm -o target/canister_b.wasm shrink
gzip -k target/canister_b.wasm --force
candid-extractor ./target/canister_b.wasm > ./target/canister_b.did

ic-wasm target/wasm32-unknown-unknown/release/canister_a.wasm -o target/canister_a.wasm shrink
gzip -k target/canister_a.wasm --force
candid-extractor ./target/canister_a.wasm > ./target/canister_a.did

cargo test -- --nocapture
