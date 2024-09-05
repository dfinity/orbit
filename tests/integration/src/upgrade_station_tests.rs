use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    canister_status, execute_request_with_extra_ticks, get_core_canister_health_status,
    get_system_info, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use sha2::{Digest, Sha256};
use station_api::{
    HealthStatus, NotifyFailedStationUpgradeInput, RequestOperationInput, RequestStatusDTO,
    SystemInstall, SystemUpgrade, SystemUpgradeOperationInput, SystemUpgradeTargetDTO,
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
    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let station_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
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
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
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

#[test]
fn failed_station_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // check if station is healthy
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    let last_uprade_timestamp = system_info.last_upgrade_timestamp;

    // submit station upgrade request with an invalid WASM
    let station_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: vec![],
            arg: None,
        });
    // extra ticks are necessary to prevent polling on the request status
    // before the station canister is upgraded and running
    let request_status = execute_request_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_upgrade_operation,
        10,
    )
    .unwrap_err()
    .unwrap();
    // check that the station upgrade request is failed
    match request_status {
        RequestStatusDTO::Failed { reason } => assert!(reason
            .unwrap()
            .contains("Canister's Wasm module is not valid")),
        _ => panic!("Unexpected request status: {:?}", request_status),
    };

    // check the station status after the failed upgrade
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    // the last upgrade timestamp did not change after a failed upgrade
    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    assert_eq!(last_uprade_timestamp, system_info.last_upgrade_timestamp);
}

#[test]
fn unauthorized_notify_failed_station_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // Calling `notify_failed_station_upgrade` on behalf of the admin user fails in authorization (only the upgrader canister can call `notify_failed_station_upgrade`).
    let notify_failed_station_upgrade_input = NotifyFailedStationUpgradeInput {
        reason: "some reason".to_string(),
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "notify_failed_station_upgrade",
        (notify_failed_station_upgrade_input.clone(),),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert!(err
        .message
        .unwrap()
        .contains("Unauthorized to access to resource `notify_failed_station_upgrade`"));

    // Calling `notify_failed_station_upgrade` on behalf of the anonymous principal fails in authorization (only the upgrader canister can call `notify_failed_station_upgrade`).
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.station,
        Principal::anonymous(),
        "notify_failed_station_upgrade",
        (notify_failed_station_upgrade_input.clone(),),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert!(err
        .message
        .unwrap()
        .contains("Unauthorized to access to resource `notify_failed_station_upgrade`"));

    // Calling `notify_failed_station_upgrade` on behalf of the upgrader canister passes the authorization and fails later because there is no processing station upgrade request.
    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.station,
        system_info.upgrader_id,
        "notify_failed_station_upgrade",
        (notify_failed_station_upgrade_input.clone(),),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert!(err
        .message
        .unwrap()
        .contains("No station upgrade request is processing."));
}
