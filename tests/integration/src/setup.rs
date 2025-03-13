use crate::interfaces::{
    NnsIndexCanisterInitPayload, NnsLedgerCanisterInitPayload, NnsLedgerCanisterPayload,
};
use crate::utils::{
    await_station_healthy, controller_test_id, minter_test_id, set_controllers,
    upload_canister_modules, NNS_ROOT_CANISTER_ID,
};
use crate::{CanisterIds, TestEnv};
use candid::{CandidType, Encode, Principal};
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};
use pocket_ic::{update_candid_as, PocketIc, PocketIcBuilder};
use serde::Serialize;
use station_api::{
    SystemInit as SystemInitArg, SystemInstall as SystemInstallArg, UserIdentityInput,
    InitUserInput,
};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub static WALLET_ADMIN_USER: Principal = Principal::from_slice(&[1; 29]);
pub static CANISTER_INITIAL_CYCLES: u128 = 100_000_000_000_000;

#[derive(CandidType, Serialize)]
pub struct SetAuthorizedSubnetworkListArgs {
    pub who: Option<Principal>,
    pub subnets: Vec<Principal>,
}

#[derive(CandidType, Serialize)]
enum UpdateSubnetTypeArgs {
    Add(String),
    //Remove(String),
}

#[derive(CandidType, Serialize)]
struct SubnetListWithType {
    pub subnets: Vec<Principal>,
    pub subnet_type: String,
}

#[derive(CandidType, Serialize)]
enum ChangeSubnetTypeAssignmentArgs {
    Add(SubnetListWithType),
    //Remove(SubnetListWithType),
}

#[derive(Serialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum ExchangeRateCanister {
    /// Enables the exchange rate canister with the given canister ID.
    Set(Principal),
}

#[derive(Serialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct CyclesCanisterInitPayload {
    pub ledger_canister_id: Option<Principal>,
    pub governance_canister_id: Option<Principal>,
    pub minting_account_id: Option<AccountIdentifier>,
    pub exchange_rate_canister: Option<ExchangeRateCanister>,
    pub cycles_ledger_canister_id: Option<Principal>,
    pub last_purged_notification: Option<u64>,
}

#[derive(Clone)]
pub struct SetupConfig {
    pub upload_canister_modules: bool,
    pub fallback_controller: Option<Principal>,
    pub start_cycles: Option<u128>,
    pub set_time_to_now: bool,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            upload_canister_modules: true,
            fallback_controller: Some(NNS_ROOT_CANISTER_ID),
            start_cycles: None,
            set_time_to_now: true,
        }
    }
}

pub fn setup_new_env() -> TestEnv {
    setup_new_env_with_config(SetupConfig::default())
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

    let mut env = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_ii_subnet()
        .with_fiduciary_subnet()
        .with_application_subnet()
        .build();

    // If we set the time to SystemTime::now, and then progress pocketIC a couple ticks
    // and then enter live mode, we would crash the deterministic state machine, as the
    // live mode would set the time back to the current time.
    // Therefore, if we want to use live mode, we need to start the tests with the time
    // set to the past.
    if config.set_time_to_now {
        env.set_time(SystemTime::now() - Duration::from_secs(24 * 60 * 60));
    }
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

    let specified_cmc_canister_id = Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap();
    let cmc_canister_id = env
        .create_canister_with_id(Some(controller), None, specified_cmc_canister_id)
        .unwrap();
    assert_eq!(cmc_canister_id, specified_cmc_canister_id);

    let specified_nns_exchange_rate_canister_id =
        Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap();
    let nns_exchange_rate_canister_id = env
        .create_canister_with_id(
            Some(controller),
            None,
            specified_nns_exchange_rate_canister_id,
        )
        .unwrap();
    assert_eq!(
        nns_exchange_rate_canister_id,
        specified_nns_exchange_rate_canister_id
    );

    let nns_governance_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let nns_cycles_ledger_canister_id =
        Principal::from_text("um5iw-rqaaa-aaaaq-qaaba-cai").unwrap();

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

    let cmc_canister_wasm = get_canister_wasm("cmc").to_vec();
    let cmc_init_args: Option<CyclesCanisterInitPayload> = Some(CyclesCanisterInitPayload {
        ledger_canister_id: Some(nns_ledger_canister_id),
        governance_canister_id: Some(nns_governance_canister_id),
        minting_account_id: None,
        exchange_rate_canister: Some(ExchangeRateCanister::Set(nns_exchange_rate_canister_id)),
        cycles_ledger_canister_id: Some(nns_cycles_ledger_canister_id),
        last_purged_notification: Some(0),
    });
    env.install_canister(
        cmc_canister_id,
        cmc_canister_wasm,
        Encode!(&cmc_init_args).unwrap(),
        Some(controller),
    );
    // set default (application) subnets on CMC
    // by setting authorized subnets associated with no principal (CMC API)
    let application_subnet_id = env.topology().get_app_subnets()[0];
    let set_authorized_subnetwork_list_args = SetAuthorizedSubnetworkListArgs {
        who: None,
        subnets: vec![application_subnet_id],
    };
    update_candid_as::<_, ((),)>(
        env,
        cmc_canister_id,
        nns_governance_canister_id,
        "set_authorized_subnetwork_list",
        (set_authorized_subnetwork_list_args,),
    )
    .unwrap();
    // add fiduciary subnet to CMC
    let update_subnet_type_args = UpdateSubnetTypeArgs::Add("fiduciary".to_string());
    update_candid_as::<_, ((),)>(
        env,
        cmc_canister_id,
        nns_governance_canister_id,
        "update_subnet_type",
        (update_subnet_type_args,),
    )
    .unwrap();
    let fiduciary_subnet_id = env.topology().get_fiduciary().unwrap();
    let change_subnet_type_assignment_args =
        ChangeSubnetTypeAssignmentArgs::Add(SubnetListWithType {
            subnets: vec![fiduciary_subnet_id],
            subnet_type: "fiduciary".to_string(),
        });
    update_candid_as::<_, ((),)>(
        env,
        cmc_canister_id,
        nns_governance_canister_id,
        "change_subnet_type_assignment",
        (change_subnet_type_assignment_args,),
    )
    .unwrap();

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

    let station_init_args = SystemInstallArg::Init(SystemInitArg {
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
                status: None,
            }],
            admin_quorum: 1,
            operator_quorum: 1,
        },
    });
    env.install_canister(
        station,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(env, station, WALLET_ADMIN_USER);

    CanisterIds {
        icp_ledger: nns_ledger_canister_id,
        icp_index: cmc_canister_id,
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
