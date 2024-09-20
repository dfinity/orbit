use crate::setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_request, execute_request_with_extra_ticks, get_core_canister_health_status,
    get_system_info,
};
use crate::{CanisterIds, TestEnv};
use candid::{CandidType, Encode, Principal};
use orbit_essentials::api::ApiResult;
use orbit_essentials::types::WasmModuleExtraChunks;
use pocket_ic::{update_candid_as, PocketIc};
use sha2::{Digest, Sha256};
use station_api::{
    HealthStatus, NotifyFailedStationUpgradeInput, RequestOperationInput, RequestStatusDTO,
    SystemInstall, SystemUpgrade, SystemUpgradeOperationInput, SystemUpgradeTargetDTO,
};
use upgrader_api::InitArg;

const EXTRA_TICKS: u64 = 50;

#[derive(CandidType)]
struct StoreArg {
    pub key: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub content_encoding: String,
    pub sha256: Option<Vec<u8>>,
}

fn hash(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn upload_canister_chunks_to_asset_canister(
    env: &PocketIc,
    canister_name: &str,
    chunk_len: usize,
) -> (Vec<u8>, WasmModuleExtraChunks) {
    // create and install the asset canister
    let asset_canister_id = create_canister(env, Principal::anonymous());
    env.install_canister(
        asset_canister_id,
        get_canister_wasm("assetstorage"),
        Encode!(&()).unwrap(),
        None,
    );

    // get station wasm
    let station_wasm = get_canister_wasm(canister_name).to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&station_wasm);
    let station_wasm_hash = hasher.finalize().to_vec();

    // chunk station
    let mut chunks = station_wasm.chunks(chunk_len);
    let base_chunk: &[u8] = chunks.next().unwrap();
    assert!(!base_chunk.is_empty());
    let chunks: Vec<&[u8]> = chunks.collect();
    assert!(chunks.len() >= 2);

    // upload chunks to asset canister
    for chunk in &chunks {
        let chunk_hash = hash(chunk.to_vec());
        let store_arg = StoreArg {
            key: hex::encode(chunk_hash.clone()),
            content: chunk.to_vec(),
            content_type: "application/octet-stream".to_string(),
            content_encoding: "identity".to_string(),
            sha256: Some(chunk_hash),
        };
        update_candid_as::<_, ((),)>(
            env,
            asset_canister_id,
            Principal::anonymous(),
            "store",
            (store_arg,),
        )
        .unwrap();
    }

    let module_extra_chunks = WasmModuleExtraChunks {
        store_canister: asset_canister_id,
        chunk_hashes_list: chunks.iter().map(|c| hash(c.to_vec())).collect(),
        wasm_module_hash: station_wasm_hash,
    };

    (base_chunk.to_vec(), module_extra_chunks)
}

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
        EXTRA_TICKS,
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
        EXTRA_TICKS,
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
        EXTRA_TICKS,
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
fn successful_station_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // get station wasm
    let station_wasm = get_canister_wasm("station").to_vec();

    // create station upgrade request
    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let station_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: station_wasm,
            module_extra_chunks: None,
            arg: Some(station_init_arg_bytes),
        });

    do_successful_station_upgrade(&env, &canister_ids, station_upgrade_operation);
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
            200_000,
        ),
        (
            SystemUpgradeTargetDTO::UpgradeUpgrader,
            upgrader_init_arg_bytes,
            "upgrader",
            50_000,
        ),
    ] {
        // upload chunks to asset canister
        let (base_chunk, mut module_extra_chunks) =
            upload_canister_chunks_to_asset_canister(&env, canister_name, chunk_len);

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
