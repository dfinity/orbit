use crate::env::ENV;
use crate::interfaces::{RegisterUserInput, RegisterUserResponse};
use crate::utils::user_test_id;
use crate::TestEnv;
use ic_canister_core::api::ApiResult;
use pocket_ic::call_candid_as;
use std::ops::Deref;

#[test]
fn basic_register_user_test() {
    let mut wrapper = ENV.deref().get();
    let TestEnv {
        env, canister_ids, ..
    } = wrapper.env();

    let user_id = user_test_id(0);

    let register_args = RegisterUserInput {
        identities: vec![user_id],
    };
    let _res: (ApiResult<RegisterUserResponse>,) = call_candid_as(
        env,
        canister_ids.wallet,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
}
