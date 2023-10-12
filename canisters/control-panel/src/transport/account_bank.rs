use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountBankDTO {
    pub canister_id: Principal,
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListBanksResponse {
    pub banks: Vec<AccountBankDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetMainBankResponse {
    pub bank: Option<AccountBankDTO>,
}
