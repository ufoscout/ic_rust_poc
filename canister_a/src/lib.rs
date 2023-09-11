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
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_async_fn_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    do_something_async().await;
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

async fn do_something_async() {
    println!("do_something_async")
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_another_canister_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    canister_b_get_counter().await;
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn increase_counter_then_call_same_canister_then_panic() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    inter_canister_get_counter_call_to_itself().await;
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
    panic!()
}

#[candid_method(update)]
#[ic_cdk::update]
async fn get_counter_from_another_canister() -> u64 {
    canister_b_get_counter().await
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

#[cfg(test)]
mod test {

    use std::env;
    use std::fs::*;
    use std::io::*;
    use std::path::PathBuf;

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

    // use ic_exports::ic_kit::{mock_principals::alice, MockContext};

    // use super::*;

    // #[tokio::test]
    // async fn get_user_should_return_none() {
    //     // Arrange
    //     MockContext::new().with_id(alice()).inject();
    //     let canister = CanisterA::from_principal(alice());

    //     // Act
    //     let result = canister_call!(canister.get_user(3), Option<Model<u32, Data>>)
    //         .await
    //         .unwrap();

    //     // Assert
    //     assert!(result.is_none())
    // }

    // #[tokio::test]
    // async fn create_user_tx_should_be_committed() {
    //     // Arrange
    //     MockContext::new().with_id(alice()).inject();
    //     let canister = CanisterA::from_principal(alice());

    //     // Act
    //     let id = 111;
    //     let username = "ufoscout";

    //     canister_call!(canister.create_user(id, username.to_string()), ())
    //         .await
    //         .unwrap();
    //     let result = canister_call!(canister.get_user(id), Option<Model<u32, Data>>)
    //         .await
    //         .unwrap();

    //     // Assert
    //     assert_eq!(
    //         Some(Model::from((
    //             id,
    //             Data {
    //                 username: username.to_string(),
    //                 tokens: 0
    //             }
    //         ))),
    //         result
    //     )
    // }

    // #[tokio::test]
    // async fn create_user_tx_should_be_rolled_back() {
    //     // Arrange
    //     MockContext::new().with_id(alice()).inject();
    //     let canister = CanisterA::from_principal(alice());

    //     // Act
    //     let id = 111;
    //     let username = "ufoscout";

    //     let create_result =
    //         canister_call!(canister.create_user_rollback(id, username.to_string()), ()).await;
    //     let result = canister_call!(canister.get_user(id), Option<Model<u32, Data>>)
    //         .await
    //         .unwrap();

    //     // Assert
    //     assert!(create_result.is_ok());
    //     assert!(result.is_none());
    // }

    // #[tokio::test]
    // async fn update_user_tx_should_be_committed() {
    //     // Arrange
    //     MockContext::new().with_id(alice()).inject();
    //     let canister = CanisterA::from_principal(alice());

    //     let id = 22211;
    //     let username = "ufo";

    //     canister_call!(canister.create_user(id, username.to_string()), ())
    //         .await
    //         .unwrap();

    //     // Act
    //     let new_tokens = 1123;
    //     canister_call!(canister.update_user(id, new_tokens), ())
    //         .await
    //         .unwrap();
    //     let result = canister_call!(canister.get_user(id), Option<Model<u32, Data>>)
    //         .await
    //         .unwrap();

    //     // Assert
    //     assert_eq!(
    //         Some(Model::from((
    //             id,
    //             1,
    //             Data {
    //                 username: username.to_string(),
    //                 tokens: new_tokens
    //             }
    //         ))),
    //         result
    //     )
    // }

    // #[tokio::test]
    // async fn update_user_tx_should_be_rolled_back() {
    //     // Arrange
    //     MockContext::new().with_id(alice()).inject();
    //     let canister = CanisterA::from_principal(alice());

    //     let id = 22211;
    //     let username = "ufo";

    //     canister_call!(canister.create_user(id, username.to_string()), ())
    //         .await
    //         .unwrap();

    //     // Act
    //     let new_tokens = 1123;

    //     let update_result = std::panic::catch_unwind(|| {
    //         let handle = tokio::runtime::Handle::current();
    //         let _guard = handle.enter();
    //         futures::executor::block_on(canister.update_user_concurrent_error(id, new_tokens))
    //     });

    //     let result = canister_call!(canister.get_user(id), Option<Model<u32, Data>>)
    //         .await
    //         .unwrap();

    //     // Assert
    //     assert!(update_result.is_err());
    //     assert_eq!(
    //         Some(Model::from((
    //             id,
    //             1,
    //             Data {
    //                 username: username.to_string(),
    //                 tokens: 0
    //             }
    //         ))),
    //         result
    //     )
    // }
}
