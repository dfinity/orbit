use super::{AccountBankDTO, AccountIdentityDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_bank: Option<Principal>,
    pub banks: Vec<Principal>,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountDetailsDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_bank: Option<Principal>,
    pub banks: Vec<AccountBankDTO>,
    pub identities: Vec<AccountIdentityDTO>,
    pub unconfirmed_identities: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountDetailsResponse {
    pub account_details: Option<AccountDetailsDTO>,
}
