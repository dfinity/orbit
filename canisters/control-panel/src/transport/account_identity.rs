use super::AccountDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentityDTO {
    pub identity: Principal,
    pub account_id: String,
    pub name: Option<String>,
    pub status: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountInput {
    pub account_id: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountResponse {
    pub account: Option<AccountDTO>,
}
