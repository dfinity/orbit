use crate::entities::Account;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountInfoResponse {
    pub account: Account,
}
