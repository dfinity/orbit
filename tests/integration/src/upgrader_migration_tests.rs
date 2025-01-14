use crate::setup::{get_canister_wasm, setup_new_env_with_config, SetupConfig, WALLET_ADMIN_USER};
use crate::upgrader_test_data::UpgraderDataGenerator;
use crate::utils::{
    compress_to_gzip, create_file, get_disaster_recovery_committee, get_system_info, read_file,
    request_disaster_recovery, upgrade_station,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use pocket_ic::PocketIc;

fn init_test_data_generator(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
) -> UpgraderDataGenerator {
    UpgraderDataGenerator::new(env, upgrader_id, station_id)
}

fn upgrade_from_v0(env: &PocketIc, upgrader_id: Principal, station_id: Principal) {
    // This is needed to avoid `install_code` rate limit error
    env.tick();
    env.tick();
    env.tick();

    // Set the stable memory of the canister to v0
    let wasm_memory = read_file("upgrader-memory-v0.bin").expect("Unexpected missing wasm memory");
    env.set_stable_memory(
        upgrader_id,
        wasm_memory,
        pocket_ic::common::rest::BlobCompression::Gzip,
    );

    let stable_memory_size_before_upgrade = env.get_stable_memory(upgrader_id).len();

    // Then upgrade the canister to trigger the migration path
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    env.upgrade_canister(
        upgrader_id,
        upgrader_wasm,
        Encode!(&()).expect("Failed to encode arguments"),
        Some(station_id),
    )
    .expect("Unexpected failure upgrading canister.");

    let stable_memory_size_after_upgrade = env.get_stable_memory(upgrader_id).len();

    // Assert that stable memory size doesn't grow after upgrade.
    assert!(stable_memory_size_after_upgrade <= stable_memory_size_before_upgrade);
}

fn upgrade_from_latest(env: &PocketIc, upgrader_id: Principal, station_id: Principal) {
    let mut canister_memory = env.get_stable_memory(upgrader_id);

    // Assert that stable memory size is 21 buckets of 1MiB each + stable structures header (64KiB) for the latest layout.
    assert_eq!(canister_memory.len(), 21 * (1 << 20) + (64 << 10));

    // This is used to store the stable memory of the canister for future use
    canister_memory = compress_to_gzip(&canister_memory);
    create_file("upgrader-memory-latest.bin", &canister_memory);

    // Then upgrade the canister with the same wasm.
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    env.upgrade_canister(
        upgrader_id,
        upgrader_wasm,
        Encode!(&()).expect("Failed to encode arguments"),
        Some(station_id),
    )
    .expect("Unexpected failure upgrading canister.");
}

fn test_upgrader_migration_from_version<F>(upgrade_from: F)
where
    F: FnOnce(&PocketIc, Principal, Principal),
{
    let config = SetupConfig {
        set_time_to_now: false,
        ..Default::default()
    };
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env_with_config(config);

    let upgrader_id = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station).upgrader_id;

    let mut test_data_generator = init_test_data_generator(&env, upgrader_id, canister_ids.station);

    // Adds the test data to the canister
    test_data_generator.generate();

    // Assert that the canister api is still working after adding the test data
    test_data_generator.test_api();

    env.stop_canister(upgrader_id, Some(canister_ids.station))
        .expect("Unexpected failure stopping canister");

    upgrade_from(&env, upgrader_id, canister_ids.station);

    let canister_memory = env.get_stable_memory(upgrader_id);
    let stable_memory_size_after_upgrade = canister_memory.len();

    env.start_canister(upgrader_id, Some(canister_ids.station))
        .expect("Unexpected failure starting canister.");

    // Assert that the canister api is still working after the upgrade
    test_data_generator.test_api();

    // Adds more test data to the canister
    test_data_generator.generate();

    // Assert that the canister api is still working after adding more test data
    test_data_generator.test_api();

    // Submit a few more large disaster recovery requests to test that
    // stable memory can grow with the latest stable memory layout.
    let committee =
        get_disaster_recovery_committee(&env, upgrader_id, canister_ids.station).unwrap();
    for (i, user) in committee.users.into_iter().take(20).enumerate() {
        let wasm_module = vec![i as u8; 2_000_000];
        let large_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
            upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                module: wasm_module,
                module_extra_chunks: None,
                arg: vec![],
                install_mode: upgrader_api::InstallMode::Reinstall,
            },
        );
        request_disaster_recovery(
            &env,
            upgrader_id,
            *user.identities.first().unwrap(),
            large_request,
        )
        .unwrap();
    }
    let canister_memory = env.get_stable_memory(upgrader_id);
    let stable_memory_size = canister_memory.len();
    assert!(stable_memory_size > stable_memory_size_after_upgrade);

    // Test that the station can still be upgraded via the upgrader with the latest stable memory layout.
    let current_station_name = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station).name;
    assert_eq!(current_station_name, "Station");
    upgrade_station(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        Some("Upgraded Station".to_string()),
    );
    let current_station_name = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station).name;
    assert_eq!(current_station_name, "Upgraded Station");
}

/// Tests that canister upgrades work if the stable memory version does not change.
#[test]
fn test_upgrader_migration_from_latest() {
    test_upgrader_migration_from_version(upgrade_from_latest);
}

/// Tests migration from v0 to latest.
#[test]
fn test_upgrader_migration_from_v0() {
    test_upgrader_migration_from_version(upgrade_from_v0);
}
