use super::{AccountBankDTO, AccountDTO, AccountDetailsDTO, AccountIdentityDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageAccountInput {
    /// The name to give the account.
    pub name: Option<String>,
    /// The main bank to use for the account.
    pub main_bank: Option<Principal>,
    /// The operation to perform on the banks.
    pub banks: Option<Vec<AccountBankDTO>>,
    /// The identities to associate with the account.
    pub identities: Option<Vec<AccountIdentityDTO>>,
    /// The unconfirmed identities to associate with the account.
    pub unconfirmed_identities: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageAccountResponse {
    pub account_details: AccountDetailsDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteAccountResponse {
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountBankSharedInput {
    pub is_main: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegisterAccountBankInput {
    PrivateBank {
        id: Principal,
        use_shared_bank: Option<RegisterAccountBankSharedInput>,
    },
    SharedBank,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountInput {
    pub name: Option<String>,
    pub bank: RegisterAccountBankInput,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountResponse {
    pub account: AccountDTO,
}
