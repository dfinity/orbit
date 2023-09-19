use crate::{core::PrincipalID, entities::Bank};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BankInput {
    pub id: PrincipalID,
    pub name: Option<String>,
    pub main: Option<bool>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ManageBanksInput {
    Append(Vec<BankInput>),
    Override(Vec<BankInput>),
    Remove(Vec<PrincipalID>),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageBanksResponse {
    pub banks: Vec<Bank>,
}
