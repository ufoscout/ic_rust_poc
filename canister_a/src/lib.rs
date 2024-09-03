use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, update};
use request::{HttpRequest, HttpResponse};

mod inspect_message;
mod request;

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

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct InitArgs {
    pub canister_b_principal: Principal,
}

#[ic_cdk::init]
fn init(arg: InitArgs) {
    CONFIG.with(|c| {
        c.replace(Config {
            canister_b_principal: arg.canister_b_principal,
        })
    });
    set_panic_hook()
}

#[query]
fn get_counter() -> u64 {
    COUNTER.with(|c| (*c.borrow()).clone())
}

#[update]
fn increase_counter() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[update]
async fn increase_counter_panic() {
    // The panic will revert this counter increase
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[update]
async fn increase_counter_then_call_async_fn_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);

    // This calls an async method but it DOES NOT
    // trigger the consensus and the previous counter increase is NOT committed
    do_something_async().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[update]
async fn increase_counter_then_call_another_canister_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);

    // This performs a intercanister call to another canister,
    // This triggers the consensus and the previous counter increase is committed
    canister_b_get_counter().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[update]
async fn increase_counter_then_call_same_canister_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);

    // This performs a intercanister call to itself,
    // This triggers the consensus and the previous counter increase is committed
    inter_canister_get_counter_call_to_itself().await;

    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[update]
async fn get_counter_from_another_canister() -> u64 {
    canister_b_get_counter().await
}

#[query]
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

// It should not be possible to call this method
#[update]
fn protected_by_inspect_message() -> bool {
    true
}

// This handles HTTP requests.
// If the response contains upgrade:true the call is upgraded to an update one
// and the http_request_update method is called by the icx-proxy.
// WARN: headers and body cannot be customized!!
#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    ic_cdk::println!("http_request called with: {:?}", req);
    HttpResponse {
        status_code: 204,
        headers: HashMap::from([("custom_header", "custom_value")]),
        body: vec![1, 2, 3, 4],
        upgrade: Some(true),
    }
}

#[update]
pub fn http_request_update(req: HttpRequest) -> HttpResponse {
    ic_cdk::println!("http_request_update called with: {:?}", req);
    HttpResponse {
        status_code: 400,
        headers: HashMap::new(),
        body: vec![],
        upgrade: None,
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

// Enable Candid export
ic_cdk::export_candid!();
