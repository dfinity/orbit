use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    advance_time_to_burn_cycles, controller_test_id, get_core_canister_health_status,
    get_system_info, user_test_id, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use control_panel_api::{
    DeployStationResponse, RegisterUserInput, RegisterUserResponse, UpdateWaitingListInput,
    UserSubscriptionStatusDTO,
};
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use sha2::{Digest, Sha256};
use station_api::HealthStatus;
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
    let register_args = RegisterUserInput { station_id: None };
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

    // deploy user station
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (),
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
    for _ in 0..2 {
        env.tick();
    }

    assert!(env.cycle_balance(newly_created_user_station) > 350_000_000_000)
}
