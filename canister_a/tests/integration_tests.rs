use utils::state_machine::with_state_machine_context;

use crate::utils::state_machine::alice;

mod utils;

#[test]
fn should_get_counter() {
    with_state_machine_context::<_, ()>(|ctx| {
        println!("counter is: {}", ctx.get_counter(alice()));
        Ok(())
    })
    .unwrap();
}