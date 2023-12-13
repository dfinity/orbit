use crate::{CriteriaDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountDTO {
    pub id: UuidDTO,
    pub owners: Vec<UuidDTO>,
    pub name: String,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub symbol: String,
    pub decimals: u32,
    pub policies: AccountPoliciesDTO,
    pub balance: Option<AccountBalanceInfoDTO>,
    pub metadata: Vec<(String, String)>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountPoliciesDTO {
    pub transfer: Option<CriteriaDTO>,
    pub edit: Option<CriteriaDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountOperationInput {
    pub account_id: UuidDTO,
    pub owners: Option<Vec<UuidDTO>>,
    pub policies: Option<AccountPoliciesDTO>,
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountOperationDTO {
    pub input: EditAccountOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub owners: Vec<UuidDTO>,
    pub policies: AccountPoliciesDTO,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<(String, String)>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccountOperationDTO {
    pub account: Option<AccountDTO>,
    pub input: AddAccountOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ApprovalThresholdPolicyDTO {
    VariableThreshold(u8),
    FixedThreshold(u8),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountInput {
    pub account_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountResponse {
    pub account: AccountDTO,
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
pub struct ListAccountResponse {
    pub accounts: Vec<AccountDTO>,
}
