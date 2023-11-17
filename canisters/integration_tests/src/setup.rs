use crate::interfaces::{
    ControlPanelCanisterInit, DefaultWalletInit, NnsIndexCanisterInitPayload,
    NnsLedgerCanisterInitPayload, NnsLedgerCanisterPayload, UpgraderInitArg,
};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};
use pocket_ic::PocketIc;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::time::SystemTime;

static POCKET_IC_BIN: &str = "./pocket-ic";

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

    let mut env = PocketIc::new();
    env.set_time(SystemTime::now());
    let minter = Principal::anonymous();
    let controller = Principal::anonymous();
    let canister_ids = install_canisters(&mut env, minter, controller);

    TestEnv {
        env,
        canister_ids,
        controller,
        minter,
    }
}

fn create_canister(env: &mut PocketIc, controller: Principal) -> Principal {
    env.create_canister_with_settings(None, Some(controller))
}

fn install_canisters(env: &mut PocketIc, minter: Principal, controller: Principal) -> CanisterIds {
    let nns_canister_ids: Vec<_> = (0..12).map(|_| create_canister(env, minter)).collect();
    let nns_ledger_canister_id = nns_canister_ids[2];
    let nns_index_canister_id = nns_canister_ids[11];

    let minting_account = AccountIdentifier::new(&minter, &DEFAULT_SUBACCOUNT);
    let controller_account = AccountIdentifier::new(&controller, &DEFAULT_SUBACCOUNT);
    let icp_ledger_canister_wasm = include_bytes!("../wasms/icp_ledger.wasm.gz").to_vec();
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
        None,
    );

    let icp_index_canister_wasm = include_bytes!("../wasms/icp_index.wasm.gz").to_vec();
    let icp_index_init_args = NnsIndexCanisterInitPayload {
        ledger_id: nns_ledger_canister_id,
    };
    env.install_canister(
        nns_index_canister_id,
        icp_index_canister_wasm,
        Encode!(&icp_index_init_args).unwrap(),
        None,
    );

    let control_panel = create_canister(env, controller);
    let upgrader = create_canister(env, controller);
    let wallet = create_canister(env, controller);

    let control_panel_wasm_bytes = include_bytes!("../wasms/control_panel.wasm.gz").to_vec();
    let control_panel_init_args = ControlPanelCanisterInit {
        default_wallet: DefaultWalletInit::SpecifiedWalletCanister(wallet),
    };
    env.install_canister(
        control_panel,
        control_panel_wasm_bytes,
        Encode!(&control_panel_init_args).unwrap(),
        None,
    );

    let upgrader_wasm_bytes = include_bytes!("../wasms/upgrader.wasm.gz").to_vec();
    let upgrader_init_args = UpgraderInitArg {
        target_canister: wallet,
    };
    env.install_canister(
        upgrader,
        upgrader_wasm_bytes,
        Encode!(&upgrader_init_args).unwrap(),
        None,
    );

    let wallet_wasm_bytes = include_bytes!("../wasms/wallet.wasm.gz").to_vec();
    env.install_canister(wallet, wallet_wasm_bytes, Encode!(&()).unwrap(), None);

    CanisterIds {
        control_panel,
        upgrader,
        wallet,
        icp_ledger: nns_ledger_canister_id,
        icp_index: nns_index_canister_id,
    }
}
