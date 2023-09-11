use utils::state_machine::with_state_machine_context;

use crate::utils::state_machine::alice;

mod utils;

#[test]
fn should_get_counter() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_increase_counter() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter(alice());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_not_increase_counter_on_panics() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter_panic(alice());
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_not_commit_data_on_await_point_before_panic() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter_then_call_async_fn_then_panic(alice());
        assert_eq!(0, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_commit_data_on_inter_canister_call_point_before_panic() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter_then_call_another_canister_then_panic(alice());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_commit_data_on_inter_canister_call_to_itself_before_panic() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(0, ctx.get_counter(alice()));
        ctx.increase_counter_then_call_same_canister_then_panic(alice());
        assert_eq!(1, ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}

#[test]
fn should_get_counter_from_another_canister() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(999_999_999, ctx.get_counter_from_another_canister(alice()));
        Ok(())
    })
    .unwrap();
}
