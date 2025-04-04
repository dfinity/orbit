use std::time::Duration;

use crate::utils::{submit_request, upload_canister_chunks_to_asset_canister, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;
use station_api::{RequestOperationInput, SystemUpgradeOperationInput};

pub fn perform_upgrader_update(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    upgrader_wasm: Vec<u8>,
) {
    let request_upgrader_upgrade = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::SystemUpgrade(
            station_api::SystemUpgradeOperationInput {
                target: station_api::SystemUpgradeTargetDTO::UpgradeUpgrader,
                module: upgrader_wasm.clone(),
                module_extra_chunks: None,
                arg: None,
                backup_snapshot: None,
            },
        ),
    );

    wait_for_request(
        env,
        requester,
        station_canister_id,
        request_upgrader_upgrade,
    )
    .expect("Failed to upgrade upgrader");
}

pub fn perform_station_update(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    station_wasm: Vec<u8>,
) {
    // upload chunks to asset canister
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(env, station_wasm, 500_000);

    // create system upgrade request from chunks
    let system_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: station_api::SystemUpgradeTargetDTO::UpgradeStation,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: None,
            backup_snapshot: None,
        });

    let request_station_upgrade = submit_request(
        env,
        requester,
        station_canister_id,
        system_upgrade_operation,
    );

    // wait with extra ticks since the canister is stopped by the upgrade process
    for _ in 0..100 {
        env.tick();
        env.advance_time(Duration::from_secs(1));
    }

    wait_for_request(env, requester, station_canister_id, request_station_upgrade)
        .expect("Failed to upgrade station");

    // wait with extra ticks to make sure the post_process logic that is async is executed
    for _ in 0..10 {
        env.tick();
    }
}
