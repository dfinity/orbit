use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_bank: Option<Principal>,
    pub banks: Vec<Principal>,
    pub identities: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountInfoResponse {
    pub account: AccountDTO,
}
