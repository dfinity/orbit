use super::{Account, ServiceError};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountInfoResultData {
    pub account: Account,
}

/// The result of managing an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum AccountInfoResult {
    /// The account that was managed if successful.
    Data(AccountInfoResultData),
    /// The errors that occurred if unsuccessful.
    Errors(Vec<ServiceError>),
}
