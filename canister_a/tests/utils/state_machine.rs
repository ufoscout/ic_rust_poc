use std::sync::Mutex;

use candid::{CandidType, Deserialize, Principal};
use candid::{Decode, Encode};

use ic_test_state_machine_client::{StateMachine, WasmResult};
use once_cell::sync::Lazy;

use crate::utils::state_machine_client::get_ic_test_state_machine_client_path;
use crate::utils::wasm::get_canister_a_bytecode;

pub fn alice() -> Principal {
    Principal::from_text("sgymv-uiaaa-aaaaa-aaaia-cai").unwrap()
}

pub fn bob() -> Principal {
    Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap()
}

pub fn john() -> Principal {
    Principal::from_text("hozae-racaq-aaaaa-aaaaa-c").unwrap()
}

pub struct StateMachineTestContext {
    pub env: StateMachine,
    pub canister_a_principal: Principal,
}

impl StateMachineTestContext {
    pub fn query_as<Result>(
        &self,
        sender: Principal,
        canister_id: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result
    where
        for<'a> Result: CandidType + Deserialize<'a>,
    {
        let res = match self
            .env
            .query_call(canister_id, sender, method, payload)
            .unwrap()
        {
            WasmResult::Reply(bytes) => bytes,
            WasmResult::Reject(e) => panic!("Unexpected reject: {:?}", e),
        };

        Decode!(&res, Result).expect("failed to decode item from candid")
    }

    pub fn update_call_as<Result>(
        &self,
        sender: Principal,
        canister_id: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result
    where
        for<'a> Result: CandidType + Deserialize<'a>,
    {
        let res = match self
            .env
            .update_call(canister_id, sender, method, payload)
            .unwrap()
        {
            WasmResult::Reply(bytes) => bytes,
            WasmResult::Reject(e) => panic!("Unexpected reject: {:?}", e),
        };

        Decode!(&res, Result).expect("failed to decode item from candid")
    }

    pub fn get_counter(&self, sender: Principal) -> u64 {
        let args = Encode!(&()).unwrap();
        let res = self.query_as(sender, self.canister_a_principal, "get_counter", args);
        res
    }

}

pub fn with_state_machine_context<'a, F, E>(f: F) -> Result<(), E>
where
    F: FnOnce(&StateMachineTestContext) -> Result<(), E>,
{

    static TEST_CONTEXT: Lazy<Mutex<StateMachineTestContext>> = Lazy::new(|| {
        let client_path = get_ic_test_state_machine_client_path("../target");
        let env = StateMachine::new(&client_path, false);
        let dummy_canister = deploy_canister_a(&env);
        StateMachineTestContext {
            env,
            canister_a_principal: dummy_canister,
        }
        .into()
    });
    let test_ctx = TEST_CONTEXT.lock().unwrap();

    f(&test_ctx)?;

    reinstall_canister_a(&test_ctx);

    Ok(())
}

fn deploy_canister_a(env: &StateMachine) -> Principal {
    let dummy_wasm = get_canister_a_bytecode();
    eprintln!("Creating dummy canister");

    let args = Encode!(&()).unwrap();

    let canister = env.create_canister(None);
    env.add_cycles(canister, 10_u128.pow(12));
    env.install_canister(canister, dummy_wasm.to_vec(), args, None);

    canister
}

pub fn reinstall_canister_a(ctx: &StateMachineTestContext) {
    let args = Encode!(&()).unwrap();

    let dummy_wasm = get_canister_a_bytecode();

    ctx.env
        .reinstall_canister(ctx.canister_a_principal, dummy_wasm, args, None)
        .unwrap();

}
