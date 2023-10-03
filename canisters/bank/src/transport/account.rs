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
