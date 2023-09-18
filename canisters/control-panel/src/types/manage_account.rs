use super::{Account, AccountIdentity, BankID, ServiceError};
use candid::{CandidType, Deserialize};

/// The input to manage an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ManageAccountInput {
    /// The name to give the account.
    pub name: Option<String>,
    /// The main bank to use for the account.
    pub bank: Option<BankID>,
    /// Whether to use a shared bank for the account.
    pub use_shared_bank: Option<bool>,
    /// The identities to associate with the account.
    pub identities: Option<Vec<AccountIdentity>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageAccountResultData {
    pub account: Account,
}

/// The result of managing an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ManageAccountResult {
    /// The account that was managed if successful.
    Data(ManageAccountResultData),
    /// The errors that occurred if unsuccessful.
    Errors(Vec<ServiceError>),
}
