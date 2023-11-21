use candid::{CandidType, Principal};
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Tokens, TransferArgs, TransferError,
    DEFAULT_SUBACCOUNT,
};
use pocket_ic::{call_candid_as, PocketIc};
use std::collections::{HashMap, HashSet};

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
    let res: (Tokens,) = call_candid_as(
        env,
        ledger_canister_id,
        user_id,
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
) -> Result<u64, TransferError> {
    let ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let transfer_args = TransferArgs {
        memo: Memo(memo),
        amount: Tokens::from_e8s(e8s),
        fee: Tokens::from_e8s(10_000),
        from_subaccount: None,
        to: beneficiary_account,
        created_at_time: None,
    };
    let res: (Result<u64, TransferError>,) = call_candid_as(
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
    send_icp_to_account(env, sender_id, to, e8s, memo)
}
