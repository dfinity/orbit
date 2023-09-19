use crate::entities::{Account, AccountID};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountInput {
    pub account_id: AccountID,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountResponse {
    pub account: Option<Account>,
}
