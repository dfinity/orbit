use candid::{CandidType, Principal};
use ic_ledger_types::Tokens;
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
