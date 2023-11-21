use crate::setup::setup_new_env;
use crate::utils::user_test_id;
use crate::TestEnv;
use control_panel_api::{
    GetMainWalletResponse, RegisterUserInput, RegisterUserResponse, RegisterUserWalletInput,
};
use ic_canister_core::api::ApiResult;
use pocket_ic::call_candid_as;

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);
    let user_name = "TestUser".to_string();

    // user has no wallet so far
    let res: (ApiResult<GetMainWalletResponse>,) = call_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_wallet",
        (),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert_eq!(err.code, "ASSOCIATED_USER_IDENTITY_NOT_FOUND");

    // register user
    let wallet_args = RegisterUserWalletInput::PrivateWallet {
        id: canister_ids.wallet,
        use_shared_wallet: None,
    };
    let register_args = RegisterUserInput {
        name: Some(user_name.clone()),
        wallet: wallet_args,
    };
    let res: (ApiResult<RegisterUserResponse>,) = call_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.name, Some(user_name));

    // get main wallet
    let res: (ApiResult<GetMainWalletResponse>,) = call_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_wallet",
        (),
    )
    .unwrap();
    let main_wallet_dto = res.0.unwrap().wallet.unwrap();
    assert_eq!(main_wallet_dto.canister_id, canister_ids.wallet);
}
