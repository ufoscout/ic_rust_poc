use candid::{CandidType, Deserialize, Principal};
use candid::{Decode, Encode};

use canister_a::InitArgs;
use ic_mple_pocket_ic::get_pocket_ic_client;
use ic_mple_pocket_ic::pocket_ic::{PocketIc, RejectResponse};

use crate::utils::wasm::{get_canister_a_bytecode, get_canister_b_bytecode};

pub fn alice() -> Principal {
    Principal::from_text("sgymv-uiaaa-aaaaa-aaaia-cai").unwrap()
}

pub struct PocketIcTestContext {
    pub client: PocketIc,
    pub canister_a_principal: Principal,
    // pub canister_a_args: InitArgs,
    // pub canister_b_principal: Principal,
}

impl PocketIcTestContext {
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
        let res = self
            .client
            .query_call(canister_id, sender, method, encode(args))
            .unwrap();

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
        let res = self
            .client
            .update_call(canister_id, sender, method, encode(args))
            .unwrap();

        decode(&res)
    }

    pub fn get_counter(&self, sender: Principal) -> u64 {
        let args = &();
        self.query_as(self.canister_a_principal, sender, "get_counter", args)
    }

    pub fn get_counter_from_another_canister(&self, sender: Principal) -> u64 {
        let args = &();
        self.update_call_as(
            self.canister_a_principal,
            sender,
            "get_counter_from_another_canister",
            args,
        )
    }

    pub fn increase_counter(&self, sender: Principal) {
        let args = &();
        self.update_call_as(self.canister_a_principal, sender, "increase_counter", args)
    }

    pub fn catch_panic(&self, sender: Principal) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.query_call(
            self.canister_a_principal,
            sender,
            "catch_panic",
            encode(args),
        )
    }

    pub fn increase_counter_panic(&self, sender: Principal) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "increase_counter_panic",
            encode(args),
        )
    }

    pub fn increase_counter_then_call_async_fn_then_panic(
        &self,
        sender: Principal,
    ) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "increase_counter_then_call_async_fn_then_panic",
            encode(args),
        )
    }

    pub fn increase_counter_then_call_another_canister_then_panic(
        &self,
        sender: Principal,
    ) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "increase_counter_then_call_another_canister_then_panic",
            encode(args),
        )
    }

    pub fn increase_counter_then_call_same_canister_then_panic(
        &self,
        sender: Principal,
    ) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "increase_counter_then_call_same_canister_then_panic",
            encode(args),
        )
    }

    pub fn protected_by_inspect_message(&self, sender: Principal) -> Result<Vec<u8>, RejectResponse> {
        let args = &();
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "protected_by_inspect_message",
            encode(args),
        )
    }

    pub fn get_drop_counter(&self, sender: Principal) -> u64 {
        let args = &();
        self.query_as(self.canister_a_principal, sender, "get_drop_counter", args)
    }

    pub fn increase_drop_counter(&self, sender: Principal, should_panic: bool) -> Result<Vec<u8>, RejectResponse> {
        let args = &(should_panic);
        self.client.update_call(
            self.canister_a_principal,
            sender,
            "increase_drop_counter",
            encode(args),
        )
    }
}

pub fn with_pocket_ic_context<'a, F, E>(f: F) -> Result<(), E>
where
    F: FnOnce(&PocketIcTestContext) -> Result<(), E>,
{
    let client = get_pocket_ic_client().build();
    let canister_b_principal = deploy_canister(&client, get_canister_b_bytecode(), &());
    let canister_a_args = InitArgs {
        canister_b_principal,
    };
    let canister_a_principal =
        deploy_canister(&client, get_canister_a_bytecode(), &canister_a_args);

    f(&PocketIcTestContext {
        client: client,
        canister_a_principal,
        // canister_a_args,
        // canister_b_principal,
    })
}

fn deploy_canister<T: CandidType>(client: &PocketIc, bytecode: Vec<u8>, args: &T) -> Principal {
    let args = encode(args);
    let canister = client.create_canister();
    client.add_cycles(canister, 10_u128.pow(12));
    client.install_canister(canister, bytecode, args, None);
    canister
}

// fn reinstall_canister<T: CandidType>(
//     ctx: &PocketIcTestContext,
//     principal: Principal,
//     bytecode: Vec<u8>,
//     args: &T,
// ) {
//     let args = encode(args);
//     ctx.client
//         .reinstall_canister(principal, bytecode, args, None)
//         .unwrap();
// }

pub fn encode<T: CandidType>(item: &T) -> Vec<u8> {
    Encode!(item).expect("failed to encode item to candid")
}

pub fn decode<'a, T: CandidType + Deserialize<'a>>(bytes: &'a [u8]) -> T {
    Decode!(bytes, T).expect("failed to decode item from candid")
}
