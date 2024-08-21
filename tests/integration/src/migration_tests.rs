use std::time::Duration;

use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    add_account, add_address_book_entry, add_user_group, add_user_v2, compress_to_gzip,
    create_file, read_file, set_next_number, submit_request, wait_for_request,
    NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::MeResponse;

const USER_GROUPS_NR: usize = 10;
const USER_NR: usize = 20;
const ACCOUNTS_NR: usize = 25;
const ADDRESS_BOOK_ENTRIES_NR: usize = 100;

#[test]
fn test_canister_migration_path_with_same_wasm() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_wasm = get_canister_wasm("station").to_vec();
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

    // Create at least one upgrader upgrade
    let request_upgrader_upgrade = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_api::RequestOperationInput::SystemUpgrade(
            station_api::SystemUpgradeOperationInput {
                target: station_api::SystemUpgradeTargetDTO::UpgradeUpgrader,
                module: upgrader_wasm.clone(),
                arg: None,
            },
        ),
    );

    wait_for_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        request_upgrader_upgrade,
    )
    .expect("Failed to upgrade upgrader");

    // Create at least one station upgrade
    let request_station_upgrade = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_api::RequestOperationInput::SystemUpgrade(
            station_api::SystemUpgradeOperationInput {
                target: station_api::SystemUpgradeTargetDTO::UpgradeStation,
                module: station_wasm.clone(),
                arg: None,
            },
        ),
    );

    // wait with extra ticks since the canister is stopped by the upgrade process
    for _ in 0..10 {
        env.tick();
        env.advance_time(Duration::from_secs(1));
    }

    wait_for_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        request_station_upgrade,
    )
    .expect("Failed to upgrade station");

    // wait with extra ticks to make sure the post_process logic that is async is executed
    for _ in 0..10 {
        env.tick();
    }

    create_stable_memory(&env, canister_ids.station);

    env.stop_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("unexpected failure stopping canister");

    let mut canister_memory = env.get_stable_memory(canister_ids.station);
    canister_memory = compress_to_gzip(&canister_memory);

    // This is used to store the stable memory of the canister for future use
    create_file("station.bin", &canister_memory);

    env.upgrade_canister(
        canister_ids.station,
        station_wasm,
        Encode!(&()).expect("failed to encode arguments"),
        Some(NNS_ROOT_CANISTER_ID),
    )
    .expect("Unexpected failure upgrading canister.");

    env.start_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("Unexpected failure starting canister.");

    // Assert that the canister api is still working after the upgrade
    assert_can_read_me_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_users_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_user_groups_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_address_book_entries(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_accounts(&env, canister_ids.station, WALLET_ADMIN_USER);
}

#[test]
fn test_canister_migration_path_with_previous_wasm_memory_version() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_wasm = get_canister_wasm("station").to_vec();
    let wasm_memory =
        read_file("station-previous.bin").expect("Unexpected missing older wasm memory");

    env.stop_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("unexpected failure stopping canister");

    // This needed to avoid `install_code` rate limit error
    env.tick();

    env.set_stable_memory(
        canister_ids.station,
        wasm_memory,
        pocket_ic::common::rest::BlobCompression::Gzip,
    );

    env.upgrade_canister(
        canister_ids.station,
        station_wasm,
        Encode!(&()).expect("failed to encode arguments"),
        Some(NNS_ROOT_CANISTER_ID),
    )
    .expect("Unexpected failure upgrading canister.");

    env.start_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("Unexpected failure starting canister.");

    // Assert that the canister api is still working after the upgrade
    assert_can_read_me_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_users_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_user_groups_endpoint(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_address_book_entries(&env, canister_ids.station, WALLET_ADMIN_USER);
    assert_can_list_accounts(&env, canister_ids.station, WALLET_ADMIN_USER);

    // Makes sure that the next number is pointing at a value that was not already used in the previous version
    set_next_number(9_999);

    // Create at least one more entry of each type to ensure the stable memory is working
    let new_user_groups = vec![add_user_group(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
    )];

    add_user_v2(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        new_user_groups,
        None,
    );

    add_account(&env, canister_ids.station, WALLET_ADMIN_USER);
    add_address_book_entry(&env, canister_ids.station, WALLET_ADMIN_USER);
}

/// Create the context of the station canister, including:
///
/// - Users
/// - User groups
/// - Accounts
/// - Address book entries
/// - Requests
fn create_stable_memory(env: &PocketIc, station_id: Principal) {
    let mut group_ids = Vec::new();
    for _ in 0..USER_GROUPS_NR {
        group_ids.push(add_user_group(env, station_id, WALLET_ADMIN_USER));
    }

    for _ in 0..USER_NR {
        add_user_v2(env, station_id, WALLET_ADMIN_USER, group_ids.clone(), None);
    }

    for _ in 0..ACCOUNTS_NR {
        add_account(env, station_id, WALLET_ADMIN_USER);
    }

    for _ in 0..ADDRESS_BOOK_ENTRIES_NR {
        add_address_book_entry(env, station_id, WALLET_ADMIN_USER);
    }
}

fn assert_can_read_me_endpoint(env: &PocketIc, station_id: Principal, requester: Principal) {
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(env, station_id, requester, "me", ()).unwrap();

    res.0.unwrap();
}

fn assert_can_list_users_endpoint(env: &PocketIc, station_id: Principal, requester: Principal) {
    let res: (ApiResult<station_api::ListUsersResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_users",
        (station_api::ListUsersInput {
            groups: None,
            search_term: None,
            statuses: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, USER_NR + 1); // 1 is the default admin user
}

fn assert_can_list_user_groups_endpoint(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
) {
    let res: (ApiResult<station_api::ListUserGroupsResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_user_groups",
        (station_api::ListUserGroupsInput {
            search_term: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, USER_GROUPS_NR + 1); // 1 is the default group
}

fn assert_can_list_accounts(env: &PocketIc, station_id: Principal, requester: Principal) {
    let res: (ApiResult<station_api::ListAccountsResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_accounts",
        (station_api::ListAccountsInput {
            search_term: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, ACCOUNTS_NR);
}

fn assert_can_list_address_book_entries(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
) {
    let res: (ApiResult<station_api::ListAddressBookEntriesResponseDTO>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_address_book_entries",
        (station_api::ListAddressBookEntriesInputDTO {
            blockchain: None,
            labels: None,
            addresses: None,
            ids: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, ADDRESS_BOOK_ENTRIES_NR);
}
