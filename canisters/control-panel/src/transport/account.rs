use super::{AccountBankDTO, AccountIdentityDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_bank: Option<Principal>,
    pub banks: Vec<AccountBankDTO>,
    pub identities: Vec<AccountIdentityDTO>,
    pub unconfirmed_identities: Vec<AccountIdentityDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetAccountResponse {
    pub account: AccountDTO,
}
