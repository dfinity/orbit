use super::{Account, BankID, ServiceError};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegisterAccountMainBankInput {
    PrivateBank(BankID),
    SharedBank,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountInput {
    pub name: Option<String>,
    pub main_bank: RegisterAccountMainBankInput,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterAccountResultData {
    pub account: Account,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegisterAccountResult {
    Data(RegisterAccountResultData),
    Errors(Vec<ServiceError>),
}
