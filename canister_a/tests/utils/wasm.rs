use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

pub fn get_canister_a_bytecode() -> Vec<u8> {
    static CANISTER_BYTECODE: OnceCell<Vec<u8>> = OnceCell::new();
    CANISTER_BYTECODE
        .get_or_init(|| load_canister_bytecode("canister_a.wasm"))
        .to_owned()
}

pub fn get_canister_b_bytecode() -> Vec<u8> {
    static CANISTER_BYTECODE: OnceCell<Vec<u8>> = OnceCell::new();
    CANISTER_BYTECODE
        .get_or_init(|| load_canister_bytecode("canister_b.wasm"))
        .to_owned()
}

fn load_canister_bytecode(wasm_name: &str) -> Vec<u8> {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = dir
        .parent()
        .unwrap()
        .join("target")
        .join(wasm_name);

    let mut f = File::open(path).expect("File does not exists");

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("Could not read file content");

    buffer
}