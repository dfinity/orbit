use super::UserWalletDTO;
use crate::UserSubscriptionStatusDTO;
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

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateWaitingListInput {
    pub users: Vec<Principal>,
    pub new_status: UserSubscriptionStatusDTO,
}
