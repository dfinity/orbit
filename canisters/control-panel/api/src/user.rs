use super::UserWalletDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserDTO {
    pub id: Principal,
    pub main_wallet: Option<Principal>,
    pub wallets: Vec<UserWalletDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetUserResponse {
    pub user: UserDTO,
}
