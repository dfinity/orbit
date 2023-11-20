use super::UserIdDTO;
use candid::{CandidType, Deserialize};

pub type AccountIdDTO = String;
pub type UuidDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountDTO {
    pub id: AccountIdDTO,
    pub owners: Vec<UuidDTO>,
    pub name: Option<String>,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub symbol: String,
    pub decimals: u32,
    pub balance: Option<AccountBalanceInfoDTO>,
    pub policies: Vec<PolicyDTO>,
    pub metadata: Vec<(String, String)>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountEditOperationInput {
    pub account_id: AccountIdDTO,
    pub owners: Option<Vec<UserIdDTO>>,
    pub policies: Option<Vec<PolicyDTO>>,
    pub name: Option<String>,
}

pub type AccountEditOperationDTO = AccountEditOperationInput;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ApprovalThresholdPolicyDTO {
    VariableThreshold(u8),
    FixedThreshold(u8),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum PolicyDTO {
    #[serde(rename = "approval_threshold")]
    ApprovalThreshold(ApprovalThresholdPolicyDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateAccountInput {
    pub owners: Vec<UserIdDTO>,
    pub name: Option<String>,
    pub blockchain: String,
    pub standard: String,
    pub policies: Vec<PolicyDTO>,
    pub metadata: Option<Vec<(String, String)>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateAccountResponse {
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountInput {
    pub account_id: AccountIdDTO,
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
