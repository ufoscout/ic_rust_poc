use std::sync::Mutex;

use candid::{CandidType, Deserialize, Principal};
use candid::{Decode, Encode};

use ic_test_state_machine_client::{StateMachine, WasmResult};
use once_cell::sync::Lazy;

use crate::utils::state_machine_client::get_ic_test_state_machine_client_path;
use crate::utils::wasm::{get_canister_a_bytecode, get_canister_b_bytecode};

pub fn alice() -> Principal {
    Principal::from_text("sgymv-uiaaa-aaaaa-aaaia-cai").unwrap()
}

// pub fn bob() -> Principal {
//     Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap()
// }

// pub fn john() -> Principal {
//     Principal::from_text("hozae-racaq-aaaaa-aaaaa-c").unwrap()
// }

pub struct StateMachineTestContext {
    pub env: StateMachine,
    pub canister_a_principal: Principal,
    pub canister_a_args: Principal,
    pub canister_b_principal: Principal,
}

impl StateMachineTestContext {
    pub fn query_as<Result, T: CandidType>(
        &self,
        canister_id: Principal,
        sender: Principal,
        method: &str,
        args: &T,
    ) -> Result
    where
        for<'a> Result: CandidType + Deserialize<'a>,
    {
        let res = match self
            .env
            .query_call(canister_id, sender, method, encode(args))
            .unwrap()
        {
            WasmResult::Reply(bytes) => bytes,
            WasmResult::Reject(e) => panic!("Unexpected reject: {:?}", e),
        };

        decode(&res)
    }

    pub fn update_call_as<Result, T: CandidType>(
        &self,
        canister_id: Principal,
        sender: Principal,
        method: &str,
        args: &T,
    ) -> Result
    where
        for<'a> Result: CandidType + Deserialize<'a>,
    {
        let res = match self
            .env
            .update_call(canister_id, sender, method, encode(args))
            .unwrap()
        {
            WasmResult::Reply(bytes) => bytes,
            WasmResult::Reject(e) => panic!("Unexpected reject: {:?}", e),
        };

        decode(&res)
    }

    pub fn get_counter(&self, sender: Principal) -> u64 {
        let args = &();
        self.query_as(self.canister_a_principal, sender,  "get_counter", args)
    }

    pub fn get_counter_from_another_canister(&self, sender: Principal) -> u64 {
        let args = &();
        self.update_call_as(self.canister_a_principal, sender,"get_counter_from_another_canister", args)
    }  

    pub fn increase_counter(&self, sender: Principal) {
        let args = &();
        self.update_call_as(self.canister_a_principal, sender, "increase_counter", args)
    }

    pub fn increase_counter_panic(&self, sender: Principal) {
        let args = &();
        let result = self.env.update_call(self.canister_a_principal, sender, "increase_counter_panic", encode(args));
        assert!(result.is_err())
    }  

    pub fn increase_counter_then_call_async_fn_then_panic(&self, sender: Principal) {
        let args = &();
        let result = self.env.update_call(self.canister_a_principal, sender, "increase_counter_then_call_async_fn_then_panic", encode(args));
        assert!(result.is_err())
    }  
    
    pub fn increase_counter_then_call_another_canister_then_panic(&self, sender: Principal) {
        let args = &();
        let result = self.env.update_call(self.canister_a_principal, sender, "increase_counter_then_call_another_canister_then_panic", encode(args));
        assert!(result.is_err())
    } 

    pub fn increase_counter_then_call_same_canister_then_panic(&self, sender: Principal) {
        let args = &();
        let result = self.env.update_call(self.canister_a_principal, sender, "increase_counter_then_call_same_canister_then_panic", encode(args));
        assert!(result.is_err())
    } 

}

pub fn with_state_machine_context<'a, F, E>(f: F) -> Result<(), E>
where
    F: FnOnce(&StateMachineTestContext) -> Result<(), E>,
{

    static TEST_CONTEXT: Lazy<Mutex<StateMachineTestContext>> = Lazy::new(|| {
        let client_path = get_ic_test_state_machine_client_path("../target");
        let env = StateMachine::new(&client_path, false);
        let canister_b_principal = deploy_canister(&env, get_canister_b_bytecode(), &());
        let canister_a_args = canister_b_principal;
        let canister_a_principal = deploy_canister(&env, get_canister_a_bytecode(), &canister_a_args);
        StateMachineTestContext {
            env,
            canister_a_principal,
            canister_a_args,
            canister_b_principal,
        }
        .into()
    });
    let test_ctx = TEST_CONTEXT.lock().unwrap();

    f(&test_ctx)?;

    reinstall_canister(&test_ctx, test_ctx.canister_a_principal, get_canister_a_bytecode(), &test_ctx.canister_a_args);
    reinstall_canister(&test_ctx, test_ctx.canister_b_principal, get_canister_b_bytecode(), &());

    Ok(())
}

fn deploy_canister<T: CandidType>(env: &StateMachine, bytecode: Vec<u8>, args: &T) -> Principal {
    let args = encode(args);
    let canister = env.create_canister(None);
    env.add_cycles(canister, 10_u128.pow(12));
    env.install_canister(canister, bytecode, args, None);
    canister
}

fn reinstall_canister<T: CandidType>(ctx: &StateMachineTestContext, principal: Principal, bytecode: Vec<u8>, args: &T) {
    let args = encode(args);
    ctx.env
        .reinstall_canister(principal, bytecode, args, None)
        .unwrap();
}

pub fn encode<T: CandidType>(item: &T) -> Vec<u8> {
    Encode!(item).expect("failed to encode item to candid")
}

pub fn decode<'a, T: CandidType + Deserialize<'a>>(bytes: &'a [u8]) -> T {
    Decode!(bytes, T).expect("failed to decode item from candid")
}