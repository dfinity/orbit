use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

pub type AccountIdDTO = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum AccountRoleDTO {
    Admin = 0,
    User = 1,
    Guest = 2,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountDTO {
    pub id: AccountIdDTO,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
    pub access_roles: Vec<AccountRoleDTO>,
    pub last_modification_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterAccountInput {
    pub identities: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterAccountResponse {
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountInput {
    pub account_id: AccountIdDTO,
    pub identities: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccountResponse {
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountInput {
    pub account_id: Option<AccountIdDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccountResponse {
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConfirmAccountInput {
    pub account_id: AccountIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConfirmAccountResponse {
    pub account: AccountDTO,
}
