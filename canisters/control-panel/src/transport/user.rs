use super::{UserBankDTO, UserIdentityDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_bank: Option<Principal>,
    pub banks: Vec<UserBankDTO>,
    pub identities: Vec<UserIdentityDTO>,
    pub unconfirmed_identities: Vec<UserIdentityDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetUserResponse {
    pub user: UserDTO,
}
