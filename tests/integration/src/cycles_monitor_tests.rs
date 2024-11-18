use crate::interfaces::{get_icp_account_balance, send_icp_to_account, ICP};
use crate::setup::{
    get_canister_wasm, setup_new_env, setup_new_env_with_config, SetupConfig, WALLET_ADMIN_USER,
};
use crate::utils::{
    advance_time_to_burn_cycles, controller_test_id, create_icp_account,
    get_core_canister_health_status, get_system_info, get_user, user_test_id, NNS_ROOT_CANISTER_ID,
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
    let account_id = AccountIdentifier::from_hex(&account.address).unwrap();

    send_icp_to_account(&env, controller, account_id, 100 * ICP, 0, None).unwrap();
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
