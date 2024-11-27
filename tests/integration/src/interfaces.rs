use candid::{CandidType, Encode, Principal};
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs, TransferError,
    DEFAULT_SUBACCOUNT,
};
use pocket_ic::{query_candid_as, update_candid_as, PocketIc};
use std::collections::{HashMap, HashSet};

use crate::setup::{create_canister_with_cycles, get_canister_wasm};

#[derive(CandidType)]
pub enum NnsLedgerCanisterPayload {
    Init(NnsLedgerCanisterInitPayload),
}

#[derive(CandidType)]
pub struct NnsLedgerCanisterInitPayload {
    pub minting_account: String,
    pub initial_values: HashMap<String, Tokens>,
    pub send_whitelist: HashSet<Principal>,
    pub transfer_fee: Option<Tokens>,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
}

#[derive(CandidType)]
pub struct NnsIndexCanisterInitPayload {
    pub ledger_id: Principal,
}

pub fn get_icp_balance(env: &PocketIc, user_id: Principal) -> u64 {
    let ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let account = AccountIdentifier::new(&user_id, &DEFAULT_SUBACCOUNT);
    let account_balance_args = AccountBalanceArgs { account };
    let res: (Tokens,) = update_candid_as(
        env,
        ledger_canister_id,
        user_id,
        "account_balance",
        (account_balance_args,),
    )
    .unwrap();
    res.0.e8s()
}

pub fn get_icp_account_balance(env: &PocketIc, account_id: AccountIdentifier) -> u64 {
    let ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let account_balance_args = AccountBalanceArgs {
        account: account_id,
    };
    let res: (Tokens,) = update_candid_as(
        env,
        ledger_canister_id,
        Principal::anonymous(),
        "account_balance",
        (account_balance_args,),
    )
    .unwrap();
    res.0.e8s()
}

pub const ICP: u64 = 100_000_000; // in e8s
pub const ICP_FEE: u64 = 10_000; // in e8s

pub fn default_account(user_id: Principal) -> String {
    AccountIdentifier::new(&user_id, &DEFAULT_SUBACCOUNT).to_hex()
}

pub fn send_icp_to_account(
    env: &PocketIc,
    sender_id: Principal,
    beneficiary_account: AccountIdentifier,
    e8s: u64,
    memo: u64,
    from_subaccount: Option<Subaccount>,
    fee: Option<u64>,
) -> Result<u64, TransferError> {
    let ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let transfer_args = TransferArgs {
        memo: Memo(memo),
        amount: Tokens::from_e8s(e8s),
        fee: Tokens::from_e8s(fee.unwrap_or(10_000)),
        from_subaccount,
        to: beneficiary_account,
        created_at_time: None,
    };
    let res: (Result<u64, TransferError>,) = update_candid_as(
        env,
        ledger_canister_id,
        sender_id,
        "transfer",
        (transfer_args,),
    )
    .unwrap();
    res.0
}

pub fn send_icp(
    env: &PocketIc,
    sender_id: Principal,
    beneficiary_id: Principal,
    e8s: u64,
    memo: u64,
) -> Result<u64, TransferError> {
    let to = AccountIdentifier::new(&beneficiary_id, &DEFAULT_SUBACCOUNT);
    send_icp_to_account(env, sender_id, to, e8s, memo, None, None)
}

pub fn mint_icp(
    env: &PocketIc,
    minter_id: Principal,
    to: &AccountIdentifier,
    e8s: u64,
) -> Result<u64, TransferError> {
    send_icp_to_account(env, minter_id, *to, e8s, 0, None, Some(0))
}

#[derive(CandidType)]
pub struct Icrc1LedgerInitArgs {
    pub minting_account: icrc_ledger_types::icrc1::account::Account,
    pub fee_collector_account: Option<icrc_ledger_types::icrc1::account::Account>,
    pub initial_balances: Vec<(icrc_ledger_types::icrc1::account::Account, candid::Nat)>,
    pub transfer_fee: candid::Nat,
    pub decimals: Option<u8>,
    pub token_name: String,
    pub token_symbol: String,
    pub metadata: Vec<(
        String,
        icrc_ledger_types::icrc::generic_metadata_value::MetadataValue,
    )>,
    pub archive_options: ArchiveOptions,
    pub max_memo_length: Option<u16>,
    pub feature_flags: Option<FeatureFlags>,
    pub maximum_number_of_accounts: Option<u64>,
    pub accounts_overflow_trim_quantity: Option<u64>,
}

#[derive(CandidType)]
pub struct ArchiveOptions {
    pub trigger_threshold: usize,
    pub num_blocks_to_archive: usize,
    pub node_max_memory_size_bytes: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub controller_id: Principal,
    pub more_controller_ids: Option<Vec<Principal>>,
    pub cycles_for_archive_creation: Option<u64>,
    pub max_transactions_per_response: Option<u64>,
}

#[derive(CandidType)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(CandidType)]
pub enum Icrc1LedgerArgument {
    Init(Icrc1LedgerInitArgs),
}

pub fn deploy_icrc1_token(
    env: &mut PocketIc,
    controller: Principal,
    init: Icrc1LedgerInitArgs,
) -> Principal {
    let wasm_module = get_canister_wasm("icrc1_ledger").to_vec();

    let canister_id = create_canister_with_cycles(env, controller, 1_000_000_000_000);

    env.install_canister(
        canister_id,
        wasm_module,
        Encode!(&Icrc1LedgerArgument::Init(init)).unwrap(),
        Some(controller),
    );

    canister_id
}

pub fn mint_icrc1_tokens(
    env: &PocketIc,
    ledger_id: Principal,
    minter: Principal,
    to: icrc_ledger_types::icrc1::account::Account,
    amount: u64,
) -> Result<
    icrc_ledger_types::icrc1::transfer::BlockIndex,
    icrc_ledger_types::icrc1::transfer::TransferError,
> {
    let res: (
        Result<
            icrc_ledger_types::icrc1::transfer::BlockIndex,
            icrc_ledger_types::icrc1::transfer::TransferError,
        >,
    ) = update_candid_as(
        env,
        ledger_id,
        minter,
        "icrc1_transfer",
        (icrc_ledger_types::icrc1::transfer::TransferArg {
            from_subaccount: None,
            to,
            fee: None,
            created_at_time: None,
            memo: None,
            amount: amount.into(),
        },),
    )
    .expect("Failed to make update call");

    res.0
}

pub fn get_icrc1_balance_of(
    env: &PocketIc,
    ledger_id: Principal,
    account: icrc_ledger_types::icrc1::account::Account,
) -> candid::Nat {
    let res: (candid::Nat,) = query_candid_as(
        env,
        ledger_id,
        Principal::anonymous(),
        "icrc1_balance_of",
        (account,),
    )
    .expect("Failed to make query call");

    res.0
}
