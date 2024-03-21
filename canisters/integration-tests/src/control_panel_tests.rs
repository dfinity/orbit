use crate::setup::setup_new_env;
use crate::utils::user_test_id;
use crate::TestEnv;
use control_panel_api::{
    DeployWalletResponse, GetMainWalletResponse, ManageUserInput, ManageUserResponse,
    RegisterUserInput, RegisterUserResponse, UserWalletDTO,
};
use ic_canister_core::api::ApiResult;
use pocket_ic::update_candid_as;
use wallet_api::HealthStatus;

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // user has no wallet so far
    let res: (ApiResult<GetMainWalletResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_wallet",
        (),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert_eq!(err.code, "NOT_FOUND");

    // register user
    let register_args = RegisterUserInput {
        wallet_id: Some(canister_ids.wallet),
    };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.id, user_id);

    // get main wallet
    let res: (ApiResult<GetMainWalletResponse>,) = update_candid_as(
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

#[test]
fn deploy_user_wallet() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { wallet_id: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.id, user_id);

    // deploy user wallet
    let res: (ApiResult<DeployWalletResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_wallet",
        (),
    )
    .unwrap();
    let newly_created_user_wallet = res.0.unwrap().canister_id;

    // get main wallet
    let res: (ApiResult<GetMainWalletResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_wallet",
        (),
    )
    .unwrap();
    let main_wallet_dto = res.0.unwrap().wallet.unwrap();
    assert_eq!(main_wallet_dto.canister_id, newly_created_user_wallet);

    // the newly created wallet should be uninitialized at first
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_wallet,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Uninitialized);

    let rounds_required_for_wallet_initialization = 5;
    for _ in 0..rounds_required_for_wallet_initialization {
        env.tick();
    }

    // the newly created wallet should be healthy at this point
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_wallet,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Healthy);
}

#[test]
fn deploy_too_many_wallets() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { wallet_id: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.id, user_id);

    // deploy the maximum amount of user wallets
    let mut wallets = vec![];
    for _ in 0..10 {
        let res: (ApiResult<DeployWalletResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "deploy_wallet",
            (),
        )
        .unwrap();
        wallets.push(res.0.unwrap().canister_id);
    }

    // reset all but one deployed wallet
    let manage_user_args = ManageUserInput {
        main_wallet: Some(wallets[0]),
        wallets: Some(vec![UserWalletDTO {
            canister_id: wallets[0],
            name: None,
        }]),
    };
    let res: (ApiResult<ManageUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "manage_user",
        (manage_user_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.wallets.len(), 1);

    // deploying an additional wallet should fail nonetheless
    let res: (ApiResult<DeployWalletResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_wallet",
        (),
    )
    .unwrap();
    assert_eq!(res.0.unwrap_err().code, "DEPLOY_WALLET_QUOTA_EXCEEDED");
}
