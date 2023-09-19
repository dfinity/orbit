use crate::entities::Bank;
use candid::{CandidType, Deserialize};

pub type BankListItem = Bank;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListBanksResponse {
    pub banks: Vec<BankListItem>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetMainBankResponse {
    pub bank: Bank,
}
