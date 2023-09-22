use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountBankDTO {
    pub canister_id: Principal,
    pub name: Option<String>,
}

pub type BankListItem = AccountBankDTO;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListBanksResponse {
    pub banks: Vec<BankListItem>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetMainBankResponse {
    pub bank: Option<AccountBankDTO>,
}
