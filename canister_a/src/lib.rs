use std::cell::RefCell;

use candid::{candid_method, Principal};

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
    static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

struct Config {
    pub canister_b_principal: Principal,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            canister_b_principal: Principal::anonymous(),
        }
    }
}

#[ic_cdk::init]
pub fn init(canister_b_principal: Principal) {
    CONFIG.with(|c| {
        c.replace(Config {
            canister_b_principal,
        })
    });
    set_panic_hook()
}

// candid_method is used by the  candid::export_service!() macro to export the did
#[candid_method(query)]
#[ic_cdk::query]
fn get_counter() -> u64 {
    COUNTER.with(|c| (*c.borrow()).clone())
}

#[candid_method(update)]
#[ic_cdk::update]
fn increase_counter() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_panic() {

    // The panic will revert this counter increase
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_async_fn_then_panic() {

    COUNTER.with(|counter| *counter.borrow_mut() += 1);

    // This calls an async method but it DOES NOT 
    // trigger the consensus and the previous counter increase is NOT committed
    do_something_async().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}


#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_another_canister_then_panic() {

    COUNTER.with(|counter| *counter.borrow_mut() += 1);

    // This performs a intercanister call to another canister,
    // This triggers the consensus and the previous counter increase is committed
    canister_b_get_counter().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_same_canister_then_panic() {
    
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    
    // This performs a intercanister call to itself,
    // This triggers the consensus and the previous counter increase is committed
    inter_canister_get_counter_call_to_itself().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn get_counter_from_another_canister() -> u64 {
    canister_b_get_counter().await
}

#[candid_method(query)]
#[ic_cdk::query]
fn catch_panic() -> String {

    // This will NOT catch the panic. 
    // wasm32-unknown-unknown is panic="abort" by default
    // even setting `panic = "unwind"` in Cargo.toml has no effect.
    let res = std::panic::catch_unwind(|| panic!());

    match res {
        Ok(_) => "success".to_string(),
        Err(_) => "error".to_string(),
    }
}

async fn canister_b_get_counter() -> u64 {
    let canister_b_principal = CONFIG.with(|c| c.borrow().canister_b_principal);
    let call_result: Result<(u64,), _> =
        ic_cdk::call(canister_b_principal, "get_counter", ((),)).await;
    call_result.unwrap().0
}

async fn inter_canister_get_counter_call_to_itself() -> u64 {
    let call_result: Result<(u64,), _> =
    ic_cdk::call(ic_cdk::api::id(), "get_counter", ((),)).await;
    call_result.unwrap().0
}

async fn do_something_async() {
    ic_cdk::println!("do_something_async")
}

/// Sets a custom panic hook
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        
        let (file, line, col) = if let Some(location) = info.location() {
            let file = location.file().to_owned();
            let line = location.line();
            let col = location.column();
            (file, line, col)
        } else {
            ("unknown".to_owned(), 0, 0)
        };

        
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        
        let err_info = format!("Panicked at '{}', {}:{}:{}", msg, file, line, col);

        ic_cdk::println!("------------------------");
        ic_cdk::println!("PANIC!");
        ic_cdk::println!("{}", err_info);
        ic_cdk::println!("------------------------");

    }));
}

#[cfg(test)]
mod test {
    
    use std::env;
    use std::fs::*;
    use std::io::*;
    use std::path::PathBuf;

    /// This exports the candid service description to a String.
    /// It includes only the methods with the `candid_method` macro
    fn export_candid() -> String {
        candid::export_service!();
        __export_service()
    }

    #[test]
    fn export_candid_file() {
        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = dir.parent().unwrap().join("target").join("canister_a.did");
        println!("path: {}", path.to_string_lossy());
        let mut file = File::create(path).unwrap();
        write!(file, "{}", export_candid()).expect("Write failed.");
    }


}
