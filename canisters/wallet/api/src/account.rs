use crate::{AllowDTO, CriteriaDTO, MetadataDTO, PaginationInput, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_transfer: bool,
    pub can_edit: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
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
    pub transfer_approval_policy: CriteriaDTO,
    pub update_approval_policy: CriteriaDTO,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountOperationInput {
    pub account_id: UuidDTO,
    pub name: Option<String>,
    pub read_access_policy: Option<AllowDTO>,
    pub update_access_policy: Option<AllowDTO>,
    pub transfer_access_policy: Option<AllowDTO>,
    pub update_approval_policy: Option<CriteriaDTO>,
    pub transfer_approval_policy: Option<CriteriaDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountOperationDTO {
    pub input: EditAccountOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<MetadataDTO>,
    pub read_access_policy: AllowDTO,
    pub update_access_policy: AllowDTO,
    pub transfer_access_policy: AllowDTO,
    pub update_approval_policy: CriteriaDTO,
    pub transfer_approval_policy: CriteriaDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccountOperationDTO {
    pub account: Option<AccountDTO>,
    pub input: AddAccountOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountInput {
    pub account_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountResponse {
    pub account: AccountDTO,
    pub privileges: AccountCallerPrivilegesDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct FetchAccountBalancesInput {
    pub account_ids: Vec<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountBalanceDTO {
    pub account_id: String,
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountBalanceInfoDTO {
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct FetchAccountBalancesResponse {
    pub balances: Vec<AccountBalanceDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountsInput {
    pub search_term: Option<String>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<AccountCallerPrivilegesDTO>,
}
