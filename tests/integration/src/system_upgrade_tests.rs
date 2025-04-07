use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_request, execute_request_with_extra_ticks, get_core_canister_health_status,
    get_request, get_system_info, submit_delayed_request_raw,
    upload_canister_chunks_to_asset_canister, wait_for_request_with_extra_ticks,
};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    HealthStatus, NotifyFailedStationUpgradeInput, RequestOperationInput, RequestStatusDTO,
    SystemInstall, SystemUpgrade, SystemUpgradeOperationInput, SystemUpgradeTargetDTO,
};
use std::time::Duration;
use upgrader_api::InitArg;

pub(crate) const STATION_UPGRADE_EXTRA_TICKS: u64 = 200;

fn do_successful_station_upgrade(
    env: &PocketIc,
    canister_ids: &CanisterIds,
    station_upgrade_operation: RequestOperationInput,
) {
    // check if station is healthy
    let health_status =
        get_core_canister_health_status(env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    // submit station upgrade request
    // extra ticks are necessary to prevent polling on the request status
    // before the station canister has been restarted
    execute_request_with_extra_ticks(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_upgrade_operation.clone(),
        STATION_UPGRADE_EXTRA_TICKS,
    )
    .unwrap();

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    let last_uprade_timestamp = system_info.last_upgrade_timestamp;

    // submit one more station upgrade request with no changes
    execute_request_with_extra_ticks(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_upgrade_operation,
        STATION_UPGRADE_EXTRA_TICKS,
    )
    .unwrap();

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(env, WALLET_ADMIN_USER, canister_ids.station);
    // check if the last upgrade timestamp is updated
    assert!(system_info.last_upgrade_timestamp > last_uprade_timestamp);
}

fn do_successful_upgrader_upgrade(
    env: &PocketIc,
    canister_ids: &CanisterIds,
    upgrader_upgrade_operation: RequestOperationInput,
) {
    // submit upgrader upgrade request
    execute_request(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        upgrader_upgrade_operation.clone(),
    )
    .unwrap();
}

fn do_failed_system_upgrade(
    env: &PocketIc,
    canister_ids: &CanisterIds,
    system_upgrade_operation: RequestOperationInput,
    expected_reason: &str,
) {
    // check if station is healthy
    let health_status =
        get_core_canister_health_status(env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    let last_uprade_timestamp = system_info.last_upgrade_timestamp;

    // submit invalid station upgrade request
    // extra ticks are necessary to prevent polling on the request status
    // before the station canister has been restarted
    let request_status = execute_request_with_extra_ticks(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        system_upgrade_operation,
        STATION_UPGRADE_EXTRA_TICKS,
    )
    .unwrap_err()
    .unwrap();

    // check that the station upgrade request is failed
    match request_status {
        RequestStatusDTO::Failed { reason } => assert!(reason.unwrap().contains(expected_reason)),
        _ => panic!("Unexpected request status: {:?}", request_status),
    };

    // check the station status after the failed upgrade
    let health_status =
        get_core_canister_health_status(env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    // the last upgrade timestamp did not change after a failed upgrade
    let system_info = get_system_info(env, WALLET_ADMIN_USER, canister_ids.station);
    assert!(system_info.raw_rand_successful);
    assert_eq!(last_uprade_timestamp, system_info.last_upgrade_timestamp);
}

#[test]
fn failed_station_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: vec![],
            module_extra_chunks: None,
            arg: None,
        });

    do_failed_system_upgrade(
        &env,
        &canister_ids,
        station_upgrade_operation,
        "Canister's Wasm module is not valid",
    );
}

#[test]
fn too_many_chunks() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let canister_wasm = get_canister_wasm("station").to_vec();
    let chunk_len = canister_wasm.len() / 150;
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, canister_wasm, chunk_len);

    // create system upgrade request from chunks
    let system_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: None,
        });

    do_failed_system_upgrade(
        &env,
        &canister_ids,
        system_upgrade_operation,
        "The total number of wasm chunks must not exceed 101",
    );
}

#[test]
fn too_large_wasm() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, vec![42_u8; 102 << 20], 1 << 20);

    // create system upgrade request from chunks
    let system_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: None,
        });

    do_failed_system_upgrade(
        &env,
        &canister_ids,
        system_upgrade_operation,
        "Wasm extra chunks length 105_906_176 exceeds the maximum wasm length 104_857_600",
    );
}

#[test]
fn system_upgrade_from_chunks() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let upgrader_init_arg = InitArg {
        target_canister: canister_ids.station,
    };
    let upgrader_init_arg_bytes = Encode!(&upgrader_init_arg).unwrap();

    for (target, arg_bytes, canister_name, chunk_len) in [
        (
            SystemUpgradeTargetDTO::UpgradeStation,
            station_init_arg_bytes,
            "station",
            500_000,
        ),
        (
            SystemUpgradeTargetDTO::UpgradeUpgrader,
            upgrader_init_arg_bytes,
            "upgrader",
            50_000,
        ),
    ] {
        // upload chunks to asset canister
        let canister_wasm = get_canister_wasm(canister_name).to_vec();
        let (base_chunk, mut module_extra_chunks) =
            upload_canister_chunks_to_asset_canister(&env, canister_wasm, chunk_len);

        // create system upgrade request from chunks
        let system_upgrade_operation =
            RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
                target: target.clone(),
                module: base_chunk.to_owned(),
                module_extra_chunks: Some(module_extra_chunks.clone()),
                arg: Some(arg_bytes.clone()),
            });

        // successful upgrade
        match target {
            SystemUpgradeTargetDTO::UpgradeStation => {
                do_successful_station_upgrade(&env, &canister_ids, system_upgrade_operation)
            }
            SystemUpgradeTargetDTO::UpgradeUpgrader => {
                do_successful_upgrader_upgrade(&env, &canister_ids, system_upgrade_operation)
            }
        };

        // create invalid system upgrade request from chunks
        let actual_wasm_module_hash = module_extra_chunks.wasm_module_hash.clone();
        module_extra_chunks.wasm_module_hash[0] ^= 1;
        let system_upgrade_operation =
            RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
                target: target.clone(),
                module: base_chunk.to_owned(),
                module_extra_chunks: Some(module_extra_chunks.clone()),
                arg: Some(arg_bytes),
            });

        // failed upgrade
        do_failed_system_upgrade(
          &env,
          &canister_ids,
          system_upgrade_operation,
          &format!("failed to install code from chunks: Error from Wasm chunk store: Wasm module hash {:?} does not match given hash WasmHash({:?}).", actual_wasm_module_hash, module_extra_chunks.wasm_module_hash)
        );
    }
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

#[test]
fn delayed_system_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // upload chunks to asset canister
    let canister_wasm = get_canister_wasm("upgrader").to_vec();
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, canister_wasm, 50_000);

    // create system upgrade request from chunks
    let upgrader_init_arg = InitArg {
        target_canister: canister_ids.station,
    };
    let upgrader_init_arg_bytes = Encode!(&upgrader_init_arg).unwrap();
    let system_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeUpgrader,
            module: base_chunk.to_owned(),
            module_extra_chunks: Some(module_extra_chunks),
            arg: Some(upgrader_init_arg_bytes),
        });

    let request = submit_delayed_request_raw(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        system_upgrade_operation,
        Duration::from_secs(30 * 24 * 60 * 60),
    )
    .unwrap()
    .0
    .unwrap()
    .request;

    for _ in 0..100 {
        let request = get_request(
            &env,
            WALLET_ADMIN_USER,
            canister_ids.station,
            request.clone(),
        );
        match request.status {
            RequestStatusDTO::Created => (),
            _ => {
                break;
            }
        };
        env.advance_time(Duration::from_secs(5));
        env.tick();
    }

    env.advance_time(Duration::from_secs(30 * 24 * 60 * 60));

    wait_for_request_with_extra_ticks(&env, WALLET_ADMIN_USER, canister_ids.station, request, 0)
        .unwrap();
}
