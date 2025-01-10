use crate::setup::{get_canister_wasm, setup_new_env_with_config, SetupConfig, WALLET_ADMIN_USER};
use crate::upgrader_test_data::UpgraderDataGenerator;
use crate::utils::{compress_to_gzip, create_file, get_system_info, read_file};
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

    // Then upgrade the canister to trigger the migration path
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    env.upgrade_canister(
        upgrader_id,
        upgrader_wasm,
        Encode!(&()).expect("Failed to encode arguments"),
        Some(station_id),
    )
    .expect("Unexpected failure upgrading canister.");
}

fn upgrade_from_latest(env: &PocketIc, upgrader_id: Principal, station_id: Principal) {
    // This is used to store the stable memory of the canister for future use
    let mut canister_memory = env.get_stable_memory(upgrader_id);
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

    env.start_canister(upgrader_id, Some(canister_ids.station))
        .expect("Unexpected failure starting canister.");

    // Assert that the canister api is still working after the upgrade
    test_data_generator.test_api();

    // Adds more test data to the canister
    test_data_generator.generate();

    // Assert that the canister api is still working after adding more test data
    test_data_generator.test_api();
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
