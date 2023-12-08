use crate::interfaces::{
    NnsIndexCanisterInitPayload, NnsLedgerCanisterInitPayload, NnsLedgerCanisterPayload,
};
use crate::utils::{controller_test_id, minter_test_id};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use control_panel_api::{CanisterInit as ControlPanelInitArg, DefaultWalletInit};
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use upgrader_api::InitArg as UpgraderInitArg;
use wallet_api::WalletCanisterInit as WalletInitArg;

static POCKET_IC_BIN: &str = "./pocket-ic";

pub static WALLET_ADMIN_USER: Principal = Principal::from_slice(&[1; 29]);

pub fn setup_new_env() -> TestEnv {
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
    env.set_time(SystemTime::now());
    let controller = controller_test_id();
    let minter = minter_test_id();
    let canister_ids = install_canisters(&mut env, controller, minter);

    TestEnv {
        env,
        canister_ids,
        controller,
        minter,
    }
}

fn create_canister(env: &mut PocketIc, controller: Principal) -> Principal {
    let canister_id = env.create_canister_with_settings(Some(controller), None);
    env.add_cycles(canister_id, 100_000_000_000_000_u128);
    canister_id
}

fn install_canisters(env: &mut PocketIc, controller: Principal, minter: Principal) -> CanisterIds {
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

    let control_panel = create_canister(env, controller);
    let upgrader = create_canister(env, controller);
    let wallet = create_canister(env, controller);

    let control_panel_wasm = get_canister_wasm("control_panel").to_vec();
    let control_panel_init_args = ControlPanelInitArg {
        default_wallet: DefaultWalletInit::SpecifiedWalletCanister(wallet),
    };
    env.install_canister(
        control_panel,
        control_panel_wasm,
        Encode!(&control_panel_init_args).unwrap(),
        Some(controller),
    );

    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let upgrader_init_args = UpgraderInitArg {
        target_canister: wallet,
    };
    env.install_canister(
        upgrader,
        upgrader_wasm,
        Encode!(&upgrader_init_args).unwrap(),
        Some(controller),
    );

    let wallet_wasm = get_canister_wasm("wallet").to_vec();
    let wallet_init_args = WalletInitArg {
        owners: Some(vec![WALLET_ADMIN_USER]),
    };
    env.install_canister(
        wallet,
        wallet_wasm,
        Encode!(&wallet_init_args).unwrap(),
        Some(controller),
    );
    // required because the admin users of the wallet are added through a timer after the canister is installed
    env.advance_time(Duration::from_secs(1));
    // required because the wallet canister adds the admin users through a timer after it is installed
    // which is required because it requires inter canister calls to initialize the UUIDs generator with a call
    // to `raw_rand` which is not allowed in init calls
    env.tick();
    env.tick();
    env.tick();

    CanisterIds {
        icp_ledger: nns_ledger_canister_id,
        icp_index: nns_index_canister_id,
        control_panel,
        upgrader,
        wallet,
    }
}

fn get_canister_wasm(canister_name: &str) -> Vec<u8> {
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
