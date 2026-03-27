use crate::utils::{
    await_station_healthy, controller_test_id, set_controllers, upload_canister_modules,
    NNS_ROOT_CANISTER_ID,
};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use pocket_ic::common::rest::{IcpFeatures, IcpFeaturesConfig};
use pocket_ic::{PocketIc, PocketIcBuilder, PocketIcState};
use station_api::{
    InitUserInput, SystemInit as SystemInitArg, SystemInstall as SystemInstallArg,
    UserIdentityInput,
};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime};

pub static WALLET_ADMIN_USER: Principal = Principal::from_slice(&[1; 29]);
pub static CANISTER_INITIAL_CYCLES: u128 = 100_000_000_000_000;

/// The governance canister is the minting account for the ICP ledger
/// when bootstrapped via PocketIC's `icp_token` feature.
pub static NNS_GOVERNANCE_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 1, 1, 1]);

#[derive(Clone)]
pub struct SetupConfig {
    pub upload_canister_modules: bool,
    pub fallback_controller: Option<Principal>,
    pub start_cycles: Option<u128>,
    pub set_time_to_now: bool,
    pub capture_state: bool,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            upload_canister_modules: true,
            fallback_controller: Some(NNS_ROOT_CANISTER_ID),
            start_cycles: None,
            set_time_to_now: true,
            capture_state: false,
        }
    }
}

struct CachedTestEnv {
    pub state: PocketIcState,
    pub canister_ids: CanisterIds,
    pub controller: Principal,
    pub minter: Principal,
}

static CACHED_TEST_ENV: OnceLock<CachedTestEnv> = OnceLock::new();

pub fn setup_new_env() -> TestEnv {
    let cached_test_env = CACHED_TEST_ENV.get_or_init(|| {
        let config = SetupConfig {
            capture_state: true,
            ..Default::default()
        };

        let test_env = setup_new_env_with_config(config);

        // serialize and expose the state
        let state = test_env.env.drop_and_take_state().unwrap();

        CachedTestEnv {
            state,
            canister_ids: test_env.canister_ids,
            controller: test_env.controller,
            minter: test_env.minter,
        }
    });

    let env = PocketIcBuilder::new()
        .with_read_only_state(&cached_test_env.state)
        .build();
    TestEnv {
        env,
        canister_ids: cached_test_env.canister_ids,
        controller: cached_test_env.controller,
        minter: cached_test_env.minter,
    }
}

pub fn setup_new_env_with_config(config: SetupConfig) -> TestEnv {
    let path = env::var_os("POCKET_IC_BIN")
        .expect("The environment variable POCKET_IC_BIN containing the absolute path to the PocketIC binary is not set")
        .clone()
        .into_string()
        .expect("Invalid string path");

    if !Path::new(&path).exists() {
        println!("
        Could not find the PocketIC binary to run canister integration tests.

        I looked for it at {:?}. You can specify another absolute path with the environment variable POCKET_IC_BIN.

        Running the testing script will automatically set the POCKET_IC_BIN environment variable:
            ./scripts/run-integration-tests.sh
        ", &path);
    }

    let mut builder = PocketIcBuilder::new();
    if config.capture_state {
        builder = builder.with_state(PocketIcState::new());
    }
    // ICP ledger, ICP index, and CMC are bootstrapped automatically via `with_icp_features`.
    // `icp_token` implies `with_nns_subnet()` and deploys ICP ledger + index.
    // `cycles_minting` deploys the CMC and keeps subnet lists in sync with PocketIC topology.
    let mut env = builder
        .with_icp_features(IcpFeatures {
            icp_token: Some(IcpFeaturesConfig::DefaultConfig),
            cycles_minting: Some(IcpFeaturesConfig::DefaultConfig),
            ..Default::default()
        })
        .with_ii_subnet()
        .with_fiduciary_subnet()
        .with_application_subnet()
        .build();

    // If we set the time to SystemTime::now, and then progress pocketIC a couple ticks
    // and then enter live mode, we would crash the deterministic state machine, as the
    // live mode would set the time back to the current time.
    // Therefore, if we want to use live mode, we need to start the tests with the time
    // set to the past.
    let system_time = SystemTime::now() - Duration::from_secs(24 * 60 * 60);
    if config.set_time_to_now {
        env.set_time(system_time.into());
    }
    let controller = controller_test_id();
    let minter = NNS_GOVERNANCE_CANISTER_ID;
    let canister_ids = install_canisters(&mut env, config, controller);

    TestEnv {
        env,
        canister_ids,
        controller,
        minter,
    }
}

pub fn create_canister(env: &PocketIc, controller: Principal) -> Principal {
    create_canister_with_cycles(env, controller, CANISTER_INITIAL_CYCLES)
}

pub fn create_canister_with_cycles(
    env: &PocketIc,
    controller: Principal,
    cycles: u128,
) -> Principal {
    let canister_id = env.create_canister_with_settings(Some(controller), None);
    env.add_cycles(canister_id, cycles);
    canister_id
}

fn install_canisters(
    env: &mut PocketIc,
    config: SetupConfig,
    controller: Principal,
) -> CanisterIds {
    // System canisters (ICP ledger, ICP index, CMC) are already deployed by PocketIC
    // via `with_icp_features` — use their well-known canister IDs.
    let nns_ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let nns_index_canister_id = Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();
    let cmc_canister_id = Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap();

    let control_panel = create_canister_with_cycles(
        env,
        controller,
        config.start_cycles.unwrap_or(CANISTER_INITIAL_CYCLES),
    );
    let station = create_canister_with_cycles(
        env,
        controller,
        config.start_cycles.unwrap_or(CANISTER_INITIAL_CYCLES),
    );

    set_controllers(env, Some(controller), station, vec![controller, station]);

    let control_panel_wasm = get_canister_wasm("control_panel").to_vec();
    env.install_canister(
        control_panel,
        control_panel_wasm,
        Encode!(&()).unwrap(),
        Some(controller),
    );

    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let station_wasm = get_canister_wasm("station").to_vec();
    if config.upload_canister_modules {
        upload_canister_modules(env, control_panel, controller);
    }

    let station_init_args = SystemInstallArg::Init(Box::new(SystemInitArg {
        name: "Station".to_string(),

        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm,
                initial_cycles: Some(5_000_000_000_000),
            },
        ),
        fallback_controller: config.fallback_controller,
        initial_config: station_api::InitialConfig::WithAllDefaults {
            users: vec![InitUserInput {
                identities: vec![UserIdentityInput {
                    identity: WALLET_ADMIN_USER,
                }],
                name: "station-admin".to_string(),
                groups: None,
                id: None,
                status: station_api::UserStatusDTO::Active,
            }],
            admin_quorum: 1,
            operator_quorum: 1,
        },
    }));
    env.install_canister(
        station,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(env, station, WALLET_ADMIN_USER);

    CanisterIds {
        icp_ledger: nns_ledger_canister_id,
        icp_index: nns_index_canister_id,
        control_panel,
        station,
        cycles_minting_canister: cmc_canister_id,
    }
}

pub(crate) fn get_canister_wasm(canister_name: &str) -> Vec<u8> {
    read_file_from_local_bin(&format!("{canister_name}.wasm.gz"))
}

fn local_bin() -> PathBuf {
    let mut file_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("Failed to read CARGO_MANIFEST_DIR env variable"),
    );
    file_path.push("wasms");
    file_path
}

fn read_file_from_local_bin(file_name: &str) -> Vec<u8> {
    let mut file_path = local_bin();
    file_path.push(file_name);

    let mut file = File::open(&file_path)
        .unwrap_or_else(|_| panic!("Failed to open file: {}", file_path.to_str().unwrap()));
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read file");
    bytes
}
