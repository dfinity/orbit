use super::AccountBankDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BankInput {
    pub id: Principal,
    pub name: Option<String>,
    pub main: Option<bool>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ManageBanksInput {
    Append(Vec<BankInput>),
    Override(Vec<BankInput>),
    Remove(Vec<Principal>),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageBanksResponse {
    pub banks: Vec<AccountBankDTO>,
}
