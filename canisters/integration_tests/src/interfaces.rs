use candid::{CandidType, Deserialize, Principal};
use ic_ledger_types::Tokens;
use std::collections::{HashMap, HashSet};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DefaultWalletInit {
    InitSharedWalletCanister,
    SpecifiedWalletCanister(Principal),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ControlPanelCanisterInit {
    pub default_wallet: DefaultWalletInit,
}

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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpgraderInitArg {
    pub target_canister: Principal,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterUserInput {
    pub identities: Vec<Principal>,
}

pub type TimestampRfc3339 = String;
pub type UserIdDTO = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UserRoleDTO {
    Admin = 0,
    User = 1,
    Guest = 2,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub id: UserIdDTO,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
    pub access_roles: Vec<UserRoleDTO>,
    pub last_modification_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
