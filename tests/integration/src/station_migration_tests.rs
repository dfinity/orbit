use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::station_test_data::asset::list_assets;
use crate::station_test_data::{set_test_data_id, StationDataGenerator};
use crate::utils::{compress_to_gzip, create_file, read_file, NNS_ROOT_CANISTER_ID};
use crate::TestEnv;
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, PocketIc};

const CURRENT_BASELINE_NR_OF_REQUEST_POLICIES: usize = 24; // can be found in the station core/init.rs
const CURRENT_BASELINE_NR_PERMISSIONS: usize = 45; // can be found in the station core/init.rs

const PREVIOUS_BASELINE_NR_OF_REQUEST_POLICIES: usize = 18; // baseline in the previous memory version core/init.rs
const PREVIOUS_BASELINE_NR_PERMISSIONS: usize = 35; // baseline in the previous memory version core/init.rs

const POLICIES_ADDED_AT_MIGRATION: usize = 3;
const PERMISSIONS_ADDED_AT_MIGRATION: usize = 5;

const USER_GROUPS_NR: usize = 10;
const USER_NR: usize = 10;
const ACCOUNTS_NR: usize = 25;
const ADDRESS_BOOK_ENTRIES_NR: usize = 25;
const PERMISSIONS_NR: usize = 5;
const REQUEST_POLICY_NR: usize = 3;
const SYSTEM_UPGRADER_UPDATES_NR: usize = 1;
const SYSTEM_STATION_UPDATES_NR: usize = 1;
const EXPECTED_GENERATED_REQUESTS: usize = 150;
const EXPECTED_ADDITIONAL_REQUEST_POLICIES_NR: usize =
    // for accounts there are transfer policies and configuration policies
    ACCOUNTS_NR * 2 + REQUEST_POLICY_NR;
const EXPECTED_ADDITIONAL_PERMISSIONS_NR: usize =
    // for accounts there are view, transfer and configuration permissions
    ACCOUNTS_NR * 3;

#[test]
fn test_canister_migration_path_is_not_triggered_with_same_wasm() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let mut test_data_generator =
        StationDataGenerator::new(&env, canister_ids.station, WALLET_ADMIN_USER)
            .with_users(USER_NR)
            .with_user_groups(USER_GROUPS_NR)
            .with_accounts(ACCOUNTS_NR)
            .with_address_book_entries(ADDRESS_BOOK_ENTRIES_NR)
            .with_station_updates(SYSTEM_STATION_UPDATES_NR)
            .with_upgrader_updates(SYSTEM_UPGRADER_UPDATES_NR)
            .with_permission_updates(PERMISSIONS_NR)
            .with_request_policy_updates(REQUEST_POLICY_NR)
            .with_max_user_groups_per_user(5)
            .with_edit_operations();

    // Adds the test data to the canister
    test_data_generator.generate();

    let station_wasm = get_canister_wasm("station").to_vec();

    env.stop_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("unexpected failure stopping canister");

    // This is used to store the stable memory of the canister for future use
    let mut canister_memory = env.get_stable_memory(canister_ids.station);
    canister_memory = compress_to_gzip(&canister_memory);
    create_file("station-memory-latest.bin", &canister_memory);

    // Then upgrade the canister with the same wasm
    // to test that upgrades work also if stable memory version does not change.
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
    assert_can_list_users_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_NR + 1, // +1 because there is the first admin user
    );
    assert_can_list_user_groups_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_GROUPS_NR + 2, // +2 because there is the first admin group and the operator group
    );
    assert_can_list_address_book_entries(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        ADDRESS_BOOK_ENTRIES_NR,
    );
    assert_can_list_accounts(&env, canister_ids.station, WALLET_ADMIN_USER, ACCOUNTS_NR);
    assert_can_list_requests(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        test_data_generator.request_count(),
    );
    assert_can_list_request_policies(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_ADDITIONAL_REQUEST_POLICIES_NR + CURRENT_BASELINE_NR_OF_REQUEST_POLICIES,
    );
    assert_can_list_permissions(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_ADDITIONAL_PERMISSIONS_NR + CURRENT_BASELINE_NR_PERMISSIONS,
    );
    assert_can_list_named_rules(&env, canister_ids.station, WALLET_ADMIN_USER, 2);
}

/// Tests migration from v1 to latest.
#[test]
fn test_station_migration_from_v1() {
    test_canister_migration_path_with_previous_stable_memory_version(1);
}

/// Tests migration from v2 to latest.
#[test]
fn test_station_migration_from_v2() {
    test_canister_migration_path_with_previous_stable_memory_version(2);
}

fn test_canister_migration_path_with_previous_stable_memory_version(stable_memory_version: u64) {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_wasm = get_canister_wasm("station").to_vec();
    let stable_memory_file = format!("station-memory-v{}.bin", stable_memory_version);
    let stable_memory =
        read_file(&stable_memory_file).expect("Unexpected missing older stable memory");

    env.stop_canister(canister_ids.station, Some(NNS_ROOT_CANISTER_ID))
        .expect("unexpected failure stopping canister");

    // This is needed to avoid `install_code` rate limit error
    env.tick();
    env.tick();
    env.tick();

    // Set the stable memory of the canister to the previous version of the canister
    env.set_stable_memory(
        canister_ids.station,
        stable_memory,
        pocket_ic::common::rest::BlobCompression::Gzip,
    );

    // Then upgrade the canister to trigger the migration path
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
    assert_can_list_users_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_NR + 1, // +1 because there is the first admin user
    );
    assert_can_list_user_groups_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_GROUPS_NR + 1, // +1 because there is the first admin group
    );
    assert_can_list_address_book_entries(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        ADDRESS_BOOK_ENTRIES_NR,
    );
    assert_can_list_accounts(&env, canister_ids.station, WALLET_ADMIN_USER, ACCOUNTS_NR);
    assert_can_list_requests(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_GENERATED_REQUESTS,
    );
    assert_can_list_request_policies(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_ADDITIONAL_REQUEST_POLICIES_NR
            + PREVIOUS_BASELINE_NR_OF_REQUEST_POLICIES
            + POLICIES_ADDED_AT_MIGRATION,
    );
    assert_can_list_permissions(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_ADDITIONAL_PERMISSIONS_NR
            + PREVIOUS_BASELINE_NR_PERMISSIONS
            + PERMISSIONS_ADDED_AT_MIGRATION,
    );

    assert_has_icp_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    // Makes sure that the next test data id number is pointing at a value that was
    // not already used in the previous version
    set_test_data_id(9_999);

    // Number of new entries to generate for each type
    let new_records = 1;

    // Adds more data to the canister to ensure everything is working
    let mut test_data_generator =
        StationDataGenerator::new(&env, canister_ids.station, WALLET_ADMIN_USER)
            .with_users(new_records)
            .with_user_groups(new_records)
            .with_accounts(new_records)
            .with_address_book_entries(new_records)
            .with_assets(new_records)
            .with_request_policy_updates(new_records)
            .with_station_updates(0)
            .with_upgrader_updates(0)
            .with_edit_operations();

    // Adding the data to the canister should not fail
    test_data_generator.generate();

    // Assert that the new data is present in the canister
    assert_can_list_users_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_NR + 1 + new_records, // +1 because there is the first admin user
    );
    assert_can_list_user_groups_endpoint(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        USER_GROUPS_NR + 1 + new_records, // +1 because there is the first admin user
    );
    assert_can_list_address_book_entries(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        ADDRESS_BOOK_ENTRIES_NR + new_records,
    );
    assert_can_list_accounts(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        ACCOUNTS_NR + new_records,
    );
    assert_can_list_requests(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EXPECTED_GENERATED_REQUESTS + test_data_generator.request_count(),
    );
    assert_can_list_request_policies(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        // for accounts there are transfer policies and configuration policies
        EXPECTED_ADDITIONAL_REQUEST_POLICIES_NR
            + PREVIOUS_BASELINE_NR_OF_REQUEST_POLICIES
            + POLICIES_ADDED_AT_MIGRATION
            + new_records
            + (new_records * 2),
    );
    assert_can_list_permissions(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        // for accounts there are view, transfer and configuration permissions
        EXPECTED_ADDITIONAL_PERMISSIONS_NR
            + PREVIOUS_BASELINE_NR_PERMISSIONS
            + PERMISSIONS_ADDED_AT_MIGRATION
            + (new_records * 3),
    );

    assert_can_list_assets(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        // there should be one asset here already: ICP
        new_records + 1,
    );
}

fn assert_can_read_me_endpoint(env: &PocketIc, station_id: Principal, requester: Principal) {
    let res: (ApiResult<station_api::MeResponse>,) =
        update_candid_as(env, station_id, requester, "me", ()).unwrap();

    res.0.unwrap();
}

fn assert_can_list_users_endpoint(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
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

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_user_groups_endpoint(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
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

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_accounts(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
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

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_named_rules(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
    let res: (ApiResult<station_api::ListNamedRulesResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_named_rules",
        (station_api::ListNamedRulesInput {
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_address_book_entries(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
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
            address_formats: None,
            ids: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
            search_term: None,
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_requests(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
    let res: (ApiResult<station_api::ListRequestsResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_requests",
        (station_api::ListRequestsInput {
            approver_ids: None,
            created_from_dt: None,
            created_to_dt: None,
            expiration_from_dt: None,
            expiration_to_dt: None,
            only_approvable: false,
            operation_types: None,
            requester_ids: None,
            sort_by: None,
            with_evaluation_results: true,
            statuses: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_request_policies(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
    let res: (ApiResult<station_api::ListRequestPoliciesResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_request_policies",
        (station_api::ListRequestPoliciesInput {
            limit: Some(1000),
            offset: Some(0),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_permissions(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
    let res: (ApiResult<station_api::ListPermissionsResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_permissions",
        (station_api::ListPermissionsInput {
            resources: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_can_list_assets(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    expected: usize,
) {
    let res: (ApiResult<station_api::ListAssetsResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "list_assets",
        (station_api::ListAssetsInput {
            paginate: Some(station_api::PaginationInput {
                offset: Some(0),
                limit: Some(25),
            }),
        },),
    )
    .unwrap();

    let res = res.0.unwrap();

    assert_eq!(res.total as usize, expected);
}

fn assert_has_icp_asset(env: &PocketIc, station_id: Principal, requester: Principal) {
    let assets = list_assets(env, station_id, requester)
        .expect("Failed to query list assets")
        .0
        .expect("Failed to list assets");

    assert!(assets.assets.len() == 1);
    assert_eq!(assets.assets[0].symbol, "ICP");
    assert_eq!(assets.assets[0].name, "Internet Computer");
    assert_eq!(&assets.assets[0].blockchain, "icp");
    assert!(
        assets.assets[0]
            .standards
            .contains(&"icp_native".to_string())
            && assets.assets[0].standards.contains(&"icrc1".to_string())
    );
}
