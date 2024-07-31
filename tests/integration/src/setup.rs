use crate::interfaces::{
    NnsIndexCanisterInitPayload, NnsLedgerCanisterInitPayload, NnsLedgerCanisterPayload,
};
use crate::utils::{controller_test_id, minter_test_id, set_controllers, NNS_ROOT_CANISTER_ID};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use control_panel_api::UploadCanisterModulesInput;
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};
use pocket_ic::{PocketIc, PocketIcBuilder};
use station_api::{AdminInitInput, SystemInit as SystemInitArg, SystemInstall as SystemInstallArg};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

static POCKET_IC_BIN: &str = "./pocket-ic";

pub static WALLET_ADMIN_USER: Principal = Principal::from_slice(&[1; 29]);
pub static CANISTER_INITIAL_CYCLES: u128 = 100_000_000_000_000;

#[derive(Clone)]
pub struct SetupConfig {
    pub upload_canister_modules: bool,
    pub fallback_controller: Option<Principal>,
    pub start_cycles: Option<u128>,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            upload_canister_modules: true,
            fallback_controller: Some(NNS_ROOT_CANISTER_ID),
            start_cycles: None,
        }
    }
}

pub fn setup_new_env() -> TestEnv {
    setup_new_env_with_config(SetupConfig::default())
}

pub fn setup_new_env_with_config(config: SetupConfig) -> TestEnv {
    let path = match env::var_os("POCKET_IC_BIN") {
        None => {
            env::set_var("POCKET_IC_BIN", POCKET_IC_BIN);
            POCKET_IC_BIN.to_string()
        }
        Some(path) => path
            .clone()
            .into_string()
            .unwrap_or_else(|_| panic!("Invalid string path for {path:?}")),
    };

    if !Path::new(&path).exists() {
        println!("
        Could not find the PocketIC binary to run canister integration tests.

        I looked for it at {:?}. You can specify another path with the environment variable POCKET_IC_BIN (note that I run from {:?}).

        Running the testing script will automatically place the PocketIC binary at the right place to be run without setting the POCKET_IC_BIN environment variable:
            ./scripts/run-integration-tests.sh
        ", &path, &env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_| "an unknown directory".to_string()));
    }

    let mut env = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_application_subnet()
        .build();

    // If we set the time to SystemTime::now, and then progress pocketIC a couple ticks
    // and then enter live mode, we would crash the deterministic state machine, as the
    // live mode would set the time back to the current time.
    // Therefore, if we want to use live mode, we need to start the tests with the time
    // set to the past.
    env.set_time(SystemTime::now() - Duration::from_secs(24 * 60 * 60));
    let controller = controller_test_id();
    let minter = minter_test_id();
    let canister_ids = install_canisters(&mut env, config, controller, minter);

    TestEnv {
        env,
        canister_ids,
        controller,
        minter,
    }
}

pub fn create_canister(env: &mut PocketIc, controller: Principal) -> Principal {
    create_canister_with_cycles(env, controller, CANISTER_INITIAL_CYCLES)
}

pub fn create_canister_with_cycles(
    env: &mut PocketIc,
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
    minter: Principal,
) -> CanisterIds {
    let specified_nns_ledger_canister_id =
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let nns_ledger_canister_id = env
        .create_canister_with_id(Some(controller), None, specified_nns_ledger_canister_id)
        .unwrap();
    assert_eq!(nns_ledger_canister_id, specified_nns_ledger_canister_id);
    let specified_nns_index_canister_id =
        Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();
    let nns_index_canister_id = env
        .create_canister_with_id(Some(controller), None, specified_nns_index_canister_id)
        .unwrap();
    assert_eq!(nns_index_canister_id, specified_nns_index_canister_id);

    let controller_account = AccountIdentifier::new(&controller, &DEFAULT_SUBACCOUNT);
    let minting_account = AccountIdentifier::new(&minter, &DEFAULT_SUBACCOUNT);

    let icp_ledger_canister_wasm = get_canister_wasm("icp_ledger").to_vec();
    let icp_ledger_init_args = NnsLedgerCanisterPayload::Init(NnsLedgerCanisterInitPayload {
        minting_account: minting_account.to_string(),
        initial_values: HashMap::from([(
            controller_account.to_string(),
            Tokens::from_e8s(1_000_000_000_000),
        )]),
        send_whitelist: HashSet::new(),
        transfer_fee: Some(Tokens::from_e8s(10_000)),
        token_symbol: Some("ICP".to_string()),
        token_name: Some("Internet Computer".to_string()),
    });
    env.install_canister(
        nns_ledger_canister_id,
        icp_ledger_canister_wasm,
        Encode!(&icp_ledger_init_args).unwrap(),
        Some(controller),
    );

    let icp_index_canister_wasm = get_canister_wasm("icp_index").to_vec();
    let icp_index_init_args = NnsIndexCanisterInitPayload {
        ledger_id: nns_ledger_canister_id,
    };
    env.install_canister(
        nns_index_canister_id,
        icp_index_canister_wasm,
        Encode!(&icp_index_init_args).unwrap(),
        Some(controller),
    );

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
        let upload_canister_modules_args = UploadCanisterModulesInput {
            station_wasm_module: station_wasm.to_owned(),
            upgrader_wasm_module: upgrader_wasm.to_owned(),
        };
        env.update_call(
            control_panel,
            controller,
            "upload_canister_modules",
            Encode!(&upload_canister_modules_args).unwrap(),
        )
        .unwrap();
    }

    let station_init_args = SystemInstallArg::Init(SystemInitArg {
        name: "Station".to_string(),
        admins: vec![AdminInitInput {
            identity: WALLET_ADMIN_USER,
            name: "station-admin".to_string(),
        }],
        quorum: Some(1),
        upgrader: station_api::SystemUpgraderInput::WasmModule(upgrader_wasm),
        fallback_controller: config.fallback_controller,
        accounts: None,
    });
    env.install_canister(
        station,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );
    // required because the station canister performs post init tasks through a one off timer
    env.tick();
    // required because it requires inter canister calls to initialize the UUIDs generator with a call
    // to `raw_rand` which is not allowed in init calls,
    env.tick();
    env.tick();
    // required because the station canister creates the upgrader canister
    env.tick();
    // required because the station canister installs the upgrader canister
    env.tick();
    env.tick();
    // required because the station canister updates its own controllers
    env.tick();
    env.tick();

    CanisterIds {
        icp_ledger: nns_ledger_canister_id,
        icp_index: nns_index_canister_id,
        control_panel,
        station,
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
