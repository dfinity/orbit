use crate::interfaces::{get_icp_account_balance, send_icp_to_account, ICP};
use crate::setup::{
    get_canister_wasm, setup_new_env, setup_new_env_with_config, SetupConfig, WALLET_ADMIN_USER,
};
use crate::utils::{
    advance_time_to_burn_cycles, controller_test_id, create_icp_account,
    get_core_canister_health_status, get_icp_account_identifier, get_system_info, get_user,
    user_test_id, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use control_panel_api::{
    AssociateWithCallerInput, DeployStationAdminUserInput, DeployStationInput,
    DeployStationResponse, RegisterUserInput, RegisterUserResponse, UpdateWaitingListInput,
    UserSubscriptionStatusDTO,
};
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::api::ApiResult;

use pocket_ic::{update_candid_as, CallError};
use sha2::{Digest, Sha256};
use station_api::{HealthStatus, SystemInfoResponse};
use std::time::Duration;

#[test]
fn successful_monitors_upgrader_and_tops_up() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // get station wasm
    let station_wasm = get_canister_wasm("station").to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&station_wasm);

    // check if canister is healthy
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let upgrader_id = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station).upgrader_id;

    let top_up_should_happen_when_cycles_below = 125_000_000_000;
    advance_time_to_burn_cycles(
        &env,
        NNS_ROOT_CANISTER_ID,
        upgrader_id,
        top_up_should_happen_when_cycles_below + 5_000_000_000,
    );

    let upgrader_cycles_balance = env.cycle_balance(upgrader_id);
    if upgrader_cycles_balance <= top_up_should_happen_when_cycles_below {
        panic!("Upgrader cycles balance is too low to run the test");
    }

    // wait for the fund manager to complete and release the lock
    for _ in 0..2 {
        env.tick();
    }

    advance_time_to_burn_cycles(
        &env,
        NNS_ROOT_CANISTER_ID,
        upgrader_id,
        top_up_should_happen_when_cycles_below - 5_000_000_000,
    );

    // wait for the fund manager to complete and top up the cycles
    for _ in 0..2 {
        env.tick();
    }

    assert!(env.cycle_balance(upgrader_id) > 250_000_000_000);
}

#[test]
fn successful_monitors_stations_and_tops_up() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { station: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.identity, user_id);

    // approve user
    let update_waiting_list_args = UpdateWaitingListInput {
        users: vec![user_id],
        new_status: UserSubscriptionStatusDTO::Approved,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller_test_id(),
        "update_waiting_list",
        (update_waiting_list_args,),
    )
    .unwrap();
    res.0.unwrap();

    let deploy_station_args = DeployStationInput {
        name: "test_station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: Vec::new() }),
        subnet_selection: None,
    };

    // deploy user station
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    let newly_created_user_station = res.0.unwrap().canister_id;

    // rounds required for station initialization
    let rounds_required_for_station_initialization = 7;
    for _ in 0..rounds_required_for_station_initialization {
        env.tick();
    }

    // the newly created station should be healthy at this point
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_station,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Healthy);

    // WALLET_ADMIN_USER is not admin of the newly created station and thus the following call should trap
    let res: Result<(ApiResult<SystemInfoResponse>,), CallError> = update_candid_as(
        &env,
        newly_created_user_station,
        WALLET_ADMIN_USER,
        "system_info",
        (),
    );
    let user_error = match res.unwrap_err() {
        CallError::UserError(user_error) => user_error,
        CallError::Reject(message) => panic!("Unexpected reject: {}", message),
    };
    assert!(user_error.description.contains(
        "Canister called `ic0.trap` with message: Unauthorized access to resources: System(SystemInfo)"
    ));

    let upgrader_id = get_system_info(&env, user_id, newly_created_user_station).upgrader_id;

    // add cycles to the upgrader so that the station won't top it up
    env.add_cycles(upgrader_id, 100_000_000_000_000);

    advance_time_to_burn_cycles(
        &env,
        NNS_ROOT_CANISTER_ID,
        newly_created_user_station,
        130_000_000_000,
    );

    let cycles_balance = env.cycle_balance(newly_created_user_station);
    if cycles_balance <= 125_000_000_000 {
        panic!(
            "Cycles balance is too low to run the test, cycles_balance: {}",
            cycles_balance
        );
    }

    advance_time_to_burn_cycles(
        &env,
        NNS_ROOT_CANISTER_ID,
        newly_created_user_station,
        120_000_000_000,
    );

    // wait for the fund manager to complete and top up the cycles
    env.advance_time(Duration::from_secs(24 * 60 * 60));
    for _ in 0..3 {
        env.tick();
    }

    assert!(env.cycle_balance(newly_created_user_station) > 350_000_000_000)
}

#[test]
fn can_mint_cycles_to_top_up_self() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env_with_config(SetupConfig {
        start_cycles: Some(4_500_000_000_000),
        ..Default::default()
    });

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    // top the upgrader up so that it wont run out of cycles
    env.add_cycles(upgrader_id, 100_000_000_000_000);

    env.stop_canister(upgrader_id, Some(NNS_ROOT_CANISTER_ID))
        .expect("stop canister failed");

    // set starting cycle balance to 200b
    advance_time_to_burn_cycles(
        &env,
        NNS_ROOT_CANISTER_ID,
        canister_ids.station,
        200_000_000_000,
    );

    let user_id = WALLET_ADMIN_USER;

    let user = get_user(&env, user_id, canister_ids.station);

    let account = create_icp_account(&env, canister_ids.station, user.id);
    let account_id = AccountIdentifier::from_hex(
        &get_icp_account_identifier(&account.addresses).expect("no icp address found"),
    )
    .unwrap();

    send_icp_to_account(&env, controller, account_id, 100 * ICP, 0, None, None).unwrap();
    let pre_account_balance = get_icp_account_balance(&env, account_id);
    let pre_cycle_balance = env.cycle_balance(canister_ids.station);
    assert_eq!(pre_account_balance, 100 * ICP);

    env.tick();
    env.advance_time(Duration::from_secs(24 * 60 * 60));
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    let post_account_balance = get_icp_account_balance(&env, account_id);
    let post_cycle_balance = env.cycle_balance(canister_ids.station);

    assert!(post_account_balance < pre_account_balance);
    assert!(post_cycle_balance > pre_cycle_balance);

    // assert that while we lose some cycles during the process, it'll be roughly what we expect
    assert!(
        post_cycle_balance - pre_cycle_balance > 249_000_000_000
            && post_cycle_balance - pre_cycle_balance < 250_000_000_000
    );
}
