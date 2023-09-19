use crate::{
    core::PrincipalID,
    entities::{Account, AccountIdentity},
};
use candid::{CandidType, Deserialize};

/// The input to manage an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageAccountInput {
    /// The name to give the account.
    pub name: Option<String>,
    /// The main bank to use for the account.
    pub bank: Option<PrincipalID>,
    /// Whether to use a shared bank for the account.
    pub use_shared_bank: Option<bool>,
    /// The identities to associate with the account.
    pub identities: Option<Vec<AccountIdentity>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageAccountResponse {
    pub account: Account,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteAccountResponse {
    pub account: Account,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegisterAccountMainBankInput {
    PrivateBank(PrincipalID),
    SharedBank,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountInput {
    pub name: Option<String>,
    pub main_bank: RegisterAccountMainBankInput,
    pub use_shared_bank: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountResponse {
    pub account: Account,
}
