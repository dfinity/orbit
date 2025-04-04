use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_request, execute_request_with_extra_ticks, get_all_upgrader_logs,
    get_core_canister_health_status, get_system_info, upload_canister_chunks_to_asset_canister,
};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::management_canister::CanisterIdRecord;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    HealthStatus, NotifyFailedStationUpgradeInput, RequestOperationInput, RequestStatusDTO,
    RestoreExternalCanisterOperationInput, SystemInstall, SystemUpgrade,
    SystemUpgradeOperationInput, SystemUpgradeTargetDTO,
};
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
            take_backup_snapshot: None,
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
            take_backup_snapshot: None,
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
            take_backup_snapshot: None,
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
                take_backup_snapshot: None,
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
                take_backup_snapshot: None,
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
fn backup_snapshot() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let upgrader_init_arg = InitArg {
        target_canister: canister_ids.station,
    };
    let upgrader_init_arg_bytes = Encode!(&upgrader_init_arg).unwrap();

    let upgrade = |system_upgrade_operation_input: SystemUpgradeOperationInput| {
        let target = system_upgrade_operation_input.target.clone();
        let system_upgrade_operation =
            RequestOperationInput::SystemUpgrade(system_upgrade_operation_input);
        match target {
            SystemUpgradeTargetDTO::UpgradeStation => {
                do_successful_station_upgrade(&env, &canister_ids, system_upgrade_operation)
            }
            SystemUpgradeTargetDTO::UpgradeUpgrader => {
                do_successful_upgrader_upgrade(&env, &canister_ids, system_upgrade_operation)
            }
        };
    };

    let snapshot = |target: &SystemUpgradeTargetDTO| -> Option<Vec<u8>> {
        let (canister_id, caller) = match target {
            SystemUpgradeTargetDTO::UpgradeStation => (canister_ids.station, upgrader_id),
            SystemUpgradeTargetDTO::UpgradeUpgrader => (upgrader_id, canister_ids.station),
        };
        let snapshots: Vec<_> = env
            .list_canister_snapshots(canister_id, Some(caller))
            .unwrap();
        if snapshots.is_empty() {
            None
        } else {
            assert_eq!(snapshots.len(), 1);
            Some(snapshots[0].id.clone())
        }
    };

    let check_snapshots = |target: &SystemUpgradeTargetDTO, snapshot_id: Option<Vec<u8>>| {
        match target {
            SystemUpgradeTargetDTO::UpgradeStation => {
                let snapshots_via_upgrader =
                    update_candid_as::<_, (ApiResult<Vec<upgrader_api::Snapshot>>,)>(
                        &env,
                        upgrader_id,
                        WALLET_ADMIN_USER,
                        "canister_snapshots",
                        (),
                    )
                    .unwrap()
                    .0
                    .unwrap();
                if let Some(snapshot_id) = snapshot_id {
                    assert_eq!(snapshots_via_upgrader.len(), 1);
                    assert_eq!(
                        snapshots_via_upgrader[0].snapshot_id,
                        hex::encode(snapshot_id)
                    );
                } else {
                    assert!(snapshots_via_upgrader.is_empty());
                }
            }
            SystemUpgradeTargetDTO::UpgradeUpgrader => {
                let snapshots_via_station =
                    update_candid_as::<_, (ApiResult<Vec<station_api::Snapshot>>,)>(
                        &env,
                        canister_ids.station,
                        WALLET_ADMIN_USER,
                        "canister_snapshots",
                        (CanisterIdRecord {
                            canister_id: upgrader_id,
                        },),
                    )
                    .unwrap()
                    .0
                    .unwrap();
                if let Some(snapshot_id) = snapshot_id {
                    assert_eq!(snapshots_via_station.len(), 1);
                    assert_eq!(
                        snapshots_via_station[0].snapshot_id,
                        hex::encode(snapshot_id)
                    );
                } else {
                    assert!(snapshots_via_station.is_empty());
                }
            }
        };
    };

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
        let (base_chunk, module_extra_chunks) =
            upload_canister_chunks_to_asset_canister(&env, canister_wasm, chunk_len);

        // create system upgrade request operation input taking a backup snapshot
        let mut system_upgrade_operation_input = SystemUpgradeOperationInput {
            target: target.clone(),
            module: base_chunk.to_owned(),
            module_extra_chunks: Some(module_extra_chunks),
            arg: Some(arg_bytes),
            take_backup_snapshot: Some(true),
        };

        // there should be no snapshots yet
        check_snapshots(&target, None);

        upgrade(system_upgrade_operation_input.clone());

        // a backup snapshot should have been taken
        let backup_snapshot_id = snapshot(&target).unwrap();
        check_snapshots(&target, Some(backup_snapshot_id.clone()));

        // create system upgrade request operation input taking no backup snapshot
        system_upgrade_operation_input.take_backup_snapshot = None;

        upgrade(system_upgrade_operation_input.clone());

        // no new backup snapshot should have been taken
        check_snapshots(&target, Some(backup_snapshot_id.clone()));

        let upgrader_logs = get_all_upgrader_logs(&env, &upgrader_id, &WALLET_ADMIN_USER);

        // create system upgrade request operation input taking a backup snapshot
        system_upgrade_operation_input.take_backup_snapshot = Some(true);

        upgrade(system_upgrade_operation_input);

        // a new backup snapshot should have been taken, replacing the previous backup snapshot
        let new_backup_snapshot_id = snapshot(&target).unwrap();
        assert_ne!(backup_snapshot_id, new_backup_snapshot_id);
        check_snapshots(&target, Some(new_backup_snapshot_id.clone()));

        // upgrader logs now also contain entries created by syncing station data into upgrader
        let new_upgrader_logs = get_all_upgrader_logs(&env, &upgrader_id, &WALLET_ADMIN_USER);
        assert_ne!(upgrader_logs.len(), new_upgrader_logs.len());

        if let SystemUpgradeTargetDTO::UpgradeUpgrader = target {
            // restore the new backup snapshot
            let restore_canister_operation = RequestOperationInput::RestoreExternalCanister(
                RestoreExternalCanisterOperationInput {
                    canister_id: upgrader_id,
                    snapshot_id: hex::encode(new_backup_snapshot_id),
                },
            );
            execute_request(
                &env,
                WALLET_ADMIN_USER,
                canister_ids.station,
                restore_canister_operation,
            )
            .unwrap();

            // the new logs have been rolled back by restoring the upgrader
            let current_upgrader_logs =
                get_all_upgrader_logs(&env, &upgrader_id, &WALLET_ADMIN_USER);
            assert_eq!(upgrader_logs.len(), current_upgrader_logs.len());
        }
    }
}
