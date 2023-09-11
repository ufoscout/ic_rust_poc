use std::cell::RefCell;

use candid::candid_method;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(999_999_999);
}

/// Get the value of the counter.
// candid_method is used by the  candid::export_service!() macro to export the did 
#[candid_method(query)]
#[ic_cdk::query]
fn get_counter() -> u64 {
    COUNTER.with(|c| (*c.borrow()).clone())
}

/// Increment the value of the counter.
#[candid_method(update)]
#[ic_cdk::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[cfg(test)]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}

#[cfg(test)]
mod test {

    use std::fs::*;
    use std::io::*;
    use std::env;
    use std::path::PathBuf;

    use super::*;

        #[test]
        fn export_candid_file() {

            let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            let path = dir
                .parent()
                .unwrap()
                .join("target")
                .join("canister_b.did");
            println!("path: {}", path.to_string_lossy());
            let mut file = File::create(path).unwrap();
            write!(file, "{}", export_candid()).expect("Write failed.");
        }
        
}
