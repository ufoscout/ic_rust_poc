use utils::pocket_ic_test_context::with_pocket_ic_context;

use crate::utils::pocket_ic_test_context::alice;

mod utils;

#[test]
fn should_get_counter() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_increase_counter() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter(alice());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_not_increase_counter_on_panics() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        assert!(ctx.increase_counter_panic(alice()).is_err());
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_not_commit_data_on_await_point_before_panic() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        assert!(ctx
            .increase_counter_then_call_async_fn_then_panic(alice())
            .is_err());
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_commit_data_on_inter_canister_call_point_before_panic() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        assert!(ctx
            .increase_counter_then_call_another_canister_then_panic(alice())
            .is_err());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_commit_data_on_inter_canister_call_to_itself_before_panic() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        assert!(ctx
            .increase_counter_then_call_same_canister_then_panic(alice())
            .is_err());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_get_counter_from_another_canister() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert_eq!(999_999_999, ctx.get_counter_from_another_canister(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_fail_to_catch_a_panic_in_wasm32() {
    with_pocket_ic_context::<_, ()>(|ctx| {
        assert!(ctx.catch_panic(alice()).is_err());
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_be_protected_by_inspect_message() {
    with_pocket_ic_context::<_, ()>(|ctx| {

        let wrapper = std::panic::AssertUnwindSafe(ctx);
        
        let err = std::panic::catch_unwind(move || {
            // This panics when rejected by inspect message
            let _ = wrapper.protected_by_inspect_message(alice());
        }).unwrap_err();
        
        assert!(err.downcast_ref::<String>().unwrap().contains("Call rejected by inspect check"));
        
        Ok(())
    })
    .unwrap();
}