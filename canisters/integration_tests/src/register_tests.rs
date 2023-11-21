use crate::setup::setup_new_env;
use crate::utils::user_test_id;
use crate::TestEnv;
use ic_canister_core::api::ApiResult;
use pocket_ic::call_candid_as;
use wallet_api::{RegisterUserInput, RegisterUserResponse};

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    let register_args = RegisterUserInput {
        identities: vec![user_id],
    };
    let _res: (ApiResult<RegisterUserResponse>,) = call_candid_as(
        &env,
        canister_ids.wallet,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
}
