use crate::{
    AllowDTO, MetadataDTO, PaginationInput, RequestPolicyRuleDTO, RequestPolicyRuleInput, UuidDTO,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AccountCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_transfer: bool,
    pub can_edit: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AccountDTO {
    pub id: UuidDTO,
    pub name: String,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub symbol: String,
    pub decimals: u32,
    pub balance: Option<AccountBalanceInfoDTO>,
    pub metadata: Vec<MetadataDTO>,
    pub transfer_request_policy: Option<RequestPolicyRuleDTO>,
    pub configs_request_policy: Option<RequestPolicyRuleDTO>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditAccountOperationInput {
    pub account_id: UuidDTO,
    pub name: Option<String>,
    pub read_permission: Option<AllowDTO>,
    pub configs_permission: Option<AllowDTO>,
    pub transfer_permission: Option<AllowDTO>,
    pub configs_request_policy: Option<RequestPolicyRuleInput>,
    pub transfer_request_policy: Option<RequestPolicyRuleInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditAccountOperationDTO {
    pub input: EditAccountOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<MetadataDTO>,
    pub read_permission: AllowDTO,
    pub configs_permission: AllowDTO,
    pub transfer_permission: AllowDTO,
    pub configs_request_policy: Option<RequestPolicyRuleDTO>,
    pub transfer_request_policy: Option<RequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddAccountOperationDTO {
    pub account: Option<AccountDTO>,
    pub input: AddAccountOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountInput {
    pub account_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountResponse {
    pub account: AccountDTO,
    pub privileges: AccountCallerPrivilegesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct FetchAccountBalancesInput {
    pub account_ids: Vec<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceDTO {
    pub account_id: String,
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceInfoDTO {
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct FetchAccountBalancesResponse {
    pub balances: Vec<AccountBalanceDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListAccountsInput {
    pub search_term: Option<String>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<AccountCallerPrivilegesDTO>,
}
