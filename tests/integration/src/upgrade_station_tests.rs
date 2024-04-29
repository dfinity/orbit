use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    canister_status, execute_request_with_extra_ticks, get_core_canister_health_status,
    get_system_info, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::Encode;
use sha2::{Digest, Sha256};
use station_api::{
    ChangeCanisterOperationInput, ChangeCanisterTargetDTO, HealthStatus, RequestOperationInput,
    SystemInstall, SystemUpgrade,
};

#[test]
fn successful_station_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // get station wasm
    let station_wasm = get_canister_wasm("station").to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&station_wasm);
    let station_wasm_hash = hasher.finalize().to_vec();

    // check if canister is healthy
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    // submit station upgrade request
    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade {});
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let station_upgrade_operation =
        RequestOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeStation,
            module: station_wasm.clone(),
            arg: Some(station_init_arg_bytes),
        });
    // extra ticks are necessary to prevent polling on the request status
    // before the station canister is upgraded and running
    execute_request_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_upgrade_operation,
        10,
    )
    .unwrap();

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    let last_uprade_timestamp = system_info.last_upgrade_timestamp;

    // submit one more station upgrade request with no changes
    let station_upgrade_operation =
        RequestOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeStation,
            module: station_wasm.clone(),
            arg: None,
        });

    execute_request_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_upgrade_operation,
        10,
    )
    .unwrap();

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    // check if the last upgrade timestamp is updated
    assert!(system_info.last_upgrade_timestamp > last_uprade_timestamp);

    let status = canister_status(&env, Some(NNS_ROOT_CANISTER_ID), canister_ids.station);
    assert_eq!(status.module_hash.unwrap(), station_wasm_hash);
}
