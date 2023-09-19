use crate::{core::UUID, entities::Account};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountInput {
    pub account_id: UUID,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithAccountResponse {
    pub account: Option<Account>,
}
