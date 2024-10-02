use std::time::Duration;

use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

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
    let request_station_upgrade = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::SystemUpgrade(
            station_api::SystemUpgradeOperationInput {
                target: station_api::SystemUpgradeTargetDTO::UpgradeStation,
                module: station_wasm.clone(),
                module_extra_chunks: None,
                arg: None,
            },
        ),
    );

    // wait with extra ticks since the canister is stopped by the upgrade process
    for _ in 0..20 {
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
