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
fn should_get_counter_from_another_canister() {
    with_state_machine_context::<_, ()>(|ctx| {
        assert_eq!(999_999_999, ctx.get_counter_from_another_canister(alice()));
        Ok(())
    })
    .unwrap();
}
