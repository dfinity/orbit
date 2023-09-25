use super::AccountDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentityDTO {
    pub identity: Principal,
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountInput {
    pub account_id: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountResponse {
    pub account: AccountDTO,
}
