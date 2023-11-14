use super::{UserIdentityDTO, UserWalletDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserDTO {
    pub id: String,
    pub name: Option<String>,
    pub main_wallet: Option<Principal>,
    pub wallets: Vec<UserWalletDTO>,
    pub identities: Vec<UserIdentityDTO>,
    pub unconfirmed_identities: Vec<UserIdentityDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetUserResponse {
    pub user: UserDTO,
}
