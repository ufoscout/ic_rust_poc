use std::cell::RefCell;

use candid::candid_method;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
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
                .join("canister_a.did");
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
