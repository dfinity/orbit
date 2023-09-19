use crate::entities::{Bank, BankID};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BankInput {
    pub id: BankID,
    pub name: Option<String>,
    pub main: Option<bool>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ManageBanksInput {
    Append(Vec<BankInput>),
    Override(Vec<BankInput>),
    Remove(Vec<BankID>),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageBanksResponse {
    pub banks: Vec<Bank>,
}
